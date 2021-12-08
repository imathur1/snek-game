use core::time;
use std::time::Duration;
use std::{thread, time::Instant};
use std::collections::HashMap;
use std::net::SocketAddr;
use crossbeam_channel::Sender;
use laminar::{ErrorKind, Packet, Socket, SocketEvent, Config};
use shared::{SnekId, StreamId, MessageType, GameResult, INVALID_ID, MAGIC_BYTE, MAX_PLAYERS};

struct ServerState {
    pub snek_ids: Vec<SnekId>,
    pub address_to_id: HashMap<SocketAddr, SnekId>,
    pub id_to_address: HashMap<SnekId, SocketAddr>,
    pub moves: HashMap<SnekId, u8>,
    pub game_started: bool
}

impl ServerState {
    pub fn get_snek_count(&self) -> usize {
        self.snek_ids.len()
    }

    pub fn get_next_id(&self) -> SnekId {
        let length = self.get_snek_count();
        if length == 0 {
            1
        } else {
            self.snek_ids[length - 1] + 1
        }
    }

    pub fn link_snek(&mut self, address: SocketAddr, id: SnekId) {
        self.address_to_id.insert(address, id);
        self.id_to_address.insert(id, address);
    }

    pub fn start_game(&mut self) {
        self.game_started = true;
    }

    pub fn end_game(&mut self) {
        self.game_started = false;
    }
}

fn send_packet(message_type: MessageType, payload: Vec<u8>, address: SocketAddr, stream_id: u8, sender: &Sender<Packet>) {
    let mut actual_payload = vec![MAGIC_BYTE, message_type as u8];
    actual_payload.extend(payload.iter());
    sender.send(Packet::reliable_sequenced(address, actual_payload, Some(0))).unwrap()
}

fn handle_packet(packet: &Packet, sender: &Sender<Packet>, state: &mut ServerState) {
    let address = packet.addr();
    let payload = packet.payload();
    // println!("{:?}", payload);
    let magic_byte = payload[0];
    if magic_byte != MAGIC_BYTE {
        return;
    }
    let message_type = payload[1];
    let received_data = &payload[2..];
    // let payload_str = String::from_utf8_lossy(packet.payload());
    // let message = payload_str.as_ref();
    match message_type {
        x if x == MessageType::JoinEvent as u8 => {
            // Only allow a maximum of MAX_PLAYERS clients to join the game
            if state.get_snek_count() == MAX_PLAYERS { return }
            if state.address_to_id.contains_key(&address) { return }

            // Assign the client an ID and link it to their address
            let id: u8 = state.get_next_id();
            state.snek_ids.push(id);
            state.link_snek(address, id);
            state.moves.insert(id, 42);
            println!("Snek with ID {} joined", id);

            println!("Sending ID {} back to snek...", id);

            send_packet(MessageType::AssignIdEvent, vec![id], address, StreamId::Event as u8, sender);
            if state.get_snek_count() == MAX_PLAYERS {
                // If MAX_PLAYERS clients have joined, start the game
                state.start_game();

                println!("Game started!");

                // Broadcast IDs & game start event
                for (&snek_address, _) in state.address_to_id.iter() {
                    send_packet(MessageType::BroadcastIdsEvent, state.snek_ids.clone(), snek_address, StreamId::Event as u8, sender);
                    send_packet(MessageType::StartEvent, vec![], snek_address, StreamId::Event as u8, sender);
                }
            }
        },
        x if x == MessageType::Heartbeat as u8 => {
            // Send a heartbeat back to the client to prevent timing out
            send_packet(MessageType::Heartbeat, vec![], address, StreamId::Heartbeat as u8, sender);
        },
        x if x == MessageType::MoveEvent as u8 => {
            // Receive moves from both clients. Once both are received send
            // them to the clients so they can update their game state simultaneously

            // println!("Received message {}", msg);
            let origin_snek_id = state.address_to_id[&address];
            if origin_snek_id != received_data[0] {
                println!("Snek ID movement mismatch!");
                return;
            }
            // println!("Received move from {}", origin_snek_id);
            *state.moves.get_mut(&origin_snek_id).unwrap() = received_data[1];
            // println!("updating move");
            // Only one move has been received, send heartbeat to prevent timing out
            // and to wait for the second move
            send_packet(MessageType::Heartbeat, vec![], address, StreamId::Heartbeat as u8, sender);
        },
        x if x == MessageType::DeathEvent as u8 => {
            println!("Death event");
            if !state.game_started {
                return;
            }
            // let origin_snek_id = state.address_to_id[&address];
            // Broadcast game end event
            match received_data.len() {
                0 => {
                    for (&snek_address, _) in state.address_to_id.iter() {
                        send_packet(MessageType::EndEvent, 
                            vec![GameResult::Tie as u8, INVALID_ID], snek_address, StreamId::Event as u8, sender);
                    }
                    state.end_game();
                },
                1 => {
                    let winner = received_data[0];
                    for (&snek_address, &snek_id) in state.address_to_id.iter() {
                        if snek_id == winner {
                            send_packet(MessageType::EndEvent, 
                                vec![GameResult::Win as u8, winner], snek_address, StreamId::Event as u8, sender);
                        } else {
                            send_packet(MessageType::EndEvent, 
                                vec![GameResult::Loss as u8, winner], snek_address, StreamId::Event as u8, sender);
                        }
                    }
                    state.end_game();
                },
                _ => {
                    for &snek_id in received_data {
                        println!("Snek ID {} is alive", snek_id);
                    }
                }
            }
        }
        _ => {}
    }
}

pub fn server(port: i32) -> Result<(), ErrorKind> {
    let mut config = Config::default();
    config.socket_event_buffer_size = 100;
    let mut socket = Socket::bind_with_config(format!("{}:{}", "127.0.0.1", port.to_string()), config)?;
    println!("Server is listening on port {}", port);

    let (sender, receiver) = (
        socket.get_packet_sender(), socket.get_event_receiver());
    let _thread = thread::spawn(move || socket.start_polling());

    let mut state = ServerState {
        snek_ids: Vec::new(),
        address_to_id: HashMap::new(),
        id_to_address: HashMap::new(),
        moves: HashMap::new(),
        game_started: false
    };

    let mut now = Instant::now();
    'outer: loop {
        if let Ok(event) = receiver.try_recv() {
            match event {
                SocketEvent::Packet(packet) => handle_packet(&packet, &sender, &mut state),
                SocketEvent::Timeout(address) => {
                    println!("Client timed out: {}", address);
                }
                _ => {}
            }
        }
        if state.game_started && now.elapsed().as_millis() >= 120 {
            // Send move to all other players
            for (&snek_address, _) in state.address_to_id.iter() {
                let mut payload = Vec::new();
                for (&origin_snek_id, &sent_move) in state.moves.iter() {
                    if sent_move == 42 {
                        now = Instant::now();
                        continue 'outer;
                    }
                    payload.push(origin_snek_id);
                    payload.push(sent_move);
                }
                send_packet(MessageType::MoveEvent, payload, snek_address, StreamId::Move as u8, &sender);
            }
            now = Instant::now();
        }
        std::thread::sleep(time::Duration::from_millis(100));
    }
}
