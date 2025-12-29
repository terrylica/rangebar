# Validation Examples

Testing, validation, and verification tools for ensuring range bar algorithm correctness and data integrity.

## Examples in this category:

### `validate_25bps_threshold.rs`

**Purpose**: Validates the 25 BPS (0.25%) threshold calculation accuracy
**Usage**: `cargo run --example validation/validate_25bps_threshold`
**Validates**:

- Exact threshold compliance (±25 BPS)
- Mathematical precision within market tick constraints
- Cross-symbol consistency
- Breach detection accuracy

### `validate_custom_threshold.rs`

**Purpose**: Tests range bar formation with custom threshold values
**Usage**: `cargo run --example validation/validate_custom_threshold`
**Features**:

- User-specified threshold testing
- Multi-threshold validation
- Algorithm correctness verification
- Performance benchmarking

## When to use these examples:

- ✅ **Algorithm verification** after code changes
- 🔬 **Testing new thresholds** before production use
- 📊 **Benchmarking performance** across different parameters
- 🛡️ **Ensuring data integrity** in processing pipeline
- 🧪 **Research validation** for academic or professional use

## Validation criteria:

### Mathematical Accuracy

- All range bars must breach exactly at ±threshold BPS
- Price precision within market tick constraints
- No lookahead bias in threshold calculations

### Temporal Integrity

- Chronological order preservation
- Proper timestamp handling
- No future data leakage

### Cross-Market Consistency

- Algorithm works identically across spot/futures
- Consistent behavior across different symbols
- Reproducible results

## Output interpretation:

### ✅ PASSED validations indicate:

- Algorithm is mathematically correct
- Data integrity is maintained
- Results are reliable for trading/research

### ❌ FAILED validations require:

- Investigation of root cause
- Algorithm review and correction
- Re-validation before proceeding

## Requirements:

- Historical data access
- Sufficient test data (recent dates)
- ~1-2 minutes runtime for thorough validation
