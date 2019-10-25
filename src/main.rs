extern crate dotenv;
extern crate reqwest;
extern crate serde_json;

mod github;

use dotenv::dotenv;
use github::{Extract, PullRequestStates, LABEL};
use regex::Regex;
use std::env;
use std::{thread, time};

fn update_string(object_id: String, re: Regex) -> String {
    let raw_update = format!(
        r#"
          {{  
            "query": "mutation {{
              addLabelsToLabelable(input:
                {{
                  labelIds:\"{}\",
                  labelableId:\"{}\"
                }}
              ) 
              {{
                clientMutationId
              }}
            }}"
          }}
        "#,
        LABEL, object_id
    );

    re.replace_all(&raw_update.replace("\n", ""), " ")
        .to_string()
}

fn wrap_query(inner: &str) -> String {
    format!("{{ \"query\": \"{}\" }}", inner)
}

fn main() {
    dotenv().ok();

    let re = Regex::new(r"\s+").unwrap();
    let request_url = "https://api.github.com/graphql";
    let github_token = env::var("GITHUB_TOKEN").unwrap();
    let github_org = env::var("GITHUB_ORG").unwrap();
    let github_repo = env::var("GITHUB_REPO").unwrap();

    let query_template = r#"
      { 
        repository(owner:"$GITHUB_ORG", name:"$GITHUB_REPO") {
          label(name: "conflicting") {
            id
          }
          pullRequests(last: 100, states: OPEN) {
            edges {
              node {
                id
                number
                mergeable
              }
            }
          }
        }
      }
    "#;

    let query = wrap_query(
        &re.replace_all(query_template, " ")
            .replace("\n", "")
            .replace("\"", "\\\"")
            .replace("$GITHUB_ORG", &github_org)
            .replace("$GITHUB_REPO", &github_repo),
    );

    println!("{}", query.to_string());

    let github = github::Github::new(
        github_token.to_string(),
        request_url.to_string(),
    );

    let result = github.query(query.to_string());

    for attempt in 1..=10 {
        if result.extract(PullRequestStates::Unknown).len() == 0 {
            let conflicting =
                result.extract(PullRequestStates::Conflicting);
            for id in conflicting.into_iter() {
                let update = update_string(id, re.clone());

                github.mutate(update.clone());
            }
            break;
        } else {
            if attempt == 10 { /* error */ }
            let delay = time::Duration::new(2 * attempt, 0);
            thread::sleep(delay);
        }
    }
}
