use ratatui::{crossterm::event::{KeyCode, KeyEvent}, style::Stylize, text::Text, widgets::{Paragraph, StatefulWidget, Widget}};

use crate::rapport::IntoRapport;

use super::{app::EtatGlobal, parametre::EtatParametre};


#[derive(Debug, Default, Clone)]
pub struct RapportUi {}

impl StatefulWidget for RapportUi {
    type State = EtatGlobal;
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut EtatGlobal) {
            let rapport = Text::from(state.rapport.into_rapport())
                        .white();
            Paragraph::new(rapport)
                .centered()
                .render(area, buf);
    }
}

impl RapportUi {
    pub fn handle_key_event(key_event: KeyEvent, state: &mut EtatParametre) {
        if key_event.code == KeyCode::Enter { crate::rapport::rapport(&state.owner, &state.repository, &state.token);};
    }
}


