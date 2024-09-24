use std::{
    net::{Ipv4Addr, SocketAddrV4},
    str::FromStr,
    sync::Arc,
};

use futures::{SinkExt, StreamExt};
use network_tables::v4::{MessageData, SubscriptionOptions};
use serde_json::to_string;
use tokio::{net::TcpListener, sync::Mutex, task::LocalSet};
use tokio_tungstenite::tungstenite::Message;

use dotenv::dotenv;
use std::env;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenv().ok();
    let local = LocalSet::new();

    local
        .run_until(async move {
            let client_r_w: Arc<
                Mutex<Vec<tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>>>,
            > = Arc::new(Mutex::new(Vec::new()));
            let local_client_r_w_0 = client_r_w.clone();

            let server_webhook = tokio::task::spawn_local(async move {
                let local_client_r_w = client_r_w.clone();
                let mut listener = Some(
                    TcpListener::bind(format!(
                        "127.0.0.1:{}",
                        env::var("SERVER_PORT").expect("SERVER_PORT not set")
                    ))
                    .await
                    .expect("Failed to bind"),
                );

                while let Ok((stream, _)) = listener.as_mut().unwrap().accept().await {
                    let ws_stream = tokio_tungstenite::accept_async(stream)
                        .await
                        .expect("Error during WebSocket handshake");

                    local_client_r_w.lock().await.push(ws_stream);
                }
            });

            let network_tables_webhook = tokio::task::spawn_local(async move {
                let client = network_tables::v4::Client::try_new_w_config(
                    SocketAddrV4::new(
                        Ipv4Addr::from_str(env::var("ROBOT_SERVER_IP").unwrap().as_str()).unwrap(),
                        u16::from_str(&env::var("ROBOT_SERVER_PORT").unwrap()).unwrap(),
                    ),
                    network_tables::v4::client_config::Config {
                        ..Default::default()
                    },
                )
                .await
                .unwrap();

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
                    let _ = write_all(local_client_r_w_0.clone(), message).await;
                }
            });

            tokio::join!(server_webhook, network_tables_webhook);
        })
        .await;

    return ();
}

// Function to write data to all WebSocket clients
async fn write_all(
    client_writes: Arc<Mutex<Vec<tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>>>>,
    data: MessageData,
) {
    let mut client_r_w = client_writes.lock().await;

    // Iterate over all WebSocket clients
    for client_write in client_r_w.iter_mut() {
        let message = Message::Text(to_string(&data).unwrap());

        // Try sending the message and handle potential errors
        if let Err(e) = client_write.send(message).await {
            eprintln!("Error sending message: {:?}", e);
        }
    }
}
