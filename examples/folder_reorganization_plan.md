# Examples Folder Reorganization Plan

## Current Issues
- All examples are in flat `examples/` directory
- Mix of different types: demos, analysis, validation, educational
- Hard to find specific type of example
- No clear categorization

## Proposed Structure

```
examples/
├── analysis/          # Market analysis and research tools
│   ├── rangebar_generation_comparison.rs
│   ├── market_efficiency_analysis.rs
│   ├── tier1_volume_comparison.rs
│   └── tier1_symbols.rs
│
├── interactive/       # Interactive demos and replay tools
│   ├── historical_replay.rs
│   ├── test_historical_replay.rs
│   ├── format_demo.rs
│   └── market_comparison_demo.rs
│
├── educational/       # Learning and reference examples
│   ├── terminology_clarification.rs
│   ├── basic_usage.rs
│   └── README.md (category explanation)
│
├── validation/        # Testing and validation tools
│   ├── validate_25bps_threshold.rs
│   ├── validate_custom_threshold.rs
│   └── README.md (validation procedures)
│
└── README.md          # Main examples index
```

## Category Descriptions

### `analysis/`
**Purpose**: Market research, statistical analysis, and comparison tools
- Performance comparisons between markets
- Statistical analysis of trading patterns
- Research tools for market behavior

### `interactive/`
**Purpose**: Interactive demonstrations and real-time tools
- Historical data replay with controls
- Live visualizations
- Demo applications

### `educational/`
**Purpose**: Learning materials and reference examples
- Terminology clarifications
- Basic usage examples
- Conceptual demonstrations

### `validation/`
**Purpose**: Testing, validation, and verification tools
- Algorithm validation
- Threshold testing
- Data integrity checks

## Benefits

1. **Easier Discovery**: Find examples by purpose
2. **Better Maintenance**: Organize by function
3. **Clear Intent**: Each category has specific purpose
4. **Scalable**: Easy to add new examples to appropriate category
5. **Documentation**: Category-specific READMEs

## Usage Examples

```bash
# Run market analysis
cargo run --example analysis/rangebar_generation_comparison

# Interactive historical replay
cargo run --example interactive/historical_replay

# Learn terminology
cargo run --example educational/terminology_clarification

# Validate algorithm
cargo run --example validation/validate_25bps_threshold
```