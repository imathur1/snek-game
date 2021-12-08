mod server;

use std::io;
use std::io::Write;
use laminar::ErrorKind;

fn main() -> Result<(), ErrorKind> {
    //Start the server
    let stdin = io::stdin();

    print!("Port the server should listen on (8080): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    stdin.read_line(&mut input)?;

    server::server(match input.parse::<i32>() {
        Ok(port) => port,
        Err(_) => 8080
    })
}