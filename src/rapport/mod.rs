
use estimation::Estimations;
use general::General;
use worklow::RapportWorfkows;


pub trait GenererRapport {
    fn generer_rapport(&self, owner: &String,  repository: &String, token: &String) -> Rapport;
}

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

#[derive(Debug, Default, Clone)]
pub struct Mb(pub f32);

impl ToString for Mb {
    fn to_string(&self) -> String {
        format!("{}Mb",self.0)
    }
}