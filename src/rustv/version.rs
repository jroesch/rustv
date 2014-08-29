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
  pub url: String
}

impl Version {
  pub fn download_to(&self, path: &Path) -> IoResult<Path> {
    // Clean up the naming and inference of packaging type here
    let dl_path = path.join(self.name.as_slice());
    let source_path = path.join(format!("source-{}", self.name).as_slice());
    let downloaded = dl_path.exists();
    println!("{} {}", format!("{}", dl_path.display()), format!("{}", source_path.display()));
    //println!("{}", downloaded);
    println!("Downloading... ")
    if !downloaded {
      //println!("url");
      let url = Url::parse(self.url.as_slice()).ok().unwrap();
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
}
