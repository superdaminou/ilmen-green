use chrono::{Months, Utc};

pub enum Action {
    REPO,
    ARTIFACTS,
    WORKFLOWS,
    CACHE
}

impl Action {
    pub fn path(&self) -> String {
        let date= Utc::now().checked_sub_months(Months::new(1)).unwrap().format("%Y-%m-%d").to_string();
        let ressource = format!("/actions/runs?created=>{}", date);
        match self {
            Action::REPO => "",
            Action::ARTIFACTS => "/actions/artifacts",
            Action::WORKFLOWS => &ressource,
            Action::CACHE => "/actions/cache/usage"
        }.to_string()
    }
}
