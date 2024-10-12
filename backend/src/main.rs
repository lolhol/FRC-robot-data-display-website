use std::{
    env,
    sync::{Arc, Mutex},
};

use colored::Colorize;
use tokio::signal;

#[macro_use]
extern crate rocket;
extern crate dotenv;

mod database;
mod network_table_bridge;
mod server;

use dotenv::dotenv;

///
/// # Function
/// This is the main function. It is essentially the entree point of the whole program as a whole.
///
/// # Usage
/// `cargo run`
///
/// # Returns
/// Nothing
///
/// # Parameters
/// None
///
/// # Rust Docs
/// You can see all the rust docs here - https://doc.rust-lang.org/book/
#[tokio::main()]
async fn main() {
    let local_set = tokio::task::LocalSet::new(); // Local set is a thing that allows you to .await multiple things at the same time in a single thread
    dotenv().ok(); // load the .env file

    let invalid_envs = get_invalid_env_list();
    if !invalid_envs.is_empty() {
        println!(
            "{}",
            format!(
                "The following environment variables are not set: {}. Please set them and try again.",
                invalid_envs.join(", ")
            )
            .red()
        );

        return ();
    }

    let database = database::SQLiteDatabase::new(
        &env::var("DATABASE_PATH").unwrap(),
        env::var("DATABASE_MIN_TIME_AFTER_UPDATE")
            .unwrap()
            .parse()
            .unwrap(),
    );

    if database.is_err() {
        println!("{}", "Failed to initialize database. Shutting down.".red());
        return ();
    }

    let database = Arc::new(Mutex::new(database.unwrap())); // Arc -> allows multiple pointers to one instance in multiprocessing environments, Mutex -> allows writing safely in a multiprocessing environment
    let server_task =
        server::rocket_launch(&database, env::var("SERVER_PORT").unwrap().parse().unwrap()); // get the rocket server start instance
    let table_task = local_set.run_until(async move /* move essentially means that all variables used inside this async function are owned by this async function are moved from the outside */ {
        // get the network table start instance
        network_table_bridge::begin_network_table(
            env::var("NETWORK_TABLE_IP").unwrap(),
            env::var("NETWORK_TABLE_PORT").unwrap().parse().unwrap(),
            env::var("TIME_BETWEEN_RECONNECT_ATTEMPTS")
                .unwrap()
                .parse()
                .unwrap(),
            Box::new(move |data, database| network_table_bridge::write_all(data, database.clone())),
            database,
        )
        .await // awaiting the network table start instance
    });

    tokio::select! { // Essentially allows you to await multiple tasks at the same time (usually not possible)
        _ = table_task => {
            println!("{}", "Table task shut down!".red());
        }
        _ = server_task => {
            println!("{}", "Backend server task shut down!".red());
        }
        _ = async { // wait for a control c signal to shut down 100% no matter where the other processes are at
            // https://docs.rs/tokio/latest/tokio/signal/fn.ctrl_c.html
            signal::ctrl_c()
                .await
                .expect(&"Failed to listen for shutdown signal (Ctrl+C)".red());

            println!("Received shutdown signal. Shutting down...");
        } => {
            println!("{}", "Gracefully shutting down Rocket server and network table tasks.".green());
        }
    };

    println!("Shut down complete.");

    return ();
}

fn get_invalid_env_list() -> Vec<String> {
    let envs: Vec<&str> = vec![
        "DATABASE_PATH",
        "DATABASE_MIN_TIME_AFTER_UPDATE",
        "SERVER_PORT",
        "NETWORK_TABLE_IP",
        "NETWORK_TABLE_PORT",
        "TIME_BETWEEN_RECONNECT_ATTEMPTS",
    ];

    let mut invalid_envs = Vec::new();
    for env in envs {
        if env::var(env).is_err()
            || (is_supposed_to_be_number(env) && env::var(env).unwrap().parse::<u16>().is_err())
        {
            invalid_envs.push(env.to_string());
        }
    }

    return invalid_envs;
}

fn is_supposed_to_be_number(s: &str) -> bool {
    let number_envs = vec![
        "DATABASE_MIN_TIME_AFTER_UPDATE",
        "NETWORK_TABLE_PORT",
        "TIME_BETWEEN_RECONNECT_ATTEMPTS",
        "SERVER_PORT",
    ];

    return number_envs.contains(&s);
}
