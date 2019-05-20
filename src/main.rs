extern crate clap;
extern crate structopt;

use std::path::PathBuf;
use structopt::StructOpt;
use clap::{Arg, App};
// use std::path::Path;
// use std::process;
// use std::fs::File;
// use std::io::{Read};

mod daemon_lib;

/// Struct for commands
#[derive(StructOpt, Debug)]
#[structopt(name = "commands")]
struct Opt {
    /// Launchd-cli type of command
    #[structopt(name="METHOD")]
    method: String,

    /// Script to run
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Option<PathBuf>,

    /// Launchd task name
    #[structopt(short = "n", long = "name", parse(from_os_str))]
    output: Option<PathBuf>,

    /// Launchd task time
    #[structopt(short = "every", long = "every")]
    time: Option<i32>
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt); 
    
    match opt.method.as_ref() {
        "create" => {
            // TODO: pass values to create method
            // TODO: create /file(.sh/.py/executable) every 10 min/sec/hours 
            if daemon_lib::create() {
                println!("creating");
            }
        },
        "list" => {
            println!("listing");
        },
        "modify" => {
            println!("modifying");
        },
        "remove" => {
            println!("removing");
        },
        _ => {
            println!("command not found");
        }
    }
}
