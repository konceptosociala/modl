pub mod app;
pub mod catch;
pub mod interop;
pub mod logger;
pub mod model;

use app::{config::Config, ModlApp};
use log::info;

use crate::logger::{LoggerLevel, ModlLogger};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_NAME: &str = "Mod-L";

fn main() -> eframe::Result<()> {
    ModlLogger::init_with_level(LoggerLevel::Info);
    info!("Initializing {APP_NAME} v{VERSION}...");

    let config = Config::load_or_default("config.toml");

    eframe::run_native(
        format!("{APP_NAME} v{VERSION}").as_str(),
        eframe::NativeOptions::from(&config),
        Box::new(|cc| Box::new(ModlApp::new(config, cc)))
    )
}