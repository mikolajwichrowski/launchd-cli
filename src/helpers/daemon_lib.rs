use std::process::Command;
use serde::{Serialize, Deserialize};
use serde_xml_rs;

use crate::helpers::file_lib;

#[derive(Serialize, Deserialize, Debug)]
struct Pair {
    key: String,
    string: String,
}

pub fn create() -> Result<i32, &'static str> {
    // I cant get this to serialize in more than one pair ... 
    // I found some issues regarding serializing Vec, this gave me no hints on what to do
    // Serializing one pair is working fine
    // This is insufficient for the required xml

    // I need to generate this xml
    // <plist version="1.0">
    //   <dict>
    //     <key>Label</key>
    //     <string>com.s.osx.{name}</string>
    //     <key>Program</kegit cy>
    //     <string>/Users/bob/pyfoo.py</string>
    //     <key>KeepAlive</key>
    //     <true/>
    //   </dict>
    // </plist>

    // The this generates <Pair><key>Label</key><string>com.s.osx.{name}</string></Pair>
    let pair: Pair = Pair { key: "Label".to_string(), string: "com.s.osx.{name}".to_string() };
    let serialized = serde_xml_rs::to_string(&pair);
    println!("creating launchd.task.name => .plist => {:?}", serialized);

    start("launchd.task.name".to_string())
}

pub fn start(plist: String) -> Result<i32, &'static str> {
    println!("Starting task.");
    if file_lib::exists(&plist) {
        let output = Command::new("echo")
            .args(&[&plist])
            .status()
            .expect("Task could not be started.");

        match output.code() {
            Some(code) => {
                println!("Exited with status code: {}.", code);
                Ok(code)
            },
            None => Err("Process terminated by signal.")
        }
    } else {
        Err("Could not find file.")
    }
}

// pub fn stop(_pid: i32) -> Result<i32, &'static str> {
//   unimplemented!(); 
// }
