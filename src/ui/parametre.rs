use ratatui::{crossterm::event::{KeyCode, KeyEvent}, layout::{Constraint, Direction, Layout}, widgets::{Block, Paragraph, StatefulWidget, Widget}};


#[derive(Debug, Default)]
pub struct ParametresUi {}

#[derive(Default,Debug)]
pub struct EtatParametre {
    pub owner: String,
    pub repository: String,
    pub token: String,
    pub index_selected: usize,
    pub selected: ParametreInput
}


impl EtatParametre {
    pub fn new(owner: &String, repo: &String, token: &String) -> EtatParametre {
        EtatParametre {
            owner: owner.clone(),
            repository: repo.clone(),
            token: token.clone(),
            ..Default::default()
        }
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub enum ParametreInput {
    #[default]
    OWNER,
    REPO,
    TOKEN
}

impl ParametreInput {
    pub fn ordered()-> Vec<ParametreInput> {
        vec![ParametreInput::OWNER, ParametreInput::REPO, ParametreInput::TOKEN]
    }
}

impl StatefulWidget for ParametreInput {
    type State = EtatParametre;
    
    fn render<'a>(self, area: ratatui::prelude::Rect, buf: &'a mut ratatui::prelude::Buffer, state: &'a mut Self::State) {
        let char_selected = if state.selected == self { "█"} else {""};

        let paragraphe = match self {
            ParametreInput::OWNER => format!("Owner: {}{}", state.owner, char_selected),
            ParametreInput::REPO => format!("Repository: {}{}", state.repository, char_selected),
            ParametreInput::TOKEN => format!("Token: {}{}", state.token, char_selected),
        };

        if state.selected == self {
            Paragraph::new(paragraphe+char_selected).block(Block::bordered()).render(area, buf);
        } else {
            Paragraph::new(paragraphe).render(area, buf);
        }
    }
}


impl StatefulWidget for ParametresUi {
    type State = EtatParametre;
    
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ])
            .split(area);

        ParametreInput::render(ParametreInput::OWNER, layout[0], buf, state);
        ParametreInput::render(ParametreInput::REPO, layout[1], buf, state);
        ParametreInput::render(ParametreInput::TOKEN, layout[2], buf, state);
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
        ParametreInput::TOKEN =>  state.token.push(to_insert),
    }
}

fn delete(state: &mut EtatParametre) {
    match state.selected {
        ParametreInput::OWNER => state.owner.pop(),
        ParametreInput::REPO => state.repository.pop(),
        ParametreInput::TOKEN => state.token.pop(),
    };
}

fn change_mode(state: &mut EtatParametre) {
    if state.index_selected == 2 {
        state.index_selected = 0;
    } else {
        state.index_selected += 1;
    }
    state.selected = ParametreInput::ordered().get(state.index_selected).unwrap().clone();
}
