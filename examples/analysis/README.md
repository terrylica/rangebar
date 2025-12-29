# Analysis Examples

Market research, statistical analysis, and comparison tools for understanding range bar behavior across different markets and instruments.

## Examples in this category:

### `market_efficiency_analysis.rs`

**Purpose**: Analyzes which markets are more efficient at generating range bars per aggTrade
**Usage**: `cargo run --example analysis/market_efficiency_analysis`
**Key insights**: Compares spot vs futures market efficiency

### `rangebar_generation_comparison.rs`

**Purpose**: Comprehensive comparison of range bar generation between spot and UM futures
**Usage**: `cargo run --example analysis/rangebar_generation_comparison`
**Output**: Detailed statistics on aggTrades vs range bars generated

### `tier1_volume_comparison.rs`

**Purpose**: Compares aggTrade counts across Tier-1 instruments
**Usage**: `cargo run --example analysis/tier1_volume_comparison`
**Shows**: Daily aggTrade patterns for major crypto pairs

### `tier1_symbols.rs`

**Purpose**: Discovery and analysis of Tier-1 cryptocurrency symbols
**Usage**: `cargo run --example analysis/tier1_symbols`
**Output**: Symbol availability across different Binance markets

## When to use these examples:

- 📊 **Research market behavior** across spot vs futures
- 📈 **Compare trading patterns** between different instruments
- 🔍 **Analyze range bar efficiency** for different market types
- 📋 **Generate statistical reports** for market analysis

## Requirements:

All analysis examples require:

- Internet connection (for historical data)
- Recent date data availability
- ~30 seconds runtime for comprehensive analysis
