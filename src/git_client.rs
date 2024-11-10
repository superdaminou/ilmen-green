
use reqwest::header::HeaderMap;
use serde::de::DeserializeOwned;

use crate::{git_structs::{Artifacts, Cache, Repository, Workflows}, rapport::{Rapport, RapportWorfkows}};

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

    pub fn rapport(&self) -> Rapport {
        let repository : Repository = self.get(GitAction::REPO);
        let artifacts : Artifacts = self.get(GitAction::ARTIFACTS);
        let workflows : Workflows = self.get(GitAction::WORKFLOWS);
        let cache : Cache = self.get(GitAction::CACHE);


        let kbytes_totale_stocke = ((repository.size * 1024) + (cache.active_caches_count * cache.active_caches_size_in_bytes) + artifacts.taille_totale()) as f32 / 1000000.0;

        let taux = if workflows.total() > 0  {workflows.nombre_succes() as f32 * 100.0 / workflows.total() as f32} else {100.0};

        Rapport {
            repo_name: self.repo.clone(),
            stockage_total: kbytes_totale_stocke,
            taille_repository: repository.size as f32 / 1000.0,
            total_artifacts: artifacts.taille_totale() as f32 / 1000000.0,
            total_cache: cache.active_caches_count * cache.active_caches_size_in_bytes,
            rapport_workflows: RapportWorfkows {
                total: workflows.total(),
                echoue: workflows.nombre_echec(),
                reussi: workflows.nombre_succes(),
                taux
            }
        }
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
