use estimation::Estimations;
use general::General;
use worklow::RapportWorfkows;

use crate::{git::git_client::GitClient, ui::GenererRapport};

pub mod general;
pub mod worklow;
pub mod estimation;

#[derive(Debug, Default, Clone)]
pub struct Rapport {
    pub general: General,
    pub rapport_workflows: RapportWorfkows,
    pub estimation: Estimations
}

pub trait IntoRapport {
    fn into_rapport(&self) -> String;
}

impl IntoRapport for  Rapport {
    fn into_rapport(&self) -> String {
        self.general.into_rapport() + 
        &self.rapport_workflows.into_rapport() +  
        &self.estimation.into_rapport()
    }
}


pub fn rapport(owner: &String, repo: &String, token: &String) -> Rapport {
    GitClient::new(reqwest::blocking::Client::new(), repo, owner, token)
        .generer_rapport(owner, repo, token)
}
