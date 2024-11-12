use ratatui::{crossterm::event::{KeyCode, KeyEvent}, style::Stylize, text::Text, widgets::{Paragraph, StatefulWidget, Widget}};

use super::app::EtatGlobal;


#[derive(Debug, Default, Clone)]
pub struct RapportUi {}

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
    pub fn handle_key_event(key_event: KeyEvent) {
        if key_event.code == KeyCode::Enter { generer_rapport() };
    }
}

fn generer_rapport() {
}