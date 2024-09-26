use std::sync::{Arc, Mutex};

use rocket::{serde::json::Json, State};

use crate::{
    database::{structs::table_entree::TableEntree, SQLiteDatabase},
    server::api::database::data_struct::Topic,
};

#[post("/get-entry-and-clean", data = "<table_topic>")]
pub fn get_entree_and_clean(
    table_topic: Json<Topic>,
    database: &State<Arc<Mutex<SQLiteDatabase>>>,
) -> Json<TableEntree> {
    let database = database.lock().unwrap();

    let topic_value = database
        .get_value(&table_topic.topic)
        .unwrap_or(TableEntree::new(
            "ERROR".to_string(),
            "ERROR".to_string(),
            u32::MIN,
        ));
    let _ = database.clean_database();

    Json(topic_value)
}
