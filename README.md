# Rust Snake CLI

A classic Snake game that runs in your terminal, written in Rust with [`crossterm`](https://crates.io/crates/crossterm) for terminal control and [`rand`](https://crates.io/crates/rand) for food placement.

You steer a snake around a bordered play area, eat food to grow longer, and try not to hit the walls or your own tail.

## Features

- Runs in the terminal's alternate screen, so your shell is left untouched when you quit
- Play area sized automatically to your terminal window
- Colored rendering: dark grey walls, green food, a dark yellow head and a yellow body
- Food is placed at random and never spawns on top of the snake
- Pause and resume at any time
- Fixed game speed of one move every 100 ms

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) with Cargo. The project uses the 2024 edition, so you need Rust **1.85 or newer**
- A terminal that supports colors and raw mode

## Getting started

Clone the repository and run the game:

```bash
git clone https://github.com/minhalalikhan/rust-snake-game.git
cd rust-snake-game
cargo run --release
```

To build the binary without running it:

```bash
cargo build --release
./target/release/rust-snake-cli
```

> **Tip:** make your terminal window as large as you like *before* starting. The play area is sized once, at launch, from the current terminal dimensions.

## How to play

1. Press any key on the welcome screen to start.
2. The snake starts in the middle of the screen, moving right.
3. Guide it onto the green food. Each piece you eat adds one segment and one point.
4. The game ends if the snake runs into a wall or into itself.
5. Press any key on the game over screen to exit.

### Controls

| Key | Action |
| --- | --- |
| `↑` `↓` `←` `→` | Change direction |
| `p` | Pause (press any key to resume) |
| `q` | Quit |

The snake cannot reverse directly into itself, so pressing the opposite direction to the one it is travelling in is ignored.

## Project structure

```
.
├── Cargo.toml     # package metadata and dependencies
├── Cargo.lock
└── src
    └── main.rs    # game loop, input handling and rendering
```

### Dependencies

| Crate | Version | Used for |
| --- | --- | --- |
| `crossterm` | 0.27 | Raw mode, alternate screen, cursor control, colors and keyboard events |
| `rand` | 0.8 | Random food placement |

## How it works

- **Game loop:** input is polled without blocking, and the game state advances once every 100 ms tick.
- **Snake:** stored as a `VecDeque<Point>`. Each tick pushes a new head onto the front and pops the tail from the back, unless the snake just ate, in which case the tail stays and the snake grows.
- **Rendering:** the screen is not cleared every tick. The snake and food are redrawn in place, and the cell the tail just left is erased, which avoids flicker.

## Ideas for future work

- Show the score on screen
- Increase speed as the score grows
- Add a high score table
- Make the tick rate configurable from the command line

## Contributing

Issues and pull requests are welcome. For larger changes, please open an issue first to discuss what you would like to change.

## License

No license has been added to this repository yet. Add one (for example MIT) before sharing the project more widely.
