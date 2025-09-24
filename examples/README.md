# Historical Range Bar Visualizer

Interactive visualization of range bar formation using historical BTCUSDT aggTrades data with time-aware accelerated playback.

## Core Features

- **25 BPS Threshold**: Range bars close at 0.25% price movements from bar open price
- **Time-Aware Playback**: Respects historical time intervals between trades (accelerated)
- **Single-Line Updates**: Clean terminal display using carriage return (`\r`)
- **Interactive Controls**: Non-blocking keyboard controls during playback
- **Real Data**: Uses Binance Vision historical aggTrades data

## Usage

### Main Visualizer (3 Months)
```bash
cargo run --example historical_replay
```

Loads 90 days of historical BTCUSDT data (~120M trades) and visualizes range bar formation at 10,000x acceleration.

### Test Visualizer (1 Day)
```bash
cargo run --example test_historical_replay
```

Loads 1 day of recent data (~1.3M trades) for quick testing and validation.

## Interactive Controls

- **`q`** - Quit visualizer
- **`+`** - Increase playback speed (2x multiplier)
- **`-`** - Decrease playback speed (0.5x multiplier)
- **`p`** - Toggle pause/resume
- **`Ctrl+C`** - Graceful shutdown

## Display Format

### Building Bar
```
Building bar #42: 15,429 trades, current: $115,284.20, open: $115,139.50
```

### Completed Bar
```
✅ RANGE BAR #42: OHLC = 115139.50/115428.90/115139.50/115428.90, Volume = 125.340000, Trades: 23528
```

## Expected Results

With 25 BPS (0.25%) threshold:
- **~55 bars per day** (varies with volatility)
- **~23,500 trades per bar** (average)
- **Exact 0.25% movements** from bar open price

## Architecture

### Consolidated Structure
```
examples/
├── historical_replay.rs        # Main 3-month visualizer
├── test_historical_replay.rs   # 1-day test version
├── basic_usage.rs             # Core library example
├── tier1_symbols.rs           # Symbol discovery
└── common/
    ├── mod.rs                # Module declaration
    └── data.rs               # Shared data structures & loader
```

### Key Components

**HistoricalDataLoader** (`common/data.rs`)
- Loads Binance Vision aggTrades data
- Handles CSV parsing and data conversion
- Supports single-day and multi-day loading

**PlaybackEngine** (`historical_replay.rs`)
- Time-aware trade replay with acceleration
- Maintains historical timing between trades
- Interactive speed control

**TerminalDisplay** (`historical_replay.rs`)
- Single-line updating using `\r`
- New line only on range bar completion
- Clean progress indicators

## Data Source

- **Binance Vision API**: `https://data.binance.vision/data/futures/um/daily/aggTrades/`
- **Market**: USD-M Futures (BTCUSDT)
- **Data Type**: aggTrades (aggregated trades)
- **Format**: CSV files compressed in ZIP archives

## Implementation Notes

### Threshold Algorithm
```rust
// 25 BPS = 0.25% = threshold_bps / 10,000
let upper_threshold = bar_open_val + (bar_open_val * threshold_bps) / 10_000;
let lower_threshold = bar_open_val - (bar_open_val * threshold_bps) / 10_000;

// Bar closes when: price >= upper_threshold OR price <= lower_threshold
```

### Time-Aware Playback
```rust
let delta_ms = (next_timestamp - current_timestamp) as f64;
let accelerated_delay_ms = delta_ms / acceleration_factor;
tokio::time::sleep(Duration::from_millis(accelerated_delay_ms as u64)).await;
```

### Error Handling
- **No fallbacks**: All errors propagate immediately
- **Exception-only failure**: No silent error handling
- **Data integrity**: Proper timestamp sorting and validation

## Performance

- **Data Loading**: ~2-3 seconds per day (network dependent)
- **Processing Speed**: 137M+ trades/second range bar construction
- **Memory Usage**: ~8MB buffer per day of data
- **Acceleration**: Up to 50,000x playback speed

## Validation

All range bars verified to breach exactly 25 BPS (0.25%) threshold:
```bash
# Example verification output
✅ BAR #1: Low breach -0.250% (114895.40 ≤ 114895.94)
✅ BAR #2: Low breach -0.250% (114608.10 ≤ 114608.16)
✅ BAR #3: High breach +0.250% (114607.30 ≥ 114607.20)
```

## Dependencies

- **rangebar**: Core range bar processing library
- **crossterm**: Non-blocking keyboard input
- **tokio**: Async runtime and timers
- **reqwest**: HTTP client for data fetching
- **csv**: CSV parsing with serde
- **zip**: ZIP archive extraction
- **chrono**: Date/time handling