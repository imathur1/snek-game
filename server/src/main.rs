mod server;

use laminar::ErrorKind;

fn main() -> Result<(), ErrorKind> {
    println!("Starting server...");
    server::server()
}