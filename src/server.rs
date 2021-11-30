use std::thread;
use std::time::Instant;
use std::{collections::HashMap, convert::TryInto};

use laminar::{ErrorKind, Packet, Socket, SocketEvent};

use crate::snek::Snek;
use crate::shared::{Coord, Direction, SnekId};

const SERVER: &str = "127.0.0.1:12351";

pub fn server() -> Result<(), ErrorKind> {
    let mut socket = Socket::bind(SERVER)?;
    let (sender, receiver) = (socket.get_packet_sender(), socket.get_event_receiver());
    let _thread = thread::spawn(move || socket.start_polling());
    
    let snek_ids: Vec<SnekId> = vec![0];
    let sneks: HashMap<SnekId, Snek> = Vec::new();
    for id in snek_ids {
        let head: Coord = (5, 5);
        let body: Vec<Coord> = Vec::new();
        sneks[id] = Snek {id, head, body, Direction::NORTH};
    }

    loop {
        if let Ok(event) = receiver.recv() {
            match event {
                SocketEvent::Packet(packet) => {
                    let msg = packet.payload();
                    if msg == Direction::NORTH {
                        printn!("mae it");
                    }
                    // let msg = String::from_utf8_lossy(msg).to_uppercase();
                    // if (msg == "W") {
                    //     snek[1] += 1;
                    // } else if (msg == "S") {
                    //     snek[1] -= 1;
                    // } else if (msg == "A") {
                    //     snek[0] -= 1;
                    // } else if (msg == "D") {
                    //     snek[0] += 1;
                    // }
                    // println!("{}, {}", snek[0], snek[1]);
                    sender.send(Packet::reliable_ordered(packet.addr(), "hi".as_bytes().to_vec(), Some(1)
                    )).expect("This should send");
                }
                SocketEvent::Timeout(address) => {
                    println!("Client timed out: {}", address);
                }
                _ => {}
            }
        }
    }
    Ok(())
}