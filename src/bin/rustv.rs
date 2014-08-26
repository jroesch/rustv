#![feature(phase)]
extern crate rustv;
extern crate serialize;
extern crate term;
#[phase(plugin)] extern crate docopt_macros;
extern crate docopt;
extern crate toml;

use docopt::FlagParser;
use rustv::{Rustv};

docopt!(Args, "
Rust's version manager
Usage: rustv install <version>
       rustv which   <command>
       rustv global  <version>
       rustv (-h | --help | --version)

Options:
    --version      Print the version.
    -h, --help     Print this message.
")

#[deriving(Decodable, Show)]
enum Command { Install, Which, Global }

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
  let mut rustv = Rustv::setup();
  //let versions = installation.load_versions();
  println!("{}", get_command(&arguments));
  match get_command(&arguments) {
    Install => {
      let version = &arguments.arg_version;
      let prefix_path = rustv.install_path_for(version.as_slice());
      println!("{} {}", arguments.arg_version.as_slice(), format!("{}", prefix_path.display()));
      rustv.install(
        arguments.arg_version.as_slice(),
        format!("{}", prefix_path.display()).as_slice()
      )
    },
    Which => rustv.which(arguments.arg_command.as_slice()),
    Global => {
      let version = &arguments.arg_version;
      rustv.change_version(version.as_slice());
    }
  };
  //installation.activate_version(&installation.current_version);
}
