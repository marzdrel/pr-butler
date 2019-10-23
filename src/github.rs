use serde::Deserialize;

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
