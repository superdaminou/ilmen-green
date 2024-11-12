use super::IntoRapport;

#[derive(Debug, Default, Clone)]
pub struct General {
    pub repo_name: String,
    pub taille_repository: f32,
    pub total_artifacts: f32,
    pub total_cache: f32,
    pub totale_stocke: f32
    
}

impl IntoRapport for General {
    fn into_rapport(&self) -> String {
        format!("Projet: {}\r\n", self.repo_name) +
        &format!("Taille du projet {}Mb\r\n", self.taille_repository) +
        &format!("Taille totale artifacts: {}Mb\r\n", self.total_artifacts) +
        &format!("Total cache: {}Mb\r\n", self.total_cache)
    }
}