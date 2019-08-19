// extern crate serde_json;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct Comment {
    body: String,
    id: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct Issue {
    id: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct Payload {
    action: String,
    issue: Issue,
    comment: Comment,
}

fn default_message() -> String {
 let message = r#"
   This is some amazing message
        }"#;
  String::from(message)
}

fn get_message() -> String {
    let key = "INPUT_MESSAGE";
    return match env::var(key) {
        Ok(message) => {
            String::from(message)
        },
        Err(_) => {
            default_message()
        },
    }
}

fn main() -> std::io::Result<()> {

    let key = "INPUT_PAYLOAD_PATH";
    match env::var(key) {
        Ok(path) => {
            let mut file = File::open(path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            // println!("{:?}",contents);
            let payload: Payload = serde_json::from_str(&contents)?;
            let message = get_message();
            println!("payload: {:?}",payload);
            println!("message: {:?}",message);
        }

        Err(e) => {
            println!("couldn't interpret {}: {}", key, e);
        }
    }
    Ok(())
}
