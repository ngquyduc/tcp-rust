use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::client::{Client, ClientTrait};
use crate::server::{Server, ServerTrait};

// Do not modify the address constant
const DEFAULT_SERVER_ADDRESS: &'static str = "127.0.0.1";
const TIMEOUT: Duration = Duration::from_secs(1);
const DEFAULT_TOTAL_CLIENTS: usize = 50;
const DEFAULT_TOTAL_MESSAGES_PER_CLIENT: usize = 50;

#[tokio::main]
async fn main() {
    let (port, seed, total_clients, total_messages_per_client) = get_args();

    println!(
        "Starting the {} client(s), each client sending {} messages, with initial seed: {}",
        total_clients, total_messages_per_client, seed
    );

    let full_server_address = format!("{}:{}", DEFAULT_SERVER_ADDRESS, port);
    let first_address = full_server_address.clone();

    let (tx, rx) = mpsc::channel();

    tokio::spawn(async move {Server.start_server(first_address, tx).await});

    println!("Server started on {}", full_server_address);

    let client_handler = thread::spawn(move || match rx.recv_timeout(TIMEOUT) {
        Ok(Ok(_)) => Client.start_client(
            seed,
            total_clients,
            total_messages_per_client,
            full_server_address,
        ),
        Ok(Err(e)) => {
            eprintln!("Server fails to start because: {}", e);
            return;
        }
        Err(e) => {
            eprintln!("Timeout: unable to get server status: {}", e);
            return;
        }
    });

    client_handler.join().unwrap();
}

fn get_args() -> (u16, u64, usize, usize) {
    let mut args = std::env::args().skip(1);

    (
        args.next()
            .map(|a| a.parse().expect("invalid port number"))
            .unwrap(),
        args.next()
            .map(|a| a.parse().expect("invalid u64 for seed"))
            .unwrap_or_else(|| rand::Rng::gen(&mut rand::thread_rng())),
        args.next()
            .map(|a| a.parse().expect("invalid usize for total clients"))
            .unwrap_or_else(|| DEFAULT_TOTAL_CLIENTS),
        args.next()
            .map(|a| {
                a.parse()
                    .expect("invalid usize for total messages per client")
            })
            .unwrap_or_else(|| DEFAULT_TOTAL_MESSAGES_PER_CLIENT),
    )
}

mod client;
mod server;
mod task;
