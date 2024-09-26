use std::sync::{Arc, Mutex};

use rocket::{serde::json::Json, State};

use crate::database::SQLiteDatabase;

#[delete("/clean-whole-database")]
pub fn clean_whole_database(database: &State<Arc<Mutex<SQLiteDatabase>>>) -> Json<String> {
    let database = database.lock().unwrap();
    let _ = database.clean_database();
    return Json("true".to_string());
}
