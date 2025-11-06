use std::{
    io::{self, BufRead, BufReader, Write},
    net::{SocketAddr, TcpStream},
    time::{Duration, Instant},
};

use socket2::{Domain, Protocol, Socket, Type};

enum ConnectionResult {
    Exit,
    Lost,
}

fn main() {
    let addr: SocketAddr = "127.0.0.1:7878".parse().unwrap();

    loop {
        match connect(&addr) {
            Ok(stream) => {
                println!("Connected to server");
                match handle_connection(stream) {
                    ConnectionResult::Exit => break,
                    ConnectionResult::Lost => {
                        eprintln!("Connection lost. Retrying...");
                        std::thread::sleep(Duration::from_secs(2));
                    }
                }
            }
            Err(e) => {
                eprintln!("Connect failed: {}. Retrying in 2s...", e);
                std::thread::sleep(Duration::from_secs(2));
            }
        }
    }
}

fn connect(addr: &SocketAddr) -> io::Result<TcpStream> {
    let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;

    socket.set_keepalive(true)?;

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        socket.set_tcp_keepalive(
            &socket2::TcpKeepalive::new()
                .with_time(Duration::from_secs(10))
                .with_interval(Duration::from_secs(5)),
        )?;
    }
    socket.connect(&(*addr).into())?;
    let stream: TcpStream = socket.into();

    stream.set_read_timeout(Some(Duration::from_secs(3)))?;

    Ok(stream)
}

fn handle_connection(stream: TcpStream) -> ConnectionResult {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let stdin = io::stdin();

    let mut line = String::new();
    if reader.read_line(&mut line).is_ok() {
        println!("{line}");
    }

    loop {
        print!("vault> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if stdin.read_line(&mut input).is_err() {
            return ConnectionResult::Lost;
        }

        let trimmed = input.trim();
        if trimmed.is_empty() {
            continue;
        }

        if trimmed.eq_ignore_ascii_case("EXIT") {
            println!("Bye!");
            return ConnectionResult::Exit;
        }

        if trimmed.eq_ignore_ascii_case("PING") {
            match send_ping(&stream, &mut reader) {
                Ok(latency) => println!("PONG ({latency}ms)"),
                Err(e) => {
                    println!("Error: server unreachable ({e})");
                    return ConnectionResult::Lost;
                }
            }
            continue;
        }

        match send_command(&stream, &mut reader, trimmed) {
            Ok(response) => println!("{response}"),
            Err(e) => {
                println!("Error: connection lost ({e})");
                return ConnectionResult::Lost;
            }
        }
    }
}

fn send_ping(mut stream: &TcpStream, reader: &mut BufReader<TcpStream>) -> io::Result<u128> {
    let start = Instant::now();

    stream.write_all(b"PING\n")?;
    stream.flush()?;

    let mut buffer = String::new();
    let bytes = reader.read_line(&mut buffer)?;

    if bytes == 0 {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "Server closed the connection",
        ));
    }

    let elapsed = start.elapsed().as_millis();

    if buffer.trim() != "PONG" {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid PING response",
        ))
    } else {
        Ok(elapsed)
    }
}

fn send_command(
    mut stream: &TcpStream,
    reader: &mut BufReader<TcpStream>,
    command: &str,
) -> io::Result<String> {
    stream.write_all(command.as_bytes())?;
    stream.write_all(b"\n")?;
    stream.flush()?;

    let mut buffer = String::new();
    let bytes = reader.read_line(&mut buffer)?;

    if bytes == 0 {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "Server closed the connection",
        ));
    }

    Ok(buffer)
}
