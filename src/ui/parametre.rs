use std::fmt::format;

use ratatui::widgets::{Paragraph, StatefulWidget, Widget};

use super::app::AppState;

#[derive(Debug, Default)]
pub struct ParametresUi {
    owner: String,
    repositoty: String
}


impl StatefulWidget for ParametresUi {
    type State = AppState;
    
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        Paragraph::new(format!("{}{}", state.owner, state.rapport.repo_name)).render(area, buf);
    }
    
    
}