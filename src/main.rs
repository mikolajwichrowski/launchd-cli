extern crate clap;

use clap::{Arg, App};
// use std::path::Path;
// use std::process;
// use std::fs::File;
// use std::io::{Read};

mod deamon_lib;

fn main() {
    let matches = App::new("launchd-cli")
      .version("0.1.0")
      .author("MMW")
      .about("A launchd cli tool written in Rust")
      .arg(Arg::with_name("script")
            .help("script to set.")
            .empty_values(false)
        )
      .get_matches();
    
    if let Some(script_location) = matches.value_of("script") {
        if deamon_lib::create() {
            println!("{}", script_location);
        }
    }
}
