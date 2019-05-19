extern crate clap;

use clap::{Arg, App};
// use std::path::Path;
// use std::process;
// use std::fs::File;
// use std::io::{Read};

mod daemon_lib;

fn main() {
    let matches = App::new("launchd-cli")
      .version("0.1.0")
      .author("MMW")
      .about("A launchd cli tool written in Rust")
      .arg(Arg::with_name("command")
            .help("create/list/modify/remove")
            .empty_values(false)
        )
      .get_matches();
    
    if let Some(command) = matches.value_of("command") {
        match command {
            "create" => {
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
        
        // if deamon_lib::create() {
        //     println!("{}", command);
        // }
    }
}
