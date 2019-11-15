use serde::Deserialize;

pub mod templates;

pub static LABEL: &str = "MDU6TGFiZWwxNjM0NjMyMDAw";

pub trait Extract {
    fn extract(&self, key: PullRequestStates) -> Vec<&Node>;
}

impl Extract for Vec<Node> {
    fn extract(&self, key: PullRequestStates) -> Vec<&Node> {
        self.into_iter()
            .filter(|node| key == node.mergeable)
            .collect()
    }
}

pub enum PullRequestStates {
    // Mergeable,
    Conflicting,
    Unknown,
}

impl PartialEq<String> for PullRequestStates {
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other
    }
}

impl PullRequestStates {
    pub fn as_str(&self) -> &'static str {
        match *self {
            // PullRequestStates::Mergeable => "MERGEABLE",
            PullRequestStates::Conflicting => "CONFLICTING",
            PullRequestStates::Unknown => "UNKNOWN",
        }
    }
}

pub struct Github {
    pub token: String,
    pub url: String,
    pub label: String,
    client: reqwest::Client,
}

impl Github {
    pub fn new(token: String, url: String) -> Self {
        Github {
            token: token,
            url: url,
            label: String::from(""),
            client: reqwest::Client::new(),
        }
    }

    pub fn mutate(&self, query: String) -> String {
        let data = self.github_response(query);

        match serde_json::from_str::<Response>(&data) {
            Ok(response) => match response.errors {
                Some(value) => value[0].message.clone(),
                None => data,
            },
            Err(err) => {
                println!(
                    "ERROR: Cannot parse JSON result returned from Github.\n\n\
                     Message: {}\nOutput: {}\n",
                    err, &data,
                );
                std::process::exit(253);
            }
        }
    }

    pub fn query(&self, query: String) -> Vec<Node> {
        let data = self.github_response(query);

        match serde_json::from_str::<Response>(&data) {
            Ok(response) => response
                .data
                .repository
                .unwrap()
                .pull_requests
                .edges
                .into_iter()
                .map(|edge| edge.node)
                .collect(),
            Err(err) => {
                println!(
                    "ERROR: Cannot parse JSON result returned from Github.\n\n\
                     Message: {}\nOutput: {}\n",
                    err, &data,
                );
                std::process::exit(253);
            }
        }
    }

    fn github_response(&self, query: String) -> String {
        let resp = self
            .client
            .post(&self.url)
            .body(query)
            .bearer_auth(&self.token)
            .send();

        let mut result = match resp {
            Ok(val) => val,
            Err(_) => {
                println!(
                    "ERROR: Unexpected http response \
                     returned from Github."
                );
                std::process::exit(254)
            }
        };

        result.text().unwrap()
    }
}

impl Node {
    pub fn inspect(&self) -> String {
        format!("#{} {}", &self.number, &self.title)
    }
}

#[derive(Deserialize, Debug)]
pub struct Response {
    pub data: Data,
    pub errors: Option<Vec<Errors>>,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub repository: Option<Repository>,
}

#[derive(Deserialize, Debug)]
pub struct Errors {
    pub message: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    pub pull_requests: PullRequests,
    pub label: Label,
}

#[derive(Deserialize, Debug)]
pub struct PullRequests {
    pub edges: Vec<Edge>,
}

#[derive(Deserialize, Debug)]
pub struct Label {
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct Edge {
    pub node: Node,
}

#[derive(Deserialize, Debug)]
pub struct Node {
    pub number: u32,
    pub id: String,
    pub title: String,
    pub mergeable: String,
}
