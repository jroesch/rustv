#![crate_name="rustv"]
#![crate_type="rlib"]

use std::io;
use std::io::{File, fs};
use std::os::{getenv};
use std::collections::hashmap::{HashMap};

static RUSTV_ENV_NAME: &'static str = "RUSTV_PATH";
static RUSTV_DIR_NAME: &'static str = ".rustv";

pub fn locate_installation_directory() -> Path {
  let root = match getenv(RUSTV_ENV_NAME) {
    None => match getenv("HOME") {
      None => fail!("Can not locate the installation directory for rustv."),
      Some(path) => Path::new(path)
      },
      Some(path) => Path::new(path)
    };
  root.join(".rustv")
}

pub struct Rustv {
  pub root: Path,
}

impl Rustv {
  pub fn new(prefix: &Path) -> Rustv {
    Rustv { root: prefix.join(RUSTV_DIR_NAME) }
  }

  pub fn build_version_registry(root: &Path) -> HashMap<Path, Path> {
    let mut hash_map = HashMap::new();
    for subdir in fs::walk_dir(root).unwrap() {
      println!("Found subdirectory: {}", subdir.display())
    }
    hash_map
  }

  fn create_rustv_directory(prefix: &Path) {
    let root = &prefix.join(".rustv");
    fs::mkdir(root, io::UserRWX);
    fs::mkdir(&root.join("versions"), io::UserRWX);
    println!("Setting up installation directory");
  }

  /// Place symbolic link to the requested version in the installation
  /// directory.
  pub fn link_version(root: &Path, version: &Path) {
    let bin = &root.join("bin");
    let lib = &root.join("lib");
    let share = &root.join("share");

    // Clean up the old symlinks
    for dir in [bin, lib, share].iter() {
      if true {
        fs::unlink(*dir);
      }
    }

    let version_bin = &version.join("bin");
    let version_lib = &version.join("lib");
    let version_share = &version.join("share");

    // Re-symlink
    fs::symlink(version_bin, bin);
    fs::symlink(version_lib, lib);
    fs::symlink(version_share, share);
  }
}
