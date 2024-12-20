
use super::{cent_dernier::CentDernier, estimation::Estimations, IntoRapport};

#[derive(Debug, Default, Clone)]
pub struct RapportWorfkows {
    pub derniere_periode: DernierePeriode,
    pub cent_dernier: CentDernier
    
}
impl RapportWorfkows {
    pub fn new(derniere_periode: DernierePeriode, cent_dernier: CentDernier)  -> RapportWorfkows{
        RapportWorfkows {
            derniere_periode,
            cent_dernier
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct DernierePeriode {
    pub total: usize,
    pub estimation: Estimations
}

impl DernierePeriode {
    pub fn new(total: usize, estimation: Estimations) -> DernierePeriode {
        DernierePeriode {
            total,
            estimation
        }
    }
}

impl IntoRapport for DernierePeriode {
    fn into_rapport(&self) -> String {
        format!("{} worflows sur les 30 derniers jours\r\n", self.total) +
        &self.estimation.into_rapport()
    }
}


impl IntoRapport for RapportWorfkows {
    fn into_rapport(&self) -> String {
        "Cent derniers worflows:\r\n".to_string() + 
        &self.cent_dernier.into_rapport() +
        "7 derniers jours:\r\n" +  
        &self.derniere_periode.into_rapport()
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

impl From<&str> for StatutWorkflow {
    fn from(value: &str) -> Self {
        match value {
            "success" => StatutWorkflow::SUCCES,
            "failure" => StatutWorkflow::ECHEC,
            _ => Self::AUTRE
        }
    }
}