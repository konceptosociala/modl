use std::fmt;

use std::format as f;
use log::error;
use serde::{Deserialize, Serialize};
use toml::map::Map;

use super::error::AppError;

#[macro_export]
macro_rules! i18n {
    ($config:expr, $lit:expr) => {
        {
            match $config.i18n.get($lit) {
                Some(v) => v,
                None => {
                    ::log::warn!("Key `{}` not found", $lit);
                    String::new()
                },
            }
        }
    };
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum Lang {
    #[default]
    English,
    Ukrainian,
    Esperanto,
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Lang::English => write!(f, "en"),
            Lang::Ukrainian => write!(f, "ua"),
            Lang::Esperanto => write!(f, "eo"),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct I18n {
    lang: Lang,
    table: toml::Table,
}

impl I18n {
    pub fn new(lang: &Lang) -> I18n {
        let mut i18n = I18n {
            lang: lang.clone(),
            table: Map::new(),
        };

        if let Err(e) = i18n.load(lang) {
            error!("{e}");
        }

        i18n
    }

    pub fn load(&mut self, lang: &Lang) -> Result<(), AppError> {
        let s = std::fs::read_to_string(f!("i18n/{lang}.toml"))
            .map_err(|e| AppError::I18nError {
                lang: lang.clone(), 
                error: e.to_string() 
            })?;

        self.table = s.parse::<toml::Table>()
            .map_err(|e| AppError::I18nError {
                lang: lang.clone(), 
                error: e.to_string() 
            })?;

        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<String> {
        if let Some(v) = self.table.get(key) {
            return match v.as_str() {
                Some(s) => Some(s.to_owned()),
                _ => Some(v.to_string()),
            };
        }

        None
    }

    pub fn lang(&self) -> Lang {
        self.lang.clone()
    }
}