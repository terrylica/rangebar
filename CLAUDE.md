# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Non-lookahead bias range bar construction from tick data (crypto: Binance aggTrades, forex: Exness EURUSD Raw_Spread).

**Core Algorithm**: See authoritative specification → [`docs/specifications/algorithm-spec.md`](/docs/specifications/algorithm-spec.md)

**Architecture**: Pure Rust implementation for performance and reliability (processes 1B+ ticks). All components native Rust: symbol discovery, data processing, and analysis.

## Workspace Structure (v5.0.0)

**Modular Crate Architecture**: 8 specialized crates organized as Cargo workspace

**Core Crates**:

- `crates/rangebar-core/` - Core algorithm, fixed-point arithmetic, types
- `crates/rangebar-providers/` - Data providers (Binance, Exness)
- `crates/rangebar-config/` - Configuration management
- `crates/rangebar-io/` - I/O operations and Polars integration

**Engine Crates**:

- `crates/rangebar-streaming/` - Real-time streaming processor
- `crates/rangebar-batch/` - Batch analytics engine

**Tools & Compatibility**:

- `crates/rangebar-cli/` - Command-line tools (all binaries)
- `crates/rangebar/` - Meta-crate for v4.0.0 backward compatibility

**Legacy**:

- `src-archived/` - v4.0.0 monolithic structure (archived, git tracked)

## Key Commands

**Dev**: `cargo build --release`, `cargo test`, `cargo clippy`, `./scripts/update-deps.sh`

**Release**: `release-plz release` - Rust-native SSoT versioning with API compatibility checks

**Deploy**: `doppler run -- shuttle deploy`

**Data Ops**: `tier1-symbol-discovery --format comprehensive`, `parallel-tier1-analysis`, `spot-tier1-processor`, `data-structure-validator --features data-integrity`

## Data Structure Validation

**Tool**: `data-structure-validator` validates Binance aggTrades across spot/futures markets

**Key Differences**:

- **Spot**: No headers, short columns (`a,p,q,f,l,T,m`), 16-digit μs timestamps
- **UM Futures**: Headers, descriptive columns, 13-digit ms timestamps
- **Parser**: Auto-detects format, normalizes timestamps to microseconds
- **Critical**: Use `aggTrades` nomenclature (not `trades`) - affects all naming

## Architecture

### Data Pipeline

1. **Symbol Discovery**: `tier1-symbol-discovery` → Multi-market symbol analysis
2. **Data Structure Validation**: `data-structure-validator` → Cross-market format verification
3. **Data Fetching**: `binance_historical_data` → Raw CSV/ZIP files with validated schemas
4. **Preprocessing**: CSV → Parquet with schema validation
5. **Computation**: Pure Rust processes Parquet → Range bars
6. **Analysis**: `parallel-tier1-analysis` → Parallel multi-symbol batch analysis
7. **Output**: Structured bar data (OHLCV format)

**Performance**: Pure Rust, Rayon parallelism, fixed-point arithmetic

## Critical Algorithm Invariants

**Specification**: [`/docs/specifications/algorithm-spec.md`](/docs/specifications/algorithm-spec.md) (authoritative)
**Breach Consistency**: `(high_breach → close_breach) AND (low_breach → close_breach)`

### Data Source Requirements

#### Binance (Primary - Crypto)

- **Source**: <https://github.com/stas-prokopiev/binance_historical_data>
- **Primary Asset Class**: `"spot"` (Default) for standard spot trading pairs
- **Optional Markets**: `"um"` (USD-M Futures) for USDT/USDC perpetuals, `"cm"` (Coin-M Futures)
- **Data Type**: `"aggTrades"` **ONLY**
- **Usage**: Specify market type via command line arguments or use spot by default

#### Exness (Primary - Forex)

- **Instruments**: 10 supported pairs via `ExnessInstrument` enum:
  - Majors: EURUSD, GBPUSD, USDJPY, AUDUSD, USDCAD, NZDUSD
  - Crosses: EURGBP, EURJPY, GBPJPY
  - Commodities: XAUUSD
- **Variant**: Raw_Spread with instrument-specific spread characteristics:
  - **Forex pairs**: Bimodal (98% zero spread at bid==ask, 2% stress events 1-9 pips), CV=8.17
  - **XAUUSD (Gold)**: Consistent ~$0.06 spreads (NOT zero, NOT bimodal), 99.6% < $0.10
- **Type-Safe API**: `ExnessFetcher::for_instrument(ExnessInstrument::XAUUSD)`
- **Legacy API**: `ExnessFetcher::new("EURUSD_Raw_Spread")` (backward compatible)
- **API Pattern**: `https://ticks.ex2archive.com/ticks/{SYMBOL}_Raw_Spread/{year}/{month}/...`
- **Format**: ZIP→CSV (Bid/Ask/Timestamp), ~60K ticks/day, validated 2019-2025
- **Data Characteristics**: No volume data (mid-price for range bars), commission-based pricing
- **Spread Validation**: Use `ExnessInstrument::spread_tolerance()` for instrument-specific thresholds
- **Thresholds**: 0.1bps (minimum), 0.2bps (HFT), 0.5bps (intraday), 1.0bps (swing)
- **Validation Reference**: `~/eon/exness-data-preprocess` (453M+ ticks across 10 instruments)

### Tier-1 Instruments Definition

**Tier-1 instruments** are crypto assets that Binance respects highly enough to list across **ALL THREE** futures markets:

1. **UM Futures (USDT-margined)**: e.g., BTCUSDT, ETHUSDT
2. **UM Futures (USDC-margined)**: e.g., BTCUSDC, ETHUSDC
3. **CM Futures (Coin-margined)**: e.g., BTCUSD_PERP, ETHUSD_PERP

**Current Count**: 18 Tier-1 instruments (BTC, ETH, SOL, ADA, AVAX, etc.)
**Key Characteristic**: Multi-market availability indicates Binance's highest confidence
**Use Cases**: Cross-market extrapolative reliability analysis, settlement currency arbitrage

**Directory Structure**: `crates/` (workspace crates), `tests/` (integration tests), `scripts/` (automation), `output/` (results), `src-archived/` (v4.0.0 legacy code)

## Common Issues

**Fixes**: `rustup update`, `cargo clean`, sort by `(timestamp, aggTradeId)`, thresholds from OPEN only

## Rust Binaries

**Location**: All command-line tools consolidated in `crates/rangebar-cli/src/bin/` (6 binaries)

**Tools**: `tier1-symbol-discovery --format [comprehensive|minimal]`, `parallel-tier1-analysis`, `spot-tier1-processor`, `data-structure-validator`, `polars-benchmark`, `temporal-integrity-test-only`

**Testing**: `cargo test`, `cargo bench` - validates non-lookahead, performance <100ms/1M ticks

**Publishing**: See [`/docs/guides/publishing.md`](/docs/guides/publishing.md) for complete workflow

- **Tool**: release-plz handles dependency-ordered publishing automatically
- **Credentials**: Doppler secret `CRATES_IO_CLAUDE_CODE` in `claude-config/dev`
- **Command**: `export CARGO_REGISTRY_TOKEN=$(doppler secrets get CRATES_IO_CLAUDE_CODE --project claude-config --config dev --plain) && release-plz release`

## Dependency Management

**Auto-Updates**: Dependabot weekly PRs (`.github/dependabot.yml`), caret requirements (`^0.51.0`), manual validation (`scripts/update-deps.sh`)

## Release Workflow

**Tool**: [release-plz](https://release-plz.dev/) - Rust-native release automation with SSoT versioning

**SSoT**: `Cargo.toml` workspace version is the single source of truth (no dual version files)

**Process**: `release-plz release` executes:

1. Analyze commits since last tag (conventional commits)
2. Run cargo-semver-checks for API breaking change detection
3. Determine version bump (MAJOR/MINOR/PATCH)
4. Update CHANGELOG.md via git-cliff integration
5. Create git tag and GitHub release
6. Publish to crates.io in dependency order

**Configuration**:

- `release-plz.toml` - Release automation config (SSoT, semver checks, publishing)
- `cliff.toml` - Detailed changelog template (developer-focused)
- `cliff-release-notes.toml` - Release notes template (user-focused)

**Commands**:

- `release-plz release --dry-run` - Preview release without changes
- `release-plz release` - Execute full release (requires CARGO_REGISTRY_TOKEN)
- `release-plz update` - Update Cargo.toml, Cargo.lock, and CHANGELOG.md
