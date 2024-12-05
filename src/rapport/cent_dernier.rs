use std::collections::HashMap;

use super::{worklow::StatutWorkflow, IntoRapport};


#[derive(Debug, Default, Clone)]
pub struct CentDernier {
    pub repartition: HashMap<StatutWorkflow, usize>,
    pub nombre_tentative: usize,
    pub taux: f32
}

impl CentDernier {
    pub fn new(repartition: HashMap<StatutWorkflow, usize>, nombre_tentative: usize, taux: f32) -> CentDernier {
        CentDernier {
            repartition,
            nombre_tentative,
            taux 
        }
    }
}

impl IntoRapport for CentDernier {
    fn into_rapport(&self) -> String {
        "Rapport 100 derniers worfkows\r\n".to_string() +
        &self.repartition.iter()
        .map(|(k,v)| format!("{}: {}", k.to_string(),v))
        .collect::<Vec<String>>()
        .to_owned()
            .join("\r\n") + "\r\n" +
        &format!("Relance: {}\r\n", self.nombre_tentative) +
        &format!("Réussite: {}%\r\n", self.taux)
    }
}

