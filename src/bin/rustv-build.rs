#![feature(phase, macro_rules)]
#![experimental]
extern crate rustv;
extern crate serialize;
extern crate term;
#[phase(plugin, link)] extern crate log;
#[phase(plugin)] extern crate docopt_macros;
extern crate docopt;

use std::io::{BufferedReader, IoResult};
use std::os;
use std::c_str::{CString};
use std::io::process;
use docopt::FlagParser;
use term::{Terminal, stdout, color};

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
  build_cargo_in(&Path::new("/Users/jroesch/Git/cargo"), prefix_path, version_name);
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
  simple_exec(
    build_path.join("configure"),
    &[Path::new(format!("--prefix={}", prefix.join(version_name).display()))]
  );

  let mut child = match process::Command::new("make").spawn() {
    Ok(child) => child,
    Err(e) => fail!("failed to execute make: {}", e)
  };

  let mut child_stdout = BufferedReader::new(child.stdout.clone().unwrap());

  for line in child_stdout.lines() {
    print!("{}", line.unwrap());
  }

  simple_exec("make", &["install"]);
}

fn build_cargo_in(build_path: &Path, prefix: &Path, version_name: String) {

}

/* this is a crap hack-y ass piece of code to get off the ground */
fn simple_exec<T: ToCStr>(cmd: T, args: &[T]) -> String {
  let formatted_args: Vec<CString> = FromIterator::from_iter(args.iter().map(|arg| arg.to_c_str()));
  debug!("Executing {} with the args: {}", cmd.to_c_str(), formatted_args);
  let mut cmd = process::Command::new(cmd);
  cmd.args(args);
  let result = cmd.output().unwrap();
  String::from_utf8(result.output).unwrap()
}
