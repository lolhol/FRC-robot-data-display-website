use std::sync::{Arc, Mutex};

use rocket::{serde::json::Json, State};

use crate::{
    database::{structs::table_entree::TableEntree, SQLiteDatabase},
    server::api::database::data_struct::Topic,
};

#[post("/get-entries", data = "<table_topic>")]
pub fn get_entries(
    table_topic: Json<Topic>,
    database: &State<Arc<Mutex<SQLiteDatabase>>>,
) -> Json<Vec<TableEntree>> {
    let database = database.lock().unwrap();

    if table_topic.amount.is_none() {
        return Json(vec![TableEntree::get_error()]);
    }

    let values = if let Some(time) = table_topic.time_since_last_update {
        database.get_values(&table_topic.topic, time, table_topic.amount.unwrap())
    } else {
        database.get_values_no_time(&table_topic.topic, table_topic.amount.unwrap())
    };

    Json(values.unwrap_or(vec![TableEntree::get_error()]))
}
