extern crate dotenv;
extern crate reqwest;
extern crate serde_json;

mod github;

use dotenv::dotenv;
use github::{Extract, PullRequestStates};
use std::env;
use std::{thread, time};

fn main() {
    dotenv().ok();

    let request_url = "https://api.github.com/graphql";
    let github_token = env::var("GITHUB_TOKEN").unwrap();
    let github_org = env::var("GITHUB_ORG").unwrap();
    let github_repo = env::var("GITHUB_REPO").unwrap();

    let query = format!(
        r#"
          {{ 
            "query": "{{
              repository(owner:\"{}\", name:\"{}\") {{
                pullRequests(last: 100, states: OPEN) {{
                  edges {{
                    node {{
                      number
                      mergeable
                    }}
                  }}
                }}
              }}
            }}"
          }}
        "#,
        github_org, github_repo,
    )
    .replace("\n", "");

    let github = github::Github::new(
        github_token.to_string(),
        request_url.to_string(),
    );

    let result = github.query(query.clone());

    for attempt in 1..=10 {
        if result.extract(PullRequestStates::Unknown).len() == 0 {
        } else {
            if attempt == 10 { /* error */ }
            let delay = time::Duration::new(2 * attempt, 0);
            thread::sleep(delay);
        }
    }
}
