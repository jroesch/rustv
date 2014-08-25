#![feature(phase)]
extern crate rustv;
extern crate serialize;
extern crate term;
#[phase(plugin)] extern crate docopt_macros;
extern crate docopt;

use std::os;
use std::c_str::{CString};
use std::io::process;
use docopt::FlagParser;
use term::{Terminal, stdout, color};
use term::terminfo::{TerminfoTerminal};

docopt!(Args, "
Rust-Build.
Usage: rust-build [-a] VERSION PREFIX
       rust-build (-h | --help)


Options:
    --version      Print the version.
    -v, --verbose  Verbose build output.
    -h, --help     Print this message.
")

fn main() {
  let arguments: Args = FlagParser::parse().unwrap_or_else(|e| e.exit());
  let version_name = arguments.arg_VERSION;
  let prefix = arguments.arg_PREFIX;
  let mut term = stdout().unwrap();
  term.fg(color::BRIGHT_GREEN);
  term.write_str("Installing: ");
  term.reset();
  term.write_str(format!("{} to {}\n", version_name, prefix).as_slice());
  build_rust_in(&Path::new("/Users/jroesch/Git/rust"), &Path::new(prefix), version_name);
}

// install a version of rust
// simple version simply checks out the right rev and builds it
// we also want to be able to deal with tagged versions
// support binary installs (for releases)?
fn build_rust_in(build_path: &Path, prefix: &Path, version_name: String) {
  os::change_dir(build_path);
  simple_exec(
    build_path.join("configure"),
    &[Path::new(format!("--prefix={}", prefix.display()))]
  );
  simple_exec("make", &[]);
  simple_exec("make", &["install"]);
}

/* this is a crap hack-y ass piece of code to get off the ground */
fn simple_exec<T: ToCStr>(cmd: T, args: &[T]) -> String {
  let formatted_args: Vec<CString> = FromIterator::from_iter(args.iter().map(|arg| arg.to_c_str()));
  println!("Running {} with the args: {}", cmd.to_c_str(), formatted_args);
  let mut cmd = process::Command::new(cmd);
  cmd.args(args);
  let result = cmd.output().unwrap();
  String::from_utf8(result.output).unwrap()
}
