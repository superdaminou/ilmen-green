use std::{collections::HashMap, fmt::format};

use super::IntoRapport;

#[derive(Debug, Default, Clone)]
pub struct RapportWorfkows {
    pub total: usize,
    pub repartition: HashMap<StatutWorkflow, usize>,
    pub nombre_tentative: usize,
    pub taux: f32
}

impl IntoRapport for RapportWorfkows {
    fn into_rapport(&self) -> String {
        format!("Nombre de Workflows sur les 7 derniers jours: {}\r\n", self.total)+
        &format!("Rapport sur les 100 derniers workflows des 7 derniers jours\r\n") + 
        &self.repartition.iter()
            .map(|(k,v)| format!("{}: {}", k.to_string(),v))
            .collect::<Vec<String>>()
            .to_owned()
            .join("\r\n") + "\r\n" +
        &format!("Retry: {}\r\n", self.nombre_tentative) +
        &format!("Pourcentage de réussite: {}\r\n", self.taux)
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StatutWorkflow {
    SUCCES,
    ECHEC,
    AUTRE
}

impl ToString for StatutWorkflow {
    fn to_string(&self) -> String {
        match self {
            StatutWorkflow::SUCCES => "Succes",
            StatutWorkflow::ECHEC => "Echec",
            StatutWorkflow::AUTRE => "Autre",
        }.to_string()
    }
}