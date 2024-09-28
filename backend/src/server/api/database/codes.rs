use serde::{Deserialize, Serialize};

///
/// # Function
/// This is essentially an enum that can be parsed from an HTTP request. It contains the error code.
///
// this code essentially says that this enum can be deserialized from a json string
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

///
/// # Function
/// This is essentially an enum that can be parsed from an HTTP request. It signifies success.
///
///
// this code essentially says that this enum can be deserialized from a json string
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum Success {
    DatabaseCleaningSuccess(),
    DatabaseClearingSuccess(),
}
