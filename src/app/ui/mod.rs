use std::{io, io::Stdout};
use termion::{
    input::MouseTerminal,
    raw::{IntoRawMode, RawTerminal},
    screen::AlternateScreen,
};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Widget},
    Terminal,
};

pub struct UI {
    terminal: Terminal<TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<Stdout>>>>>,
}

impl UI {
    pub fn new() -> anyhow::Result<Self> {
        let stdout = io::stdout().into_raw_mode()?;
        let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        terminal.hide_cursor()?;

        Ok(UI { terminal })
    }

    pub fn render(&mut self) -> anyhow::Result<()> {
        self.terminal.draw(|mut f| {
            let c1 = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Min(0), Constraint::Length(5)].as_ref())
                .split(f.size());
            let c2 = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Length(25), Constraint::Min(0)].as_ref())
                .split(c1[0]);
            let c3 = Layout::default()
                .direction(Direction::Vertical)
                .horizontal_margin(1)
                .constraints([Constraint::Length(5), Constraint::Min(0)].as_ref())
                .split(c2[1]);

            Block::default()
                .title("Player")
                .borders(Borders::TOP)
                .render(&mut f, c1[1]);
            Block::default()
                .title("Playlists")
                .borders(Borders::TOP | Borders::RIGHT)
                .render(&mut f, c2[0]);
            Block::default()
                .title("Search")
                .borders(Borders::TOP)
                .render(&mut f, c3[0]);
            Block::default()
                .title("Tracks")
                .borders(Borders::TOP)
                .render(&mut f, c3[1]);
        })?;
        Ok(())
    }
}
