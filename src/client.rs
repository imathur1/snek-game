use std::io::stdin;
use std::time::Instant;
use std::{thread, time};
use std::collections::HashMap;

use laminar::{ErrorKind, Packet, Socket, SocketEvent};
use rand::seq::SliceRandom;
use macroquad::prelude::{next_frame, clear_background, Conf, BLACK, get_time};
use crate::game::Game;
use crate::shared::{Coord, Direction, SnekId};

const WINDOW_WIDTH: i32 = 1200; // 800;
const WINDOW_HEIGHT: i32 = 900; // 600;

const SERVER: &str = "192.168.1.3:12351"; // "127.0.0.1:12351";

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
    let addresses = vec!["127.0.0.1:12352", "127.0.0.1:12353", "127.0.0.1:12354",  "127.0.0.1:12356", "127.0.0.1:12357", "127.0.0.1:12358"]; // vec!["192.168.1.3:12352", "192.168.1.3:12353", "192.168.1.3:12354", "192.168.1.3:12355", "192.168.1.3:12356"];
    let mut rng = rand::thread_rng();
    let addr = addresses.choose(&mut rng).unwrap();

    let mut socket = Socket::bind(addr)?;
    println!("Connected on {}", addr);

    let server = SERVER.parse().unwrap();
    socket.send(Packet::reliable_ordered(server, "join".as_bytes().to_vec(), Some(1),))?;
    socket.manual_poll(Instant::now());

    let one_sec = time::Duration::from_millis(1000);
    let buffer = time::Duration::from_millis(100);
    let mut snek_id = String::from("-1");
    let mut game_start = false;
    let mut game = Game::new(
        WINDOW_WIDTH,  WINDOW_HEIGHT,
        20, 45, 40
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

                            socket.send(Packet::reliable_ordered(
                                server,
                                "D".as_bytes().to_vec(),
                                Some(7),
                            ));
                        }
                        if game_start && data != "heartbeat" && data != "start" {
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
                            println!("{}, {}, {}", game.sneks[&2].head.0, game.sneks[&2].head.1, get_time());
                            game.update(&mut socket, &server);
                            if (game.sneks.len() == 1) {
                                for (id, snek) in game.sneks.iter() {
                                    if (id.to_string() == snek_id) {
                                        println!("You win!");
                                    } else {
                                        println!("You lose!");
                                    }
                                }
                                break;
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