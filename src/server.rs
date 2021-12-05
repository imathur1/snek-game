use std::thread;
use std::collections::{HashMap, HashSet};

use laminar::{ErrorKind, Packet, Socket, SocketEvent};

use crate::snek::Snek;
use crate::shared::{Coord, Direction, SnekId};

const SERVER: &str = "127.0.0.1:12351";

pub fn server() -> Result<(), ErrorKind> {
    let mut socket = Socket::bind(SERVER)?;
    let (sender, receiver) = (socket.get_packet_sender(), socket.get_event_receiver());
    let _thread = thread::spawn(move || socket.start_polling());

    let mut snek_ids: Vec<SnekId> = Vec::new();
    let mut addresses = HashSet::new();
    let mut sneks: HashMap<SnekId, Snek> = HashMap::new();
    let mut game_start = false;

    loop {

        if let Ok(event) = receiver.recv() {
            match event {
                SocketEvent::Packet(packet) => {
                    let msg = packet.payload();
                    let msg = String::from_utf8_lossy(msg);
                    if msg == "join" {
                        let id: u8 = join_game(&mut snek_ids, &mut sneks);
                        println!("sending id {} back to client", id);
                        addresses.insert(packet.addr());
                        sender.send(Packet::reliable_ordered(packet.addr(), id.to_string().as_bytes().to_vec(), Some(1)));
                        if snek_ids.len() == 2 {
                            game_start = true;
                            println!("Game Started");
                            for addr in addresses.iter() {
                                sender.send(Packet::reliable_ordered(*addr, "start".as_bytes().to_vec(), Some(1)));
                            }
                        }
                    } else if msg == "heartbeat" {
                        // println!("sending heartbeat");
                        sender.send(Packet::reliable_ordered(packet.addr(), "heartbeat".as_bytes().to_vec(), Some(0)));
                    } else {
                        println!("Received message {}", msg);
                        update_game(&msg, &mut snek_ids, &mut sneks);
                        sender.send(Packet::reliable_ordered(packet.addr(), "heartbeat".as_bytes().to_vec(), Some(1)));
                    }
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

pub fn join_game(snek_ids: &mut Vec<SnekId>, sneks: &mut HashMap<SnekId, Snek>) -> u8 {
    let mut id: u8 = 0;
    if snek_ids.len() != 0 {
        id = snek_ids[snek_ids.len() - 1] + 1;
    }
    snek_ids.push(id);
    let head: Coord = (5, 5);
    let body: Vec<Coord> = Vec::new();
    let direction: Direction = Direction::NORTH;
    sneks.insert(id, Snek {id, head, body, direction});
    println!("Snek with id {} joined", id);
    return id;
}

pub fn update_game(msg: &str, snek_ids: &mut Vec<SnekId>, sneks: &mut HashMap<SnekId, Snek>) {
    let msg: Vec<&str> = msg.split(",").collect();
    let id: u8 = msg[0].parse().unwrap();
    let direction = msg[1].to_uppercase();
    let snek = &mut *sneks.get_mut(&id).unwrap();
    if direction == "W" {
        snek.set_direction(Direction::NORTH);
    } else if direction == "S" {
        snek.set_direction(Direction::SOUTH);
    } else if direction == "A" {
        snek.set_direction(Direction::WEST);
    } else if direction == "D" {
        snek.set_direction(Direction::EAST);
    }
    snek.update();
    println!("Updated snek with id {}", id);
}