# CodeInvaders
CodeInvaders is a simple game developed in Rust for the course "Concepts of Higher Programming Languages". The game is inspired by the classic arcade game "Space Invaders" and is built using the [`macroquad`](https://github.com/not-fl3/macroquad) library.

## Table of Contents
* Installation
* Usage
* Gameplay
* Developement
* License

## Installation
To install it on your system just download the latest release package on the [GitHub page](https://github.com/lossphilipp/CodeInvaders/releases) and execute it.

## Usage
Once the game is running, most of the available controls are showed on screen. These are the most important ones:

* **Arrow Keys**: Move the player left and right.
* **Space**: Shoot bullets.
* **Enter**: Start the game or proceed to the next level.
* **Escape**: Exit to the main menu or finish the game.

## Gameplay
The objective of CodeInvaders is to defeat all enemies on the screen by shooting bullets at them. The game consists of multiple levels, each with increasing difficulty. Your score is displayed at the end of each level and can be saved to the high scores list if it qualifies.

### Game States
* **Menu**: The main menu where you can start the game or view high scores.
* **Playing**: The main gameplay state where you control the player and shoot enemies.
* **LevelComplete**: Displayed when you complete a level.
* **GameOver**: Displayed when you lose the game.
* **HighScores**: Displays the list of high scores.
* **EnterName**: Allows you to enter your name if your score qualifies for the high scores list.

## Developement
To program and debug CodeInvaders, you need to have Rust and Cargo installed on your system. Follow these steps to get started:

1. Clone the repository:
```sh
git clone https://github.com/lossphilipp/CodeInvaders.git
cd CodeInvaders
```

2. Run the game:
```sh
cargo run
```

## License
This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.