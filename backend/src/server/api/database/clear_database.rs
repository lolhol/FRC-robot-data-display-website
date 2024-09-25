use std::sync::{Arc, Mutex};

use rocket::State;

use crate::database::{SQLiteDatabase};

#[delete("/clear-database")]
pub fn clear_database(database: &State<Arc<Mutex<SQLiteDatabase>>>) {
    let _ = database.lock().unwrap().clear_database();
}
