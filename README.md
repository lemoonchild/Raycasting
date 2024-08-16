# Feed The Cat - Rust Game

### Description
Feed The Cat is a first-person adventure game developed in Rust, where the player's mission is to feed a cat by collecting fish and finding keys to progress through locked doors. The game is noted for its intuitive handling and immersive experience, with controls that allow for fluid and natural interaction.

### Features
- **Versatile Controls:** The game supports both keyboard and gamepad to suit the preferences of each player.
  - **Keyboard:** Use WASD to navigate the maze.
  - **Mouse:** Control horizontal camera rotation to explore the world.
  - **Gamepad:** Full support for gamepad gameplay.
- **Dynamic Interaction:** Collect fish and keys to solve puzzles and unlock new areas.
- **Real-Time Rendered Graphics:** Utilizes a ray-casting system for rendering 3D environments.

### Technologies and Libraries Used
The game has been developed using various Rust libraries, including:
- `minifb`: for creating the game window and handling input events.
- `nalgebra-glm`: for complex mathematical operations and vector management.
- `gilrs`: for integration and handling of game controllers.
- `once_cell`: for lazy initialization of resources.
- `Arc`: for safe resource management across threads.

### Installation
To run the game, you will need to have Rust and Cargo installed on your system. Follow these steps to get started:

1. Clone this repository:

`git clone https://github.com/lemoonchild/Raycasting.git`

2. Navigate to the game directory:
`cd src`

3. Compile and run the project:

`cargo run --release`

### Contributions
Contributions are welcome. If you have suggestions for improving the game or want to report bugs, please open an issue or send a pull request.

### License
This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

### Credits
- Design and development: Madeline Castro
- Special thanks to everyone who contributed ideas and technical support.

We hope you enjoy playing Feed The Cat as much as we enjoyed creating it!

### Demo of the game 

Video of the game: https://youtu.be/CXb7RXPuzyQ


Video using controller: https://youtu.be/aa15qhOOoo4 


