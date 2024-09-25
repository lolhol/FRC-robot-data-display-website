use std::sync::{Arc, Mutex};

use rocket::State;

use crate::database::{SQLiteDatabase};

#[delete("/clean-whole-database")]
pub fn clean_whole_database(database: &State<Arc<Mutex<SQLiteDatabase>>>) {
    let database = database.lock().unwrap();
    let _ = database.clean_database();
}
