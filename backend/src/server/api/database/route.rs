use std::sync::{Arc, Mutex};

use rocket::{serde::json::Json, State};

use crate::database::{self, structs::table_entree::TableEntree, SQLiteDatabase};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Topic {
    topic: String,
}

#[post("/get_entree", data = "<table_topic>")]
pub fn get_entree(
    table_topic: Json<Topic>, // Use Json to accept POST request data
    database: &State<Arc<Mutex<SQLiteDatabase>>>,
) -> Json<TableEntree> {
    println!("Getting Entree");
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
#[delete("/clean_database")]
pub fn clean_database(database: &State<Arc<Mutex<SQLiteDatabase>>>) {
    println!("Cleaning Database");
    let database = database.lock().unwrap();
    let _ = database.clean_database();
}
