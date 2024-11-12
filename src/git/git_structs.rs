
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
    total_count: Option<usize>,
    workflow_runs: Vec<Workflow>
}

impl Workflows {
    pub fn total(&self) -> usize {
        self.total_count.unwrap_or_default()
    }

    pub fn nombre_succes(&self) -> usize {
        self.nombre_conclusion(Status::SUCCESS) + self.nombre_conclusion(Status::COMPLETED) +
            self.nombre_statut(Status::COMPLETED) + self.nombre_statut(Status::SUCCESS)
    }

    pub fn nombre_echec(&self) -> usize {
        self.nombre_conclusion(Status::FAILURE)
    }

    pub fn complete(&self) -> usize {
        self.nombre_statut(Status::COMPLETED)
    }

    fn nombre_conclusion(&self, conclusion: Status) -> usize{
        self.workflow_runs.iter().filter(|w| w.conclusion.as_ref().is_some_and(|c|c.eq(&conclusion.to_string()))).collect::<Vec<_>>().len()
    }

    fn nombre_statut(&self, statut: Status) -> usize{
        self.workflow_runs.iter().filter(|w| w.status.eq(&statut.to_string())).collect::<Vec<_>>().len()
    }
}

enum Status {
    SUCCESS,
    COMPLETED,
    FAILURE
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::SUCCESS => "success",
            Status::FAILURE => "failure",
            Status::COMPLETED => "completed"
        }
        .to_string()
    }
}

#[derive(Deserialize, Debug)]
pub struct Workflow {
    pub status: String,
    pub conclusion: Option<String>
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