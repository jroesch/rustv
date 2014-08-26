use std::io::{BufferedReader, IoResult};
use std::io::process::{Command};

/* Don't know what to do with this right */
pub struct Shell {
  command: Command
}

impl Shell {
  pub fn new(cmd: Command) -> Shell {
    Shell { command: cmd }
  }

  pub fn block(self) -> IoResult<()> {
    let child = match self.command.spawn() {
      Ok(child) => child,
      Err(e) => fail!("failed to execute make: {}", e)
    };

    /* Why do I need to clone here? */
    let mut child_stdout = BufferedReader::new(child.stdout.clone().unwrap());

    for io_line in child_stdout.lines() {
      let line = try!(io_line);
      print!("{}", line);
    }

    Ok(())
  }
}
