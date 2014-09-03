use http::client::RequestWriter;
use http::method::Get;
use http::headers::HeaderEnum;
use std::os;
use std::io;
use std::str;
use std::io::println;
use url::Url;
use std::io::{File, fs, IoResult, IoError, IoErrorKind, PathDoesntExist};
use shell::Shell;
use std::io::process::Command;

#[deriving(Show, Encodable, Decodable)]
pub struct Version {
  pub name: String,
  pub binary_url: String,
  pub source_url: String,
  pub cargo: bool,
  pub from_source: bool
}

impl Version {
  pub fn url(&self) -> Url {
    let dl_url = if self.from_source {
      &self.source_url
    } else {
      &self.binary_url
    };

    match Url::parse(dl_url.as_slice()).ok() {
      None => fail!("issue parsing download url"),
      Some(url) => url
    }
  }

  pub fn install(&self, download_path: &Path, prefix: &Path) -> IoResult<()> {
    let src = &try!(self.download_to(download_path));
    try!(self.build_rust_in(src, prefix, &self.name));
    Ok(())
  }

  fn download_to(&self, path: &Path) -> IoResult<Path> {
    // Clean up the naming and inference of packaging type here
    let dl_path = path.join(self.name.as_slice());
    let source_path = path.join(format!("source-{}", self.name).as_slice());
    let downloaded = dl_path.exists();
    println!("{} {}", format!("{}", dl_path.display()), format!("{}", source_path.display()));
    //println!("{}", downloaded);
    println!("Downloading... ")
    if !downloaded {
      //println!("url");
      let url = self.url();
      // println!("request");
      let request: RequestWriter = try!(RequestWriter::new(Get, url));
      //println!("response");
      let mut response = match request.read_response() {
        Ok(resp) => resp,
        _ => fail!("bleh")
      };
      // println!("body");
      let body = try!(response.read_to_end());
      // println!("writing to file");
      try!(File::create(&dl_path).write(body.as_slice()))
    }

    println!("Untaring...")
    if !source_path.exists() {
      fs::mkdir(&source_path, io::UserRWX);
      let mut tar = Command::new("tar");
      tar.arg("-xzvf").
          arg(format!("{}", dl_path.display())).
          arg("-C").
          arg(format!("{}", source_path.display())).
          arg("--strip-components=1");
      try!(tar.spawn());
    }

    Ok(path.join(source_path))
  }

  fn build_rust_in(&self, build_path: &Path, prefix: &Path, version_name: &String) -> IoResult<()> {
    /* fetch a version of rust to build */
    os::change_dir(build_path);

    if self.from_source {
      println!("Invoking source build process ...");
      let mut configure = Command::new(build_path.join("configure"));
      configure.arg(Path::new(format!("--prefix={}", prefix.join(version_name.as_slice()).display())));
      Shell::new(configure).block().unwrap();

      let make = Command::new("make");
      Shell::new(make).block().unwrap();

      let mut make_install = Command::new("make");
      make_install.arg("install");
      try!(Shell::new(make_install).block())
      println!("Finished source installation.")
    } else {
      println!("Invoking binary build process...");
      let mut install = Command::new(build_path.join("install.sh"));
      install.arg(Path::new(format!("--prefix={}", prefix.join(version_name.as_slice()).display())));
      try!(Shell::new(install).block());
      println!("Finshed binary installation.")
    }

    Ok(())
  }
}
// fn build_cargo_in(build_path: &Path, prefix: &Path, version_name: String) {
//
// }
