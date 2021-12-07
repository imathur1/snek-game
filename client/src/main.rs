mod client;
mod game;
mod snek;

use laminar::{ErrorKind};

fn main() -> Result<(), ErrorKind> {
    println!("Starting client...");
    client::client()
}