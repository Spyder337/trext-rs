use std::cell::{RefCell, RefMut};

use common::tui::{App, AppResult, Term};
use ratatui::{
    layout::Alignment,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Borders, Paragraph, Widget,
    },
};

pub(crate) struct Application {
    term: Option<RefCell<Term>>,
    exit: bool,
    counter: u8,
}

impl Application {
    pub fn new() -> Self {
        Self {
            term: None,
            exit: false,
            counter: 0,
        }
    }
}

impl App for Application {
    fn can_exit(&self) -> bool {
        self.exit
    }

    fn toggle_exit(&mut self) {
        self.exit = !self.exit
    }

    fn set_term(&mut self, term: Term) -> std::io::Result<bool> {
        self.term = Some(RefCell::new(term));
        return Ok(true);
    }

    fn get_term(&self) -> Option<RefMut<Term>> {
        let res = if self.term.is_some() {
            Some(self.term.as_ref().unwrap().borrow_mut())
        } else {
            None
        };
        res
    }

    fn handle_key_press(&mut self, key: crossterm::event::KeyCode) -> AppResult {
        match key {
            crossterm::event::KeyCode::Backspace => todo!(),
            crossterm::event::KeyCode::Enter => todo!(),
            crossterm::event::KeyCode::Left => {
                if self.counter > 0 {
                    self.counter -= 1;
                }
                Ok(())
            }
            crossterm::event::KeyCode::Right => {
                let max = u8::MAX;
                if self.counter < max {
                    self.counter += 1;
                }
                Ok(())
            }
            crossterm::event::KeyCode::Up => {
                self.counter = u8::MAX;
                Ok(())
            }
            crossterm::event::KeyCode::Down => {
                self.counter = 0;
                Ok(())
            }
            crossterm::event::KeyCode::Home => todo!(),
            crossterm::event::KeyCode::End => todo!(),
            crossterm::event::KeyCode::PageUp => todo!(),
            crossterm::event::KeyCode::PageDown => todo!(),
            crossterm::event::KeyCode::Tab => todo!(),
            crossterm::event::KeyCode::BackTab => todo!(),
            crossterm::event::KeyCode::Delete => todo!(),
            crossterm::event::KeyCode::Insert => todo!(),
            crossterm::event::KeyCode::F(_) => todo!(),
            crossterm::event::KeyCode::Char(_) => todo!(),
            crossterm::event::KeyCode::Null => todo!(),
            crossterm::event::KeyCode::Esc => Ok(self.toggle_exit()),
            crossterm::event::KeyCode::CapsLock => todo!(),
            crossterm::event::KeyCode::ScrollLock => todo!(),
            crossterm::event::KeyCode::NumLock => todo!(),
            crossterm::event::KeyCode::PrintScreen => todo!(),
            crossterm::event::KeyCode::Pause => todo!(),
            crossterm::event::KeyCode::Menu => todo!(),
            crossterm::event::KeyCode::KeypadBegin => todo!(),
            crossterm::event::KeyCode::Media(_) => todo!(),
            crossterm::event::KeyCode::Modifier(_) => todo!(),
        }
    }

    fn handle_key_release(&mut self, key: crossterm::event::KeyCode) -> AppResult {
        Ok(())
    }

    fn handle_key_repeat(&mut self, key: crossterm::event::KeyCode) -> AppResult {
        Ok(())
    }

    fn render(&self) {
        if let Some(mut term) = self.get_term() {
            term.draw(|frame| frame.render_widget(self, frame.size()))
                .unwrap();
        }
    }
}

impl Widget for &Application {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let title = Title::from(" Counter App Tutorial ".bold());
        let instructions = Title::from(Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
