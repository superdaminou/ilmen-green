use ratatui::{crossterm::event::{KeyCode, KeyEvent}, style::Stylize, symbols::border, text::{Line, Text}, widgets::{Block, Paragraph, StatefulWidget, Widget}};

use crate::{git::git_client::GitClient, rapport::{self, Rapport}};

use super::{app::EtatGlobal, parametre::EtatParametre};


#[derive(Debug, Default, Clone)]
pub struct RapportUi {
    active: bool,
    pub rapport: Rapport
}


impl StatefulWidget for RapportUi {
    type State = EtatGlobal;
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut EtatGlobal) {
            let rapport = Text::from(state.rapport.to_string())
                        .white();
            Paragraph::new(rapport)
                .centered()
                .render(area, buf);
    }
}

impl RapportUi {
    pub fn handle_key_event(key_event: KeyEvent, state: &mut EtatGlobal) {
        match key_event.code {
            KeyCode::Enter => generer_rapport(),
            _ => {}
        };
    }
}

fn generer_rapport() {
}