use std::io::stdin;
use std::time::Instant;
use std::{thread, time};
use rand::seq::SliceRandom;

use laminar::{ErrorKind, Packet, Socket, SocketEvent};

const SERVER: &str = "127.0.0.1:12351";

pub fn client() -> Result<(), ErrorKind> {
    let addresses = vec!["127.0.0.1:12352", "127.0.0.1:12353", "127.0.0.1:12354",  "127.0.0.1:12355"];
    let mut rng = rand::thread_rng();
    let addr = addresses.choose(&mut rng).unwrap();

    let mut socket = Socket::bind(addr)?;
    println!("Connected on {}", addr);

    let server = SERVER.parse().unwrap();
    socket.send(Packet::reliable_ordered(
        server,
        "join".as_bytes().to_vec(),
        Some(0),
    ))?;
    socket.manual_poll(Instant::now());

    let one_sec = time::Duration::from_millis(1000);
    let mut snek_id = String::from("-1");
    let mut game_start = false;
    
    loop {
        thread::sleep(one_sec);
        socket.manual_poll(Instant::now());
        match socket.recv() {
            Some(SocketEvent::Packet(packet)) => {
                if packet.addr() == server {
                    if (snek_id == "-1") {
                        snek_id = String::from_utf8_lossy(packet.payload()).to_string();
                        println!("Server gave id: {}", snek_id);
                    } else {
                        let msg = String::from_utf8_lossy(packet.payload()).to_string();
                        if msg == "start" {
                            game_start = true;
                            println!("Game Started")
                        }
                        if game_start {
                            game_start = true;
                            let snek_move = (snek_id.clone() + ",W");
                            socket.send(Packet::reliable_ordered(
                                server,
                                snek_move.as_bytes().to_vec(),
                                Some(1),
                            ))?;
                        }
                    }
                }
            }
            Some(SocketEvent::Timeout(_)) => {}
            _ => {}
            // _ => println!("Silence.."),
        }
        socket.send(Packet::reliable_ordered(
            server,
            "heartbeat".as_bytes().to_vec(),
            Some(0),
        ))?;
    }

    // println!("Direction: ");
    // let stdin = stdin();
    // let mut s_buffer = String::new();
    // loop {
    //     s_buffer.clear();
    //     stdin.read_line(&mut s_buffer)?;
    //     let line = s_buffer.replace(|x| x == '\n' || x == '\r', "");
    //     socket.send(Packet::reliable_ordered(
    //         server,
    //         line.clone().into_bytes(),
    //         Some(0),
    //     ))?;

    //     socket.manual_poll(Instant::now());

    //     match socket.recv() {
    //         Some(SocketEvent::Packet(packet)) => {
    //             if packet.addr() == server {
    //                 println!("Server sent: {}", String::from_utf8_lossy(packet.payload()));
    //             }
    //         }
    //         Some(SocketEvent::Timeout(_)) => {}
    //         _ => println!("Silence.."),
    //     }
    // }
    Ok(())
}