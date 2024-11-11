use std::io;
use dotenv::dotenv;
use ui::app::App;

mod ui;
mod git;
mod rapport;


fn main() -> io::Result<()>{
    dotenv().ok();
    //env_logger::init();

    let terminal = ratatui::init();
    let app_result = App::default().run(terminal);
    ratatui::restore();
    app_result
}
