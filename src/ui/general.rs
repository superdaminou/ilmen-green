

use ratatui::{crossterm::event::{KeyCode, KeyEvent}, widgets::{Paragraph, StatefulWidget, Widget}};

use crate::{git, rapport::{GenererRapport, IntoRapport}};

use super::{app::EtatGlobal, parametre::EtatParametre};


#[derive(Debug, Default, Clone)]
pub struct GeneralUi {}

impl StatefulWidget for GeneralUi {
    type State = EtatGlobal;
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut EtatGlobal) {
        Paragraph::new(state.rapport.general.into_rapport())
            .centered()
            .render(area, buf);
    }
}

impl GeneralUi {
    pub fn handle_key_event(key_event: KeyEvent, state: &mut EtatParametre) {
        if key_event.code == KeyCode::Enter { git::client::Client::new().generer_rapport(&state.owner, &state.repository, &state.token);};
    }
}


