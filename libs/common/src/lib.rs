use gui::{EventHandler, Executable, KeyEventHandler, Renderer};
use ratatui::{backend::Backend, Terminal};

pub mod buffers;
pub mod gui;
pub mod tests;

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
