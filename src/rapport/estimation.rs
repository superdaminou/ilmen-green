use super::{IntoRapport, Mb};

#[derive(Debug, Default, Clone)]
pub struct Estimations {
    pub echange_reseaux: Mb  
} 

impl Estimations {
    pub fn new(echange_reseaux: Mb) -> Estimations {
        Estimations {
            echange_reseaux
        }
    } 
}

impl IntoRapport for Estimations {
    fn into_rapport(&self) -> String {
        "Estimations:\r\n".to_string() +
        &format!("échange réseaux: {}\r\n", self.echange_reseaux.to_string())
    }
}