use super::IntoRapport;

#[derive(Debug, Default, Clone, Copy)]
pub struct RapportWorfkows {
    pub total: usize,
    pub reussi: usize,
    pub echoue: usize,
    pub taux: f32
}

impl IntoRapport for RapportWorfkows {
    fn into_rapport(&self) -> String {
        format!("Workflows du dernier mois: {}\r\n", self.total)+
        &format!("  Worflows Reussis: {}\r\n", self.reussi) +
        &format!("  Worflows Echoué: {}\r\n", self.echoue) + 
        &format!("  Pourcentage de réussite: {}\r\n", self.taux)
    }
}