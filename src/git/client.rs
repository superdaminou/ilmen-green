
use log::info;
use reqwest::header::HeaderMap;
use serde::de::DeserializeOwned;

use crate::{rapport::{estimation::Estimations, general::General, worklow::RapportWorfkows, Mb, Rapport}, ui::GenererRapport};

use super::{actions::Action, models::{Artifacts, Cache, Repository, Workflows}};

const BASE_URL : &str = "https://api.github.com/repos";

#[derive(Debug, Default, Clone)]
pub struct Client {
    client_http: reqwest::blocking::Client
}

impl Client {
    pub fn new(client: reqwest::blocking::Client) -> Client {
        Client {
            client_http: client
        }
    }

    pub fn get<T: DeserializeOwned>(&self, action: Action, owner: &String, repo: &String, token: &String) -> T {
        let mut headers= HeaderMap::new();
        headers.insert("user-agent","ILMEN/1.0".parse().unwrap());

        info!("Getting {}", format!("{}/{}/{}{}", BASE_URL, owner, repo,  action.path()));
        self.client_http.get(format!("{}/{}/{}{}", BASE_URL, owner, repo,  action.path()))
            .headers(headers.clone())
            .bearer_auth(token.clone())
            .send()
            .unwrap()
            .error_for_status()
            .unwrap()
            .json::<T>()
            .unwrap()
    }

    
}

impl GenererRapport for Client {
    fn generer_rapport(&self, owner: &String,  repo_name: &String, token: &String) -> Rapport {
        let repository : Repository = self.get(Action::REPO, owner,repo_name, token);
        let artifacts : Artifacts = self.get(Action::ARTIFACTS,owner,repo_name, token);
        let workflows : Workflows = self.get(Action::WORKFLOWS,owner,repo_name, token);
        let cache : Cache = self.get(Action::CACHE,owner,repo_name, token);

        let general = General::new(repo_name, Mb(repository.size as f32 / 1000.0), 
        Mb(artifacts.taille_totale() as f32 / 1000000.0), Mb(cache.active_caches_size_in_bytes as f32 / 1000000.0));


        let taux = if workflows.total() > 0  {workflows.nombre_succes() as f32 * 100.0 / workflows.total() as f32} else {100.0};
        let rapport = RapportWorfkows {
            total: workflows.total(),
            echoue: workflows.nombre_echec(),
            reussi: workflows.nombre_succes(),
            taux
        };

        let estimation = Estimations {
            echange_reseaux: Mb(general.taille_repository.0 * workflows.total() as f32)
        };

        Rapport {
            general,
            rapport_workflows: rapport,
            estimation
        }
    }
}
