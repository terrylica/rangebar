//! Custom Threshold Validation
//!
//! Tests range bar algorithm with different BPS thresholds to ensure
//! mathematical accuracy across various threshold levels.

use rangebar::range_bars::ExportRangeBarProcessor;
use rangebar::data::HistoricalDataLoader;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let symbol = std::env::args().nth(1).unwrap_or_else(|| "BTCUSDT".to_string());
    let threshold_bps: u32 = std::env::args().nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(50); // Default 50 BPS = 0.50%

    println!("🔬 Custom Threshold Validation: {} BPS ({:.2}%)", threshold_bps, threshold_bps as f64 / 100.0);
    println!("Symbol: {}", symbol);
    println!("========================================");

    // Load recent day of data
    let loader = HistoricalDataLoader::new(&symbol);
    let trades = loader.load_recent_day().await?;
    println!("📊 Loaded {} trades", trades.len());

    // Process trades into range bars
    let mut processor = ExportRangeBarProcessor::new(threshold_bps);
    processor.process_trades_continuously(&trades);
    let bars = processor.get_all_completed_bars();

    println!("📈 Generated {} range bars", bars.len());

    if bars.is_empty() {
        println!("⚠️ No range bars generated - market too stable for {} BPS threshold", threshold_bps);
        return Ok(());
    }

    // Validate first 10 range bars in detail
    let sample_size = std::cmp::min(10, bars.len());
    let threshold_pct = threshold_bps as f64 / 10_000.0;

    println!("\n📋 DETAILED VALIDATION (First {} bars):", sample_size);
    println!("=============================================");

    let mut all_valid = true;

    for (i, bar) in bars.iter().take(sample_size).enumerate() {
        let open = bar.open.to_f64();
        let close = bar.close.to_f64();
        let high = bar.high.to_f64();
        let low = bar.low.to_f64();

        let expected_upper = open * (1.0 + threshold_pct);
        let expected_lower = open * (1.0 - threshold_pct);

        let actual_movement_pct = ((close - open) / open) * 100.0;

        let breach_type = if close >= expected_upper {
            "UPPER"
        } else if close <= expected_lower {
            "LOWER"
        } else {
            "NONE"
        };

        let is_valid = match breach_type {
            "UPPER" => close >= expected_upper && (high >= expected_upper || low <= expected_lower),
            "LOWER" => close <= expected_lower && (high >= expected_upper || low <= expected_lower),
            "NONE" => false, // Range bars should always breach
            _ => false,
        };

        if !is_valid {
            all_valid = false;
        }

        let status = if is_valid { "✅" } else { "❌" };

        println!("Bar #{}: {} | Open: {:.6} | Close: {:.6} | Movement: {:.4}% | Breach: {}",
                 i + 1, status, open, close, actual_movement_pct, breach_type);
        println!("  Expected Thresholds: Upper {:.8} | Lower {:.8}", expected_upper, expected_lower);
        println!("  Actual OHLC: High {:.8} | Low {:.8}", high, low);

        // Verify close matches expected threshold
        if breach_type == "UPPER" {
            let threshold_error = ((close - expected_upper) / expected_upper) * 100.0;
            println!("  Threshold accuracy: {:.6}% error from upper threshold", threshold_error.abs());
        } else if breach_type == "LOWER" {
            let threshold_error = ((expected_lower - close) / expected_lower) * 100.0;
            println!("  Threshold accuracy: {:.6}% error from lower threshold", threshold_error.abs());
        }
        println!();
    }

    // Overall statistics
    let valid_count = bars.iter().take(sample_size).enumerate().filter(|(_i, bar)| {
        let open = bar.open.to_f64();
        let close = bar.close.to_f64();
        let high = bar.high.to_f64();
        let low = bar.low.to_f64();

        let expected_upper = open * (1.0 + threshold_pct);
        let expected_lower = open * (1.0 - threshold_pct);

        (close >= expected_upper && (high >= expected_upper || low <= expected_lower)) ||
        (close <= expected_lower && (high >= expected_upper || low <= expected_lower))
    }).count();

    println!("📊 SUMMARY:");
    println!("============");
    println!("Tested Bars: {}", sample_size);
    println!("Valid Bars: {} ({:.1}%)", valid_count, (valid_count as f64 / sample_size as f64) * 100.0);
    println!("Threshold: {} BPS ({:.2}%)", threshold_bps, threshold_pct * 100.0);

    if all_valid {
        println!("\n✅ SUCCESS: All sampled bars correctly implement {} BPS threshold!", threshold_bps);
    } else {
        println!("\n🚨 FAILURE: Some bars failed {} BPS threshold validation!", threshold_bps);
        std::process::exit(1);
    }

    Ok(())
}