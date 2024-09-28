use std::{
    future::Future,
    sync::{Arc, Mutex},
};

use api::database::{
    clean_whole_db::clean_whole_database, clear_database::clear_database, get_entries::get_entries,
    get_entry::get_entry, get_entry_and_clean::get_entry_and_clean,
};
use rocket::{Config, Ignite, Rocket};

use crate::database::SQLiteDatabase;

mod api;

///
/// # Function
/// This code launches the internal server for the backend. It is used to serve the client with the database API.
///
/// # Parameters
/// - `database_instance`: An `Arc<Mutex<SQLiteDatabase>>` that will be used to communicate with the database. That should be a single instance of the DB.
///
/// # Usage
/// This function is there to simplify the code of the main function. If I were to put the whole code in the main function, it would become too big and unreadable.
///
/// # Returns
/// An `impl Future` that returns a `Result<Rocket<Ignite>, rocket::Error>`
/// - The rocket server can ONLY RUN IN MULTITHREAD ENV.
/// - Essentially a future that you have to await / attach to make the server actually run
///
/// # Await Usage
/// Awaits in rust require you to "ping" the future every so often. This is the function of .await. However, the problem with this is that it freezes the whole current thread.
/// To fix this, you can attach two different Futures together and wait on that (pinging both processes).
///
/// # Thread Documentation
/// https://doc.rust-lang.org/book/ch20-02-multithreaded.html
///
pub fn rocket_launch(
    database_instance: &Arc<Mutex<SQLiteDatabase>>,
    port: u16,
) -> impl Future<Output = Result<Rocket<Ignite>, rocket::Error>> {
    let database_instance = database_instance.clone();
    let config = Config {
        port,                // Set the desired port here
        ..Config::default()  // Use the default configuration for other settings
    };
    rocket::custom(config)
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
