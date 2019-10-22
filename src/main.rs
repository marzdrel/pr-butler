extern crate dotenv;
extern crate reqwest;
extern crate serde_json;

use dotenv::dotenv;
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct GhResponse {
    data: GhData,
}

#[derive(Deserialize)]
struct GhData {
    repository: GhRepository,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GhRepository {
    pull_requests: GhPullRequests,
}

#[derive(Deserialize)]
struct GhPullRequests {
    edges: Vec<GhEdge>,
}

#[derive(Deserialize)]
struct GhEdge {
    node: GhNode,
}

#[derive(Deserialize, Debug)]
struct GhNode {
    number: u32,
    mergeable: String,
}

fn main() {
    dotenv().ok();

    let request_url = "https://api.github.com/graphql";
    let client = reqwest::Client::new();

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

    let resp = client
        .post(request_url)
        .body(query)
        .bearer_auth(github_token)
        .send();

    let mut response = match resp {
        Ok(val) => val,
        Err(_) => {
            println!("ERROR: Unexpected resul returned from Github.");
            std::process::exit(254)
        }
    };

    let data = &response.text().unwrap();

    let gh_response: GhResponse = serde_json::from_str(data).unwrap();
    let content: Vec<GhNode> = gh_response
        .data
        .repository
        .pull_requests
        .edges
        .into_iter()
        .filter(|edge| edge.node.mergeable != "MERGEABLE")
        .map(|edge| edge.node)
        .collect();

    // let waiting = content.into_iter().any(|node| node.mergeable == "UNKNOWN");
    // println!("{:?}", waiting);

    let conflicting: Vec<u32> = content
        .into_iter()
        .filter(|node| node.mergeable == "CONFLICTING")
        .map(|node| node.number)
        .collect();

    println!("{:?}", &conflicting);
}
