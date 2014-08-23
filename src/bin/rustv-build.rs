#![feature(phase)]
extern crate rustv;
extern crate serialize;
extern crate term;
#[phase(plugin)] extern crate docopt_macros;
extern crate docopt;

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
  // install a version of rust
  // simple version simply checks out the right rev and builds it
  // we also want to be able to deal with tagged versions
  // support binary installs (for releases)?
}
