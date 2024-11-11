use std::io;

use ratatui::{buffer::Buffer, crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind}, layout::Rect, style::Stylize, symbols::border, text::{Line, Text}, widgets::{Block, Paragraph, Widget}, DefaultTerminal, Frame};

use crate::{git::git_client::GitClient, rapport::Rapport};



#[derive(Debug, Default)]
pub struct App {
    rapport: Rapport,
    exit: bool,
}

impl App {

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let git_client = GitClient::new(
            reqwest::blocking::Client::new(),
            std::env::var("REPO").expect("MISSING REPO"), 
            std::env::var("OWNER").expect("MISSING OWNER"),
            std::env::var("TOKEN").expect("MISSING TOKEN"));
    
        self.rapport = git_client.rapport();
        
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?
        };
        
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
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
            _ => {}
        }
    }
}


impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Consommation Données CI/CD ".bold());
        let instructions = Line::from(vec![
            " Owner ".into(),
            self.rapport.repo_name.as_str().blue().bold(),
            " Repository ".into(),
            self.rapport.repo_name.as_str().blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let rapport = Text::from(self.rapport.to_string())
                    .white()
                    .on_blue();

        Paragraph::new(rapport)
            .centered()
            .block(block)
            .render(area, buf);
    }
}