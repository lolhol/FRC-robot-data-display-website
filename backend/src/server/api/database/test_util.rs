use std::{
    fs::File,
    panic,
    sync::{Arc, Mutex},
    thread::spawn,
};

use colored::Colorize;
use rocket::{Build, Rocket};

use crate::database::{structs::table_entree::TableEntree, SQLiteDatabase};

use super::{
    clean_whole_db::clean_whole_database, clear_database::clear_database, get_entries::get_entries,
    get_entry::get_entry, get_entry_and_clean::get_entry_and_clean,
};

pub fn get_database(min_time_between_cleans: u32) -> SQLiteDatabase {
    if !std::path::Path::new("test.db").exists() {
        let _ = File::create("test.db"); // create an empty file
    }

    let db: SQLiteDatabase = SQLiteDatabase::new("test.db", min_time_between_cleans).unwrap();
    let _ = db.clear_database();
    db
}

pub fn put_data_in_database(
    mut database: SQLiteDatabase,
    data_amt: u32,
    time_step: u32,
) -> SQLiteDatabase {
    for i in 0..data_amt {
        let _ = database.add_value(TableEntree::new(
            "test".to_string(),
            "test".to_string(),
            i * time_step,
        ));
    }

    database
}

pub fn get_rocket_build(db: Arc<Mutex<SQLiteDatabase>>) -> Rocket<Build> {
    rocket::build().manage(db).mount(
        "/",
        routes![
            get_entry_and_clean,
            get_entries,
            get_entry,
            clear_database,
            clean_whole_database
        ],
    )
}

pub fn make_db_poisoned(db: Arc<Mutex<SQLiteDatabase>>) {
    let handle = spawn(move || {
        let a = db.lock();
        panic!(
            "{}",
            "This is OK! I'm going to panic. (intended panic)".green()
        );
    });

    let _ = handle.join();
}
