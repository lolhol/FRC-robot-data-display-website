use std::sync::{Arc, Mutex};

use api::database::route::{clean_database, get_entree};
use tokio::spawn;

use crate::database::SQLiteDatabase;

mod api;

pub fn rocket_launch(database_instance: Arc<Mutex<SQLiteDatabase>>) -> tokio::task::JoinHandle<()> {
    spawn(async move {
        let database_instance = database_instance.clone();
        rocket::build()
            .manage(database_instance)
            .mount("/", routes![get_entree, clean_database])
            .launch()
            .await
            .expect("Rocket failed to launch");
    })
}
