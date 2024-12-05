use ratatui::{crossterm::event::{KeyCode, KeyEvent}, widgets::{Paragraph, StatefulWidget, Widget}};
use crate::{git, rapport::{GenererRapport, IntoRapport}};
use super::{app::EtatGlobal, parametre::EtatParametre};


#[derive(Debug, Default, Clone)]
pub struct CentDerniersUi {}

impl StatefulWidget for CentDerniersUi {
    type State = EtatGlobal;
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut EtatGlobal) {
        Paragraph::new(state.rapport.rapport_workflows.cent_dernier.into_rapport())
            .centered()
            .render(area, buf);
    }
}

impl CentDerniersUi {
    pub fn handle_key_event(key_event: KeyEvent, state: &mut EtatParametre) {
        if key_event.code == KeyCode::Enter { git::client::Client::new().generer_rapport(&state.owner, &state.repository, &state.token);};
    }
}


