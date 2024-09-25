use std::{
    net::{Ipv4Addr, SocketAddrV4},
    str::FromStr,
    sync::{Arc, Mutex},
};

use network_tables::v4::{MessageData, SubscriptionOptions};
use tokio::task::spawn_local;

use crate::database::{structs::table_entree::TableEntree, SQLiteDatabase};

/// Starts the NetworkTable bridge loop. This will work inf as long as the program is running and will try to re-connect if the connection fails.
pub fn begin_network_table(
    url: String,
    port: i32,
    time_between_reconnects: u64,
    function_to_call: Box<dyn Fn(MessageData) -> ()>,
) -> tokio::task::JoinHandle<()> {
    spawn_local(async move {
        println!("Starting NetworkTable Bridge");
        loop {
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
                tokio::time::sleep(tokio::time::Duration::from_millis(time_between_reconnects))
                    .await;
                continue;
            }

            let client = client.unwrap();
            let mut subscription = client
                .subscribe_w_options(
                    &[""],
                    Some(SubscriptionOptions {
                        all: Some(true),
                        prefix: Some(true),
                        ..Default::default()
                    }),
                )
                .await
                .unwrap();

            while let Some(message) = subscription.next().await {
                let _ = function_to_call(message);
            }
        }
    })
}

pub fn write_all(message: MessageData, database: Arc<Mutex<SQLiteDatabase>>) -> () {
    println!("Writing to database");

    let _res = database
        .lock()
        .unwrap()
        .add_value(TableEntree::from_message(message));
}
