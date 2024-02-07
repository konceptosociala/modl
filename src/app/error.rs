use std::path::PathBuf;

use thiserror::Error;

use super::i18n::Lang;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Error loading config `{config_path}`: {error}")]
    ConfigError {
        config_path: PathBuf,
        error: String,
    },
    #[error("Error loading localization for `{lang}` language: {error}")]
    I18nError {
        lang: Lang,
        error: String,
    },
}