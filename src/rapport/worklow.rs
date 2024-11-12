use std::collections::HashMap;

use super::IntoRapport;

#[derive(Debug, Default, Clone)]
pub struct RapportWorfkows {
    pub total: usize,
    pub repartition: HashMap<StatutWorkflow, usize>,
    pub taux: f32
}

impl IntoRapport for RapportWorfkows {
    fn into_rapport(&self) -> String {
        format!("Workflows du dernier mois: {}\r\n", self.total)+
        &self.repartition.iter()
            .map(|(k,v)| format!("  Worflows {}: {}", k.to_string(),v))
            .collect::<Vec<String>>()
            .to_owned()
            .join("\r\n") + "\r\n" + 
        &format!("  Pourcentage de réussite: {}\r\n", self.taux)
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