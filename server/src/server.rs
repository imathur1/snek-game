use std::thread;
use std::collections::HashMap;

use laminar::{ErrorKind, Packet, Socket, SocketEvent};

use shared::SnekId;

const SERVER: &str = "127.0.0.1:8080";

pub fn server() -> Result<(), ErrorKind> {
    // Establish server socket
    let mut socket = Socket::bind(SERVER)?;
    let (sender, receiver) = (socket.get_packet_sender(), socket.get_event_receiver());
    let _thread = thread::spawn(move || socket.start_polling());

    let mut snek_ids: Vec<SnekId> = Vec::new();
    let mut addresses: HashMap<std::net::SocketAddr, SnekId> = HashMap::new();
    let mut moves: HashMap<std::net::SocketAddr, String> = HashMap::new();
    let mut game_start = false;

    loop {
        if let Ok(event) = receiver.recv() {
            // Run if event happens
            match event {
                SocketEvent::Packet(packet) => {
                    let msg = String::from_utf8_lossy(packet.payload());
                    if msg == "join" {
                        // Only allow a maximum of 2 clients to join the game
                        if snek_ids.len() == 2 { continue }

                        // Get new id for the client and send it back to the client
                        let id: u8 = join_game(&mut snek_ids);
                        println!("sending id {} back to client", id);
                        addresses.insert(packet.addr(), id);
                        sender.send(Packet::reliable_ordered(packet.addr(), id.to_string().as_bytes().to_vec(), Some(2))).unwrap();
                        if snek_ids.len() == 2 {
                            // If 2 clients have joined, start the game
                            game_start = true;
                            println!("Game Started");
                            for (addr, _) in addresses.iter() {
                                sender.send(Packet::reliable_ordered(*addr, "start".as_bytes().to_vec(), Some(3))).unwrap();
                            }
                        }
                    } else if msg == "heartbeat" {
                        // Send a heartbeat back to the client to prevent timing out
                        sender.send(Packet::reliable_ordered(packet.addr(), "heartbeat".as_bytes().to_vec(), Some(0))).unwrap();
                    } else {
                        // Receive moves from both clients. Once both are received send
                        // them to the clients so they can update their game state simultaneously

                        // println!("Received message {}", msg);
                        moves.insert(packet.addr(), msg.to_string());
                        if moves.len() == 2 {
                            // Send each move to all other players
                            for (addr, snek_move) in moves.iter() {
                                for (o_addr, _) in moves.iter() {
                                    if addr != o_addr {
                                        sender.send(Packet::reliable_ordered(*o_addr, snek_move.as_bytes().to_vec(), Some(4))).unwrap();
                                    }
                                }
                            }
                            // Reset the moves for the next frame
                            moves = HashMap::new();
                        } else {
                            // Only one move has been received, send heartbeat to prevent timing out
                            // and to wait for the second move
                            sender.send(Packet::reliable_ordered(packet.addr(), "heartbeat".as_bytes().to_vec(), Some(0))).unwrap();
                        }
                    }
                }
                // Check if a player has timed out
                SocketEvent::Timeout(address) => {
                    println!("Client timed out: {}", address);
                }
                _ => {}
            }
        }
    }
}


pub fn join_game(snek_ids: &mut Vec<SnekId>) -> u8 {
    // When new snek joins the game, give it a new id and it to current players
    let mut id: u8 = 1;
    if snek_ids.len() != 0 {
        id = snek_ids[snek_ids.len() - 1] + 1;
    }
    snek_ids.push(id);
    println!("Snek with id {} joined", id);
    return id;
}