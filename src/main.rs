extern crate dotenv;
extern crate reqwest;
extern crate serde_json;

mod github;

use dotenv::dotenv;
use std::env;

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

    let content = github.query(query.clone());

    let conflicting: Vec<u32> = content
        .into_iter()
        .filter(|node| node.mergeable == "CONFLICTING")
        .map(|node| node.number)
        .collect();

    println!("{:?}", &conflicting);
}
