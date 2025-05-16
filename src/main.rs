use app::App;

mod app;
mod models;
mod ui;
mod utils;

fn main() -> color_eyre::Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}
