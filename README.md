# 🎮 Tic Tac Toe Advanced

A terminal-based Tic Tac Toe game written in **Rust** featuring an **unbeatable AI** opponent powered by the Minimax algorithm.

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Terminal](https://img.shields.io/badge/Terminal-4D4D4D?style=for-the-badge&logo=windows-terminal&logoColor=white)

---

## ✨ Features

- 🤖 **Unbeatable AI** using the Minimax algorithm
- 🎨 **Colorful terminal UI** with cursor-based navigation
- ⚡ **Cross-platform** support (Windows, macOS, Linux)
- 🔄 **Restart/Quit** options after each game
- 🎯 **Choose your side** - play as X (first) or O (second)

---

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70 or higher recommended)

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/tic-tac-toe-advanced.git
cd tic-tac-toe-advanced

# Build and run
cargo run --release
```

---

## 🎯 How to Play

### Controls

| Key         | Action                       |
| ----------- | ---------------------------- |
| `Tab`       | Move to next option/cell     |
| `Shift+Tab` | Move to previous option/cell |
| `Enter`     | Select/confirm               |
| `Esc`       | Quit game                    |

### Game Flow

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│  Choose Player  │ ──▶ │    Game Play    │ ──▶ │    End Menu     │
│   (X or O)      │     │  (the actual    │     │  (Win/Lose/Tie) │
│                 │     │   game board)   │     │                 │
└─────────────────┘     └─────────────────┘     └─────────────────┘
        ▲                                                │
        └────────────────── RESTART ─────────────────────┘
```

---

## 🏗️ Architecture

### Project Structure

```
src/
├── main.rs              # Entry point, game loop, terminal setup
├── lib.rs               # Library root, module exports
├── models.rs            # Re-exports models
├── screen.rs            # Re-exports screen modules
├── utils.rs             # Re-exports utility modules
├── models/
│   ├── player.rs        # Player enum (X, O) with helper methods
│   └── tabs.rs          # Generic Tabs<T> struct for UI navigation
├── screen/
│   ├── choose_player.rs # Player selection screen
│   ├── game_play.rs     # Main game board screen
│   └── end_menu.rs      # Win/Lose/Tie result screen
└── utils/
    ├── get_best_move.rs # Minimax AI algorithm
    └── get_winner.rs    # Winner detection logic
```

### Core Data Structures

#### Player

```rust
pub enum Player {
    X,  // Always goes first
    O,
}
```

- `char()` → returns `'X'` or `'O'` for display
- `other()` → returns the opponent (`X.other()` = `O`)

#### Board Representation

```
 0 | 1 | 2
---+---+---
 3 | 4 | 5
---+---+---
 6 | 7 | 8
```

The board is a simple `[char; 9]` array where:

- `' '` (space) = empty cell
- `'X'` or `'O'` = occupied cell

#### Tabs (UI Navigation)

```rust
pub struct Tabs<T> {
    index: isize,                    // Currently selected index
    positions: Vec<(u16, u16, T)>,   // (x, y, value) for each tab
}
```

A generic component for cycling through selectable items with screen coordinates.

---

## 🧠 The AI: Minimax Algorithm

The computer opponent uses the **Minimax algorithm**, making it **impossible to beat**. The best outcome you can achieve is a draw!

### How It Works

The algorithm simulates **every possible future** of the game:

```
Current Board        Computer thinks:
                     "If I play here..."
 X |   |                    │
---+---+---                 ▼
   |   |             ┌──────────────┐
---+---+---          │ Simulate all │
   |   |             │ possible     │
                     │ outcomes     │
                     └──────────────┘
                            │
              ┌─────────────┼─────────────┐
              ▼             ▼             ▼
          I win (+1)    Draw (0)    I lose (-1)
```

### The Logic

1. **Computer's turn (maximizing):** Pick the move with the **highest** score
2. **Human's turn (minimizing):** Assume human picks the move with the **lowest** score
3. **Scores:**
   - Computer wins → `+1`
   - Human wins → `-1`
   - Draw → `0`

The computer explores the entire game tree recursively, assuming the human plays optimally.

---

## 🔄 Game Loop

```
┌──────────────────────────────────────────────────────┐
│                    GAME LOOP                         │
└──────────────────────────────────────────────────────┘
                         │
                         ▼
              ┌─────────────────────┐
              │   Draw the board    │
              └─────────────────────┘
                         │
                         ▼
              ┌─────────────────────┐
              │  Check for winner   │──── Yes ──▶ Return winner
              │     or draw?        │
              └─────────────────────┘
                         │ No
                         ▼
              ┌─────────────────────┐
         ┌────│  Whose turn is it?  │────┐
         │    └─────────────────────┘    │
         ▼                               ▼
   ┌───────────┐                 ┌───────────────┐
   │  COMPUTER │                 │     HUMAN     │
   │           │                 │               │
   │ Call      │                 │ Wait for      │
   │ minimax   │                 │ Tab/Enter     │
   │ to find   │                 │ input         │
   │ best move │                 │               │
   └───────────┘                 └───────────────┘
         │                               │
         └───────────┬───────────────────┘
                     ▼
              ┌─────────────────────┐
              │   Place the mark    │
              │   Switch turns      │
              └─────────────────────┘
                     │
                     └──────────▶ (loop back to top)
```

---

## 🏆 Winner Detection

The game checks all 8 possible winning combinations:

```rust
let winning_combinations: [[usize; 3]; 8] = [
    [0, 1, 2], // Top row
    [3, 4, 5], // Middle row
    [6, 7, 8], // Bottom row
    [0, 3, 6], // Left column
    [1, 4, 7], // Middle column
    [2, 5, 8], // Right column
    [0, 4, 8], // Diagonal ↘
    [2, 4, 6], // Diagonal ↙
];
```

---

## 🖥️ Terminal UI

Built with [crossterm](https://docs.rs/crossterm) for cross-platform terminal manipulation:

- **Raw mode**: Capture keypresses immediately (no Enter required)
- **Cursor control**: Position text anywhere on screen
- **Colors**: Red highlight for selection, cyan for borders
- **Screen clearing**: Smooth redraw each frame

---

## 📦 Dependencies

| Crate                                           | Purpose                                       |
| ----------------------------------------------- | --------------------------------------------- |
| [crossterm](https://crates.io/crates/crossterm) | Terminal manipulation, input handling, colors |

---

## 💡 Can You Beat the AI?

Since the AI explores **all possible futures** and always picks the optimal move:

| Your Play   | Outcome         |
| ----------- | --------------- |
| Perfect     | **Draw** 🤝     |
| Any mistake | **You lose** ❌ |

The only way to "win" is to enjoy the challenge of achieving a draw! 🎯

---

## 🛠️ Technical Highlights

- **Zero unsafe code** - 100% safe Rust
- **No external runtime** - Compiles to a single binary
- **Efficient AI** - Minimax explores the game tree optimally
- **Clean architecture** - Separation of concerns (models, screens, utils)
- **Cross-platform input handling** - Works in Windows Terminal, VS Code, and Unix terminals

---

## 📄 License

This project is open source and available under the [MIT License](LICENSE).

---

## 🤝 Contributing

Contributions, issues, and feature requests are welcome! Feel free to check the [issues page](https://github.com/yourusername/tic-tac-toe-advanced/issues).

---

<p align="center">
  Made with ❤️ and Rust 🦀
</p>
