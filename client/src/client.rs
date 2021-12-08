use std::{net::SocketAddr, time::Duration};
use std::time::Instant;

use laminar::{ErrorKind, Packet, Socket, SocketEvent, Config};
use rand::seq::SliceRandom;
use macroquad::prelude::{next_frame, get_time, clear_background, Conf, BLACK};
use shared::{StreamId, MessageType, MAGIC_BYTE, Direction, GameResult};
use crate::game::Game;

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 800;

const SERVER_ADDRESS: &str = "127.0.0.1:8080";

pub fn client() {
    main();
}

fn send_packet(message_type: MessageType, payload: Vec<u8>, address: SocketAddr, stream_id: u8, sender: &mut Socket) {
    // if message_type != MessageType::Heartbeat {
    //     println!("sending packet with payload {:?}", payload);
    // }
    let mut actual_payload = vec![MAGIC_BYTE, message_type as u8];
    actual_payload.extend(payload.iter());
    sender.send(Packet::reliable_sequenced(address, actual_payload, Some(0))).unwrap();
    sender.manual_poll(Instant::now());
}

fn handle_packet(packet: Packet, game: &mut Game) -> bool {
    let payload = packet.payload();
    // println!("payload: {:?}", payload);
    let magic_byte = payload[0];
    if magic_byte != MAGIC_BYTE {
        return false;
    }
    let message_type = payload[1];
    let received_data = &packet.payload()[2..];
    match message_type {
        x if x == MessageType::AssignIdEvent as u8 => {
            let assigned_id = received_data[0];
            println!("Assigned ID {}", assigned_id);
            game.spawn_snek(assigned_id).unwrap();
            game.set_my_snek_id(assigned_id);
            return false;
        },
        x if x == MessageType::BroadcastIdsEvent as u8 => {
            println!("IDs: {:?}", received_data);
            for &id in received_data {
                if id != game.get_my_snek_id() {
                    game.spawn_snek(id).unwrap();
                }
            }
            return false;
        },
        x if x == MessageType::StartEvent as u8 => {
            println!("Starting!");
            game.start_game();
            return true;
        },
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
        x if x == MessageType::Heartbeat as u8 => {
            // println!("Heartbeat!");
            return false;
        }
        _ => { return false; }
    }
}

fn window_conf() -> Conf {
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
    // Get random address because clients can't have the same address
    let addresses = vec!["127.0.0.1:12352", "127.0.0.1:12353", "127.0.0.1:12354",  "127.0.0.1:12356", "127.0.0.1:12357", "127.0.0.1:12358"]; // vec!["192.168.1.3:12352", "192.168.1.3:12353", "192.168.1.3:12354", "192.168.1.3:12355", "192.168.1.3:12356"];
    let mut rng = rand::thread_rng();
    let addr = addresses.choose(&mut rng).unwrap();

    let mut config = Config::default();
    config.heartbeat_interval = Some(Duration::from_millis(500));
    config.idle_connection_timeout = Duration::from_millis(10000);
    let mut socket = Socket::bind_with_config(addr, config)?;
    println!("Binded to IP {}", addr);

    // Tell server to add the client
    let server_address = SERVER_ADDRESS.parse::<SocketAddr>().unwrap();
    send_packet(MessageType::JoinEvent, vec![], server_address, StreamId::Event as u8, &mut socket);
    socket.manual_poll(Instant::now());
    println!("Attempting to join server {} ...", SERVER_ADDRESS);

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
        if game.has_started() {
            let my_id = game.get_my_snek_id();
            if !game.is_alive(my_id) {
                let mut payload = vec![];
                for snek_id in game.get_all_snek_ids() {
                    payload.push(snek_id);
                }
                send_packet(MessageType::DeathEvent, payload, server_address, 
                    StreamId::Event as u8, &mut socket);
                game.end_game();
            } else {
                let time_passed = (get_time() - last_send_move_time) >= 0.03;
                if time_passed {
                    // println!("Updating movement!");
                    let direction = game.get_snek_direction(my_id);
                    if game.get_previous_snek_direction(my_id) != direction {
                        println!("Updating movement!");
                        send_packet(MessageType::MoveEvent, vec![my_id, direction as u8], server_address, 
                            StreamId::Event as u8, &mut socket);
                        game.set_previous_snek_direction(my_id, direction);
                    }
                    last_send_move_time = get_time();
                }
            }
        }

        let time_passed = (get_time() - last_heartbeat_time) >= 1.0;
        if time_passed {
            send_packet(MessageType::Heartbeat, vec![], server_address, 
                StreamId::Event as u8, &mut socket);
            last_heartbeat_time = get_time();
        }

        clear_background(BLACK);
        game.update(should_update);
        game.handle_events();

        // send_packet(MessageType::Heartbeat, vec![], server_address, StreamId::Heartbeat as u8, &mut socket);

        next_frame().await;
    }
}