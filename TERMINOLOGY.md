# Binance Data Terminology Guide

This guide clarifies the three-level hierarchy of trading data used in the rangebar crate and addresses common confusion between different types of "trades."

## Three-Level Data Hierarchy

### Level 1: Individual Market Executions (Exchange Level)
- **What**: Actual buy/sell orders executed on Binance exchange
- **Example**: Trade ID 4426785115 - someone bought 0.1 BTC at $42,313.90
- **Access**: We only see these as ID ranges in AggTrade records, not individual records
- **Characteristics**:
  - Each has unique `trade_id` (sequential)
  - Executed at specific price and timestamp
  - Has buyer/seller maker/taker roles

### Level 2: AggTrade Records (Data Format)
- **What**: Binance aggregates multiple individual executions at same price within ~100ms
- **Example**: AggTrade ID 1965151410 contains individual trades 4426785115-4426785123 (9 trades)
- **Purpose**: Reduces data volume while preserving market microstructure information
- **Fields**:
  - **Spot Markets (8 fields)**: Including `is_best_match` field
  - **Futures Markets (7 fields)**: Without `is_best_match` field

### Level 3: Range Bars (Analysis Output)
- **What**: Price-movement based aggregation we produce for analysis
- **Contains**: Multiple AggTrade records aggregated by price movement thresholds
- **Tracks**: Both individual execution count AND AggTrade record count
- **Purpose**: Non-lookahead bias technical analysis

## Data Structure Details

### Binance AggTrade Record Structure

#### Spot Market AggTrades (8 columns, no headers)
```
Column  | Field Name      | Description
--------|----------------|--------------------------------------------------
1       | agg_trade_id   | Unique ID for this aggregated record
2       | price          | Execution price (all individual trades same price)
3       | quantity       | Total volume across all individual trades
4       | first_trade_id | First individual trade ID in aggregation
5       | last_trade_id  | Last individual trade ID in aggregation
6       | timestamp      | Timestamp (milliseconds)
7       | is_buyer_maker | true = sell pressure, false = buy pressure
8       | is_best_match  | Was trade best available price? (SPOT ONLY)
```

#### UM Futures AggTrades (7 columns, with headers)
```
Column  | Field Name      | Description
--------|----------------|--------------------------------------------------
1       | agg_trade_id   | Unique ID for this aggregated record
2       | price          | Execution price
3       | quantity       | Total volume across all individual trades
4       | first_trade_id | First individual trade ID in aggregation
5       | last_trade_id  | Last individual trade ID in aggregation
6       | transact_time  | Timestamp (milliseconds)
7       | is_buyer_maker | true = sell pressure, false = buy pressure
```

### Range Bar Structure (Enhanced in v1.0.0)

#### New Primary Fields
- `individual_trade_count: u32` - Number of actual exchange executions
- `agg_record_count: u32` - Number of AggTrade records processed
- `first_trade_id: i64` - First individual trade ID in range bar
- `last_trade_id: i64` - Last individual trade ID in range bar
- `data_source: DataSource` - Market type (spot/futures)

#### Deprecated Fields (Removed in v1.0.0)
- ~~`trade_count: i64`~~ - **REMOVED**: Use `individual_trade_count` instead
- ~~`first_id: i64`~~ - **REMOVED**: Use `first_trade_id` instead
- ~~`last_id: i64`~~ - **REMOVED**: Use `last_trade_id` instead

## Counting Methodology

### Individual Trade Count Calculation
```rust
// From AggTrade record
let individual_trades = agg_trade.last_trade_id - agg_trade.first_trade_id + 1;

// Example: first_id=4426785115, last_id=4426785123
// individual_trades = 4426785123 - 4426785115 + 1 = 9 trades
```

### Range Bar Aggregation
```rust
// Range bar accumulates across multiple AggTrade records
for agg_record in agg_records {
    range_bar.individual_trade_count += agg_record.individual_trade_count();
    range_bar.agg_record_count += 1;
}
```

### Aggregation Efficiency Metric
```rust
// New method in v1.0.0
let efficiency = range_bar.aggregation_efficiency();
// Returns: individual_trades / agg_records
// Higher values = more efficient aggregation by Binance
```

## Method Naming Conventions (v1.0.0)

### New Primary Methods
- `process_agg_trade_records(&[AggTrade])` - Clear input type
- `individual_trade_count()` - Clear counting semantics
- `aggregation_efficiency()` - New metric

### Deprecated Methods (Removed in v1.0.0)
- ~~`process_trades(&[AggTrade])`~~ - **REMOVED**: Use `process_agg_trade_records` instead
- ~~`trade_count()`~~ - **REMOVED**: Use `individual_trade_count` instead

## Common Confusion Points Clarified

### ❌ INCORRECT Understanding
"We process individual trades and count them"

### ✅ CORRECT Understanding
"We process AggTrade records (which contain multiple individual trades) and count both the records and the individual executions within them"

### Real Example from Data
```
AggTrade ID: 1965151412
Price: 42313.9
Quantity: 4.418 BTC
First Trade ID: 4426785125
Last Trade ID: 4426785146
Individual Trades: 4426785146 - 4426785125 + 1 = 22 trades

This ONE AggTrade record represents 22 actual market executions
```

## Market Differences

### Spot vs Futures Data
| Aspect | Spot Market | UM Futures |
|--------|-------------|------------|
| Columns | 8 | 7 |
| Headers | None | Present |
| Best Match | Yes (`is_best_match`) | No |
| Boolean Format | `True/False` | `true/false` |
| Timestamp Field | Column 6 | `transact_time` |

### Aggregation Patterns
- **Futures**: Higher aggregation ratios (more individual trades per AggTrade)
- **Spot**: Lower aggregation ratios (fewer individual trades per AggTrade)
- **Reason**: Futures have more frequent, smaller-size algorithmic trading

## Implementation Guidelines

### When Creating AggTrade Records
```rust
// Always include is_best_match for compatibility
AggTrade {
    agg_trade_id: 123,
    // ... other fields
    is_best_match: Some(true), // For spot data
    // or
    is_best_match: None,       // For futures data
}
```

### When Processing in Range Bars
```rust
// Use new method names
let bars = processor.process_agg_trade_records(&agg_records)?;

// Access new fields
println!("Individual trades: {}", bar.individual_trade_count);
println!("AggTrade records: {}", bar.agg_record_count);
println!("Aggregation efficiency: {:.2}", bar.aggregation_efficiency());
```

### Migration Path (Completed in v1.0.0)
1. **v0.x**: Old methods and field names
2. **v1.0.0**: New methods and field names - old methods removed entirely

This terminology guide ensures crystal-clear understanding of what each "trade" count represents and eliminates ambiguity in the rangebar crate's data processing pipeline.