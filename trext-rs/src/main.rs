mod app;

use app::Application;
use common::tui::App;
use std::io::Result;

fn main() -> Result<()> {
    let mut app = Application::new();
    app.run()?;
    Ok(())
}
