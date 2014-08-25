extern crate rustv;

use rustv::{Rustv};

fn main() {
  let root = rustv::locate_installation_directory();
  let installation = Rustv::new(&root);
  println!("Root path: {}", installation.root.display());
  installation.activate_version(&installation.current_version);
}
