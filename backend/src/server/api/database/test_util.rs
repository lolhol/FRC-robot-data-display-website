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

///
/// # Function
/// This function is used to get a database for testing purposes. It creates an empty database if it doesn't exist.
///
/// # Parameters
/// - `min_time_between_cleans`: The minimum time between database cleans.
///
pub fn get_database(min_time_between_cleans: u32) -> SQLiteDatabase {
    if !std::path::Path::new("test.db").exists() {
        let _ = File::create("test.db"); // create an empty file
    }

    let db: SQLiteDatabase = SQLiteDatabase::new("test.db", min_time_between_cleans).unwrap();
    let _ = db.clear_database();
    db
}

///
/// # Function
/// This function is used to put data in the database for testing purposes. It puts a number of test values into the database so you don't have to do that manually.
///
/// # Parameters
/// - `database`: The database that will be used to store the data. It is mutable and it is taken ownership of.
/// - `data_amt`: The amount of data that will be put in the database.
/// - `time_step`: The time step that will be used to put the data in the database.
///
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

///
/// # Function
/// This function is just an easy way to get a Rocket instance without having the write out the whole contents of this function inside the test (which gets very long)
///
/// # Parameters
/// - `db`: The database that will be used to store the data. Again, Arc acts like a mut pointer to the database.
///
/// # Returns
/// A `Rocket<Build>` instance which is basically an instance of a server that can take API HTTP requests. This instance is not running yet but is ready to run.
///
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

///
/// # Function
/// This function is used to manually make a database poisoned. Poisoned means that something crashed while using the Mutex - making the data in the Mutex inaccessible.
/// This simulates that for testing purposes. It has to be in another thread because we don't want to crash the current - main - thread but just want to poison the.
/// database. You can find out more information about the Mutex - https://doc.rust-lang.org/std/sync/struct.Mutex.html
///
/// # Parameters
/// - `db`: The database that will be poisoned
///
/// # Returns
/// Nothing because the Arc is a reference for the database - a kind of mut pointer to the database. You can find more info about this - https://doc.rust-lang.org/std/sync/struct.Arc.html
///
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
