use reqwest::{Response as ReqwestResponse, Result};
use serde::Deserialize;

pub struct Github {
    pub token: String,
    pub url: String,
    client: reqwest::Client,
}

impl Github {
    pub fn new(token: String, url: String) -> Self {
        Github {
            token: token,
            url: url,
            client: reqwest::Client::new(),
        }
    }

    pub fn query(self, query: String) -> Result<ReqwestResponse> {
        self.client
            .post(&self.url)
            .body(query)
            .bearer_auth(self.token)
            .send()
    }
}

#[derive(Deserialize)]
pub struct Response {
    pub data: Data,
}

#[derive(Deserialize)]
pub struct Data {
    pub repository: Repository,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    pub pull_requests: PullRequests,
}

#[derive(Deserialize)]
pub struct PullRequests {
    pub edges: Vec<Edge>,
}

#[derive(Deserialize)]
pub struct Edge {
    pub node: Node,
}

#[derive(Deserialize, Debug)]
pub struct Node {
    pub number: u32,
    pub mergeable: String,
}