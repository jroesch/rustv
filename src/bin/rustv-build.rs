#![feature(phase, macro_rules)]
#![experimental]
extern crate rustv;
extern crate serialize;
extern crate term;
#[phase(plugin, link)] extern crate log;
#[phase(plugin)] extern crate docopt_macros;
extern crate docopt;

use std::io::{IoResult};
use std::os;
use std::io::process;
use docopt::FlagParser;
use term::{Terminal, stdout, color};
use rustv::shell::{Shell};

docopt!(Args, "
rustv-build
Usage: rustv-build [-v] VERSION PREFIX
       rustv-build (-h | --help | --version)

Options:
    --version      Print the version.
    -v, --verbose  Verbose build output.
    -h, --help     Print this message.
")

macro_rules! fail_with(
    ($e:expr) => (match $e { Ok(e) => e, Err(e)})
)

fn main() {
  debug!("Starting: rustv-build");
  let arguments: Args = FlagParser::parse().unwrap_or_else(|e| e.exit());
  let version_name = arguments.arg_VERSION;
  let prefix = arguments.arg_PREFIX;
  print_install_message(&prefix, &version_name).unwrap();
  let prefix_path = &Path::new(prefix);
  build_rust_in(&Path::new("/Users/jroesch/Git/rust"), prefix_path, version_name.clone());
  //build_cargo_in(&Path::new("/Users/jroesch/Git/cargo"), prefix_path, version_name);
}

fn print_install_message(prefix: &String, version_name: &String) -> IoResult<()> {
  let mut term = stdout().unwrap();
  try!(term.fg(color::BRIGHT_GREEN));
  try!(term.write_str("Installing: "));
  try!(term.reset());
  try!(term.write_str(format!("{} to {}\n", version_name, prefix).as_slice()));
  Ok(())
}

fn build_rust_in(build_path: &Path, prefix: &Path, version_name: String) {
  /* fetch a version of rust to build */
  os::change_dir(build_path);
  let mut configure = process::Command::new(build_path.join("configure"));
  configure.arg(Path::new(format!("--prefix={}", prefix.join(version_name).display())));
  //Shell::new(configure).block().unwrap();

  let make = process::Command::new("make");
  Shell::new(make).block().unwrap();

  let mut make_install = process::Command::new("make");
  make_install.arg("install");
  Shell::new(make_install).block().unwrap();
}

//fn build_cargo_in(build_path: &Path, prefix: &Path, version_name: String) {}
