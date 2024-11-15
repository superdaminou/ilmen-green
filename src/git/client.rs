
use std::collections::HashMap;

use log::info;
use reqwest::header::HeaderMap;
use serde::de::DeserializeOwned;

use crate::rapport::{cent_dernier::CentDernier, estimation::Estimations, general::General, worklow::{ DernierePeriode, RapportWorfkows, StatutWorkflow}, GenererRapport, Mb, Rapport};

use super::{actions::Action, models::{Artifacts, Cache, Repository, Workflows}};

const BASE_URL : &str = "https://api.github.com/repos";

#[derive(Debug, Default, Clone)]
pub struct Client {
    client_http: reqwest::blocking::Client
}

impl Client {
    pub fn new() -> Client {
        Client {
            client_http: reqwest::blocking::Client::new()
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

        let mut repartitions: HashMap<StatutWorkflow, usize> = HashMap::new();

        let finished_workflows = workflows.workflow_runs.iter()
            .filter(|w| w.status == "completed").collect::<Vec<_>>(); 

        finished_workflows.iter()
            .for_each(|workflow| {
                match workflow.conclusion.as_ref() {
                    Some(conclusion) => {
                        match conclusion.as_str() {
                            "success" => repartitions.insert(StatutWorkflow::SUCCES, repartitions.get(&StatutWorkflow::SUCCES).unwrap_or(&0) +1),
                            "failure" => repartitions.insert(StatutWorkflow::ECHEC, repartitions.get(&StatutWorkflow::ECHEC).unwrap_or(&0) +1),
                            _ => repartitions.insert(StatutWorkflow::AUTRE, repartitions.get(&StatutWorkflow::AUTRE).unwrap_or(&0) +1)
                        }
                    },
                    None => repartitions.insert(StatutWorkflow::AUTRE, repartitions.get(&StatutWorkflow::AUTRE).unwrap_or(&0)+1),
                };
        });

        let nb_retry= workflows.workflow_runs.iter()
            .filter(|w| w.run_attempt > 1)
            .map(|w|w.run_attempt-1)
            .reduce(|acc, x|acc+x)
            .unwrap_or_default();
        
        let taux = if !finished_workflows.is_empty()  {*(repartitions.get(&StatutWorkflow::SUCCES).unwrap_or(&0)) as f32 * 100.0 / finished_workflows.len() as f32} else {100.0};

        let cents_dernier = CentDernier {
            repartition: repartitions,
            nombre_tentative: nb_retry,
            taux 
        };

        let estimation = Estimations {
            echange_reseaux: Mb(general.taille_repository.0 * finished_workflows.len() as f32)
        };

        let dernier_periode= DernierePeriode {
            total: workflows.total(), 
            estimation
        };

        let rapport = RapportWorfkows {
            cent_dernier: cents_dernier,
            derniere_periode: dernier_periode
        };

        

        Rapport {
            general,
            rapport_workflows: rapport
        }
    }
}
