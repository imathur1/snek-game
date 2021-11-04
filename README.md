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
    - Create a basic UI with a grid and scoreboard showing the state of the game (snek positions, scores, sneks alive). The UI will be using 
    - **Task List**
        - [ ] Window using [Quicksilver](https://ryanisaacg.com/quicksilver)
        - [ ] Gridboard
        - [ ] Scoreboard
        - [ ] Title screen with configuration for names & potentially lobbies?
- Game logic
    - Handle interactions between sneks and keep track of scoring.
    - **Task List**
        - [ ] Snek to snek collision
        - [ ] Alive/death snek state
        - [ ] Board representation
        - [ ] Scoring for every snek
- Networking
    - Synchronize client data and broastcast a global game state.
    - **Task List**
        - Client
            - [ ] Receiving and updating game state
            - [ ] Serializing and sending game state
        - Server
            - [ ] Handling client connections and disconnections
            - [ ] Broadcast game state

### Possible Challenges
- Writing the functionality to determine collisions between sneks
- Continously updating the state of the board and the player positions
- Handling simultaenous requests from every player in a parallelizable manner 
- Developing the graphical interface to handle user interaction and display the current game state

### References
Slither.io

Snake from a Minecraft minigame
