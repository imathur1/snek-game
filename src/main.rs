mod shared;
mod game;
mod snek;
mod server;
mod client;

use std::io::stdin;
use laminar::{ErrorKind};

fn main() -> Result<(), ErrorKind> {
    let stdin = stdin();
    println!("Please type in `server` or `client`.");

    let mut s = String::new();
    stdin.read_line(&mut s)?;
    if s.starts_with('s') {
        println!("Starting server..");
        server::server()
    } else {
        println!("Starting client..");
        client::client()
    }
}