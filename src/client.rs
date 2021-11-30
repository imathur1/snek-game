use std::io::stdin;
use std::time::Instant;

use laminar::{ErrorKind, Packet, Socket, SocketEvent};

const SERVER: &str = "127.0.0.1:12351";

pub fn client() -> Result<(), ErrorKind> {
    let addr = "127.0.0.1:12352";
    let mut socket = Socket::bind(addr)?;
    println!("Connected on {}", addr);

    let server = SERVER.parse().unwrap();
    println!("Direction: ");

    let stdin = stdin();
    let mut s_buffer = String::new();

    loop {
        s_buffer.clear();
        stdin.read_line(&mut s_buffer)?;
        let line = "0,".to_owned() + &s_buffer.replace(|x| x == '\n' || x == '\r', "");
        socket.send(Packet::reliable_ordered(
            server,
            line.clone().into_bytes(),
            Some(1)
        ))?;

        socket.manual_poll(Instant::now());

        match socket.recv() {
            Some(SocketEvent::Packet(packet)) => {
                if packet.addr() == server {
                    println!("Server sent: {}", String::from_utf8_lossy(packet.payload()));
                } else {
                    println!("Unknown sender.");
                }
            }
            Some(SocketEvent::Timeout(_)) => {}
            _ => println!("Silence.."),
        }
    }
    Ok(())
}