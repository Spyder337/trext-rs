use std::error;

use crossterm::event::{KeyEvent as CrosstermKeyEvent, MouseEvent};
use ratatui::{backend::Backend, Terminal};
use tokio::sync::mpsc;

pub mod app;
pub mod events;
pub mod program;
pub mod tui;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Terminal events.
#[derive(Clone, Copy, Debug)]
pub enum Event {
    /// Terminal tick.
    Tick,
    /// Key press.
    Key(CrosstermKeyEvent),
    /// Mouse click/scroll.
    Mouse(MouseEvent),
    /// Terminal resize.
    Resize(u16, u16),
}

/// Terminal event handler.
#[allow(dead_code)]
#[derive(Debug)]
pub struct EventHandler {
    /// Event sender channel.
    sender: mpsc::UnboundedSender<Event>,
    /// Event receiver channel.
    receiver: mpsc::UnboundedReceiver<Event>,
    /// Event handler thread.
    handler: tokio::task::JoinHandle<()>,
}

pub trait Renderer {
    fn render(&mut self, frame: &mut ratatui::prelude::Frame<'_>);
}

pub trait KeyEventHandler {
    fn handle_key_event(&mut self, ke: CrosstermKeyEvent) -> AppResult<()>;
}

pub trait Executable {
    fn is_running(&self) -> bool;
    fn tick(&self) {}
}


/// Default counter application example.
/// Just plug it into a [`Program`] and run it.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,
}

/**
Default example of a program using the [`ratatui`] library.
It contains an [`App`] struct that contains the data for the application
and also implements the [`Renderer`], [`KeyEventHandler`], [`Executable`] traits.
*/
pub struct Program<T: Renderer + KeyEventHandler + Executable> {
    app: T,
}

/// Representation of a terminal user interface.
///
/// It is responsible for setting up the terminal,
/// initializing the interface and handling the draw events.
#[derive(Debug)]
pub struct Tui<B: Backend> {
    /// Interface to the Terminal.
    terminal: Terminal<B>,
    /// Terminal event handler.
    pub events: EventHandler,
}