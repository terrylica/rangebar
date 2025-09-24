#!/usr/bin/env cargo

//! Timestamp Analysis Investigation
//!
//! This investigates the timestamp parsing differences between spot and UM futures
//! to understand why spot data shows impossible coverage periods.

use rangebar::data::HistoricalDataLoader;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🕒 Timestamp Analysis Investigation");
    println!("==================================");
    println!();

    let symbol = "BTCUSDT";
    let markets = ["spot", "um"];

    for market in &markets {
        println!("📊 Market: {} ({})", market.to_uppercase(), symbol);
        println!("────────────────────────────────────────");

        let loader = HistoricalDataLoader::new_with_market(symbol, market);

        match loader.load_recent_day().await {
            Ok(trades) => {
                println!("✅ Loaded {} trades", trades.len());

                if !trades.is_empty() {
                    // Examine the first few trades in detail
                    println!("\n🔍 First 5 trades timestamp analysis:");
                    for (i, trade) in trades.iter().take(5).enumerate() {
                        println!(
                            "  Trade {}: raw_ts={}, agg_id={}",
                            i + 1,
                            trade.timestamp,
                            trade.agg_trade_id
                        );

                        // Try to interpret as milliseconds
                        if let Some(dt_ms) =
                            chrono::DateTime::from_timestamp_millis(trade.timestamp)
                        {
                            println!(
                                "    As milliseconds: {}",
                                dt_ms.format("%Y-%m-%d %H:%M:%S%.3f")
                            );
                        } else {
                            println!("    As milliseconds: INVALID");
                        }

                        // Try to interpret as microseconds
                        if let Some(dt_us) =
                            chrono::DateTime::from_timestamp_micros(trade.timestamp)
                        {
                            println!(
                                "    As microseconds: {}",
                                dt_us.format("%Y-%m-%d %H:%M:%S%.6f")
                            );
                        } else {
                            println!("    As microseconds: INVALID");
                        }

                        // Try to interpret as seconds
                        if let Some(dt_s) = chrono::DateTime::from_timestamp(trade.timestamp, 0) {
                            println!("    As seconds: {}", dt_s.format("%Y-%m-%d %H:%M:%S"));
                        } else {
                            println!("    As seconds: INVALID");
                        }
                    }

                    // Check last few trades
                    println!("\n🔍 Last 5 trades timestamp analysis:");
                    let len = trades.len();
                    for (i, trade) in trades.iter().skip(len.saturating_sub(5)).enumerate() {
                        println!(
                            "  Trade {}: raw_ts={}, agg_id={}",
                            len - 5 + i + 1,
                            trade.timestamp,
                            trade.agg_trade_id
                        );

                        if let Some(dt_ms) =
                            chrono::DateTime::from_timestamp_millis(trade.timestamp)
                        {
                            println!(
                                "    As milliseconds: {}",
                                dt_ms.format("%Y-%m-%d %H:%M:%S%.3f")
                            );
                        } else {
                            println!("    As milliseconds: INVALID");
                        }
                    }

                    // Calculate time span different ways
                    let first_ts = trades[0].timestamp;
                    let last_ts = trades[len - 1].timestamp;
                    let span_raw = last_ts - first_ts;

                    println!("\n📊 Time span analysis:");
                    println!("  Raw timestamp difference: {}", span_raw);
                    println!(
                        "  If milliseconds: {:.1} hours",
                        span_raw as f64 / (1000.0 * 3600.0)
                    );
                    println!(
                        "  If microseconds: {:.1} hours",
                        span_raw as f64 / (1_000_000.0 * 3600.0)
                    );
                    println!("  If seconds: {:.1} hours", span_raw as f64 / 3600.0);

                    // Check for sorted order
                    let mut is_sorted = true;
                    for i in 1..trades.len() {
                        if trades[i].timestamp < trades[i - 1].timestamp {
                            is_sorted = false;
                            break;
                        }
                    }
                    println!("  Chronologically sorted: {}", is_sorted);
                }
            }
            Err(e) => {
                println!("❌ Failed to load data: {}", e);
            }
        }

        println!();
    }

    println!("🎯 **HYPOTHESIS:**");
    println!("   Spot data might have timestamp format differences (microseconds vs milliseconds)");
    println!("   or timestamp parsing issues causing impossible date ranges.");

    Ok(())
}
