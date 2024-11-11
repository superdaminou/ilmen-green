use std::io;

use ratatui::{buffer::Buffer, crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind}, layout::{Constraint, Layout, Rect}, style::{palette::tailwind, Color, Stylize}, symbols::border, text::{Line, Text}, widgets::{Block, Paragraph, StatefulWidget, Tabs, Widget}, DefaultTerminal, Frame};

use crate::{git::git_client::GitClient, rapport::Rapport};

use super::{parametre::ParametresUi, rapport::RapportUi};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

#[derive(Debug, Default)]
pub struct App {
    formulaire: ParametresUi,
    rapport: RapportUi,
    client: GitClient,
    exit: bool,
    selected: SelectedTab
}

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter, Debug)]
enum SelectedTab {
    #[default]
    #[strum(to_string = "Rapport")]
    Rapport,
    #[strum(to_string = "Parametres")]
    Parametres,
}

impl App {

    /// runs the application's main loop until the user quits
    pub fn run(mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        let git_client = GitClient::new(
            reqwest::blocking::Client::new(),
            std::env::var("REPO").expect("MISSING REPO"), 
            std::env::var("OWNER").expect("MISSING OWNER"),
            std::env::var("TOKEN").expect("MISSING TOKEN"));
        
        let mut state =  AppState {
            rapport: git_client.rapport(),
            owner: std::env::var("OWNER").expect("MISSING OWNER")
        };
        
        while !self.exit {
            terminal.draw(|frame| frame.render_stateful_widget(&self, frame.area(), &mut state))?;
            self.handle_events()?
        };
        
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                self.handle_key_event(key);
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Tab => self.selected =  {
                match self.selected {
                    SelectedTab::Rapport => SelectedTab::Parametres,
                    SelectedTab::Parametres => SelectedTab::Rapport,
                }
            }, 
            _ => {}
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
    type State = AppState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut AppState) {
        use Constraint::{Length, Min};
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);

        let horizontal = Layout::horizontal([Min(0), Length(20)]);
        let [tabs_area, title_area] = horizontal.areas(header_area);

        render_title(title_area, buf);
        self.render_tabs(tabs_area, buf);
        self.selected.render(inner_area, buf, state);
        render_footer(footer_area, buf);
    }
}

fn render_title(area: Rect, buf: &mut Buffer) {
    Line::from(" Consommation Données CI/CD ".bold()).centered().render(area, buf)
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    Line::raw("press Tab to change tab | Press q to quit")
        .centered()
        .render(area, buf);
}

impl StatefulWidget for SelectedTab {
    type State = AppState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut AppState) {
        // in a real app these might be separate widgets
        match self {
            Self::Rapport => RapportUi::default().render( area, buf, state),
            Self::Parametres => ParametresUi::default().render(area, buf, state)
        }
    }
}

pub struct AppState {
    pub rapport: Rapport,
    pub owner: String,
}