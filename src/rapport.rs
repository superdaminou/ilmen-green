
pub struct Rapport {
    pub repo_name: String,
    pub taille_repository: f32,
    pub stockage_total: f32,
    pub total_artifacts: f32,
    pub total_cache: f32,
    pub rapport_workflows: RapportWorfkows,
}

impl ToString for  Rapport {
    fn to_string(&self) -> String {
        format!("Stockage total pour {}:  {}Mbytes\r\n",self.repo_name, self.stockage_total) +
        &format!("Taille du projet {}Mb\r\n", self.taille_repository) +
        &format!("Total artifacts: {}Mb\r\n", self.total_artifacts) +
        &format!("Total cache: {}Mb\r\n", self.total_cache) + 
        &format!("Total Workflows: {}\r\n", self.rapport_workflows.total)+
        &format!("  Worflows Reussis: {}\r\n", self.rapport_workflows.reussi) +
        &format!("  Worflows Echoué: {}\r\n", self.rapport_workflows.echoue) + 
        &format!("  Pourcentage de réussite: {}\r\n", self.rapport_workflows.taux)
    }
}

pub struct RapportWorfkows {
    pub total: usize,
    pub reussi: usize,
    pub echoue: usize,
    pub taux: f32
}