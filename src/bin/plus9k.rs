// extern crate serde_json;
use plus9k;
use std::env;

fn main() {
    let path = env::var("GITHUB_EVENT_PATH").expect("GITHUB_EVENT_PATH is required");
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN is required");
    // optional, fine to have none
    let maybe_message = env::var("MESSAGE").ok();

    plus9k::run(token, path, maybe_message);
}
