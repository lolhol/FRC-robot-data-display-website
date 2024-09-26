use std::{
    env,
    sync::{Arc, Mutex},
};

use colored::Colorize;
use tokio::signal;

#[macro_use]
extern crate rocket;

mod database;
mod network_table_bridge;
mod server;

#[tokio::main()]
async fn main() {
    // runs the main code.
    let local_set = tokio::task::LocalSet::new();
    let database = Arc::new(Mutex::new(
        database::SQLiteDatabase::new(
            &env::var("DATABASE_PATH").unwrap(),
            env::var("DATABASE_MIN_TIME_AFTER_UPDATE")
                .unwrap()
                .parse()
                .unwrap(),
        )
        .unwrap(),
    ));

    let server_task = server::rocket_launch(database.clone());

    let table_task = local_set.run_until(async move {
        let database = Arc::clone(&database);
        let _ = database.lock().unwrap().clear_database();
        let _ = network_table_bridge::begin_network_table(
            env::var("NETWORK_TABLE_IP").unwrap(),
            env::var("NETWORK_TABLE_PORT").unwrap().parse().unwrap(),
            env::var("TIME_BETWEEN_RECONNECT_ATTEMPTS")
                .unwrap()
                .parse()
                .unwrap(),
            Box::new(move |data| network_table_bridge::write_all(data, database.clone())),
        )
        .await;
    });

    let shutdown_signal = async {
        signal::ctrl_c()
            .await
            .expect(&"Failed to listen for shutdown signal (Ctrl+C)".red());
        println!("Received shutdown signal. Shutting down...");
    };

    tokio::select! {
        _ = table_task => {
            println!("{}", "Table task shut down!".red());
        }
        _ = server_task => {
            println!("{}", "Backend server task shut down!".red());
        }
        _ = shutdown_signal => {
            println!("{}", "Gracefully shutting down Rocket server and network table tasks.".green());
        }
    };

    println!("Shut down complete.");

    ()
}
