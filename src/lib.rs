use regex::Regex;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

const DEFAULT_MESSAGE : &str = r#"
Thanks for supporting this discussion by sharing your opinion. ❤️
Did you know? Dedicated +1-comments can make it hard to follow the discussion.
Sharing your support via emoji reactions on comments avoids that problem and helps us get a complete picture of everybody's opinion.
Make sure to use a reaction next time to upvote an idea.
}"#;

#[derive(Serialize, Deserialize, Debug)]
struct Comment {
    body: String,
    id: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct Issue {
    id: usize,
    number: usize,
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

fn default_message() -> String {
    String::from(DEFAULT_MESSAGE)
}

/// struct representing the payload for creating comments
/// See https://developer.github.com/v3/issues/comments/#create-a-comment
#[derive(Serialize, Deserialize, Debug)]
struct CreateComment {
    body: String,
}

fn get_message(maybe_message: Option<String>) -> String {
    maybe_message.map_or(default_message(), |text| {
        if text.is_empty() {
            default_message()
        } else {
            text
        }
    })
}

fn read_payload(path: String) -> std::io::Result<String> {
    println!("opening event payload file: {}", path);
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
    Regex::new(r"^\+1[!]*$").unwrap().is_match(comment)
}

// TODO make API request to get recent comments and check for bot comments to de-noise
// Not a V1 feature
fn already_responded(_issue_number: usize) -> bool {
    false
}

fn should_reply(payload: &Payload) -> bool {
    is_plus_one(&payload.comment.body) || already_responded(payload.issue.id)
}

fn reply(
    token: String,
    issue_number: &usize,
    repo_name: &String,
    message: String,
) -> Result<String, serde_json::error::Error> {
    let new_comment = CreateComment { body: message };

    let client = reqwest::Client::new();
    let url: String = format!(
        "https://api.github.com/repos/{}/issues/{}/comments",
        repo_name, issue_number
    );

    let res = client
        .post(&url[..])
        .header(USER_AGENT, "Plus9k GitHub Action")
        .header(CONTENT_TYPE, "application/vnd.github.antiope-preview+json")
        .header(AUTHORIZATION, String::from(format!("token {}", token)))
        .json(&new_comment)
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

pub fn run(token: String, path: String, maybe_message: Option<String>) -> Option<String> {
    let message = get_message(maybe_message);

    let contents = read_payload(path).ok()?;
    let payload: Payload = serde_json::from_str(&contents).ok()?;

    // println!("payload: {:?}", payload);
    if should_reply(&payload) {
        let response = reply(
            token,
            &payload.issue.number,
            &payload.repository.full_name,
            message,
        );
        response.ok()
    } else {
        Some(String::from("Comment seems legit."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plus_one_is_detected() {
        assert!(is_plus_one("+1"));
    }

    #[test]
    fn plus_one_exclam_is_detected() {
        assert!(is_plus_one("+1!!"));
    }

    #[test]
    fn normal_comments_are_fine() {
        assert!(!is_plus_one("This is a great idea. I like it. Sweet!"));
    }

    #[test]
    fn default_message_for_none() {
        assert_eq!(get_message(None), default_message());
    }

    #[test]
    fn default_message_for_empty_string() {
        assert_eq!(get_message(Some("".to_string())), default_message());
    }
}
