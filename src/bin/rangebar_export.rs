use std::fs;
use std::io::{Cursor, Read};
use std::path::Path;

use chrono::{Duration, NaiveDate};
use csv::{ReaderBuilder, WriterBuilder};
use reqwest::Client;
use serde::Serialize;
use zip::ZipArchive;

// Data integrity support
#[cfg(feature = "data-integrity")]
use sha2::{Digest, Sha256};

// Use library types and consolidated data loading
use rangebar::{AggTrade, RangeBar, Settings, ExportRangeBarProcessor};
use rangebar::data::{CsvAggTrade, detect_csv_headers};

// Legacy statistics support disabled - requires statistics module restructuring
// #[cfg(feature = "statistics")]
// use rangebar::statistics::RangeBarMetadata;

// Enhanced output result with comprehensive metadata
#[derive(Debug, Serialize)]
struct EnhancedExportResult {
    /// Basic export information (existing)
    #[serde(flatten)]
    pub basic_result: ExportResult,

    /// Comprehensive metadata (if statistics feature enabled)
    // #[cfg(feature = "statistics")]
    // pub metadata: Option<RangeBarMetadata>,

    /// File format information
    pub files: ExportedFiles,
}

#[derive(Debug, Serialize)]
struct ExportedFiles {
    /// Primary data files
    pub data_files: Vec<ExportedFile>,

    /// Metadata files
    pub metadata_files: Vec<ExportedFile>,
}

#[derive(Debug, Serialize)]
struct ExportedFile {
    pub filename: String,
    pub format: String, // "csv", "json", "parquet"
    pub size_bytes: u64,
    pub market_type: String, // "um", "cm", "spot"
}

// CSV parsing functionality now imported from rangebar::data module
// ExportRangeBarProcessor now imported from rangebar crate

#[derive(Debug, Serialize)]
struct ExportResult {
    symbol: String,
    threshold_bps: u32,
    date_range: (String, String),
    total_bars: usize,
    total_trades: u64,
    total_volume: f64,
    processing_time_seconds: f64,
    csv_file: String,
    json_file: String,
}

struct RangeBarExporter {
    client: Client,
    output_dir: String,
    market_type: String,
}

impl RangeBarExporter {
    fn new(output_dir: String, market_type: String) -> Result<Self, Box<dyn std::error::Error>> {
        // SECURITY: Validate output directory to prevent path traversal attacks
        let validated_output_dir = Self::validate_output_directory(&output_dir)?;

        // Create output directory with proper error handling (no panic)
        if let Err(e) = fs::create_dir_all(&validated_output_dir) {
            return Err(format!(
                "Failed to create output directory '{}': {}",
                validated_output_dir, e
            )
            .into());
        }

        Ok(Self {
            client: Client::new(),
            output_dir: validated_output_dir,
            market_type,
        })
    }

    /// Validates output directory path to prevent path traversal attacks
    fn validate_output_directory(output_dir: &str) -> Result<String, Box<dyn std::error::Error>> {
        use std::path::{Component, Path};

        // Security checks
        let path = Path::new(output_dir);

        // Check for path traversal attempts
        for component in path.components() {
            match component {
                Component::ParentDir => {
                    return Err(
                        "Path traversal detected: '..' components not allowed in output directory"
                            .into(),
                    );
                }
                Component::RootDir => {
                    return Err("Absolute paths not allowed for security reasons".into());
                }
                Component::Prefix(_) => {
                    return Err("Drive prefixes not allowed for security reasons".into());
                }
                Component::Normal(_) | Component::CurDir => {
                    // These are safe
                }
            }
        }

        // Additional validation
        if output_dir.is_empty() {
            return Err("Output directory cannot be empty".into());
        }

        if output_dir.len() > 255 {
            return Err("Output directory path too long (max 255 characters)".into());
        }

        // Convert to canonical form and ensure it's still safe
        let canonical_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        let canonical_str = canonical_path.to_string_lossy().to_string();

        // Ensure the canonical path doesn't escape the current working directory
        let current_dir = std::env::current_dir()?;
        if !canonical_path.starts_with(&current_dir) && !path.is_relative() {
            return Err("Output directory must be within current working directory".into());
        }

        Ok(canonical_str)
    }

    /// Build the URL path based on market type
    fn get_market_path(&self) -> &str {
        match self.market_type.as_str() {
            "spot" => "spot",
            "um" => "futures/um",
            _ => "spot", // Default fallback
        }
    }

    async fn export_symbol_range_bars(
        &self,
        symbol: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
        threshold_bps: u32,
    ) -> Result<EnhancedExportResult, Box<dyn std::error::Error + Send + Sync>> {
        let start_time = std::time::Instant::now();
        // Use threshold basis points directly (no conversion needed)
        let mut processor = ExportRangeBarProcessor::new(threshold_bps);
        let mut all_range_bars: Vec<RangeBar> = Vec::new(); // Unified boundary-safe processing
        let mut total_trades = 0u64;
        let mut current_date = start_date;

        // Initialize statistical engine for comprehensive analysis
        // #[cfg(feature = "statistics")]
        // let mut statistical_engine = rangebar::statistics::StatisticalEngine::new();

        #[cfg(feature = "statistics")]
        // OPTIMIZATION: Use Vec::with_capacity to avoid reallocations
        // Estimate: 131M trades = ~131M * 80 bytes, but we'll process streaming
        // Unified algorithm processes trades day-by-day, no need for global trade collection

        println!("🚀 Range Bar Exporter");
        println!("====================");
        println!("📊 Symbol: {}", symbol);
        println!("📅 Date Range: {} to {}", start_date, end_date);
        println!(
            "📈 Threshold: {} bps ({}%)",
            threshold_bps,
            threshold_bps as f64 / 100.0
        );
        println!("📁 Output: {}/", self.output_dir);

        // PHASE 1: Process days continuously using boundary-safe mode for deterministic results
        // This unifies both statistics and non-statistics paths to ensure identical algorithm
        println!("   🔄 Phase 1: Processing days continuously (boundary-safe mode)...");
        while current_date <= end_date {
            print!("   📊 Loading {}...\r", current_date.format("%Y-%m-%d"));

            match self
                .load_single_day_trades_boundary_safe(
                    symbol,
                    current_date,
                    &mut processor,
                    &mut all_range_bars,
                )
                .await
            {
                Ok(trades_count) => {
                    total_trades += trades_count;
                    println!(
                        "   📊 {} {} → {} trades loaded (total: {})",
                        symbol,
                        current_date.format("%Y-%m-%d"),
                        trades_count,
                        total_trades
                    );
                }
                Err(e) => {
                    eprintln!(
                        "   ⚠️  {} {}: {}",
                        symbol,
                        current_date.format("%Y-%m-%d"),
                        e
                    );
                }
            }

            current_date += Duration::days(1);
        }

        // PHASE 2: Processing complete - unified boundary-safe algorithm used
        println!(
            "\n   ✅ Boundary-safe processing complete: {} range bars generated",
            all_range_bars.len()
        );

        // PHASE 3: Add final incomplete bar if exists (unified handling)
        if let Some(incomplete_bar) = processor.get_incomplete_bar() {
            all_range_bars.push(incomplete_bar);
            println!(
                "   📊 Added final incomplete bar (total: {} bars)",
                all_range_bars.len()
            );
        }

        let processing_time = start_time.elapsed().as_secs_f64();

        // Export to CSV and JSON
        let total_volume: f64 = all_range_bars.iter().map(|b| b.volume.to_f64()).sum();
        let date_str = format!(
            "{}_{}",
            start_date.format("%Y%m%d"),
            end_date.format("%Y%m%d")
        );
        // threshold_bps already available as parameter
        let csv_filename = format!(
            "{}_{}_rangebar_{}_{:04}bps.csv",
            self.market_type, // Always include market type
            symbol,
            date_str,
            threshold_bps
        );
        let json_filename = format!(
            "{}_{}_rangebar_{}_{:04}bps.json",
            self.market_type, // Always include market type
            symbol,
            date_str,
            threshold_bps
        );

        self.export_to_csv(&all_range_bars, &csv_filename)?;

        // Generate comprehensive metadata with statistical analysis
        // #[cfg(feature = "statistics")]
        // let metadata = {
        //     println!("   🔬 Generating comprehensive statistical analysis...");
        //     let metadata_result = statistical_engine.compute_comprehensive_metadata(
        //         &Vec::new(), // Empty trades - unified algorithm processes day-by-day
        //         &all_range_bars,
        //         symbol,
        //         threshold_bps,
        //         &start_date.format("%Y-%m-%d").to_string(),
        //         &end_date.format("%Y-%m-%d").to_string(),
        //     );
        //     metadata_result.ok()
        // };

        // #[cfg(not(feature = "statistics"))]
        // let metadata = None;

        self.export_to_json_with_metadata(&all_range_bars, &json_filename, None)?;

        println!("\n✅ Export Complete!");
        println!("   📊 Total Bars: {}", all_range_bars.len());
        println!("   💰 Total Trades: {}", total_trades);
        println!("   🌊 Total Volume: {:.2}", total_volume);
        println!("   ⚡ Processing Time: {:.1}s", processing_time);
        println!("   📄 CSV: {}/{}", self.output_dir, csv_filename);
        println!("   📄 JSON: {}/{}", self.output_dir, json_filename);

        // #[cfg(feature = "statistics")]
        // if metadata.is_some() {
        //     println!("   🔬 Statistical Analysis: 200+ metrics included in JSON");
        // }

        let basic_result = ExportResult {
            symbol: symbol.to_string(),
            threshold_bps,
            date_range: (
                start_date.format("%Y-%m-%d").to_string(),
                end_date.format("%Y-%m-%d").to_string(),
            ),
            total_bars: all_range_bars.len(),
            total_trades,
            total_volume,
            processing_time_seconds: processing_time,
            csv_file: csv_filename.clone(),
            json_file: json_filename.clone(),
        };

        let files = ExportedFiles {
            data_files: vec![
                ExportedFile {
                    filename: csv_filename,
                    format: "csv".to_string(),
                    size_bytes: 0, // TODO: Get actual file size
                    market_type: self.market_type.clone(),
                },
                ExportedFile {
                    filename: json_filename,
                    format: "json".to_string(),
                    size_bytes: 0, // TODO: Get actual file size
                    market_type: self.market_type.clone(),
                },
            ],
            metadata_files: vec![],
        };

        Ok(EnhancedExportResult {
            basic_result,
            // #[cfg(feature = "statistics")]
            // metadata,
            files,
        })
    }

    #[allow(dead_code)] // Alternative processing method
    async fn process_single_day(
        &self,
        processor: &mut ExportRangeBarProcessor,
        symbol: &str,
        date: NaiveDate,
    ) -> Result<(u64, Vec<RangeBar>), Box<dyn std::error::Error + Send + Sync>> {
        let date_str = date.format("%Y-%m-%d");
        let url = format!(
            "https://data.binance.vision/data/{}/daily/aggTrades/{}/{}-aggTrades-{}.zip",
            self.get_market_path(),
            symbol,
            symbol,
            date_str
        );

        let response = tokio::time::timeout(
            tokio::time::Duration::from_secs(30),
            self.client.get(&url).send(),
        )
        .await??;

        if !response.status().is_success() {
            return Err(format!("HTTP {}", response.status()).into());
        }

        let zip_bytes = response.bytes().await?;

        // Verify data integrity using SHA256 checksum
        self.verify_file_integrity(&zip_bytes, symbol, &date_str.to_string())
            .await?;

        let cursor = Cursor::new(zip_bytes);
        let mut archive = ZipArchive::new(cursor)?;

        let csv_filename = format!("{}-aggTrades-{}.csv", symbol, date_str);
        let mut csv_file = archive.by_name(&csv_filename)?;

        let mut buffer = String::with_capacity(8 * 1024 * 1024);
        csv_file.read_to_string(&mut buffer)?;

        let mut reader = ReaderBuilder::new()
            .has_headers(detect_csv_headers(&buffer))
            .from_reader(buffer.as_bytes());

        let mut all_trades = Vec::new();
        for result in reader.deserialize() {
            let csv_trade: CsvAggTrade = result?;
            let agg_trade: AggTrade = csv_trade.into();
            all_trades.push(agg_trade);
        }

        let trades_count = all_trades.len() as u64;
        processor.process_trades_continuously(&all_trades);
        let completed_bars = processor.get_all_completed_bars();

        Ok((trades_count, completed_bars))
    }

    // DATA INTEGRITY VERIFICATION METHODS

    /// Download and verify SHA256 checksum for a data file
    #[cfg(feature = "data-integrity")]
    async fn verify_file_integrity(
        &self,
        zip_data: &[u8],
        symbol: &str,
        date_str: &str,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        // Download the corresponding CHECKSUM file
        let checksum_url = format!(
            "https://data.binance.vision/data/{}/daily/aggTrades/{}/{}-aggTrades-{}.zip.CHECKSUM",
            self.get_market_path(),
            symbol,
            symbol,
            date_str
        );

        let response = tokio::time::timeout(
            tokio::time::Duration::from_secs(10),
            self.client.get(&checksum_url).send(),
        )
        .await??;

        if !response.status().is_success() {
            return Err(format!("Failed to download checksum: HTTP {}", response.status()).into());
        }

        let checksum_text = response.text().await?;
        let expected_hash = checksum_text
            .split_whitespace()
            .next()
            .ok_or("Invalid checksum format")?;

        // Compute SHA256 of the downloaded zip data
        let mut hasher = Sha256::new();
        hasher.update(zip_data);
        let computed_hash = format!("{:x}", hasher.finalize());

        if computed_hash != expected_hash {
            return Err(format!(
                "SHA256 mismatch for {}-aggTrades-{}.zip: expected {}, got {}",
                symbol, date_str, expected_hash, computed_hash
            )
            .into());
        }

        println!(
            "✓ SHA256 verification passed for {}-aggTrades-{}.zip",
            symbol, date_str
        );
        Ok(true)
    }

    /// Fallback for when data-integrity feature is disabled
    #[cfg(not(feature = "data-integrity"))]
    async fn verify_file_integrity(
        &self,
        _zip_data: &[u8],
        symbol: &str,
        date_str: &str,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        println!(
            "⚠ SHA256 verification skipped for {}-aggTrades-{}.zip (data-integrity feature disabled)",
            symbol, date_str
        );
        Ok(true)
    }

    // CONTINUOUS PROCESSING METHODS FOR DAY-BOUNDARY CONTINUITY

    #[cfg(feature = "statistics")]
    #[allow(dead_code)]
    async fn load_single_day_trades(
        &self,
        symbol: &str,
        date: NaiveDate,
        all_raw_trades: &mut Vec<AggTrade>,
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let date_str = date.format("%Y-%m-%d");
        let url = format!(
            "https://data.binance.vision/data/{}/daily/aggTrades/{}/{}-aggTrades-{}.zip",
            self.get_market_path(),
            symbol,
            symbol,
            date_str
        );

        let response = tokio::time::timeout(
            tokio::time::Duration::from_secs(30),
            self.client.get(&url).send(),
        )
        .await??;

        if !response.status().is_success() {
            return Err(format!("HTTP {}", response.status()).into());
        }

        let zip_bytes = response.bytes().await?;

        // Verify data integrity using SHA256 checksum
        self.verify_file_integrity(&zip_bytes, symbol, &date_str.to_string())
            .await?;

        let cursor = Cursor::new(zip_bytes);
        let mut archive = ZipArchive::new(cursor)?;

        let csv_filename = format!("{}-aggTrades-{}.csv", symbol, date_str);
        let mut csv_file = archive.by_name(&csv_filename)?;

        let mut buffer = String::with_capacity(8 * 1024 * 1024);
        csv_file.read_to_string(&mut buffer)?;

        let mut reader = ReaderBuilder::new()
            .has_headers(detect_csv_headers(&buffer))
            .from_reader(buffer.as_bytes());

        // OPTIMIZATION: Pre-allocate daily trades vector (typical day = 1-4M trades)
        let mut day_trades = Vec::with_capacity(2_000_000);
        for result in reader.deserialize() {
            let csv_trade: CsvAggTrade = result?;
            let agg_trade: AggTrade = csv_trade.into();
            day_trades.push(agg_trade);
        }

        // Sort by timestamp to ensure chronological order for continuous processing
        day_trades.sort_by_key(|trade| trade.timestamp);

        let trades_count = day_trades.len() as u64;

        // OPTIMIZATION: Pre-allocation already handled above to avoid vector reallocations

        all_raw_trades.extend(day_trades);

        Ok(trades_count)
    }

    /// Boundary-safe daily trade processing that preserves range bar state across days
    /// This method prevents the boundary violation issue by reusing the same processor
    async fn load_single_day_trades_boundary_safe(
        &self,
        symbol: &str,
        date: NaiveDate,
        processor: &mut ExportRangeBarProcessor,
        all_range_bars: &mut Vec<RangeBar>,
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let date_str = date.format("%Y-%m-%d");
        let url = format!(
            "https://data.binance.vision/data/{}/daily/aggTrades/{}/{}-aggTrades-{}.zip",
            self.get_market_path(),
            symbol,
            symbol,
            date_str
        );

        let response = tokio::time::timeout(
            tokio::time::Duration::from_secs(30),
            self.client.get(&url).send(),
        )
        .await??;

        if !response.status().is_success() {
            return Err(format!("HTTP {}", response.status()).into());
        }

        let zip_bytes = response.bytes().await?;

        // Verify data integrity using SHA256 checksum
        self.verify_file_integrity(&zip_bytes, symbol, &date_str.to_string())
            .await?;

        let cursor = Cursor::new(zip_bytes);
        let mut archive = ZipArchive::new(cursor)?;

        let csv_filename = format!("{}-aggTrades-{}.csv", symbol, date_str);
        let mut csv_file = archive.by_name(&csv_filename)?;

        let mut buffer = String::with_capacity(8 * 1024 * 1024);
        csv_file.read_to_string(&mut buffer)?;

        let mut reader = ReaderBuilder::new()
            .has_headers(detect_csv_headers(&buffer))
            .from_reader(buffer.as_bytes());

        // OPTIMIZATION: Pre-allocate daily trades vector
        let mut day_trades = Vec::with_capacity(2_000_000);
        for result in reader.deserialize() {
            let csv_trade: CsvAggTrade = result?;
            let agg_trade: AggTrade = csv_trade.into();
            day_trades.push(agg_trade);
        }

        let trades_count = day_trades.len() as u64;

        // CRITICAL FIX: Use existing processor to preserve range bar state across days
        processor.process_trades_continuously(&day_trades);
        let completed_bars = processor.get_all_completed_bars();
        all_range_bars.extend(completed_bars);

        Ok(trades_count)
    }

    #[allow(dead_code)] // Alternative processing method with statistics
    async fn process_single_day_with_stats(
        &self,
        processor: &mut ExportRangeBarProcessor,
        symbol: &str,
        date: NaiveDate,
        all_raw_trades: &mut Vec<AggTrade>,
    ) -> Result<(u64, Vec<RangeBar>), Box<dyn std::error::Error + Send + Sync>> {
        let date_str = date.format("%Y-%m-%d");
        let url = format!(
            "https://data.binance.vision/data/{}/daily/aggTrades/{}/{}-aggTrades-{}.zip",
            self.get_market_path(),
            symbol,
            symbol,
            date_str
        );

        let response = tokio::time::timeout(
            tokio::time::Duration::from_secs(30),
            self.client.get(&url).send(),
        )
        .await??;

        if !response.status().is_success() {
            return Err(format!("HTTP {}", response.status()).into());
        }

        let zip_bytes = response.bytes().await?;

        // Verify data integrity using SHA256 checksum
        self.verify_file_integrity(&zip_bytes, symbol, &date_str.to_string())
            .await?;

        let cursor = Cursor::new(zip_bytes);
        let mut archive = ZipArchive::new(cursor)?;

        let csv_filename = format!("{}-aggTrades-{}.csv", symbol, date_str);
        let mut csv_file = archive.by_name(&csv_filename)?;

        let mut buffer = String::with_capacity(8 * 1024 * 1024);
        csv_file.read_to_string(&mut buffer)?;

        let mut reader = ReaderBuilder::new()
            .has_headers(detect_csv_headers(&buffer))
            .from_reader(buffer.as_bytes());

        // OPTIMIZATION: Pre-allocate daily trades vector (typical day = 1-4M trades)
        let mut day_trades = Vec::with_capacity(2_000_000);
        for result in reader.deserialize() {
            let csv_trade: CsvAggTrade = result?;
            let agg_trade: AggTrade = csv_trade.into();
            day_trades.push(agg_trade.clone());
            all_raw_trades.push(agg_trade); // Collect for statistical analysis
        }

        let trades_count = day_trades.len() as u64;
        processor.process_trades_continuously(&day_trades);
        let completed_bars = processor.get_all_completed_bars();

        Ok((trades_count, completed_bars))
    }

    fn export_to_csv(
        &self,
        bars: &[RangeBar],
        filename: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let filepath = Path::new(&self.output_dir).join(filename);
        let mut wtr = WriterBuilder::new().from_path(filepath)?;

        for bar in bars {
            wtr.serialize(bar)?;
        }

        wtr.flush()?;
        Ok(())
    }

    #[allow(dead_code)] // Basic export method, superseded by export_to_json_with_metadata
    fn export_to_json(
        &self,
        bars: &[RangeBar],
        filename: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let filepath = Path::new(&self.output_dir).join(filename);
        let json_content = serde_json::to_string_pretty(bars)?;
        fs::write(filepath, json_content)?;
        Ok(())
    }

    #[cfg(feature = "statistics")]
    fn export_to_json_with_metadata(
        &self,
        bars: &[RangeBar],
        filename: &str,
        // metadata: Option<&rangebar::statistics::RangeBarMetadata>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use serde_json::json;

        let filepath = Path::new(&self.output_dir).join(filename);

        let comprehensive_export = {
            // Simple JSON structure without statistics (statistics module disabled)
            json!({
                "schema_version": "1.0.0",
                "export_type": "basic_rangebar_data",
                "range_bars": bars,
                "summary": {
                    "total_bars": bars.len(),
                    "note": "Statistical analysis not available (statistics feature disabled)"
                }
            })
        };

        let json_content = serde_json::to_string_pretty(&comprehensive_export)?;
        fs::write(filepath, json_content)?;
        Ok(())
    }

    #[cfg(not(feature = "statistics"))]
    fn export_to_json_with_metadata(
        &self,
        bars: &[RangeBar],
        filename: &str,
        _metadata: Option<&()>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Fallback when statistics feature is disabled
        self.export_to_json(bars, filename)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 6 || args.len() > 7 {
        eprintln!(
            "Usage: {} <symbol> <start_date> <end_date> <threshold_bps> <output_dir> [market_type]",
            args[0]
        );
        eprintln!("Market types: spot (default), um (UM Futures)");
        eprintln!("Threshold: basis points (25 = 0.25%, 80 = 0.80%)");
        eprintln!("Examples:");
        eprintln!(
            "  {} BTCUSDT 2025-09-01 2025-09-09 25 ./output           # SPOT (default), 0.25%",
            args[0]
        );
        eprintln!(
            "  {} BTCUSDT 2025-09-01 2025-09-09 80 ./output spot      # SPOT (explicit), 0.80%",
            args[0]
        );
        eprintln!(
            "  {} BTCUSDT 2025-09-01 2025-09-09 25 ./output um        # UM Futures, 0.25%",
            args[0]
        );
        std::process::exit(1);
    }

    // Load configuration
    let _config = Settings::load().unwrap_or_else(|_| Settings::default());

    let symbol = &args[1];
    let start_date = NaiveDate::parse_from_str(&args[2], "%Y-%m-%d")?;
    let end_date = NaiveDate::parse_from_str(&args[3], "%Y-%m-%d")?;
    let threshold_bps: u32 = args[4].parse()?;
    let output_dir = args[5].clone();

    // Default to "spot", optional "um" for UM Futures
    let market_type = if args.len() == 7 {
        match args[6].as_str() {
            "spot" | "um" => args[6].clone(),
            _ => {
                eprintln!(
                    "Error: market_type must be 'spot' or 'um', got '{}'",
                    args[6]
                );
                std::process::exit(1);
            }
        }
    } else {
        "spot".to_string()
    };

    let exporter = RangeBarExporter::new(output_dir, market_type)?;
    let result = exporter
        .export_symbol_range_bars(symbol, start_date, end_date, threshold_bps)
        .await
        .map_err(|e| -> Box<dyn std::error::Error> {
            Box::new(std::io::Error::other(e.to_string()))
        })?;

    // Export enhanced summary information
    let summary_file = format!("{}/export_summary.json", exporter.output_dir);
    let summary_json = serde_json::to_string_pretty(&result)?;
    fs::write(&summary_file, summary_json)?;
    println!("   📄 Enhanced Summary: {}", summary_file);

    Ok(())
}
