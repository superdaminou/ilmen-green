use ratatui::{crossterm::event::{KeyCode, KeyEvent}, layout::{Constraint, Direction, Layout}, widgets::{Block, Paragraph, StatefulWidget, Widget}};


#[derive(Debug, Default)]
pub struct ParametresUi {
    owner: String,
    repositoty: String
}

#[derive(Default,Debug)]
pub struct EtatParametre {
    pub owner: String,
    pub repository: String,
    pub selected: ParametreInput 
}


impl EtatParametre {
    pub fn new(owner: &String, repo: &String) -> EtatParametre {
        EtatParametre {
            owner: owner.clone(),
            repository: repo.clone(),
            ..Default::default()
        }
    }
}
#[derive(Default, Debug)]
enum ParametreInput {
    #[default]
    OWNER,
    REPO
}


impl StatefulWidget for ParametresUi {
    type State = EtatParametre;
    
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(area);

        match state.selected {
            ParametreInput::OWNER => {
                Paragraph::new(format!("Owner: {}", state.owner)).block(Block::bordered()).render(layout[0], buf);
                Paragraph::new(format!("Repository:  {}", state.repository)).render(layout[1], buf);
            },
            ParametreInput::REPO => {
                Paragraph::new(format!("Owner: {}", state.owner)).render(layout[0], buf);
                Paragraph::new(format!("Repository:  {}", state.repository)).block(Block::bordered()).render(layout[1], buf);
            },
        }
        
    }
}

impl ParametresUi {    
    pub fn handle_key_event(key_event: KeyEvent, state: &mut EtatParametre) {
        match key_event.code {
            KeyCode::Char(to_insert) => insert(to_insert, state), 
            KeyCode::Backspace => delete(state),
            KeyCode::Tab => change_mode(state),
            _ => {}
        };
    }
}

fn insert(to_insert: char, state: &mut EtatParametre) {
    match state.selected {
        ParametreInput::OWNER => state.owner.push(to_insert),
        ParametreInput::REPO => state.repository.push(to_insert),
    }
}

fn delete(state: &mut EtatParametre) {
    match state.selected {
        ParametreInput::OWNER => state.owner.pop(),
        ParametreInput::REPO => state.repository.pop(),
    };
}

fn change_mode(state: &mut EtatParametre) {
    match state.selected {
        ParametreInput::OWNER => state.selected = ParametreInput::REPO,
        ParametreInput::REPO => state.selected = ParametreInput::OWNER,
    };
}
