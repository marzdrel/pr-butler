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

    pub fn query(self, query: String) -> String {
        self.github_response(query)
    }

    fn github_response(self, query: String) -> String {
        let resp = self
            .client
            .post(&self.url)
            .body(query)
            .bearer_auth(self.token)
            .send();

        let mut result = match resp {
            Ok(val) => val,
            Err(_) => {
                println!("ERROR: Unexpected result returned from Github.");
                std::process::exit(254)
            }
        };

        result.text().unwrap()
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
