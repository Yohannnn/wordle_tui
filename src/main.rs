mod app;
use anyhow::Result;
fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let mut app = app::App::new();
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}
