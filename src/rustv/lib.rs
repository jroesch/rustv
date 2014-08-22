#![crate_name="rustv"]
#![crate_type="rlib"]

use std::io;
use std::io::{File, fs};
use std::os::{getenv};
use std::collections::hashmap::{HashMap};

static RUSTV_ENV_NAME: &'static str = "RUSTV_PATH";

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

pub fn build_version_registry(root: &Path) -> HashMap<Path, Path> {
  let mut hash_map = HashMap::new();
  for subdir in fs::walk_dir(root).unwrap() {
    println!("Found subdirectory: {}", subdir.display())
  }
  hash_map
}
