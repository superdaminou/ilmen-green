use git_client::{GitAction, GitClient};
use git_structs::{Artifacts, Cache, Repository, Workflows};
use dotenv::dotenv;

mod git_client;
mod git_structs;

fn main() {
    dotenv().ok();

    let git_client = GitClient::new(
        reqwest::blocking::Client::new(),
        std::env::var("REPO").expect("MISSING REPO"), 
        std::env::var("OWNER").expect("MISSING OWNER"),
        std::env::var("TOKEN").expect("MISSING TOKEN"));

    let repository : Repository = git_client.get(GitAction::REPO);
    let artifacts : Artifacts = git_client.get(GitAction::ARTIFACTS);
    let workflows : Workflows = git_client.get(GitAction::WORKFLOWS);
    let cache : Cache = git_client.get(GitAction::CACHE);


    let kbytes_totale_stocke = ((repository.size * 1024) + (cache.active_caches_count * cache.active_caches_size_in_bytes) + artifacts.taille_totale()) as f32 / 1000000.0;

    let taux = if workflows.total() > 0  {workflows.nombre_succes() * 100 / workflows.total()} else {100};

    println!("Stockage total pour repository {}Mbytes", kbytes_totale_stocke);
    println!("Taille du projet {}Mb", repository.size as f32 / 1000.0);
    println!("Total artifacts: {}Mb", artifacts.taille_totale() as f32 / 1000000.0);
    println!("Total cache: {}", cache.active_caches_count * cache.active_caches_size_in_bytes);
    println!("Total Workflows: {}", workflows.total());
    println!("  Worflows Reussis: {}", workflows.nombre_succes());
    println!("  Worflows Echoué: {}", workflows.nombre_echec());
    println!("  Pourcentage de réussite: {taux}");
}

