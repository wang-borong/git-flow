extern crate git_flow;

use std::process::exit;
use git_flow::cli_run;
use git_flow::gitflow::error::Error;

fn main() {
    // the main is a handler to all final results
    exit(match cli_run() {
        Ok(s) => {
            println!("{}", s);
            0
        }
        Err(e) => {
            match e {
                Error::Generic(generr) => {
                    eprintln!("{}", generr);
                    1
                }
                Error::NoHead => {
                    eprintln!("Git2: no head");
                    2
                }
                Error::Git(giterr) => {
                    eprintln!("Git2 {}", giterr);
                    3
                }
                Error::Io(ioerr) => {
                    eprintln!("Io {}", ioerr);
                    4
                }
            }
        }
    });
}
