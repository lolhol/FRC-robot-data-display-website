use std::sync::{Arc, Mutex};

use rocket::{serde::json::Json, State};

use crate::database::{self, SQLiteDatabase};

use super::codes::{self, Success};

///
/// # Function
/// This function will clear the database
///
/// # Parameters
/// - `database`: The database that will be used to clear the database
///     - note that the database param is passed into the function by default
///
/// # Docs
/// See more about how Rocket server works here - https://api.rocket.rs/
///
#[delete("/clear-database")]
pub fn clear_database(
    database: &State<Arc<Mutex<SQLiteDatabase>>>,
) -> Json<Result<Success, codes::Error>> {
    let database = database.lock();
    if database.is_err() {
        return Json(Err(codes::Error::new(
            &codes::Error::DatabasePoisonedError(-1),
        )));
    }

    let _ = database.unwrap().clear_database();

    return Json(Ok(codes::Success::DatabaseClearingSuccess()));
}

#[cfg(test)]
mod tests {
    use std::{os::unix::thread, thread::spawn};

    use rocket::{http::ContentType, local::blocking::Client};

    use crate::server::api::database::test_util;

    use super::*;

    #[test]
    #[serial_test::serial]
    fn test_simulate_poison() {
        let database = Arc::new(Mutex::new(test_util::put_data_in_database(
            test_util::get_database(2),
            5,
            1,
        )));
        let rocket = test_util::get_rocket_build(database.clone());

        let client = Client::tracked(rocket).expect("valid rocket instance");

        test_util::make_db_poisoned(database);

        let response = client.delete("/clear-database").dispatch();

        let body = response.into_string().unwrap();
        let expected: Result<Success, codes::Error> =
            Err(codes::Error::new(&codes::Error::DatabasePoisonedError(-1)));

        let expected_error = serde_json::to_string(&expected).unwrap();
        assert_eq!(body, expected_error);
    }

    #[test]
    #[serial_test::serial]
    fn test_clear_database() {
        let database = Arc::new(Mutex::new(test_util::put_data_in_database(
            test_util::get_database(2),
            5,
            1,
        )));
        let rocket = test_util::get_rocket_build(database.clone());
        let client = Client::tracked(rocket).expect("valid rocket instance");

        let response = client.delete("/clear-database").dispatch();

        let body = response.into_string().unwrap();
        let expected: Result<Success, codes::Error> = Ok(codes::Success::DatabaseClearingSuccess());

        let expected_error = serde_json::to_string(&expected).unwrap();
        assert_eq!(body, expected_error);
        assert_eq!(database.lock().unwrap().length().unwrap(), 0);
    }
}
