use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum Error {
    DatabasePoisonedError(i32),
    DatabaseInvalidAmountError(i32),
}

impl Error {
    pub fn new(&self) -> Self {
        match self {
            Error::DatabasePoisonedError(_) => Error::DatabasePoisonedError(0),
            Error::DatabaseInvalidAmountError(_) => Error::DatabaseInvalidAmountError(1),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum Success {
    DatabaseCleaningSuccess(),
    DatabaseClearingSuccess(),
}
