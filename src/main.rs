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

fn main() {
    dotenv().ok();

    let re = Regex::new(r"\s+").unwrap();
    let request_url = "https://api.github.com/graphql";
    let github_token = env::var("GITHUB_TOKEN").unwrap();
    let github_org = env::var("GITHUB_ORG").unwrap();
    let github_repo = env::var("GITHUB_REPO").unwrap();

    let query = format!(
        r#"
          {{ 
            "query": "{{
              repository(owner:\"{}\", name:\"{}\") {{
                label(name: \"conflicting\") {{
                  id
                }}
                pullRequests(last: 100, states: OPEN) {{
                  edges {{
                    node {{
                      id
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
            let conflicting =
                result.extract(PullRequestStates::Conflicting);
            for id in conflicting.into_iter() {
                let _update = update_string(id, re.clone());
                github.query(query.clone());
            }
            break;
        } else {
            if attempt == 10 { /* error */ }
            let delay = time::Duration::new(2 * attempt, 0);
            thread::sleep(delay);
        }
    }
}
