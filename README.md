# rangebar

[![Crates.io](https://img.shields.io/crates/v/rangebar)](https://crates.io/crates/rangebar)
[![Documentation](https://docs.rs/rangebar/badge.svg)](https://docs.rs/rangebar)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

<!-- cargo-rdme start -->

Non-lookahead range bar construction for cryptocurrency and forex trading.

[![Crates.io](https://img.shields.io/crates/v/rangebar.svg)](https://crates.io/crates/rangebar)
[![Documentation](https://docs.rs/rangebar/badge.svg)](https://docs.rs/rangebar)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

This crate provides algorithms for constructing range bars from tick data
with temporal integrity guarantees, ensuring no lookahead bias in financial backtesting.

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
rangebar = "5.2"
```

### Meta-Crate

This is a meta-crate that re-exports all rangebar sub-crates for backward compatibility
with v4.0.0. New code should depend on specific sub-crates directly:

- `rangebar-core` - Core algorithm and types
- `rangebar-providers` - Data providers (Binance, Exness)
- `rangebar-config` - Configuration management
- `rangebar-io` - I/O operations and Polars integration
- `rangebar-streaming` - Real-time streaming processor
- `rangebar-batch` - Batch analytics engine
- `rangebar-cli` - Command-line tools

### Features

- `core` - Core algorithm (always enabled)
- `providers` - Data providers (Binance, Exness)
- `config` - Configuration management
- `io` - I/O operations and Polars integration
- `streaming` - Real-time streaming processor
- `batch` - Batch analytics engine
- `full` - Enable all features

### Basic Usage

```rust
use rangebar::{RangeBarProcessor, AggTrade, FixedPoint};

// Create processor with 250 basis points threshold (2.5%)
let mut processor = RangeBarProcessor::new(250).unwrap();

// Create sample aggTrade
let trade = AggTrade {
    agg_trade_id: 1,
    price: FixedPoint::from_str("50000.0").unwrap(),
    volume: FixedPoint::from_str("1.0").unwrap(),
    first_trade_id: 1,
    last_trade_id: 1,
    timestamp: 1609459200000,
    is_buyer_maker: false,
    is_best_match: None,
};

// Process aggTrade records into range bars
let agg_trade_records = vec![trade];
let bars = processor.process_agg_trade_records(&agg_trade_records).unwrap();
```

### Dual-Path Architecture

#### Streaming Mode (Real-time)
```rust
use rangebar::streaming::StreamingProcessor;

let threshold_bps = 25; // 0.25% range bars
let processor = StreamingProcessor::new(threshold_bps);
// Real-time processing with bounded memory
```

#### Batch Mode (Analytics)
```rust
use rangebar::batch::BatchAnalysisEngine;
use rangebar::core::types::RangeBar;

let range_bars: Vec<RangeBar> = vec![]; // Your range bar data
let engine = BatchAnalysisEngine::new();
// let result = engine.analyze_single_symbol(&range_bars, "BTCUSDT").unwrap();
```

### Links

- [GitHub Repository](https://github.com/terrylica/rangebar)
- [API Documentation](https://docs.rs/rangebar)
- [Changelog](https://github.com/terrylica/rangebar/blob/main/CHANGELOG.md)
- [Algorithm Specification](https://github.com/terrylica/rangebar/blob/main/docs/specifications/algorithm-spec.md)

<!-- cargo-rdme end -->

## License

MIT license. See [LICENSE](https://github.com/terrylica/rangebar/blob/main/LICENSE) for details.
