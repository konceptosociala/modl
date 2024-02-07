use std::path::Path;

use eframe::{NativeOptions, Theme};
use log::warn;
use serde::{Deserialize, Serialize};

use crate::catch::CatchError;

use super::{error::AppError, i18n::{I18n, Lang}};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum AppTheme {
    #[default]
    Dark,
    Light,
}

impl From<AppTheme> for Theme {
    fn from(theme: AppTheme) -> Self {
        use AppTheme::*;

        match theme {
            Dark => Theme::Dark,
            Light => Theme::Light,
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum TopBar {
    #[default]
    Builtin,
    Native,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub theme: AppTheme,
    pub top_bar: TopBar,
    pub lang: Lang,
    #[serde(skip)]
    pub i18n: I18n,
}

impl Config {
    pub fn load_or_default(path: impl AsRef<Path>) -> Config {
        if let Ok(config) = Config::load(path.as_ref()) {
            return config;
        }

        warn!("Config `{}` is not initialized", path.as_ref().to_str().unwrap());
        let mut config = Config::default();
        std::fs::write(path.as_ref(), toml::to_string_pretty(&config).unwrap()).catch();

        config.i18n = I18n::new(&config.lang);

        config
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Config, AppError> {
        let s = std::fs::read_to_string(path.as_ref()).map_err(|e| AppError::ConfigError {
            config_path: path.as_ref().to_owned(), 
            error: e.to_string(),
        })?;

        let mut config: Config = toml::from_str(&s).map_err(|e| AppError::ConfigError {
            config_path: path.as_ref().to_owned(), 
            error: e.to_string(),
        })?;

        config.i18n = I18n::new(&config.lang);

        Ok(config)
    }
}

impl From<&Config> for NativeOptions {
    fn from(config: &Config) -> Self {
        NativeOptions {
            follow_system_theme: false,
            default_theme: config.theme.clone().into(),
            centered: true,
            ..Default::default()
        }
    }
}