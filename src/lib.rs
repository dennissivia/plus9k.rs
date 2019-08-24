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
struct Repository {
  full_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Payload {
  action: String,
  issue: Issue,
  repository: Repository,
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
  println!("opening file: {}", path);
  let mut file = File::open(path)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  return Ok(contents);
}

// TODO add more rules like:
// * +1 and at most 4 characters
// * also need
// * +1!
fn is_plus_one(comment: &str) -> bool {
  comment == "+1"
}

// TODO make API request to get recent comments and check for bot comments to de-noise
// Not a V1 feature
fn already_responsed(_issue_id: usize) -> bool {
  false
}

fn should_reply(payload: &Payload) -> bool {
  is_plus_one(&payload.comment.body) || already_responsed(payload.issue.id)
}

fn reply(
  token: String,
  issue_id: &usize,
  repo_name: &String,
  message: String,
) -> Result<String, serde_json::error::Error> {
  let new_comment = CreateComment { body: message };
  let json = serde_json::to_string(&new_comment)?;

  let client = reqwest::Client::new();
  let url: String = format!(
    "https://api.github.com/repos/{}/issues/{}/comments",
    repo_name, issue_id
  );

  let res = client
    .post(&url[..])
    .header(USER_AGENT, "Plus9k GitHub Action")
    .header(CONTENT_TYPE, "application/vnd.github.antiope-preview+json")
    .header(AUTHORIZATION, String::from(format!("bearer {}", token)))
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
    reply(
      token,
      &payload.issue.id,
      &payload.repository.full_name,
      message,
    );
  }

  Ok(())
}
