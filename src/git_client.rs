use reqwest::header::HeaderMap;
use serde::de::DeserializeOwned;

const BASE_URL : &str = "https://api.github.com/repos";

pub struct GitClient {
    client_http: reqwest::blocking::Client,
    repo: String,
    owner: String,
    token: String
}

impl GitClient {
    pub fn new(client: reqwest::blocking::Client, repo: String, owner: String, token: String) -> GitClient {
        GitClient {
            client_http: client,
            owner,
            repo,
            token
        }
    }

    pub fn get<T: DeserializeOwned>(&self, action: GitAction) -> T {
        let mut headers= HeaderMap::new();
        headers.insert("user-agent","CUSTOM_NAME/1.0".parse().unwrap());

        self.client_http.get(format!("{}/{}/{}{}", BASE_URL, self.owner, self.repo,  action.path()))
            .headers(headers.clone())
            .bearer_auth(self.token.clone())
            .send()
            .unwrap()
            .error_for_status()
            .unwrap()
            .json::<T>()
            .unwrap()
    }
}


pub enum GitAction {
    REPO,
    ARTIFACTS,
    WORKFLOWS,
    CACHE
}

impl GitAction {
    pub fn path(&self) -> &str {
        match self {
            GitAction::REPO => "",
            GitAction::ARTIFACTS => "/actions/artifacts",
            GitAction::WORKFLOWS => "/actions/runs",
            GitAction::CACHE => "/actions/cache/usage"
        }
    }
}
