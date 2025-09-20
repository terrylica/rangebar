//! Market-specific logic and data structures
//!
//! This module contains market-specific functionality including
//! symbol management, exchange adaptations, and microstructure analysis.

pub mod symbols;

// Re-export commonly used types
pub use symbols::{
    get_tier1_symbols, get_tier1_usdt_pairs, is_tier1_symbol, TIER1_SYMBOLS
};