Commands
1. `git clone https://github.com/imathur1/rust-project` -> Clone repository
2. `cd rust-project` -> Change to the rust-project directory
3. `cargo run --bin snek-server` -> Create the server
4. `8080` -> Port that the server should listen on
5. Open a new terminal window
6. `cargo run --bin snek-client` -> Creates player 1
7. `SERVER_IP:8080` -> Address of the server that the client will connect to
8. Open a new terminal window
9. `cargo run --bin snek-client` -> Creates player 2
10. `SERVER_IP:8080` -> Address of the server that the client will connect to

Two windows will open, each one representing a different player, and the game will start. Use the arrow keys to move the sneks.

Currently:
- The game can only function locally, so the both the clients and server needs to open on the same machine
    - Technically the code for cross-machine networking is there, we just haven't managed to figure out if its a problem with port-forwarding or the firewall

Future: 
- Plan to put the game on a web server so that multiple machines can connect to the game