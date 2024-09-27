use std::{
    future::Future,
    net::{Ipv4Addr, SocketAddrV4},
    str::FromStr,
    sync::{Arc, Mutex},
};

use network_tables::v4::{MessageData, SubscriptionOptions};
use tokio::task::spawn_local;

use crate::database::{self, structs::table_entree::TableEntree, SQLiteDatabase};

/// Starts the NetworkTable bridge loop. This will work inf as long as the program is running and will try to re-connect if the connection fails.
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

pub fn write_all(message: MessageData, database: Arc<Mutex<SQLiteDatabase>>) -> () {
    let _res = database
        .lock()
        .unwrap()
        .add_value(TableEntree::from_message(message));
}
