use super::{IntoRapport, Mb};

#[derive(Debug, Default, Clone)]
pub struct Estimations {
    pub echange_reseaux: Mb  
} 

impl IntoRapport for Estimations {
    fn into_rapport(&self) -> String {
        format!("Estimation échange réseaux: {}\r\n", self.echange_reseaux.to_string())
    }
}