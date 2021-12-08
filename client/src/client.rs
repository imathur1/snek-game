use std::net::SocketAddr;
use std::io;
use std::io::Write;
use std::time::Instant;

use laminar::{ErrorKind, Packet, Socket, SocketEvent};
use macroquad::prelude::{next_frame, get_time, clear_background, Conf, BLACK};
use shared::{MessageType, MAGIC_BYTE, Direction, GameResult};
use crate::game::Game;

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 800;

pub fn client() {
    main();
}

fn send_packet(message_type: MessageType, payload: Vec<u8>, address: SocketAddr, sender: &mut Socket) {
    // if message_type != MessageType::Heartbeat {
    //     println!("sending packet with payload {:?}", payload);
    // }
    // Client sends a packet to server
    let mut actual_payload = vec![MAGIC_BYTE, message_type as u8];
    actual_payload.extend(payload.iter());
    sender.send(Packet::reliable_sequenced(address, actual_payload, Some(0))).unwrap();
    sender.manual_poll(Instant::now());
}

fn handle_packet(packet: Packet, game: &mut Game) -> bool {
    // Client receives a packet from server
    let payload = packet.payload();
    // println!("payload: {:?}", payload);
    let magic_byte = payload[0];
    if magic_byte != MAGIC_BYTE {
        return false;
    }
    let message_type = payload[1];
    let received_data = &packet.payload()[2..];
    // Handle different events recieved
    match message_type {
        // Assign the snek ids
        x if x == MessageType::AssignIdEvent as u8 => {
            let assigned_id = received_data[0];
            println!("Assigned ID {}", assigned_id);
            game.spawn_snek(assigned_id).unwrap();
            game.set_my_snek_id(assigned_id);
            return false;
        },
        // Broadcast the current sneks playing
        x if x == MessageType::BroadcastIdsEvent as u8 => {
            println!("IDs: {:?}", received_data);
            for &id in received_data {
                if id != game.get_my_snek_id() {
                    game.spawn_snek(id).unwrap();
                }
            }
            return false;
        },
        // Start the game
        x if x == MessageType::StartEvent as u8 => {
            println!("Starting!");
            game.start_game();
            return true;
        },
        // Update game from snek moves
        x if x == MessageType::MoveEvent as u8 => {
            // println!("Moving!");
            for i in (0..received_data.len()).step_by(2) {
                let snek_id = received_data[i];
                let direction = match received_data[i + 1] {
                    x if x == Direction::North as u8 => Some(Direction::North),
                    x if x == Direction::South as u8 => Some(Direction::South),
                    x if x == Direction::East as u8 => Some(Direction::East),
                    x if x == Direction::West as u8 => Some(Direction::West),
                    _ => None
                };
                // println!("Snek {} should be going {}", snek_id, direction.unwrap() as u8);
                game.move_snek(snek_id, direction.unwrap());
            }
            return true;
        },
        // End the game and broadcast the result
        x if x == MessageType::EndEvent as u8 => {
            println!("Game ended!");
            match received_data[0] {
                x if x == GameResult::Win as u8 => {
                    println!("You won!");
                },
                x if x == GameResult::Tie as u8 => {
                    println!("You tied!");
                },
                x if x == GameResult::Loss as u8 => {
                    println!("You lost to snek {}!", received_data[1]);
                },
                _ => {}
            }
            game.end_game();
            return false;
        },
        // Send heartbeat
        x if x == MessageType::Heartbeat as u8 => {
            // println!("Heartbeat!");
            return false;
        }
        _ => { return false; }
    }
}

fn window_conf() -> Conf {
    // Configures the client's window
    Conf {
        window_title: "Snek".to_owned(),
        window_resizable: false,
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> Result<(), ErrorKind> {
    let mut port = 8432;
    let mut addr: String;
    // Bind the client to the server socket 
    let mut socket: Socket;
    loop {
        addr = format!("{}:{}", "127.0.0.1", port);
        match Socket::bind(&addr) {
            Ok(s) => {
                socket = s;
                break;
            },
            Err(_) => {
                port += 1;
            }
        };
    }
    println!("Binded to IP {}", &addr);

    let stdin = io::stdin();

    print!("Enter server IP:port (127.0.0.1:8080): ");
    io::stdout().flush().unwrap();

    let mut server_address_string = String::new();
    stdin.read_line(&mut server_address_string)?;
    if server_address_string.ends_with('\n') {
        server_address_string.pop();
        if server_address_string.ends_with('\r') {
            server_address_string.pop();
        }
    }

    // Tell server to add the client
    let server_address = match server_address_string.parse::<SocketAddr>() {
        Ok(address) => address,
        Err(_) => "127.0.0.1:8080".parse::<SocketAddr>().unwrap()
    };
    send_packet(MessageType::JoinEvent, vec![], server_address, &mut socket);
    socket.manual_poll(Instant::now());
    println!("Attempting to join server {}...", server_address);

    let mut game = Game::new(
        WINDOW_WIDTH,  WINDOW_HEIGHT,
        20, 35, 35
    );

    let mut last_send_move_time: f64 = -10.0;
    let mut last_heartbeat_time: f64 = -10.0;

    loop {
        socket.manual_poll(Instant::now());

        let mut should_update = false;
        match socket.recv() {
            Some(SocketEvent::Packet(packet)) => {
                if packet.addr() == server_address {
                    should_update = handle_packet(packet, &mut game);
                    // send_packet(MessageType::Heartbeat, vec![], server_address, 
                    //     StreamId::Heartbeat as u8, &mut socket);
                }
            },
            _ => {}
        }
        // send_packet(MessageType::DeathEvent, vec![], server_address, StreamId::Heartbeat as u8, &mut socket);
        // Handles the game state 
        if game.has_started() {
            let my_id = game.get_my_snek_id();
            if !game.is_alive(my_id) {
                let mut payload = vec![];
                for snek_id in game.get_all_snek_ids() {
                    payload.push(snek_id);
                }
                send_packet(MessageType::DeathEvent, payload, server_address, 
                    &mut socket);
                game.end_game();
            } else {
                let time_passed = (get_time() - last_send_move_time) >= 0.03;
                if time_passed {
                    // println!("Updating movement!");
                    let direction = game.get_snek_direction(my_id);
                    if game.get_previous_snek_direction(my_id) != direction {
                        println!("Updating movement!");
                        send_packet(MessageType::MoveEvent, vec![my_id, direction as u8], server_address, 
                            &mut socket);
                        game.set_previous_snek_direction(my_id, direction);
                    }
                    last_send_move_time = get_time();
                }
            }
        }
        // Send heartbeat if no event has occurred during specified period to prevent timeout
        let time_passed = (get_time() - last_heartbeat_time) >= 1.0;
        if time_passed {
            send_packet(MessageType::Heartbeat, vec![], server_address, 
                &mut socket);
            last_heartbeat_time = get_time();
        }

        clear_background(BLACK);
        game.update(should_update);
        game.handle_events();

        // send_packet(MessageType::Heartbeat, vec![], server_address, StreamId::Heartbeat as u8, &mut socket);

        next_frame().await;
    }
}