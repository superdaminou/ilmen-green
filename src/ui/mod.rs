use crate::rapport::Rapport;

pub mod app;
mod rapport;
mod parametre;

pub trait GenererRapport {
    fn generer_rapport(&self, owner: &String,  repository: &String, token: &String) -> Rapport;
}
