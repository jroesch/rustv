#![feature(phase, macro_rules)]
#![allow(non_snake_case)]
#![experimental]
extern crate rustv;
extern crate serialize;
extern crate term;
#[phase(plugin, link)] extern crate log;
#[phase(plugin)] extern crate docopt_macros;
extern crate docopt;

use std::io::{IoResult};
use docopt::FlagParser;
use term::{Terminal, stdout, color};
use rustv::Rustv;
use rustv::version::{Source, Binary};

docopt!(Args, "
rustv-build
Usage: rustv-build [-v | --source] VERSION PREFIX
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
  let rustv = Rustv::setup();
  let version = rustv.version(&version_name);
  debug!("About to Install! source: {}", arguments.flag_source);
  let source = if arguments.flag_source { Source } else { Binary };
  println!("{}", source);
  version.install(&rustv.download_path(), prefix_path, source).unwrap_or_else(|err| {
    fail!("rustv-build failed: {}", err);
  });
}

fn print_install_message(prefix: &String, version_name: &String) -> IoResult<()> {
  let mut term = stdout().unwrap();
  try!(term.fg(color::BRIGHT_GREEN));
  try!(term.write_str("Installing: "));
  try!(term.reset());
  try!(term.write_str(format!("{} to {}\n", version_name, prefix).as_slice()));
  Ok(())
}
