use std::io::stdin;
use std::time::Instant;
use std::{thread, time};
use std::collections::HashMap;

use laminar::{ErrorKind, Packet, Socket, SocketEvent};
use rand::seq::SliceRandom;
use macroquad::prelude::{next_frame, clear_background, Conf, BLACK, get_time};
use crate::game::Game;
use crate::shared::{Coord, Direction, SnekId};

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;

const SERVER: &str = "127.0.0.1:12351";

pub fn client() -> Result<(), ErrorKind> {
    main();
    Ok(())
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
    let addresses = vec!["127.0.0.1:12352", "127.0.0.1:12353", "127.0.0.1:12354",  "127.0.0.1:12355"];
    let mut rng = rand::thread_rng();
    let addr = addresses.choose(&mut rng).unwrap();

    let mut socket = Socket::bind(addr)?;
    println!("Connected on {}", addr);

    let server = SERVER.parse().unwrap();
    socket.send(Packet::reliable_ordered(server, "join".as_bytes().to_vec(), Some(1),))?;
    socket.manual_poll(Instant::now());

    let one_sec = time::Duration::from_millis(1000);
    let buffer = time::Duration::from_millis(500);
    let mut snek_id = String::from("-1");
    let mut game_start = false;
    let mut game = Game::new(
        WINDOW_WIDTH,  WINDOW_HEIGHT,
        20, 30, 20
    );

    loop {
        if game_start {
            thread::sleep(buffer);
        } else {
            thread::sleep(one_sec);
        }
        socket.manual_poll(Instant::now());
        match socket.recv() {
            Some(SocketEvent::Packet(packet)) => {
                if packet.addr() == server {
                    if (snek_id == "-1") {
                        snek_id = String::from_utf8_lossy(packet.payload()).to_string();
                        println!("Server gave id: {}", snek_id);
                    } else {
                        let data = String::from_utf8_lossy(packet.payload()).to_string();
                        if data == "start" {
                            game_start = true;
                            println!("Game Started");
                        
                            if (snek_id == "1") {
                                game.spawn_snek(true);
                                game.spawn_snek(false);
                            } else {
                                game.spawn_snek(false);
                                game.spawn_snek(true);
                            }
                        }
                        if data.len() != 1 && &data[..4] == "dead" {
                            let msg_vec: Vec<&str> = data.split(",").collect();
                            let dead_id: String = msg_vec[1].parse().unwrap();
                            if snek_id == dead_id {
                                println!("You lose!");
                            } else {
                                println!("You win!");
                            }
                            break;
                        }
                        if game_start && data != "heartbeat" {
                            for (id, snek) in game.sneks.iter_mut() { 
                                if id.to_string() != snek_id {
                                    if data == "W" {
                                        snek.set_direction(Direction::North);
                                    }  else if data == "S" {
                                        snek.set_direction(Direction::South);
                                    } else if data == "A" {
                                        snek.set_direction(Direction::West);
                                    } else {
                                        snek.set_direction(Direction::East);
                                    }
                                }
                            }
                            clear_background(BLACK);
                            println!("{}, {}, {}", game.sneks[&1].head.0, game.sneks[&1].head.1, get_time());
                            game.update(&mut socket, &server);
                            if (game.sneks.len() == 1) {
                                for (id, snek) in game.sneks.iter() {
                                    if (id.to_string() == snek_id) {
                                        println!("You win!");
                                    } else {
                                        println!("You lose!");
                                    }
                                }
                                // break;
                            }
                            next_frame().await
                        }
                    }
                }
            }
            Some(SocketEvent::Timeout(_)) => {}
            _ => {}
            // _ => println!("Silence.."),
        }
        if !game_start {
            socket.send(Packet::reliable_ordered(
                server,
                "heartbeat".as_bytes().to_vec(),
                Some(0),
            ))?;
        }
    }
    Ok(())
}