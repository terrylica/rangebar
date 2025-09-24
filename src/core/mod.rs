//! Core range bar processing algorithms
//!
//! This module contains the fundamental range bar construction algorithm
//! with temporal integrity guarantees and non-lookahead bias.

pub mod fixed_point;
pub mod processor;
pub mod timestamp;
pub mod types;

// Re-export commonly used types
pub use fixed_point::FixedPoint;
pub use processor::{ExportRangeBarProcessor, ProcessingError, RangeBarProcessor};
pub use timestamp::{normalize_timestamp, validate_timestamp, create_aggtrade_with_normalized_timestamp};
pub use types::{AggTrade, RangeBar};
