# Gomoku Terminal Game (5 in a row)

A terminal-based implementation of the classic Gomoku game written in Rust. Play against another player or challenge a computer opponent powered by a heuristic AI algorithm.

## Features

- 15x15 game board (configurable in `config.rs`)
- Two game modes:
  - Player vs Player
  - Player vs Computer
- Adjustable AI difficulty levels (0–9)
- Keyboard-based controls
- Colored terminal rendering and cursor highlighting
- Undo functionality in Player vs Computer mode
- Win detection:
  - Horizontal
  - Vertical
  - Diagonal
- Efficient heuristic AI with candidate move filtering
- Move history tracking

## Running Instructions

Before running the project, make sure you have Rust installed.

**Clone the Repository**

```bash
git clone https://github.com/IliaPoliak/5-in-a-row
cd 5-in-a-row
```

**Compile and run the project**

```bash
# To play against computer (the number 0–9 controls the difficulty level)
cargo run -- c 9
```

```bash
# To play against another person on the same device
cargo run -- p
```

## Gameplay

The goal of Gomoku is simple: place 5 of your tiles in a row before your opponent does.

The game runs entirely in the terminal and uses keyboard controls for movement and actions.

**Controls:**

| Key           | Action                         |
| ------------- | ------------------------------ |
| Arrow Keys    | Move cursor                    |
| Enter / Space | Place tile                     |
| U / u         | Undo last move (PvC mode only) |
| ESC           | Exit game                      |

## AI Implementation

The AI evaluates all possible candidate moves and assigns scores based on board patterns.

**This approach provides:**

- Good performance
- Low computational cost
- Fast response times

### Candidate Move Filtering

Only positions near already placed tiles are evaluated.

**Benefits:**

- Reduces search space
- Improves performance significantly
- Avoids unnecessary evaluations

### Sliding Window Evaluation

The AI evaluates every possible 5-cell window instead of only checking streaks.

**Example:**

```
XX_XX
```

A streak-based evaluation would treat this as two separate sequences.

The sliding window approach recognizes it as a strong potential threat and responds accordingly.
