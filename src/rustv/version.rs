use http::client::RequestWriter;
use http::method::Get;
use std::os;
use std::os::tmpdir;
use std::io;
use url::Url;
use std::io::{File, fs, IoResult};
use std::io::fs::PathExtensions;
use shell::Shell;
use std::io::process::Command;
use time;

#[deriving(Show)]
pub enum BuildType {
  Source,
  Binary
}

#[deriving(Show, Encodable, Decodable)]
pub struct Version {
  pub name: String,
  pub binary_url: String,
  pub source_url: String,
  pub cargo: bool,
  pub from_source: bool
}

impl Version {
  pub fn url(&self, build_type: BuildType) -> Url {
    let dl_url = match build_type {
      Source => &self.source_url,
      Binary => &self.binary_url
    };

    match Url::parse(dl_url.as_slice()).ok() {
      None => fail!("issue parsing download url"),
      Some(url) => url
    }
  }

  pub fn install(&self, download_path: &Path, prefix: &Path, build_type: BuildType) -> IoResult<()> {
    let src = try!(self.download(build_type));
    try!(self.build_rust_in(&src, prefix, &self.name, build_type));
    Ok(())
  }

  // Old dl_cache sucks, gonna remove caching for now, and come back in another
  // pass.
  fn download(&self, build_type: BuildType) -> IoResult<Path> {
    let path = tmpdir().join(time::now_utc().tm_nsec.to_string().as_slice());
    try!(fs::mkdir(&path, io::UserRWX));
    let dl_path = path.join(self.name.as_slice());
    let source_path = path.join(format!("source-{}", self.name).as_slice());

    verbose!("Downloading to {} ... ", dl_path.display())

    let url = self.url(build_type);
    let request: RequestWriter = try!(RequestWriter::new_request(Get, url, true, false));
    let mut response = match request.read_response() {
        Ok(resp) => resp,
        Err((resp, err)) => { println!("{}", err); fail!("done.") }
    };

    let body = try!(response.read_to_end());
    try!(File::create(&dl_path).write(body.as_slice()))

    verbose!("Unpacking source to {} ...", source_path.display());

    try!(fs::mkdir(&source_path, io::UserRWX));
    let mut tar = Command::new("tar");
    tar.arg("-xzvf").
        arg(format!("{}", dl_path.display())).
        arg("-C").
        arg(format!("{}", source_path.display())).
        arg("--strip-components=1");
    try!(tar.spawn());

    Ok(path.join(source_path))
  }

  fn build_rust_in(&self, build_path: &Path, prefix: &Path, version_name: &String, build_type: BuildType) -> IoResult<()> {
    /* fetch a version of rust to build */
    os::change_dir(build_path);

    match build_type {
      Source => {
        verbose!("Invoking source build process ...");
        verbose!("Invoking configure ...")
        let mut configure = Command::new(build_path.join("configure"));
        configure.arg(Path::new(format!("--prefix={}", prefix.join(version_name.as_slice()).display())));
        Shell::new(configure).block().unwrap();

        verbose!("Invoking make ...");
        let make = Command::new("make");
        Shell::new(make).block().unwrap();

        verbose!("Invoking make install ...");
        let mut make_install = Command::new("make");
        make_install.arg("install");
        try!(Shell::new(make_install).block())
        verbose!("Finished source installation.")
      },
      Binary => {
        println!("Invoking binary build process...");
        let mut install = Command::new(build_path.join("install.sh"));
        install.arg(Path::new(format!("--prefix={}", prefix.display())));
        try!(Shell::new(install).block());
        verbose!("Finshed binary installation.")
      }
    }

    Ok(())
  }
}
// fn build_cargo_in(build_path: &Path, prefix: &Path, version_name: String) {
//
// }
