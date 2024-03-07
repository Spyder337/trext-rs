#![allow(dead_code)]
use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::{CrosstermBackend, Terminal}, Frame};
use std::io::{stdout, Error, ErrorKind, Result, Stdout};

type AppResult = Result<()>;
type Term = Terminal<CrosstermBackend<Stdout>>;

/**
`BaseApp` contains all of the data necessary to run a terminal
application.

It is to be combined with an implementation of the `App` trait.
*/
struct BaseApp {
    term: Option<Term>,
    exit: bool,
}

impl BaseApp {
    fn new() -> Self {
        Self {
            term: None,
            exit: false,
        }
    }
}

/**
`App` contains all of the function definitions to run a terminal
application. It does need several getters and setters to be implemented
along with
*/
pub trait App {
    //  Getters and Setters
    //  Rust traits can't enforce possessing data types so this is
    //  the best way to ensure implementing traits can access data
    //  that I know of.
    /**
    Indicates if the application can currently exit.
    */
    fn can_exit(&self) -> bool;
    /**
    Toggles the application exiting.
     */
    fn toggle_exit(&mut self);
    /**
    Sets the terminal to the new value.
    */
    fn set_term(&mut self, term: Term) -> Result<bool>;
    /**
    Retruns the application's `term` variable.
    */
    fn get_term(&mut self) -> Result<Option<Term>>;

    //  Terminal Specific Functions
    //  handle_events and handle_key_press will be implemented
    //  in traits that extend this one.
    /**
    Contains logic for `KeyEventKind::Press` events.
    */
    fn handle_key_press(&mut self, key: KeyCode) -> AppResult;
    /**
    Contains logic for `KeyEventKind::Release` events.
    */
    fn handle_key_release(&mut self, key: KeyCode) -> AppResult;
    /**
    Contains logic for `KeyEventKind::Repeat` events.
    */
    fn handle_key_repeat(&mut self, key: KeyCode) -> AppResult;
    /**
    Contains the render logic for the application's frame. 
    
    The logic could contain references to a dictionary of widgets that get optionally 
    rendered or a single widget application.
    */
    fn render(&mut self, frame: &mut Frame);

    //  Core Application Functions
    //  These functions are predefined and not meant to be overwritten.
    //  They control the overall program flow and editing them shouldn't
    //  be necessary in most cases.
    /**
    Initialize the application by setting `term` and running
    `crossterm` init functions to prep the terminal.
    */
    fn init(&mut self) -> Result<bool> {
        execute!(stdout(), EnterAlternateScreen)?;
        enable_raw_mode()?;
        if let Ok(res) = self.set_term(Terminal::new(
            CrosstermBackend::new(stdout())
        )?) {
            Ok(res)
        } else {
            Err(Error::new(
                ErrorKind::InvalidData,
                "Unable to initialize terminal.",
            ))
        }
    }

    /**
    Resets the changes made by `crossterm` to return the terminal to
    normal function.
    */
    fn deinit(&self) -> AppResult {
        execute!(stdout(), LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    /**
    Contains the logic for handling key events.
    */
    fn handle_key_events(&mut self, event: KeyEvent) -> AppResult {
        let res = match event.kind {
            KeyEventKind::Press => self.handle_key_press(event.code),
            KeyEventKind::Repeat => self.handle_key_repeat(event.code),
            KeyEventKind::Release => self.handle_key_release(event.code),
        };
        res
    }

    /**
    Contains the logic for the main event loop. Window selection
    */
    fn handle_events(&mut self, term: &mut Term) -> AppResult {
        let res = match read()? {
            Event::Key(ke) => self.handle_key_events(ke),
            _ => Ok(()),
        };
        res
    }

    /**
    Initializes the application, runs the main loop, then exits.
    */
    fn run(&mut self) -> AppResult {
        //  Check that the program initialized correctly.
        if let Err(err) = self.init() {
            return Err(err);
        }

        //  Check that the terminal was created successfully.
        let term_res = self.get_term();
        if let Err(err) = term_res {
            //  Close out the terminal.
            self.deinit()?;
            return Err(err);
        } 
        else {
            //  If the terminal has a value then run the program.
            if let Some(mut term) = term_res.unwrap() {
                while !self.can_exit() {
                    let ref_term: &mut Term = &mut term;
                    ref_term.draw(|frame| self.render(frame))?;
                    //  TODO: Create error handling for events and application.
                    self.handle_events(ref_term)?;
                }
            }
        }

        //  Close out the terminal.
        self.deinit()?;

        Ok(())
    }
}
