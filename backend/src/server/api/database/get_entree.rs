use std::sync::{Arc, Mutex};

use rocket::{serde::json::Json, State};

use crate::{
    database::{structs::table_entree::TableEntree, SQLiteDatabase},
    server::api::database::data_struct::Topic,
};

#[post("/get-entree", data = "<table_topic>")]
pub fn get_entree(
    table_topic: Json<Topic>, // Use Json to accept POST request data
    database: &State<Arc<Mutex<SQLiteDatabase>>>,
) -> Json<TableEntree> {
    let database = database.lock().unwrap();

    Json(
        database
            .get_value(&table_topic.topic)
            .unwrap_or(TableEntree::new(
                "ERROR".to_string(),
                "ERROR".to_string(),
                u32::MIN,
            )),
    )
}
