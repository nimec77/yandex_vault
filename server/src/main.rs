use std::{
    net::TcpListener,
    sync::{Arc, Mutex},
    thread,
};

use crate::{server::handle_client, vault::Vault};

mod server;
mod vault;

const VAULT_CAPACITY: usize = 10;

const SERVER_PORT: &str = "127.0.0.1:7878";

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(SERVER_PORT)?;
    println!("Server is running on {SERVER_PORT}");

    let vault = Arc::new(Mutex::new(Vault::new(VAULT_CAPACITY)));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let vault = Arc::clone(&vault);
                thread::spawn(move || {
                    handle_client(stream, vault);
                });
            }
            Err(e) => eprintln!("Connection failed: {e}"),
        }
    }
    Ok(())
}
