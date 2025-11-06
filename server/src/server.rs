use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    sync::{Arc, Mutex}, time::Duration,
};

use rand::Rng;

use crate::vault::{Item, Vault, VaultError};

const MAX_CELL_SIZE: u32 = 100;

pub fn handle_client(stream: TcpStream, vault: Arc<Mutex<Vault>>) {
    let mut writer = stream.try_clone().expect("Failed to clone stream");
    let mut reader = BufReader::new(stream);

    let _ = writer.write_all(b"Welcome to the vault!\n");
    let _ = writer.flush();

    let mut rnd = rand::rng();
    let mut line = String::new();
    loop {
        line.clear();

        match reader.read_line(&mut line) {
            Ok(0) => {
                return;
            }
            Ok(_) => {
                let input = line.trim();
                if input.is_empty() {
                    let _ = writer.flush();
                    continue;
                }

                let mut parts = input.split_whitespace();
                let response = match parts.next() {
                    Some("PUT") => {
                        let id = parts.next().and_then(|s| s.parse::<u32>().ok());
                        let name = parts.next();
                        let size = parts.next().and_then(|s| s.parse::<u32>().ok());

                        if let (Some(id), Some(name), Some(size)) = (id, name, size) {
                            let item = Item {
                                name: name.to_string(),
                                size,
                            };
                            let mut v = vault.lock().unwrap();
                            match v.put(id, item, MAX_CELL_SIZE) {
                                Ok(_) => "OK: item stored\n".to_string(),
                                Err(VaultError::VaultFull) => "ERROR: vault is full\n".to_string(),
                                Err(VaultError::CellFull) => "ERROR: cell is full\n".to_string(),
                                _ => "ERROR: unknown error\n".to_string(),
                            }
                        } else {
                            "ERROR: usage PUT <id> <name> <size>\n".to_string()
                        }
                    }
                    Some("GET") => {
                        if let Some(id_str) = parts.next() {
                            if let Ok(id) = id_str.parse::<u32>() {
                                let v = vault.lock().unwrap();
                                match v.get(id) {
                                    Ok(Some(items)) => items,
                                    Ok(None) => "Cell is empty\n".to_string(),
                                    Err(VaultError::CellNotFound) => {
                                        "ERROR: cell not found\n".to_string()
                                    }

                                    _ => "ERROR: unknown error\n".to_string(),
                                }
                            } else {
                                "ERROR: invalid id\n".to_string()
                            }
                        } else {
                            "ERROR: usage GET <id>\n".to_string()
                        }
                    }

                    Some("LIST") => {
                        let v = vault.lock().unwrap();
                        v.list().unwrap_or_else(|| "Vault is empty\n".to_string())
                    }

                    Some("TAKE") => {
                        let id = parts.next().and_then(|s| s.parse::<u32>().ok());
                        let name = parts.next();
                        if let (Some(id), Some(name)) = (id, name) {
                            let mut v = vault.lock().unwrap();
                            v.take(id, name).map_or_else(
                                |_| "ERROR: item not found\n".to_string(),
                                |item| format!("OK: taken {} {}\n", item.name, item.size),
                            )
                        } else {
                            "ERROR: usage TAKE <id> <name>\n".to_string()
                        }
                    }

                    Some("PING") => {
                        let delay = rnd.random_range(1..=5);
                        std::thread::sleep(Duration::from_secs(delay));
                        "PONG\n".to_string()
                    }

                    Some("EXIT") => {
                        let _ = writer.write_all(b"Bye!\n");
                        let _ = writer.flush();
                        return;
                    }

                    _ => "ERROR: unknown command\n".to_string(),
                };

                let _ = writer.write_all(response.as_bytes());
                let _ = writer.flush();
            }
            Err(_) => {
                return;
            }
        }
    }
}
