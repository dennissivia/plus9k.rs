use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use serde::{Deserialize, Serialize};
// use reqwest::Response;

// use serde_json::Result;
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

/// https://developer.github.com/v3/issues/comments/#create-a-comment
#[derive(Serialize, Deserialize, Debug)]
struct CreateComment {
  body: String,
}

fn default_message() -> String {
  let message = r#"
   This is some amazing message
        }"#;
  String::from(message)
}

fn get_message(maybe_message: Option<String>) -> String {
  match maybe_message {
    Some(message) => message,
    None => default_message(),
  }
}

fn read_payload(path: String) -> std::io::Result<String> {
  let mut file = File::open(path)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  return Ok(contents);
}

fn is_plus_one(comment: String) -> bool {
  comment == "+1"
}

fn already_responsed(issue_id: usize) -> bool {
  false
}

fn should_reply(payload: &Payload) -> bool {
  is_plus_one(payload.comment.body.clone()) || already_responsed(payload.issue.id)
}

fn reply(
  token: String,
  issue_id: &usize,
  message: String,
) -> Result<String, serde_json::error::Error> {
  //reqwest::Result<reqwest::Response> {

  let new_comment = CreateComment { body: message };
  let json = serde_json::to_string(&new_comment)?;

  let client = reqwest::Client::new();
  let res = client
    .post("https://api.github.com/repos/:owner/:repo/issues/:issue_number/comments")
    .header(USER_AGENT, "foo")
    .header(CONTENT_TYPE, "application/vnd.github.antiope-preview+json")
    .header(AUTHORIZATION, "bearer -insert-valid-token-")
    .json(&json)
    .send();

  match res {
    Ok(good_result) => {
      println!("Request failed {:?}", good_result);
    }
    Err(err) => {
      println!("Request failed {:?}", err);
    }
  }
  Ok(String::from("Success"))
}

pub fn run(token: String, path: String, maybe_message: Option<String>) -> std::io::Result<()> {
  let message = get_message(maybe_message);
  println!("message: {:?}", message);
  let contents = read_payload(path)?;
  // println!("{:?}",contents);
  let payload: Payload = serde_json::from_str(&contents)?;
  println!("payload: {:?}", payload);
  if should_reply(&payload) {
    reply(token, &payload.issue.id, message);
  }

  Ok(())
}
