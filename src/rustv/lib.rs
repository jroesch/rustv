#![crate_name="rustv"]
#![crate_type="rlib"]

use std::io;
use std::io::{File, fs};
use std::os::{getenv};
use std::collections::hashmap::{HashMap};

static RUSTV_ENV_NAME: &'static str = "RUSTV_PATH";
static RUSTV_DIR_NAME: &'static str = ".rustv";
static HOME_NOT_FOUND: &'static str =
  "Could not locate active rustv installation or HOME environment variable.";

pub fn locate_installation_directory() -> Path {
  match getenv(RUSTV_ENV_NAME) {
    None => match getenv("HOME") {
      None => fail!(HOME_NOT_FOUND),
      Some(path) => Path::new(path)
    },
    Some(path) => Path::new(path)
  }
}

pub struct Rustv {
  pub root: Path,
  pub current_version: Path
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

impl Rustv {
  pub fn new(prefix: &Path) -> Rustv {
    let directory_exists = match fs::stat(prefix) {
      Err(_) => false,
      Ok(_) => true
    };

    if !directory_exists {
      Rustv::create_rustv_directory(prefix)
    }

    let current_version = Rustv::read_current_version(&prefix.join(RUSTV_DIR_NAME));

    Rustv {
      root: prefix.join(RUSTV_DIR_NAME),
      current_version: prefix.join(RUSTV_DIR_NAME).join("versions").join(current_version.as_slice())
    }
  }

  fn read_current_version(root: &Path) -> String {
    File::open(&root.join("current_version")).read_to_string().unwrap().chomp()
  }

  fn create_rustv_directory(prefix: &Path) {
    let root = &prefix.join(".rustv");
    fs::mkdir(root, io::UserRWX);
    fs::mkdir(&root.join("bin"), io::UserRWX);
    fs::mkdir(&root.join("versions"), io::UserRWX);
    println!("Setting up installation directory");
  }

  pub fn build_version_registry(root: &Path) -> HashMap<Path, Path> {
    let mut hash_map = HashMap::new();
    for subdir in fs::walk_dir(root).unwrap() {
      println!("Found subdirectory: {}", subdir.display())
    }
    hash_map
  }

  /// Place symbolic link to the requested version in the installation
  /// directory.
  pub fn activate_version(&self, version: &Path) {
    // Both `lib` and `share` are easy just symlink to them.

    // Clean up
    let lib = &self.root.join("lib");
    let share = &self.root.join("share");

    fs::unlink(lib);
    fs::unlink(share);

    // Re-symlink
    let version_lib = &version.join("lib");
    let version_share = &version.join("share");

    fs::symlink(version_lib, lib);
    fs::symlink(version_share, share);

    // Handle Binary here
    let bin_dir = &version.join("bin");
    for exec in fs::walk_dir(bin_dir).unwrap() {
      println!("Generating shim for: {}", exec.display());
      self.create_binary_shim(&exec);
    }
  }

  fn insert_bin_shim() {
    println!("shim!");
  }

  fn detect_system_rust() {
    fail!("Not yet implemented: detect_system_rust")
  }

  pub fn create_binary_shim(&self, exec_path: &Path) {
    let lib_path = self.current_version.join("lib");
    let env_setup = format!("DYLD_LIBRARY_PATH={}", lib_path.display());
    let contents = format!("#!/bin/sh\n{} {} $@", env_setup, exec_path.display());

    match exec_path.filename_str() {
      None => fail!("I don't know why this would ever fail - Jared"),
      Some(file_name) => {
        let file_path = &self.root.join("bin").join(file_name);
        println!("Putting shim here: {}", file_path.display()); //.write(contents.as_bytes()));
        File::create(file_path).write(contents.as_bytes());
        fs::chmod(file_path, io::UserExec);
      }
    }
  }
}
