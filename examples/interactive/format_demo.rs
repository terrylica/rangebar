#!/usr/bin/env cargo

//! Demo of new range bar formatting

fn main() {
    println!("🚀 Range Bar Formatting Demo");
    println!("============================");
    println!();

    // Simulate some range bar completions with different directions and values
    let bars = vec![
        (1, 0.2675, 0.2678, 0.2668, 0.2668, 45104469.00, 3559, false), // Down bar - like screenshot
        (10, 0.2668, 0.2675, 0.2667, 0.2675, 99218858.00, 586, true),  // Up bar
        (100, 0.2675, 0.2681, 0.2675, 0.2678, 97747463.00, 378, true), // Up bar
        (999, 0.2681, 0.2685, 0.2678, 0.2680, 50659969.00, 2184, false), // Down bar
    ];

    for (bar_num, open, high, low, close, volume, trades, is_up) in bars {
        let direction = if is_up {
            "\x1b[32m↑\x1b[0m"  // Green up arrow
        } else {
            "\x1b[31m↓\x1b[0m"  // Red down arrow
        };

        let duration = match bar_num {
            1 => "34m 47s",
            10 => "5m 44s",
            100 => "4m 5s",
            _ => "9m 26s"
        };

        println!(
            "{} BAR {:>4} • O:{:.4} H:{:.4} L:{:.4} C:{:.4} • Vol:{:>12.2} • Trades:{:>6} • {:>10}",
            direction, bar_num, open, high, low, close, volume, trades, duration
        );
    }

    println!();
    println!("📊 Final: 4 bars (\x1b[32m↑2\x1b[0m \x1b[31m↓2\x1b[0m) from 10943 trades");
    println!();
    println!("✨ Perfect vertical alignment with:");
    println!("   • Colored directional arrows (↑/↓)");
    println!("   • Fixed-width bar numbers (:>4)");
    println!("   • Consistent OHLC formatting (.4 precision)");
    println!("   • Right-aligned volume field (:>12.2) - handles large volumes");
    println!("   • Right-aligned trade counts (:>6) - handles 4+ digit counts");
    println!("   • Right-aligned duration (:>10) - consistent spacing");
}