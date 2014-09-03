#![crate_name="rustv"]
#![crate_type="rlib"]
#![feature(phase)]

#[phase(plugin, link)] extern crate log;
extern crate toml;
extern crate serialize;
extern crate http;
extern crate url;

use std::io;
use std::io::{File, fs, IoResult};
use std::os::{getenv};
use std::io::process;
use std::collections::HashMap;
use version::Version;

static RUSTV_ENV_NAME: &'static str = "RUSTV_PATH";
pub static RUSTV_DIR_NAME: &'static str = ".rustv";
static HOME_NOT_FOUND: &'static str =
  "Could not locate active rustv installation or HOME environment variable.";
pub static DOWNLOAD_CACHE_DIR: &'static str = "dl_cache";

pub mod shell;
pub mod version;

pub fn locate_installation_directory() -> Path {
  match getenv(RUSTV_ENV_NAME) {
    None => match getenv("HOME") {
      None => fail!(HOME_NOT_FOUND),
      Some(path) => Path::new(path).join(".rustv")
    },
    Some(path) => Path::new(path)
  }
}

pub struct Rustv {
  pub root: Path,
  pub current_version: Path,
  versions: HashMap<String, Version>
}

trait StringUtil {
  fn chomp(&self) -> String;
}

impl StringUtil for String {
  fn chomp(&self) -> String {
    let mut result = self.clone();
    for c in result.as_slice().chars().rev() {
      if c == '\n' || c == '\r' {
        result.pop_char();
      }
    }
    result
  }
}

fn load_versions(root: &Path) -> HashMap<String, Version> {
  let toml = File::open(&root.join("versions.toml")).read_to_string().unwrap();
  let table = toml::Parser::new(toml.as_slice()).parse().unwrap();
  let versions = table.find(&"version".to_string()).unwrap();
  let versions: Vec<Version> = toml::decode(versions.clone()).unwrap();
  let mut hash_map = HashMap::new();
  for version in versions.move_iter() {
    hash_map.insert(version.name.clone(), version);
  };
  hash_map
}

fn create_rustv_directory(root: &Path) -> IoResult<()>{
  println!("Setting up .rustv");
  try!(fs::mkdir(root, io::UserRWX))
  try!(fs::mkdir(&root.join("bin"), io::UserRWX))
  try!(fs::mkdir(&root.join("versions"), io::UserRWX));
  fs::mkdir(&root.join("dl_cache"), io::UserRWX).unwrap();
  Ok(())
}

// parse toml listing allow for multiple entries
// only load strcture when installation is going to happen
// have a heuristic for selecting each version
// check versisions and build
// allow for update to fetch a new listing
impl Rustv {
  pub fn install_path_for(&self, version: &str) -> Path {
    self.root.join("versions").join(version)
  }

  pub fn new(root: &Path) -> Rustv {
    let directory_exists = root.exists();

    println!("Above here! with path as {}", root.display());

    if !directory_exists {
      println!("Doing Rustv init")
      create_rustv_directory(root).unwrap()
    }

    let version = Rustv::read_current_version(root);
    let current_version = root.join("versions").join(version.as_slice());
    println!("Loading versions ...");
    let versions = load_versions(root);
    println!("Loaded versions.")

    Rustv {
      root: root.clone(),
      current_version: current_version,
      versions: versions
    }
  }

  pub fn setup() -> Rustv {
    println!("Invoking setup ...");
    Rustv::new(&locate_installation_directory())
  }

  fn read_current_version(root: &Path) -> String {
    let current_version = &root.join("current_version");
    if current_version.exists() {
      File::open(&root.join("current_version")).read_to_string().unwrap().chomp()
    } else {
      "system".to_string()
    }
  }

  /// Place symbolic link to the requested version in the installation
  /// directory.

  fn detect_system_rust() {
    fail!("Not yet implemented: detect_system_rust")
  }

  pub fn download_path(&self) -> Path {
    self.root.join(DOWNLOAD_CACHE_DIR)
  }

  pub fn versions(&self) -> IoResult<()> {
    for version in try!(fs::readdir(&self.root.join("versions"))).iter() {
      print!("{}", version.display());
      if version == &self.current_version { print!("*"); }
      println!("");
    }
    Ok(())
  }

  pub fn refresh(&self) -> IoResult<()> {
    let bin_dir = &self.current_version.join("bin");
    for exec in try!(fs::readdir(bin_dir)).iter() {
      println!("Generating binary shim for: {}", exec.display());
      try!(self.create_binary_shim(exec));
    }

    Ok(())
  }

  fn create_binary_shim(&self, exec_path: &Path) -> IoResult<()> {
    let rustv_path = format!("export RUSTV_PATH={}", self.root.display());
    let env_setup = "export DYLD_LIBRARY_PATH=$RUSTV_PATH/versions/`cat $RUSTV_PATH/current_version`/lib".to_string();
    let bin = format!("$RUSTV_PATH/versions/`cat $RUSTV_PATH/current_version`/bin/{} $@", exec_path.filename_str().unwrap());
    let contents = format!("#!/bin/sh\n{}\n{}\n{}", rustv_path, env_setup, bin);

    println!("{}", contents);

    match exec_path.filename_str() {
      None => fail!("I don't know why this would ever fail - Jared"),
      Some(file_name) => {
        let file_path = &self.root.join("bin").join(file_name);
        println!("Putting shim here: {}", file_path.display()); //.write(contents.as_bytes()));
        try!(File::create(file_path).write(contents.as_bytes()));
        try!(fs::chmod(file_path, io::UserExec))
      }
    }
    Ok(())
  }

  fn write_version_file(&self, version: &str) -> IoResult<()> {
    File::open_mode(&self.root.join("current_version"), io::Truncate, io::Write).write(version.as_bytes());
    fs::chmod(&self.root.join("current_version"), io::UserRWX)
  }

  pub fn change_version(&mut self, version: &str) -> IoResult<()> {
    self.current_version = self.install_path_for(version);
    self.write_version_file(version);
    // Both `lib` and `share` are easy just symlink to them.

    // Clean up
    let lib = &self.root.join("lib");
    let share = &self.root.join("share");

    try!(fs::unlink(lib));
    try!(fs::unlink(share))

    // Re-symlink
    let version_lib = &self.current_version.join("lib");
    let version_share = &self.current_version.join("share");

    try!(fs::symlink(version_lib, lib))
    try!(fs::symlink(version_share, share));

    try!(self.refresh());

    Ok(())
  }

  pub fn install(&self, version: &str, prefix: &str) {
    let mut command = process::Command::new("rustv-build");
    println!("executing: rustv-build {} {}", version, prefix);
    command.arg(version).arg(prefix);
    shell::Shell::new(command).block().unwrap()
  }

  pub fn version<'a>(&'a self, version_name: &String) -> &'a Version {
    self.versions.find(version_name).unwrap()
  }

  fn cache_path(&self) -> Path {
    self.root.join("cache")
  }

  pub fn which(&self, command: &str) {
    println!("{}", self.current_version.join("bin").join(command).display())
  }
}
