extern crate dotenv;
extern crate reqwest;
extern crate serde_json;

mod github;

use dotenv::dotenv;
use github::templates::{gh_add_labels, gh_pull_requests};
use github::{Extract, PullRequestStates, LABEL};
use std::env;
use std::{thread, time};

fn main() {
    dotenv().ok();

    let request_url = "https://api.github.com/graphql";
    let github_token = env::var("GITHUB_TOKEN").unwrap();
    let github_org = env::var("GITHUB_ORG").unwrap();
    let github_repo = env::var("GITHUB_REPO").unwrap();

    let github = github::Github::new(
        github_token.to_string(),
        request_url.to_string(),
    );

    let query = gh_pull_requests(github_org, github_repo);

    let result = github.query(query.to_string());

    println!("QUERY: {:?}", result);

    for attempt in 1..=10 {
        if result.extract(PullRequestStates::Unknown).len() == 0 {
            let conflicting =
                result.extract(PullRequestStates::Conflicting);
            for node in conflicting.into_iter() {
                println!("Updating repo: {}", node.inspect());

                let update =
                    gh_add_labels(LABEL.to_string(), node.id.clone());

                println!("Response -> {}", github.mutate(update.clone()));
            }
            break;
        } else {
            if attempt == 10 {
                println!(
                    "ERROR: There still conflicting PRs after {} tries",
                    attempt
                );
                break;
            }

            let delay = time::Duration::new(2 * attempt, 0);

            println!(
                "Waiting for {} seconds before retry...",
                delay.as_secs()
            );

            thread::sleep(delay);
        }
    }
}
