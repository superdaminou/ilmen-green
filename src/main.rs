use dotenv::dotenv;
use ilmen_green::git_client::GitClient;

fn main() {
    dotenv().ok();
    let git_client = GitClient::new(
        reqwest::blocking::Client::new(),
        std::env::var("REPO").expect("MISSING REPO"), 
        std::env::var("OWNER").expect("MISSING OWNER"),
        std::env::var("TOKEN").expect("MISSING TOKEN"));

    let rapport = git_client.rapport();

    println!("{}",rapport.to_string());
}

