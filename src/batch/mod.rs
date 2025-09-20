//! Batch processing and analysis using Polars
//!
//! This module provides batch analytics capabilities powered by Polars
//! for research, backtesting, and advanced statistical analysis.

#[cfg(feature = "polars-analytics")]
pub mod engine;

#[cfg(feature = "polars-analytics")]
pub mod analytics;

#[cfg(feature = "polars-analytics")]
pub mod research;

// Re-export commonly used types when polars-analytics feature is enabled
#[cfg(feature = "polars-analytics")]
pub use engine::{
    BatchAnalysisEngine, BatchConfig, BatchError, BatchResult, AnalysisReport
};

#[cfg(feature = "polars-analytics")]
pub use analytics::{
    CrossSymbolAnalyzer, MarketRegimeAnalyzer, OrderFlowAnalyzer,
    StatisticalAnalyzer, AnalyticsError
};

#[cfg(feature = "polars-analytics")]
pub use research::{
    ResearchEngine, ResearchConfig, ResearchError, ResearchReport
};