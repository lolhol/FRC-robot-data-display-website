use std::sync::{Arc, Mutex};

use rocket::{serde::json::Json, State};

use crate::{
    database::{self, structs::table_entree::TableEntree, SQLiteDatabase},
    server::api::database::data_struct::Topic,
};

use super::codes;

///
/// # Function
/// Gets entries from the database based on the provided topic. This is one of the api endpoints that you can call from the frontend.
///
/// # Parameters
/// - `table_topic`: A `Json<Topic>` that contains the topic to get from the database
/// - `database`: The database that will be used to get the data
///     - note that the database param is passed into the function by default
///
/// # Docs
/// See more about how Rocket server works here - https://api.rocket.rs/
///

// This code essentially means that the "get_entries" function will be called when you make an api request to the /get-entries endpoint.
#[post("/get-entries", data = "<table_topic>")]
pub fn get_entries(
    table_topic: Json<Topic>,
    database: &State<Arc<Mutex<SQLiteDatabase>>>,
) -> Json<Result<Vec<TableEntree>, codes::Error>> {
    let database = database.lock();

    if database.is_err() {
        return Json(Err(codes::Error::new(
            &codes::Error::DatabasePoisonedError(-1),
        )));
    }

    if table_topic.amount.is_none() {
        return Json(Err(codes::Error::new(
            &codes::Error::DatabaseInvalidAmountError(-1),
        )));
    }

    let database = database.unwrap();
    let amount = table_topic.amount.unwrap();
    let values = if let Some(time) = table_topic.time_since_last_update {
        database.get_values(&table_topic.topic, time, amount)
    } else {
        database.get_values_no_time(&table_topic.topic, amount)
    };

    Json(Ok(values.unwrap_or(vec![])))
}

#[cfg(test)]
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
            .post("/get-entries")
            .header(ContentType::JSON)
            .body(r#"{"topic":"non_existent_topic"}"#)
            .dispatch();

        let body = response.into_string().unwrap();
        let expected: Result<Success, codes::Error> =
            Err(codes::Error::new(&codes::Error::DatabasePoisonedError(-1)));

        let expected_error = serde_json::to_string(&expected).unwrap();
        assert_eq!(body, expected_error);
    }

    #[test]
    #[serial_test::serial]
    fn test_simulate_invalid_amount_time() {
        let database = Arc::new(Mutex::new(test_util::put_data_in_database(
            test_util::get_database(2),
            5,
            1,
        )));
        let rocket = test_util::get_rocket_build(database.clone());
        let client = Client::tracked(rocket).expect("valid rocket instance");

        let response = client
            .post("/get-entries")
            .header(ContentType::JSON)
            .body(r#"{"topic":"non_existent_topic"}"#)
            .dispatch();

        let body = response.into_string().unwrap();
        let expected: Result<Success, codes::Error> = Err(codes::Error::new(
            &codes::Error::DatabaseInvalidAmountError(-1),
        ));

        let expected_error = serde_json::to_string(&expected).unwrap();
        assert_eq!(body, expected_error);
    }

    #[test]
    #[serial_test::serial]
    fn test_simulate_get_amount_no_time() {
        let database = Arc::new(Mutex::new(test_util::put_data_in_database(
            test_util::get_database(2),
            5,
            1,
        )));
        let rocket = test_util::get_rocket_build(database.clone());
        let client = Client::tracked(rocket).expect("valid rocket instance");

        let response = client
            .post("/get-entries")
            .header(ContentType::JSON)
            .body(r#"{"topic": "test", "amount": 3}"#)
            .dispatch();

        let body = response.into_string().unwrap();
        let expected: Result<Vec<TableEntree>, codes::Error> = Ok(vec![
            TableEntree::new("test".to_string(), "test".to_string(), 4),
            TableEntree::new("test".to_string(), "test".to_string(), 3),
            TableEntree::new("test".to_string(), "test".to_string(), 2),
        ]);

        assert_eq!(body, serde_json::to_string(&expected).unwrap());
    }

    #[test]
    #[serial_test::serial]
    fn test_simulate_get_amount_with_time() {
        let database = Arc::new(Mutex::new(test_util::put_data_in_database(
            test_util::get_database(2),
            5,
            1,
        )));
        let rocket = test_util::get_rocket_build(database.clone());
        let client = Client::tracked(rocket).expect("valid rocket instance");

        let response = client
            .post("/get-entries")
            .header(ContentType::JSON)
            .body(r#"{"topic": "test", "amount": 5, "time_since_last_update": 4}"#)
            .dispatch();

        let body = response.into_string().unwrap();
        let expected: Result<Vec<TableEntree>, codes::Error> = Ok(vec![
            TableEntree::new("test".to_string(), "test".to_string(), 4),
            TableEntree::new("test".to_string(), "test".to_string(), 3),
            TableEntree::new("test".to_string(), "test".to_string(), 2),
            TableEntree::new("test".to_string(), "test".to_string(), 1),
            TableEntree::new("test".to_string(), "test".to_string(), 0),
        ]);

        assert_eq!(body, serde_json::to_string(&expected).unwrap());
    }
}
