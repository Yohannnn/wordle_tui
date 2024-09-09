mod app;
mod tui;

use anyhow::Result;
fn main() -> Result<()> {
    let mut terminal = tui::init()?;
    let mut app = app::App::new();
    let app_result = app.run(&mut terminal);
    tui::restore()?;
    app_result
}
