use std::io::stderr;

use crate::gui::Event;
use crate::gui::{AppResult, Executable, KeyEventHandler, Renderer};
use crate::gui::{Program, Tui};

use crossterm::event::KeyEvent as CrosstermKeyEvent;
use ratatui::Terminal;

impl<T: Renderer + KeyEventHandler + Executable> Program<T> {
    /**
    Constructs a new instance of [`Program`].
    */
    pub fn new(app: T) -> Self {
        Self { app }
    }

    fn handle_key_event(&mut self, ke: CrosstermKeyEvent) -> AppResult<()> {
        self.app.handle_key_event(ke)
    }

    pub async fn run(&mut self) -> AppResult<()> {
        let backend = ratatui::backend::CrosstermBackend::new(stderr());
        let terminal = Terminal::new(backend)?;
        let events = crate::gui::EventHandler::new(250);
        let mut tui = Tui::new(terminal, events);
        tui.init()?;

        while self.app.is_running() {
            tui.draw(&mut self.app)?;

            match tui.events.next().await? {
                Event::Tick => self.app.tick(),
                Event::Key(ke) => self.handle_key_event(ke)?,
                Event::Mouse(_) => (),
                Event::Resize(_, _) => (),
            }
        }

        tui.exit()?;

        Ok(())
    }
}
