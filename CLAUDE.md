# CLAUDE.md

Guidance for Claude Code when working with this repository.

## Project Overview

Non-lookahead range bar construction from tick data. Pure Rust, processes 1B+ ticks.

**Algorithm**: [`/docs/specifications/algorithm-spec.md`](/docs/specifications/algorithm-spec.md) (authoritative SSoT)

**Architecture**: [`/docs/ARCHITECTURE.md`](/docs/ARCHITECTURE.md) (crate structure, data flow, APIs)

## Quick Reference

| Task | Command |
|------|---------|
| Build | `cargo build --release` |
| Test | `cargo nextest run` |
| Lint | `cargo clippy` |
| Release | `release-plz release` ([guide](/docs/guides/publishing.md)) |

## Workspace Structure

8-crate modular workspace (v5.0.0+):

```
crates/
├── rangebar-core/      # Algorithm, fixed-point, types
├── rangebar-providers/ # Binance, Exness data sources
├── rangebar-config/    # Configuration management
├── rangebar-io/        # Polars, export formats
├── rangebar-streaming/ # Real-time processor
├── rangebar-batch/     # Batch analytics
├── rangebar-cli/       # All binaries (6 tools)
└── rangebar/           # Meta-crate (v4.0 compat)
```

See [`crates/CLAUDE.md`](/crates/CLAUDE.md) for crate-specific details.

## Data Sources

| Provider | Market | Data Type | Details |
|----------|--------|-----------|---------|
| Binance | Crypto | aggTrades | Spot, UM, CM futures |
| Exness | Forex | Raw_Spread | 10 instruments via `ExnessInstrument` |

**Tier-1 instruments**: 18 assets listed on ALL THREE Binance futures markets (BTC, ETH, SOL, etc.)

## Key Invariants

- **Breach consistency**: `(high_breach -> close_breach) AND (low_breach -> close_breach)`
- **Threshold units**: Decimal basis points (0.1bps). `250` = 25bps = 0.25%
- **Timestamp normalization**: All sources normalized to microseconds

## Documentation Hub

| Topic | Location |
|-------|----------|
| Algorithm spec | [`/docs/specifications/algorithm-spec.md`](/docs/specifications/algorithm-spec.md) |
| Architecture | [`/docs/ARCHITECTURE.md`](/docs/ARCHITECTURE.md) |
| Publishing | [`/docs/guides/publishing.md`](/docs/guides/publishing.md) |
| Compilation | [`/docs/development/compilation-optimization.md`](/docs/development/compilation-optimization.md) |
| Planning index | [`/docs/planning/INDEX.md`](/docs/planning/INDEX.md) |
| Diagrams | [`/docs/diagrams/INDEX.md`](/docs/diagrams/INDEX.md) |

## Child CLAUDE.md Files

Claude Code loads these automatically when working in subdirectories:

- [`crates/CLAUDE.md`](/crates/CLAUDE.md) - Workspace crate details, dependencies, APIs
- [`docs/CLAUDE.md`](/docs/CLAUDE.md) - Documentation structure, contribution guidelines

## Release Workflow

**Tool**: release-plz (Rust-native SSoT versioning)

```bash
# Dry run
release-plz release --dry-run

# Execute (requires CARGO_REGISTRY_TOKEN)
export CARGO_REGISTRY_TOKEN=$(doppler secrets get CRATES_IO_CLAUDE_CODE \
  --project claude-config --config dev --plain)
release-plz release
```

Full guide: [`/docs/guides/publishing.md`](/docs/guides/publishing.md)
