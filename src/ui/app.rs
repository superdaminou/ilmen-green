use std::io;

use ratatui::{buffer::Buffer, crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind}, layout::{Constraint, Layout, Rect}, style::Stylize, text::Line, widgets::{StatefulWidget, Tabs, Widget}, DefaultTerminal};

use crate::{git::git_client::GitClient, rapport::Rapport};

use super::{parametre::{EtatParametre, ParametresUi}, rapport::RapportUi};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    selected: SelectedTab
}

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        let repo = std::env::var("REPO").expect("MISSING REPO");
        let owner = std::env::var("OWNER").expect("MISSING OWNER");
        let mut git_client = GitClient::new(
            reqwest::blocking::Client::new(),
            &owner, 
           &repo,
            &std::env::var("TOKEN").expect("MISSING TOKEN"));
        
        let mut etat =  EtatGlobal {
            rapport: git_client.rapport(&owner, &repo),
            owner: std::env::var("OWNER").expect("MISSING OWNER"),
            parametre_state: EtatParametre::new(&owner,&repo),
            client: git_client
        };

        terminal.clear()?;
        
        while !self.exit {
            terminal.draw(|frame| frame.render_stateful_widget(&self, frame.area(), &mut etat))?;
            self.handle_events(&mut etat)?
        };
        
        Ok(())
    }

    fn handle_events(&mut self, etat: &mut EtatGlobal) -> io::Result<()> {
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                self.handle_key_event(key, etat);
            }
        }

        Ok(())
    }


    fn handle_key_event(&mut self, key_event: KeyEvent, etat: &mut EtatGlobal) {
        let switch = match key_event.code {
            KeyCode::Esc => {self.exit = true; false},
            KeyCode::Char('&') => {self.selected=SelectedTab::Parametres; true}, 
            KeyCode::Char('é') => {self.selected= SelectedTab::Rapport; true}, 
            KeyCode::Enter => {
                self.selected = SelectedTab::Rapport;
                etat.generer_rapport();
                true
            },
            _ => false
        };

        if switch { return; }

        
        match self.selected {
            SelectedTab::Parametres => ParametresUi::handle_key_event(key_event, &mut etat.parametre_state),
            SelectedTab::Rapport => RapportUi::handle_key_event(key_event),
        }
        
    }

    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = SelectedTab::iter().map(|l|Line::from(l.to_string()));
        let selected_tab_index = self.selected as usize;
        Tabs::new(titles)
            .select(selected_tab_index)
            .padding("", "")
            .divider(" ")
            .render(area, buf);
    }
}


impl StatefulWidget for &App {
    type State = EtatGlobal;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut EtatGlobal) {
        let vertical = Layout::vertical([Constraint::Percentage(10), Constraint::Percentage(10), Constraint::Percentage(70), Constraint::Percentage(10)]);
        let [header_area, tabs_area, inner_area, footer_area] = vertical.areas(area);
        render_title(header_area, buf);
        self.render_tabs(tabs_area, buf);
        self.selected.render(inner_area, buf, state);
        render_footer(footer_area, buf);
    }
}

fn render_title(area: Rect, buf: &mut Buffer) {
    Line::from(" Consommation Données CI/CD ".bold()).centered().render(area, buf)
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    Line::raw("Enter: generate report | Esc: quit | Tab: switch input")
        .centered()
        .render(area, buf);
}



#[derive(Debug, Default)]
pub struct EtatGlobal {
    pub rapport: Rapport,
    pub owner: String,
    pub parametre_state: EtatParametre,
    pub client: GitClient,
}

impl EtatGlobal {
    fn generer_rapport(&mut self) {
        self.rapport = self.client.rapport(&self.parametre_state.owner, &self.parametre_state.repository);
    }
}


#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter, Debug)]
enum SelectedTab {
    #[default]
    #[strum(to_string = "& - Parametres")]
    Parametres,
    
    #[strum(to_string = "é - Rapport")]
    Rapport,
}

impl StatefulWidget for SelectedTab {
    type State = EtatGlobal;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut EtatGlobal) {
        match self {
            Self::Rapport => RapportUi::default().render( area, buf, state),
            Self::Parametres => ParametresUi::default().render(area, buf, &mut state.parametre_state)
        }
    }
}
