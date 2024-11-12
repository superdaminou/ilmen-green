
use estimation::Estimations;
use general::General;
use worklow::RapportWorfkows;

use crate::{git::client::Client, ui::GenererRapport};

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
    Client::new(reqwest::blocking::Client::new())
        .generer_rapport(owner, repo, token)
}


#[derive(Debug, Default, Clone)]
pub struct Mb(pub f32);

impl ToString for Mb {
    fn to_string(&self) -> String {
        format!("{}Mb",self.0)
    }
}