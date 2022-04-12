#[cfg(feature = "crossterm")]
mod crossterm;
#[cfg(feature = "termion")]
mod termion;
mod ui;
mod app;
mod controller;
mod storage;
mod event;
mod handler;

use argh::FromArgs;
use std::{error::Error, time::Duration};
use std::sync::Arc;
use crate::app::App;
use crate::controller::Manager;


/// Demo
#[derive(Debug, FromArgs)]
struct Cli {
    /// time in ms between two ticks.
    #[argh(option, default = "250")]
    tick_rate: u64,
    /// whether unicode symbols are used to improve the overall look of the app
    #[argh(option, default = "true")]
    enhanced_graphics: bool,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tui_logger::init_logger(log::LevelFilter::Info).unwrap();
    tui_logger::set_default_level(log::LevelFilter::Info);
    let cli: Cli = argh::from_env();

    let db = storage::shared_storage();

    let app = Arc::new(tokio::sync::Mutex::new(App::new(db.clone())));
    let manager = Manager::new(db).await;
    {
        let _ = &manager.check_connection().await;
    }

    //
    //
    tokio::select! {
        _ = ui::start_ui(app) => {},
        _ =  manager.run() => {}
    }
    Ok(())
}