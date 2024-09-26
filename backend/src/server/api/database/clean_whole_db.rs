use std::sync::{Arc, Mutex};

use rocket::{serde::json::Json, State};

use crate::database::{self, SQLiteDatabase};

use super::codes::{self, Success};

#[delete("/clean-whole-database")]
pub fn clean_whole_database(
    database: &State<Arc<Mutex<SQLiteDatabase>>>,
) -> Json<Result<Success, codes::Error>> {
    let database = database.lock();

    if database.is_err() {
        return Json(Err(codes::Error::new(
            &codes::Error::DatabasePoisonedError(-1),
        )));
    }

    let _ = database.unwrap().clean_database();

    return Json(Ok(codes::Success::DatabaseCleaningSuccess()));
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

        let response = client.delete("/clean-whole-database").dispatch();

        let body = response.into_string().unwrap();
        let expected: Result<Success, codes::Error> =
            Err(codes::Error::new(&codes::Error::DatabasePoisonedError(-1)));

        let expected_error = serde_json::to_string(&expected).unwrap();
        assert_eq!(body, expected_error);
    }

    #[test]
    #[serial_test::serial]
    fn test_clean_database() {
        let database = Arc::new(Mutex::new(test_util::put_data_in_database(
            test_util::get_database(0),
            5,
            1,
        )));
        let rocket = test_util::get_rocket_build(database.clone());
        let client = Client::tracked(rocket).expect("valid rocket instance");

        let response = client.delete("/clean-whole-database").dispatch();

        let body = response.into_string().unwrap();
        let expected: Result<Success, codes::Error> = Ok(codes::Success::DatabaseCleaningSuccess());

        let expected_error = serde_json::to_string(&expected).unwrap();
        assert_eq!(body, expected_error);
        assert_eq!(database.lock().unwrap().length().unwrap(), 0);
    }
}
