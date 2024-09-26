use std::sync::{Arc, Mutex};

use rocket::{serde::json::Json, State};

use crate::database::SQLiteDatabase;

#[delete("/clear-database")]
pub fn clear_database(database: &State<Arc<Mutex<SQLiteDatabase>>>) -> Json<String> {
    let _ = database.lock().unwrap().clear_database();
    return Json("true".to_string());
}
