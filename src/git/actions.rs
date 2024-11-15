use chrono::{Days, Months, Utc};

pub enum Action {
    REPO,
    ARTIFACTS,
    WORKFLOWS,
    CACHE
}

impl Action {
    pub fn path(&self) -> String {
        let date= Utc::now().checked_sub_days(Days::new(7)).unwrap().format("%Y-%m-%d").to_string();
        let ressource = format!("/actions/runs?created=>{}&per_page=100", date);
        match self {
            Action::REPO => "",
            Action::ARTIFACTS => "/actions/artifacts",
            Action::WORKFLOWS => &ressource,
            Action::CACHE => "/actions/cache/usage"
        }.to_string()
    }
}
