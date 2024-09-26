use std::sync::{Arc, Mutex};

use api::database::{
    clean_whole_db::clean_whole_database, clear_database::clear_database, get_entries::get_entries,
    get_entry::get_entry, get_entry_and_clean::get_entree_and_clean,
};
use tokio::spawn;

use crate::database::SQLiteDatabase;

mod api;

pub fn rocket_launch(database_instance: Arc<Mutex<SQLiteDatabase>>) -> tokio::task::JoinHandle<()> {
    spawn(async move {
        let database_instance = database_instance.clone();
        rocket::build()
            .manage(database_instance)
            .mount(
                "/",
                routes![
                    get_entry,
                    clean_whole_database,
                    get_entree_and_clean,
                    get_entries,
                    clear_database
                ],
            )
            .launch()
            .await
            .expect("Rocket failed to launch");
    })
}
