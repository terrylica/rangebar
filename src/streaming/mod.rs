//! Real-time streaming processing
//!
//! This module provides high-performance streaming capabilities for
//! real-time range bar processing and technical indicator computation.

pub mod engine;
pub mod indicators;
pub mod stats;

// Re-export commonly used types
pub use engine::{
    MetricsSummary, RangeBarStream, StreamingError, StreamingMetrics,
    StreamingProcessor, StreamingProcessorConfig
};
pub use indicators::{
    CCI, ExponentialMovingAverage, IndicatorError, MACD, MACDValue,
    RSI, SimpleMovingAverage
};
pub use stats::{
    BarStats, OhlcStatistics, PriceStatistics, RollingStats,
    StatisticsSnapshot, StreamingConfig, StreamingStatsEngine,
    TradeStats, VolumeStatistics
};