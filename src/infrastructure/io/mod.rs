//! Input/output operations for range bar data
//!
//! This module provides efficient I/O operations for reading and writing
//! range bar data in various formats including CSV, Parquet, and Arrow.

#[cfg(feature = "polars-io")]
pub mod polars_io;

#[cfg(feature = "polars-io")]
pub mod formats;

// Re-export commonly used types when polars feature is enabled
#[cfg(feature = "polars-io")]
pub use formats::{ConversionError, DataFrameConverter};

#[cfg(feature = "polars-io")]
pub use polars_io::{
    ArrowExporter, ExportError, ParquetExporter, PolarsExporter, PolarsExporterConfig,
    StreamingCsvExporter,
};
