# Prisoner's Dilemma Tournament Simulator

A high-performance Rust implementation of an iterated Prisoner's Dilemma tournament, where different strategies compete against each other in a round-robin fashion. Inspired by The Selfish Gene by Richard Dawkins and Axelrod's seminal work on the evolution of cooperation.

## Overview

The Prisoner's Dilemma is a fundamental concept in game theory. This simulator runs a tournament where different strategies play against each other thousands of times, allowing you to analyze which strategies perform best under various conditions.

### The Game

In each round, two players simultaneously choose to either **Cooperate** or **Defect**:

| Outcome | Payoff |
|---------|--------|
| Both Cooperate | 2 points each |
| Both Defect | 1 point each |
| One Cooperates, One Defects | Defector gets 3, Cooperator gets 0 |

The key insight: mutual cooperation yields the best collective outcome (4 total points), but defection is always individually rational.

## Features

- **Multi-threaded Execution**: Automatically uses all available CPU cores for fast tournament simulation
- **Configurable Parameters**: Adjust thread count, iteration count, and strategy selection via CLI
- **Multiple Strategies**: Test built-in strategies or easily add your own
- **Verbose Reporting**: Optional detailed output for tournament analysis
- **Fast Tournament Play**: Plays millions of iterations efficiently

## Installation

### Prerequisites

- Rust 1.70 or later ([install Rust](https://www.rust-lang.org/tools/install))

### Building

```bash
cargo build --release
```

The compiled binary will be at `target/release/prisoners_dilemma`.

## Usage

### Basic Usage

Run a tournament with all default strategies:

```bash
cargo run --release
```

### Command-Line Options

```bash
cargo run --release -- [OPTIONS]
```

#### Options

- `-t, --threads <THREADS>`: Number of threads to use (default: number of CPU cores)
- `-i, --iterations <ITERATIONS>`: Number of iterations per game (default: 1,000,000)
- `-s, --strategies <STRATEGIES>`: Comma-separated list of strategies to include
- `-v, --verbose`: Show additional tournament details

#### Examples

**Run with specific strategies:**

```bash
cargo run --release -- --strategies tit-for-tat,always-defect,random
```

**Run with fewer iterations (faster, for testing):**

```bash
cargo run --release -- --iterations 10000
```

**Run with custom thread count and verbose output:**

```bash
cargo run --release -- --threads 4 --iterations 100000 --verbose
```

**Limit to two strategies:**

```bash
cargo run --release -- --strategies always-cooperate,always-defect --iterations 1000000 --verbose
```

## Available Strategies

### Always Cooperate
Always plays Cooperate, regardless of the opponent's moves. Vulnerable to exploitation but promotes cooperation.

### Always Defect
Always plays Defect, regardless of the opponent's moves. Guarantees individual payoff but prevents mutual cooperation.

### Tit for Tat
Cooperates on the first move, then copies the opponent's previous move on subsequent rounds. A simple, highly effective strategy that is nice (never defects first), retaliatory (punishes defection), and forgiving (returns to cooperation).

### Random
Randomly chooses to Cooperate or Defect with 50/50 probability. Useful for testing robustness of strategies.

### Two Tits for Tat
Similar to Tit for Tat but requires two consecutive defections from the opponent before retaliating. More forgiving than Tit for Tat, allowing recovery from mutual misunderstandings.

## Understanding the Output

Each line of output represents one matchup result:

```
TitForTat vs Always Defect: 999000 vs 1000001
```

This means:
- TitForTat scored 999,000 points
- Always Defect scored 1,000,001 points
- The matchup ran for the specified number of iterations

**Note**: In a tournament, each strategy plays against every other strategy (including itself), so the total number of lines equals `num_strategies²`.

## Architecture

### Core Components

- **Strategy Trait**: Defines the interface for implementing game strategies
- **Player**: Manages a strategy and tracks score throughout a game
- **PrisonerDilemmaGame**: Executes the game, manages history, and calculates payoffs
- **ThreadPool**: Distributes games across available CPU cores

### Design Patterns

- **Arc (Atomic Reference Counting)**: Safely shares strategy objects across threads
- **MPSC Channels**: Collects results from worker threads
- **Trait Objects**: Allows polymorphic strategy behavior without compile-time specialization

## Performance

The simulator is optimized for speed:

- **Multi-threaded**: Automatically parallelizes across all CPU cores
- **Efficient History Tracking**: Only keeps necessary move history
- **Static Dispatch**: Uses trait objects with minimal overhead

On modern hardware, expect to simulate ~1 million iterations per strategy pair in a few seconds.

## Extending the Simulator

### Adding a New Strategy

To add a new strategy, implement the `Strategy` trait:

```rust
pub struct MyStrategy;

impl Strategy for MyStrategy {
    fn play(&self, hist: &History, hist_inx: usize) -> Move {
        // hist contains all previous moves: hist[i] = [player1_move, player2_move]
        // hist_inx is 0 for player 1, 1 for player 2
        // Return your strategy's move
        Move::Cooperate
    }

    fn name(&self) -> String {
        "My Strategy".to_string()
    }
}
```

Then add it to the strategies vector in the `parse_strategies()` function or directly in main.

## Testing

The project is easily testable with reduced iteration counts:

```bash
cargo run --release -- --iterations 1000 --verbose
```

## Project Structure

```
prisoners_dilemma/
├── Cargo.toml           # Project manifest and dependencies
├── Cargo.lock           # Locked dependency versions
├── README.md            # This file
└── src/
    └── main.rs          # All implementation (single-file project)
```

## Dependencies

- **clap**: Command-line argument parsing with derive macros
- **num_cpus**: Detects available CPU cores
- **rand**: Random number generation for Random strategy
- **threadpool**: Thread pool implementation for parallel execution

## License

This project is provided as-is for educational purposes.

## Related Resources

- [Prisoner's Dilemma on Wikipedia](https://en.wikipedia.org/wiki/Prisoner%27s_dilemma)
- [Axelrod's Tournament](https://en.wikipedia.org/wiki/Prisoner%27s_dilemma#Axelrod's_tournament_and_successful_strategy_conditions) - The classic tournament study

## Future Enhancements

Potential improvements for the project:

- Export results to CSV or JSON format
- Calculate aggregate statistics (total score, win rate, etc.)
- Support for weighted tournament (multiple rounds with different iterations)
- Custom payoff matrix configuration
- Strategy performance analysis and ranking
- Interactive mode for designing custom strategies
