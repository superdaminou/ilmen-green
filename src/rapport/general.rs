use super::{IntoRapport, Mb};

#[derive(Debug, Default, Clone)]
pub struct General {
    pub repo_name: String,
    pub taille_repository: Mb,
    pub total_artifacts: Mb,
    pub total_cache: Mb,
    pub totale_stocke: Mb
    
}




impl General {
    pub fn new(repository_name: &String, taille_repository: Mb, total_artifacts: Mb, total_cache: Mb) -> General {
        General {
            repo_name: repository_name.clone(),
            taille_repository: taille_repository.clone(),
            total_artifacts: total_artifacts.clone(),
            total_cache: total_cache.clone(),
            totale_stocke: Mb(taille_repository.0 + total_artifacts.0 + total_cache.0)
        }
    }
}

impl IntoRapport for General {
    fn into_rapport(&self) -> String {
        format!("Projet: {}\r\n", self.repo_name) +
        &format!("Taille du projet {}\r\n", self.taille_repository.to_string()) +
        &format!("Taille totale artifacts: {}\r\n", self.total_artifacts.to_string()) +
        &format!("Total cache: {}\r\n", self.total_cache.to_string()) +
        &format!("Total stockage permanent: {}\r\n", self.total_cache.to_string())
    }
}