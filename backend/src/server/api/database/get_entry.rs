use std::sync::{Arc, Mutex};

use rocket::{serde::json::Json, State};

use crate::{
    database::{structs::table_entree::TableEntree, SQLiteDatabase},
    server::api::database::data_struct::Topic,
};

use super::codes;

///
/// # Function
/// Gets an entry from the database based on the provided topic. This is one of the api endpoints that you can call from the frontend.
///
/// # Parameters
/// - `topic`: A `String` that contains the topic to get from the database
/// - `database`: The database that will be used to get the data
///     - note that the database param is passed into the function by default
///
#[get("/get-entry?<topic>")]
pub fn get_entry(
    topic: String,
    database: &State<Arc<Mutex<SQLiteDatabase>>>,
) -> Json<Result<Option<TableEntree>, codes::Error>> {
    let database = database.lock();

    if database.is_err() {
        return Json(Err(codes::Error::new(
            &codes::Error::DatabasePoisonedError(-1),
        )));
    }

    let database = database.unwrap();
    Json(Ok(if let Ok(topic_value) = database.get_value(&topic) {
        Some(topic_value)
    } else {
        None
    }))
}
#[cfg(test)]
/// # Function
/// This function is used to test the get_entry function
mod tests {
    use std::{os::unix::thread, thread::spawn};

    use codes::Success;
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

        let response = client
            .get(test_util::to_get_request(
                Topic {
                    topic: "abc/non_existent_topic".to_string(),
                    amount: None,
                    time_since_last_update: None,
                },
                "/get-entry",
            ))
            .header(ContentType::JSON)
            .dispatch();

        let body = response.into_string().unwrap();
        let expected: Result<Success, codes::Error> =
            Err(codes::Error::new(&codes::Error::DatabasePoisonedError(-1)));

        let expected_error = serde_json::to_string(&expected).unwrap();
        assert_eq!(body, expected_error);
    }

    #[test]
    #[serial_test::serial]
    fn test_get_value() {
        let database = Arc::new(Mutex::new(test_util::put_data_in_database(
            test_util::get_database(2),
            5,
            1,
        )));
        let rocket = test_util::get_rocket_build(database.clone());
        let client = Client::tracked(rocket).expect("valid rocket instance");

        let response = client
            .get(test_util::to_get_request(
                Topic {
                    topic: "test".to_string(),
                    amount: None,
                    time_since_last_update: None,
                },
                "/get-entry",
            ))
            .header(ContentType::JSON)
            .dispatch();

        let body = response.into_string().unwrap();

        let expected: Result<Option<TableEntree>, codes::Error> = Ok(Some(TableEntree::new(
            "test".to_string(),
            "test".to_string(),
            4,
        )));

        assert_eq!(body, serde_json::to_string(&expected).unwrap());
    }
}
