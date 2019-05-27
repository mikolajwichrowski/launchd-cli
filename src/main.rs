#![deny(warnings)]
mod helpers;

use std::path::PathBuf;
use structopt;
use structopt::StructOpt;
use crate::helpers::daemon_lib;


/// Struct for commands
#[derive(StructOpt, Debug)]
#[structopt(name = "commands")]
struct Opt {
    /// Launchd-cli type of command
    #[structopt(name = "METHOD")]
    method: String,

    /// Script to run
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Option<PathBuf>,

    /// Launchd task name
    #[structopt(short = "n", long = "name", parse(from_os_str))]
    output: Option<PathBuf>,

    /// Launchd task time
    #[structopt(short = "every", long = "every")]
    time: Option<i32>,
}

/// Main method
fn main() {
    let opt = Opt::from_args();

    let result = match opt.method.as_ref() {
        // This creates the nessecary xml file and starts the launchd command
        "create" => daemon_lib::create(),
        // This lists all the generated launchd jobs
        "list" => {
            unimplemented!();
        }
        // This changes the launchd job xml file and restarts the job
        "modify" => {
            unimplemented!();
        }
        // This removes the launchd job and the xml file 
        "remove" => {
            unimplemented!();
        }
        _ => {
            println!("Command not found");
        }
    };

    match result {
        Err(result) => {
            println!("Something went wrong, Error: {:?}", result);
        },
        _ => {
            println!("Command executed succesfully");
        }
    };
}
