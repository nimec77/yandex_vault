use std::{io::{self, BufRead, BufReader, Write}, net::TcpStream};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    let mut reader = BufReader::new(stream.try_clone()?);

    for _ in 0..1 {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        println!("{}", line);
    }

    let stdin = io::stdin();
    loop {
        print!("vault> ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        stdin.read_line(&mut input)?;
        let trimmed = input.trim();

        if trimmed.is_empty() {
            continue;
        }

        stream.write_all(trimmed.as_bytes())?;
        stream.write_all(b"\n")?;
        stream.flush()?;

        let mut buffer = String::new();
        let bytes = reader.read_line(&mut buffer)?;

        if bytes == 0 {
            println!("Server closed the connection");
            return Ok(());
        }

        println!("{}", buffer);

        if trimmed.eq_ignore_ascii_case("EXIT") {
            break;
        }
    }   
    Ok(())
}
