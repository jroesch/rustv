#![feature(phase)]
extern crate rustv;
extern crate serialize;
extern crate term;
#[phase(plugin)] extern crate docopt_macros;
extern crate docopt;
extern crate toml;

use std::io::{BufferedReader};
use std::os;
use std::c_str::{CString};
use std::io::process;
use docopt::FlagParser;
use term::{Terminal, stdout, color};
use term::terminfo::{TerminfoTerminal};
use rustv::{Rustv};

docopt!(Args, "
Rust's version manager
Usage: rustv install <version>
       rustv which   <command>
       rustv (-h | --help | --version)

Options:
    --version      Print the version.
    -h, --help     Print this message.
")

#[deriving(Decodable, Show)]
enum Command { Install, Which }

fn get_command(args: &Args) -> Command {
  if args.cmd_install {
    Install
  } else {
    Which
  }
}

fn main() {
  let mut conf = docopt::DEFAULT_CONFIG.clone();
  conf.version = Some("rustv pre-0.0.1".to_string());
  let arguments: Args = FlagParser::parse_conf(conf).unwrap_or_else(|e| e.exit());
  let rustv = Rustv::setup();
  //let versions = installation.load_versions();
  println!("{}", get_command(&arguments));
  match get_command(&arguments) {
    Install => {
      let version = &arguments.arg_version;
      let prefix_path = rustv.install_path_for(version.as_slice());
      rustv.install(
        arguments.arg_version.as_slice(),
        prefix_path.filename_str().unwrap()
      )
    },
    Which => rustv.which(&arguments.arg_command),
  };
  //installation.activate_version(&installation.current_version);
}
