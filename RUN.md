{command} -> {description of step}
1. git clone https://github.com/imathur1/rust-project -> Clone repository
2. cd ../rust-project/ -> Change to the rust-project directory
3. cargo run --bin snek-client -> Create the server
4. 8080 -> Port that the server should listen on
5. Open a new terminal window
6. cargo run --bin snek-client -> Creates player 1
7. 8080 -> Port of the server that the client will connect to
8. Open a new terminal window
9. cargo run --bin snek-client -> Creates player 2
10. 8080 -> Port of the server that the client will connect to

Two windows will open, each one representing a different player, and the game will start. Use the arrow keys to move the sneks.

Currently:
- The game can only function locally, so the both the clients and server needs to open on the same machine

Future: 
- Plan to put the game on a web server so that multiple machines can connect to the game