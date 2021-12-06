# CS128H Final Project

### Group Name: Team::Team

### Names and NetIDs
Ishaan Mathur, imathur3
James Huang, jamesjh4
Louis Asanaka, asanaka2

### Project Introduction
- Description:
    - A multiplayer version of Snake (Snek)
- Goals, Objectives and Why we chose this project: 
    - Branch and build off of the skills we learned
    - Familiarize ourselves with networking and graphics in Rust
    - Interested in building a simple game that we are familiar with

### System Overview
- Graphics
    - Create a basic UI with a grid showing the state of the game (snek positions, head, and body). The UI will be using Macroquad
    - **Task List**
        - [x] Window using [Macroquad](https://github.com/not-fl3/macroquad)
        - [x] Gridboard
        - [x] Snek head and body
        - [x] Snek movement
- Game logic
    - Handle interactions between sneks and their environment.
    - **Task List**
        - [x] Snek to snek collision
        - [x] Snek to wall collision
        - [x] Alive/death snek state
        - [x] Board representation
- Networking
    - Synchronize client data and broastcast a global game state.
    - **Task List**
        - Client
            - [x] Receiving and updating game state
            - [x] Serializing and sending game state
        - Server
            - [x] Handling client connections and disconnections
            - [x] Broadcast game state

### Possible Challenges
- Writing the functionality to determine collisions between sneks
- Continously updating the state of the board and the player positions
- Handling simultaenous requests from every player in a parallelizable manner 
- Developing the graphical interface to handle user interaction and display the current game state

### References
Slither.io

Snake from a Minecraft minigame
