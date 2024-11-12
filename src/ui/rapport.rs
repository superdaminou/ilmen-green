

use ratatui::{crossterm::event::{KeyCode, KeyEvent}, layout::{Constraint, Layout}, widgets::{Paragraph, StatefulWidget, Widget}};

use crate::rapport::IntoRapport;

use super::{app::EtatGlobal, parametre::EtatParametre};


#[derive(Debug, Default, Clone)]
pub struct RapportUi {}

impl StatefulWidget for RapportUi {
    type State = EtatGlobal;
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut EtatGlobal) {

        let horizontal = Layout::horizontal([Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(33)]);
        let [general, worflows, estimation] = horizontal.areas(area);
        
        Paragraph::new(state.rapport.general.into_rapport())
            .centered()
            .render(general, buf);
        Paragraph::new(state.rapport.rapport_workflows.into_rapport())
            .centered()
            .render(worflows, buf);
        Paragraph::new(state.rapport.estimation.into_rapport())
            .centered()
            .render(estimation, buf);
    }
}

impl RapportUi {
    pub fn handle_key_event(key_event: KeyEvent, state: &mut EtatParametre) {
        if key_event.code == KeyCode::Enter { crate::rapport::rapport(&state.owner, &state.repository, &state.token);};
    }
}


