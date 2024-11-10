use reqwest::header::HeaderMap;
use serde::Deserialize;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let owner = std::env::var("OWNER").expect("MISSING OWNER");
    let repo = std::env::var("REPO").expect("MISSING REPO");
    let token = std::env::var("TOKEN").expect("MISSING TOKEN");
    let base_url = "https://api.github.com/repos";

    let client = reqwest::blocking::Client::new();

    let mut headers= HeaderMap::new();
    headers.insert("user-agent","CUSTOM_NAME/1.0".parse().unwrap());

    let repo = client.get(format!("{}/{}/{}",base_url, owner, repo))
        .headers(headers.clone())
        .bearer_auth(token.clone())
        .send()
        .unwrap()
        .json::<Repository>()
        .unwrap()
        .size;

    println!("Taille du projet {repo:#?}Kb");

    let taille_artifacts = client.get(format!("{}/{}/{}/actions/artifacts",base_url, owner, repo))
        .headers(headers.clone())
        .bearer_auth(token.clone())
        .send()
        .unwrap()
        .json::<Artifacts>()
        .unwrap()
        .artifacts
        .iter()
        .map(|arti| arti.size_in_bytes)
        .reduce(|acc, e| acc + e)
        .map(|b|b as f32 / 1024.0)
        .unwrap_or_default();
    println!("Tailles artifacts {taille_artifacts:#?}Kb");


    let workflows = client.get(format!("{}/{}/{}/actions/runs", base_url, owner, repo))
        .headers(headers.clone())
        .bearer_auth(token.clone())
        .send()
        .unwrap()
        .json::<Workflows>()
        .unwrap();

    let total = workflows.total_count;
    let workflow_reussi = workflows.workflow_runs.iter().filter(|w| w.conclusion.eq("success")).collect::<Vec<_>>().len();
    let taux = workflow_reussi * 100  / total;
    println!("{workflow_reussi} workflow reussi sur  {total:#?}. Pourcentage {taux}");
}

#[derive(Deserialize, Debug)]
struct Artifacts{
    pub artifacts: Vec<Artifact>
}

#[derive(Deserialize, Debug)]
struct Artifact {
    size_in_bytes: i32
}

#[derive(Deserialize, Debug)]
struct Workflows{
    total_count: usize,
    workflow_runs: Vec<Workflow>
}

#[derive(Deserialize, Debug)]
struct Workflow {
    status: String,
    conclusion: String
}


#[derive(Deserialize, Debug)]
struct Repository {
    size: i32
}
