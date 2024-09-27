use std::{
    future::Future,
    sync::{Arc, Mutex},
};

use api::database::{
    clean_whole_db::clean_whole_database, clear_database::clear_database, get_entries::get_entries,
    get_entry::get_entry, get_entry_and_clean::get_entry_and_clean,
};
use rocket::{Ignite, Rocket};

use crate::database::SQLiteDatabase;

mod api;

pub fn rocket_launch(
    database_instance: &Arc<Mutex<SQLiteDatabase>>,
) -> impl Future<Output = Result<Rocket<Ignite>, rocket::Error>> {
    let database_instance = database_instance.clone();
    rocket::build()
        .manage(database_instance)
        .mount(
            "/",
            routes![
                get_entry,
                clean_whole_database,
                get_entry_and_clean,
                get_entries,
                clear_database
            ],
        )
        .launch()
}
