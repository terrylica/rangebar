#!/usr/bin/env cargo

//! Date Availability Investigation
//!
//! This investigates which dates are actually loaded when using load_recent_day()
//! for different markets to understand data availability patterns.

use std::time::Instant;
use chrono::Utc;
use rangebar::data::HistoricalDataLoader;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Investigating Data Availability Patterns");
    println!("===========================================");
    println!();

    let symbol = "BTCUSDT";
    let markets = ["spot", "um"];

    for market in &markets {
        println!("📊 Market: {} ({})", market.to_uppercase(), symbol);
        println!("────────────────────────────────────────");

        let loader = HistoricalDataLoader::new_with_market(symbol, market);

        // Test each of the last 7 days individually to see what's available
        for days_back in 1..=7 {
            let test_date = Utc::now().date_naive() - chrono::Duration::days(days_back);

            print!("  {} ({} days back): ", test_date.format("%Y-%m-%d"), days_back);

            let start_time = Instant::now();
            match loader.load_single_day_trades(test_date).await {
                Ok(trades) => {
                    let duration = start_time.elapsed();
                    println!("✅ {} trades ({:.1}s)", trades.len(), duration.as_secs_f64());

                    // Show timestamp range for this data
                    if !trades.is_empty() {
                        let first_ts = trades[0].timestamp;
                        let last_ts = trades[trades.len() - 1].timestamp;
                        let first_time = chrono::DateTime::from_timestamp_millis(first_ts).unwrap();
                        let last_time = chrono::DateTime::from_timestamp_millis(last_ts).unwrap();
                        println!("     Time range: {} to {}",
                                first_time.format("%H:%M:%S"),
                                last_time.format("%H:%M:%S"));

                        // Calculate duration of data coverage
                        let coverage_hours = (last_ts - first_ts) as f64 / (1000.0 * 3600.0);
                        println!("     Coverage: {:.1} hours", coverage_hours);
                    }
                }
                Err(e) => {
                    let duration = start_time.elapsed();
                    println!("❌ {} ({:.1}s)", e, duration.as_secs_f64());
                }
            }
        }

        println!();

        // Now test load_recent_day() to see which date it actually picks
        println!("  🎯 load_recent_day() result:");
        let start_time = Instant::now();
        match loader.load_recent_day().await {
            Ok(trades) => {
                let duration = start_time.elapsed();
                println!("     ✅ {} trades ({:.1}s)", trades.len(), duration.as_secs_f64());

                if !trades.is_empty() {
                    let first_ts = trades[0].timestamp;
                    let last_ts = trades[trades.len() - 1].timestamp;
                    let first_time = chrono::DateTime::from_timestamp_millis(first_ts).unwrap();
                    let last_time = chrono::DateTime::from_timestamp_millis(last_ts).unwrap();

                    // Determine which date this data is from
                    let data_date = first_time.date_naive();
                    println!("     Data from: {}", data_date.format("%Y-%m-%d"));
                    println!("     Time range: {} to {}",
                            first_time.format("%H:%M:%S"),
                            last_time.format("%H:%M:%S"));

                    let coverage_hours = (last_ts - first_ts) as f64 / (1000.0 * 3600.0);
                    println!("     Coverage: {:.1} hours", coverage_hours);
                }
            }
            Err(e) => {
                let duration = start_time.elapsed();
                println!("     ❌ {} ({:.1}s)", e, duration.as_secs_f64());
            }
        }

        println!();
    }

    println!("🎯 **KEY HYPOTHESIS TO TEST:**");
    println!("   If spot and UM futures load data from DIFFERENT DATES,");
    println!("   that could explain the dramatic difference in range bar formation times!");

    Ok(())
}