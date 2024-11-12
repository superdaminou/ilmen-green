use super::IntoRapport;

#[derive(Debug, Default, Clone)]
pub struct Estimations {
    pub echange_reseaux: f32  
} 

impl IntoRapport for Estimations {
    fn into_rapport(&self) -> String {
        format!("Estimation échange réseaux: {}\r\n", self.echange_reseaux)
    }
}