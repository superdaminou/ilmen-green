
use general::General;
use worklow::RapportWorfkows;
pub mod cent_dernier;
pub mod general;
pub mod worklow;
pub mod estimation;

pub trait GenererRapport {
    fn generer_rapport(&self, owner: &String,  repository: &String, token: &String) -> Rapport;
}

#[derive(Debug, Default, Clone)]
pub struct Rapport {
    pub general: General,
    pub rapport_workflows: RapportWorfkows
}

impl Rapport {
    pub fn new(general: General, rapport_workflows: RapportWorfkows) -> Rapport {
        Rapport {
            general,
            rapport_workflows
        }
    }
}

pub trait IntoRapport {
    fn into_rapport(&self) -> String;
}

impl IntoRapport for  Rapport {
    fn into_rapport(&self) -> String {
        self.general.into_rapport() + 
        &self.rapport_workflows.into_rapport()
    }
}

#[derive(Debug, Default, Clone)]
pub struct Mb(pub f32);

impl ToString for Mb {
    fn to_string(&self) -> String {
        format!("{}Mb",self.0)
    }
}