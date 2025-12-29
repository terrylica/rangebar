# Comprehensive Rangebar Codebase Survey

**Survey Date**: October 16, 2025
**Codebase Version**: 5.0.0 (Rust 1.90)
**Total Rust Code**: ~17,075 LOC (crates directory)

---

## 1. WORKSPACE STRUCTURE

### Primary Organization

```
/crates/
├── rangebar-core/          (1.2K LOC) - Core algorithm & types
├── rangebar-providers/     (1.4K LOC) - Data source adapters
├── rangebar-config/        (1.1K LOC) - Configuration management
├── rangebar-io/            (600 LOC)  - I/O & Polars integration
├── rangebar-streaming/     (1.8K LOC) - Real-time streaming engine
├── rangebar-batch/         (600 LOC)  - Batch analytics
├── rangebar-cli/           (3.2K LOC) - Command-line tools (6 binaries)
└── rangebar/               (800 LOC)  - Meta-crate (v4.0.0 compat)
```

### Legacy/Archived Code

```
/src-archived/             (59 files) - v4.0.0 monolithic structure
├── core/
├── engines/
├── bin/
├── providers/
├── infrastructure/
└── test_utils.rs

/archived_modules/         (5 files) - Individual module archives
```

### Documentation

- **30 markdown files** across organized hierarchy
- `/docs/planning/` - Current architecture & research
- `/docs/reports/` - Validation & analysis reports
- `/docs/archive/` - Historical audit trails (Sept 2025)
- `/docs/development/` - Process guides

---

## 2. ALL BINARIES (6 CLI Tools)

### Tier1 Symbol Discovery

**File**: `/crates/rangebar-cli/src/bin/tier1_symbol_discovery.rs` (22K)

- Multi-market symbol analysis (Binance spot/UM/CM)
- Output formats: comprehensive, minimal
- Tier-1 instruments: 18 confirmed (BTC, ETH, SOL, ADA, AVAX, etc.)

### Data Structure Validator

**File**: `/crates/rangebar-cli/src/bin/data_structure_validator.rs` (25K)

- Cross-market validation (Binance spot/UM futures)
- Date range: 2022-2025 quarterly sampling
- Format detection: headers, timestamps, boolean formats
- SHA256 checksum verification
- Async/parallel workers (default 8)

### Spot Tier1 Processor

**File**: `/crates/rangebar-cli/src/bin/spot_tier1_processor.rs` (14K)

- Spot-specific processing pipeline
- Real data integration testing

### Parallel Tier1 Analysis

**File**: `/crates/rangebar-cli/src/bin/parallel_tier1_analysis.rs` (21K)

- Parallel analytics across tier-1 symbols
- Comprehensive reporting

### Polars Benchmark

**File**: `/crates/rangebar-cli/src/bin/polars_benchmark.rs` (8.7K)

- DataFrame performance analysis
- Parquet operations testing

### Temporal Integrity Test

**File**: `/crates/rangebar-cli/src/bin/temporal_integrity_test_only.rs` (8.4K)

- Timestamp normalization validation
- Format consistency checks

---

## 3. CORE ALGORITHM IMPLEMENTATIONS

### Fixed-Point Arithmetic (`fixed_point.rs`)

**Key Features**:

- SCALE: 100,000,000 (8 decimal precision)
- BASIS_POINTS_SCALE: 100,000 (v3.0.0: 0.1bps units)
- Methods:
    - `from_str()` - Parse decimal strings
    - `to_string()` - Format with 8 decimals
    - `to_f64()` - Convert to f64 for output
    - `compute_range_thresholds()` - Calculate upper/lower from price + threshold_bps

**Critical Detail**: v3.0.0 breaking change multiplied all thresholds by 10 (from 1bps to 0.1bps units)

### Range Bar Processor (`processor.rs`)

**Two Implementations**:

1. **RangeBarProcessor** (Primary)
    - Stateful streaming: `process_single_trade()` maintains state across calls
    - Batch mode: `process_agg_trade_records()` - clears state before processing
    - Analysis mode: `process_agg_trade_records_with_incomplete()` - includes incomplete bars
    - Validation: Trade ordering check `(timestamp, agg_trade_id)` ascending
    - Algorithm: Thresholds fixed from bar OPEN (non-lookahead bias)

2. **ExportRangeBarProcessor** (Legacy)
    - Continuous processing: `process_trades_continuously()`
    - Proven 100% breach consistency using fixed-point arithmetic
    - Separate internal buffer for completed bars

**Critical Algorithm Invariants**:

```
- (high_breach → close_breach) AND (low_breach → close_breach)
- Thresholds computed from OPEN only, fixed for bar lifetime
- Breaching trade INCLUDED in closing bar
- Next bar opens with trade AFTER breach
```

### Type System (`types.rs`)

**AggTrade Fields**:

- `agg_trade_id`, `price`, `volume` (FixedPoint)
- `first_trade_id`, `last_trade_id` (range of individual trades)
- `timestamp` (microseconds, i64)
- `is_buyer_maker` (bool) - Critical for order flow segregation
- `is_best_match` (Option<bool>) - Spot only

**RangeBar Fields** (Enhanced with market microstructure):

- OHLCV: `open_time`, `close_time`, `open`, `high`, `low`, `close`, `volume`
- Counts: `individual_trade_count`, `agg_record_count`, `first/last_trade_id`
- **Market Microstructure**:
    - `buy_volume`, `sell_volume` (segregated by is_buyer_maker)
    - `buy_trade_count`, `sell_trade_count`
    - `vwap` (Volume Weighted Average Price)
    - `buy_turnover`, `sell_turnover`

### Timestamp Handling (`timestamp.rs`)

**Normalization Strategy**:

- Threshold: 10,000,000,000,000 (13-digit boundary)
- Below threshold: 13-digit milliseconds → multiply by 1,000
- At/above threshold: 16-digit microseconds → pass through
- Valid range: 2000-01-01 to 2035-01-01 UTC

**Data Source Support**:

- Binance aggTrades: 16-digit microseconds
- Exness EURUSD: Converts to microseconds
- Flexible format detection for CSV imports

---

## 4. DATA PROVIDERS

### Binance Adapter (`rangebar-providers/binance/`)

**Modules**:

- `historical.rs` - REST API for historical data
- `websocket.rs` - Real-time WebSocket streams
- `symbols.rs` - Symbol discovery & validation
- `mod.rs` - Public API

**Tier-1 Definition**: Assets listed in ALL THREE futures markets:

- UM Futures (USDT-margined): BTCUSDT, ETHUSDT, etc.
- UM Futures (USDC-margined): BTCUSDC, ETHUSDC, etc.
- CM Futures (Coin-margined): BTCUSD_PERP, ETHUSD_PERP, etc.

### Exness Adapter (`rangebar-providers/exness/`)

**Modules**:

- `client.rs` - ZIP/CSV download
- `types.rs` - Provider-specific types
- `builder.rs` - Range bar builder
- `conversion.rs` - Convert to AggTrade format

**Data Spec**:

- Variant: EURUSD Standard (only approved option)
- Frequency: ~1.26M ticks/month (2019-2025)
- Format: ZIP→CSV (Bid/Ask/Timestamp)
- Recommended hours: 0-12, 14-17 UTC (avoid rollover at 22 UTC)
- Thresholds: 0.2bps (HFT), 0.5bps (intraday), 1.0bps (swing)

---

## 5. TEMPORAL/TIME-SENSITIVE CODE PATTERNS

### Critical Temporal Logic Found

1. **Zero-Duration Bars** (Valid by design)
    - Same timestamp for open & close allowed
    - Test: `test_zero_duration_bars_are_valid()`
    - Use case: Fast market execution

2. **Timestamp Ordering Validation**
    - Pre-condition: All trades must be sorted by `(timestamp, agg_trade_id)` ascending
    - Error type: `ProcessingError::UnsortedTrades` with full context
    - Location: `processor.rs::validate_trade_ordering()`

3. **Cross-Year Boundary Handling**
    - Test: `cross_year_speed_comparison.rs`
    - Multi-month memory tests: `multi_month_memory_tests.rs`
    - Handles DST, leap seconds, year rollover

4. **Temporal Integrity Validator** (Dedicated Binary)
    - Timestamp format detection (13-digit ms vs 16-digit μs)
    - Boolean format consistency
    - Validates quarterly samples across 2022-2025

### Floating-Point Usage (Potential Leakage Risk)

Location: `processor.rs` only

```rust
let trade_turnover = (trade.price.to_f64() * trade.volume.to_f64()) as i128;
```

**Risk Assessment**: LOW

- Only for intermediate turnover calculation
- Final value cast back to i128 (integer) → no precision loss
- Used in ExportRangeBarProcessor (legacy path)

---

## 6. TESTING & VALIDATION

### Integration Tests (4,091 LOC total)

**Test Files**:

1. `integration_test.rs` - Primary entry point (real BTCUSDT data)
2. `binance_btcusdt_real_data_test.rs` - Focused real data
3. `binance_ethusdt_real_data_test.rs` - Focused real data
4. `boundary_consistency_tests.rs` - Edge cases
5. `large_boundary_tests.rs` - High-volume scenarios
6. `multi_month_memory_tests.rs` - Memory/stability
7. `cross_year_speed_comparison.rs` - Performance
8. `exness_eurusd_integration_test.rs` - Forex validation
9. `exness_eurusd_statistical_analysis.rs` - Statistical properties
10. `production_streaming_validation.rs` - Real streaming

**Test Data**:

- Real Binance BTCUSDT data (~5,000 trades)
- Centralized test generators (`test_utils/generators.rs`)
- CSV loader for external data (`test_data_loader.rs`)

**Key Test Invariants**:

```
✓ Non-lookahead bias: thresholds from OPEN only
✓ Breach consistency: (high_breach → close_breach)
✓ OHLCV integrity: high ≥ open/close, low ≤ open/close
✓ Temporal ordering: bar timestamps monotonically increasing
✓ Volume conservation: checked but temporarily disabled
```

### Code Quality Metrics

- **unwrap() count**: 183 instances (mostly test code)
- **unsafe blocks**: 0 (pure Rust)
- **TODO/FIXME**: 1 instance (integration_test.rs)
- **Test coverage**: ~4,000+ LOC dedicated to validation

---

## 7. DEPENDENCIES ANALYSIS

### Core Dependencies (Minimal)

```toml
serde = "1.0" with derive
serde_json = "1.0" with arbitrary_precision
chrono = "0.4" with serde
thiserror = "2.0"
```

### Async/Concurrency

```toml
tokio = "1.0" (full features)
rayon = "1.11" (data parallelism)
async-trait = "0.1"
futures = "0.3"
```

### Data Processing

```toml
csv = "1.3"
zip = "2.2"
polars = "0.51.0" (lazy, temporal, parquet, csv, ipc, rolling_window)
```

### Crypto/Validation

```toml
md5 = "0.7" (structure profiling)
sha2 = "0.10" (checksum validation)
```

### Web/API (Optional)

```toml
axum = "0.7" (web framework)
tower = "0.4" (middleware)
utoipa = "4.2" (OpenAPI docs)
```

**No direct Python dependencies** - Pure Rust implementation

---

## 8. TECHNICAL DEBT & LEGACY CODE

### Archived Components

1. **v4.0.0 Monolithic Structure** (`src-archived/` - 59 files)
    - Complete old architecture preserved
    - Git-tracked but not compiled
    - Acts as historical reference

2. **v4.0.0 Backward Compatibility** (`rangebar/` meta-crate)
    - Re-exports from modular crates
    - Maintains API compatibility

### Known Limitations

1. **Volume Conservation Check** (Disabled in tests)
    - Currently disabled in `integration_test.rs`
    - TODO: Re-enable when processor handles all trades correctly

2. **StatisticalEngine Refactoring**
    - Legacy statistics module restructured
    - Disabled in tests: `test_statistics_mode_consistency()`
    - Alternative path exists

3. **ExportRangeBarProcessor**
    - Legacy implementation still in use
    - Duplicate algorithm code (fixed-point version)
    - Maintained for backward compatibility

---

## 9. DOCUMENTATION ORGANIZATION

### Current Documentation (30 files)

**Planning** (`/docs/planning/`):

- `/current/` - Walk-forward pattern recognition system
- `/architecture/` - Algorithm specification, v2.3 restructuring
- `/research/` - Exness variant analysis, tick data evaluation
- `/legacy/` - Phase 6 data flow, dukascopy cleanup plan

**Development** (`/docs/development/`):

- MIGRATION.md - Breaking changes (v3.0.0)
- USABILITY_ROADMAP.md
- automated-updates.md
- disk-cleanup-workflow.md

**Reports** (`/docs/reports/`):

- BPS_STANDARDIZATION_COMPLETE.md
- BPS_VALIDATION_SUCCESS.md

**Archive** (`/docs/archive/`):

- Adversarial testing reports
- GPU/CPU analysis
- Large-scale benchmarking
- Security audit trails

### Documentation Quality

- Well-organized hub-and-spoke architecture
- Proper linking and cross-references
- Clear deprecation markers
- Phase-based tracking (Phases 1-5 documented)

---

## 10. OPPORTUNITIES FOR SOTA LIBRARY REPLACEMENTS

### Current Libraries with Modern Alternatives

1. **Fixed-Point Arithmetic**
    - Current: Custom `FixedPoint` implementation
    - Alternative: `rust_decimal` (more features) or `fixed` (compile-time precision)
    - Consideration: Custom implementation is optimal for 8 decimal precision

2. **CSV Parsing**
    - Current: `csv` v1.3
    - Alternative: `polars` native CSV (already using for Parquet)
    - Benefit: Single codec family for CSV→Parquet pipeline

3. **Timestamp Handling**
    - Current: `chrono` v0.4
    - Alternative: `time` crate (newer, more features)
    - Note: `chrono` already sufficient; migration not critical

4. **HTTP Client**
    - Current: `reqwest` v0.12
    - Status: Already SOTA (async, feature-complete)

5. **Statistics/Aggregation**
    - Current: `rolling-stats`, `tdigests`
    - Alternative: `polars` lazy expressions for streaming stats
    - Benefit: Unified data processing pipeline

### SOTA Recommendations (Priority)

**HIGH Priority**:

- Consolidate CSV I/O: Use `polars` native for all CSV operations
- Migrate to `time` crate: Better async support, smaller binary

**MEDIUM Priority**:

- Replace `rolling-stats` with `polars` expressions
- Consider `quantiles` for better percentile tracking

**LOW Priority**:

- Custom `FixedPoint` is well-optimized; no change needed

---

## 11. POTENTIAL ISSUES & EDGE CASES

### Identified Risks

1. **Floating-Point Turnover Calculation** (LOW RISK)
    - Location: `processor.rs` (2 instances)
    - Impact: Intermediate float → integer cast
    - Mitigation: Already in place, well-tested

2. **Timestamp Boundary Conditions** (MITIGATED)
    - 13-digit vs 16-digit detection
    - Validated in tests and data validator
    - Handles 2000-2035 range

3. **CSV Format Variations** (HANDLED)
    - Spot vs UM futures columns
    - Header detection and flexible deserialization
    - Structure validator addresses this

4. **Zero-Duration Bars** (INTENTIONAL)
    - Valid for fast execution scenarios
    - Explicitly tested
    - Not a bug

5. **Unsorted Trade Handling** (VALIDATED)
    - Pre-condition: trades must be sorted by (timestamp, agg_trade_id)
    - Clear error message if violated
    - Test: `test_unsorted_trades_error()`

---

## 12. CODE METRICS SUMMARY

| Metric                | Value             | Status                   |
| --------------------- | ----------------- | ------------------------ |
| Total Crate LOC       | ~17,075           | ✓ Manageable             |
| Test LOC              | ~4,091            | ✓ Good coverage          |
| Binaries              | 6                 | ✓ Focused tools          |
| Documentation Files   | 30                | ✓ Well-organized         |
| Archived Files        | 59 (src-archived) | ℹ Legacy                |
| Unwrap Instances      | 183               | ⚠ Mostly tests          |
| Unsafe Blocks         | 0                 | ✓ Pure Rust              |
| TODOs/FIXMEs          | 1                 | ✓ Minimal                |
| Temporal Logic Files  | 7+                | ✓ Well-tested            |
| Floating-Point Usages | 15+               | ⚠ For display/calc only |

---

## 13. RECENT DEVELOPMENT ACTIVITY

### Recent Commits (Last 20)

- `342aae1` feat: add focused real data tests for BTCUSDT and ETHUSDT (Phase 4)
- `e4fc449` fix: relocate workspace tests/ to crates/rangebar/tests/
- `9db84ef` refactor: Phase 3 - replace synthetic data with real CSV data
- `9282142` refactor: centralize test helpers to generators.rs (Phase 1.5)
- `1924586` feat: add CSV loader for real test data (Phase 1)

### Version Evolution

- **v5.0.0**: Modular workspace, 8 specialized crates
- **v4.0.0**: Monolithic structure (archived)
- **v3.0.0**: Basis points granularity change (0.1bps units)

---

## CONCLUSIONS

### Strengths

1. ✓ **Clean modular architecture** - 8 well-separated crates
2. ✓ **Comprehensive testing** - Real data + synthetic scenarios
3. ✓ **Excellent documentation** - 30+ markdown files, organized structure
4. ✓ **Temporal integrity** - Explicit handling, well-validated
5. ✓ **Pure Rust** - No unsafe blocks, minimal unwraps
6. ✓ **Non-lookahead bias** - Algorithm invariants enforced
7. ✓ **Market microstructure** - Buy/sell segregation included

### Areas for Improvement

1. ⚠ **Volume conservation check** - Disabled, needs re-enabling
2. ⚠ **Statistical engine** - Needs re-architecture documentation
3. ⚠ **Duplicate algorithm code** - ExportRangeBarProcessor duplicates logic
4. ⚠ **CSV consolidation** - Multiple CSV sources could use `polars` native
5. ℹ **Archived code** - 59 files in src-archived could be Git-archived

### Recommended Next Steps

1. Consolidate CSV I/O through `polars` native codec
2. Re-enable volume conservation validation
3. Document statistical engine refactoring
4. Consider consolidating algorithm into single processor
5. Archive `src-archived/` as Git tag reference

---

**Survey Completed**: October 16, 2025
**Analyst**: Comprehensive Codebase Review
**Next Review**: Recommended after Phase 5 completion
