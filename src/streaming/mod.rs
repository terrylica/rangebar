//! Real-time streaming processing
//!
//! This module provides high-performance streaming capabilities for
//! real-time range bar processing and technical indicator computation.

pub mod indicators;
pub mod replay_buffer;
pub mod stats;
pub mod universal;
pub mod websocket;
// Re-export commonly used types
pub use indicators::{
    CCI, ExponentialMovingAverage, IndicatorError, MACD, MACDValue, RSI, SimpleMovingAverage,
};
pub use replay_buffer::ReplayBuffer;
pub use stats::{
    BarStats, OhlcStatistics, PriceStatistics, RollingStats, StatisticsSnapshot, StreamingConfig,
    StreamingStatsEngine, TradeStats, VolumeStatistics,
};
pub use universal::{StreamMode, TradeStream, UniversalStream};
pub use websocket::{BinanceWebSocketStream, WebSocketError};
