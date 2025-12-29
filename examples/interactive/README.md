# Interactive Examples

Real-time demonstrations and interactive tools for visualizing range bar formation and understanding market behavior.

## Examples in this category:

### `historical_replay.rs`

**Purpose**: Interactive historical range bar visualization with time-aware playback
**Usage**: `cargo run --example interactive/historical_replay [SYMBOL] [MARKET_TYPE]`
**Features**:

- Real-time range bar formation visualization
- Keyboard controls (q=quit, +=faster, -=slower, p=pause)
- Colored directional arrows (↑/↓)
- Smart column alignment
- 2-day historical data playback

**Examples**:

```bash
# Default: DOGEUSDT spot market
cargo run --example interactive/historical_replay

# Specific symbol and market
cargo run --example interactive/historical_replay BTCUSDT um
```

### `test_historical_replay.rs`

**Purpose**: Faster test version of historical replay (1-day data)
**Usage**: `cargo run --example interactive/test_historical_replay [SYMBOL] [MARKET_TYPE]`
**Use when**: Testing or demonstrating with shorter datasets

### `format_demo.rs`

**Purpose**: Demonstrates the new aligned output formatting
**Usage**: `cargo run --example interactive/format_demo`
**Shows**: Before/after formatting improvements with realistic data

### `market_comparison_demo.rs`

**Purpose**: Shows usage examples for different market types
**Usage**: `cargo run --example interactive/market_comparison_demo`
**Output**: Command-line examples and market type explanations

## When to use these examples:

- 🎮 **Interactive learning** about range bar formation
- 👀 **Visual demonstration** of market behavior differences
- ⚡ **Real-time insights** into trading patterns
- 📚 **Teaching tool** for understanding range bars

## Controls (where applicable):

- `q` - Quit
- `+` - Increase speed
- `-` - Decrease speed
- `p` - Pause/resume
- `Ctrl+C` - Emergency exit

## Requirements:

- Terminal with color support recommended
- Internet connection for historical data
- Interactive terminal (for keyboard controls)
