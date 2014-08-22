extern crate rustv;

fn main() {
  let root = rustv::locate_installation_directory();
  let registry = rustv::build_version_registry(&root);
  println!("Root path: {}", root.display());
}
