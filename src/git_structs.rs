
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Artifacts{
    pub artifacts: Vec<Artifact>
}

impl Artifacts {
    pub fn taille_totale(&self) -> usize {
        self.artifacts.iter()
            .map(|arti| arti.size_in_bytes)
            .reduce(|acc, e| acc + e)
            .unwrap_or_default() as usize
    }
}

#[derive(Deserialize, Debug)]
pub struct Artifact {
    pub size_in_bytes: i32
}

#[derive(Deserialize, Debug)]
pub struct Workflows{
    total_count: usize,
    workflow_runs: Vec<Workflow>
}

impl Workflows {
    pub fn total(&self) -> usize {
        self.total_count
    }

    pub fn nombre_succes(&self) -> usize {
        self.nombre_conclusion(Conclusion::SUCCESS)
    }

    pub fn nombre_echec(&self) -> usize {
        self.nombre_conclusion(Conclusion::FAILURE)
    }

    fn nombre_conclusion(&self, conclusion: Conclusion) -> usize{
        self.workflow_runs.iter().filter(|w| w.conclusion.eq(&conclusion.to_string())).collect::<Vec<_>>().len()
    }
}

enum Conclusion {
    SUCCESS,
    FAILURE
}

impl ToString for Conclusion {
    fn to_string(&self) -> String {
        match self {
            Conclusion::SUCCESS => "success",
            Conclusion::FAILURE => "failure",
        }
        .to_string()
    }
}

#[derive(Deserialize, Debug)]
pub struct Workflow {
    pub status: String,
    pub conclusion: String
}


#[derive(Deserialize, Debug)]
pub struct Repository {
    pub  size: usize
}


#[derive(Deserialize, Debug)]
pub struct Cache {
    pub active_caches_size_in_bytes: usize,
    pub active_caches_count: usize
}