use std::{
    future::Future,
    net::{Ipv4Addr, SocketAddrV4},
    str::FromStr,
    sync::{Arc, Mutex},
};

use network_tables::v4::{MessageData, SubscriptionOptions};
use tokio::task::spawn_local;

use crate::database::{self, structs::table_entree::TableEntree, SQLiteDatabase};

/// # Function
/// This function is used to connect to the network table and to keep the data in sync. It will periodically try to reconnect if it fails to connect.
/// This is needed because, again, the "main" function would be too long if I were to put the contents of this function inside it
///
/// # Parameters
/// - `url`: The url of the network table
/// - `port`: The port of the network table
/// - `time_between_reconnects`: The time between reconnect attempts in milliseconds
/// - `function_to_call`: The function that will be called when a new message is received
/// - `database`: The database that will be used to store the data
///
/// # Returns
/// A `tokio::task::JoinHandle<()>`. This is essentially something you can .await with tokio crate - that provides you with a way to wait for the task to finish in a local setting.
///
/// # Await Usage
/// Awaits in rust require you to "ping" the future every so often. This is the function of .await. However, the problem with this is that it freezes the whole current thread.
/// To fix this, you can attach two different Futures together and wait on that (pinging both processes).
///
/// # Await Documentation
/// https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html
///
pub fn begin_network_table(
    url: String,
    port: i32,
    time_between_reconnects: u64,
    function_to_call: Box<dyn Fn(MessageData, Arc<Mutex<SQLiteDatabase>>) -> ()>,
    database: Arc<Mutex<SQLiteDatabase>>,
) -> tokio::task::JoinHandle<()> {
    spawn_local(async move {
        println!("Starting NetworkTable Bridge");
        let mut first = true;
        loop {
            if !first {
                tokio::time::sleep(tokio::time::Duration::from_millis(time_between_reconnects))
                    .await;
            }

            let client = network_tables::v4::Client::try_new_w_config(
                SocketAddrV4::new(Ipv4Addr::from_str(&url).unwrap(), port as u16),
                network_tables::v4::client_config::Config {
                    ..Default::default()
                },
            )
            .await;

            println!("Connected to NetworkTables");

            if client.is_err() {
                println!("Failed to connect to NetworkTables");
                first = false;
                continue;
            }

            let client = client.unwrap();
            let subscription = client
                .subscribe_w_options(
                    &[""],
                    Some(SubscriptionOptions {
                        all: Some(true),
                        prefix: Some(true),
                        ..Default::default()
                    }),
                )
                .await;

            if subscription.is_err() {
                println!("Failed to subscribe to NetworkTables");
                first = false;
                continue;
            }

            let mut subscription = subscription.unwrap();
            while let Some(message) = subscription.next().await {
                //println!("Received message: {:?}", message);
                let _ = function_to_call(message, database.clone());
            }
        }
    })
}

/// # Function
/// This function is used to write the data to the database when the message is received. This is used because a local database is needed for the data coming in from the network table.
/// This is essentially the function that you pass inside the `begin_network_table` function. I separated it out into a function to make it easier to test and read.
///
/// # Parameters
/// - `message`: The message that will be written to the database
/// - `database`: The database that will be used to store the data
///
pub fn write_all(message: MessageData, database: Arc<Mutex<SQLiteDatabase>>) -> () {
    let mut message = message;
    message.timestamp /= 1000;
    let _res = database
        .lock()
        .unwrap()
        .add_value(TableEntree::from_message(message));
}
