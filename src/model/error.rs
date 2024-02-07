use thiserror::Error;

use super::Args;

#[derive(Debug, Error)]
pub enum ModelError {
    #[error("Non natural arguments: `{0}`")]
    NonNaturalArguments(Args),
    #[error("Function is undefined!")]
    Undefined,
}