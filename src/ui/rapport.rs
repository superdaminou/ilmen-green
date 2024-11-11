use ratatui::{style::Stylize, symbols::border, text::{Line, Text}, widgets::{Block, Paragraph, StatefulWidget, Widget}};

use crate::{git::git_client::GitClient, rapport::{self, Rapport}};

use super::app::AppState;


#[derive(Debug, Default, Clone)]
pub struct RapportUi {
    active: bool,
    pub rapport: Rapport
}


impl StatefulWidget for RapportUi {
    type State = AppState;
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut AppState) {
            let rapport = Text::from(state.rapport.to_string())
                        .white();
    
            Paragraph::new(rapport)
                .centered()
                .render(area, buf);
    }
}