use std::error::Error;

pub trait CatchError {
    fn catch(&self);

    fn catch_warn(&self);
}

impl<E: Error> CatchError for Result<(), E> {
    fn catch(&self) {
        if let Err(e) = self {
            ::log::error!("{e}");
        }
    }

    fn catch_warn(&self) {
        if let Err(e) = self {
            ::log::warn!("{e}");
        }
    }
}