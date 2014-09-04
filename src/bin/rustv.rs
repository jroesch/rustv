#![feature(phase)]
extern crate rustv;
extern crate serialize;
extern crate term;
#[phase(plugin, link)] extern crate log;
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
       rustv refresh
       rustv versions
       rustv (-h | --help | --version)

Options:
    --version      Print the version.
    -h, --help     Print this message.
")

#[deriving(Decodable, Show)]
enum Command { Install, Which, Global, Refresh, Versions }

fn get_command(args: &Args) -> Command {
  if args.cmd_install {
    Install
  } else if args.cmd_which {
    Which
  } else if args.cmd_global {
    Global
  } else if args.cmd_refresh {
    Refresh
  } else if args.cmd_versions {
    Versions
  } else  {
    fail!("unsupported command")
  }
}

fn main() {
  debug!("Starting ...");
  let mut conf = docopt::DEFAULT_CONFIG.clone();
  conf.version = Some("rustv pre-0.0.1".to_string());
  let arguments: Args = FlagParser::parse_conf(conf).unwrap_or_else(|e| e.exit());
  let mut rustv = Rustv::setup();
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
      rustv.change_version(version.as_slice()).unwrap_or_else(|err| {
        println!("rustv failed: when changing to version {} {}", version, err);
      });
    },
    Refresh => {
      rustv.refresh().unwrap();
    },
    Versions => {
      println!("Running command: versions")
      rustv.versions().unwrap();
    }
  };
  //installation.activate_version(&installation.current_version);
}
