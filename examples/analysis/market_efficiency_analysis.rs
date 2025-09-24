#!/usr/bin/env cargo

//! Market Efficiency Analysis: The Surprising Results

fn main() {
    println!("🎯 Market Efficiency Analysis: The Surprising Truth");
    println!("===================================================");
    println!();

    println!("📊 **COMPREHENSIVE RESULTS SUMMARY:**");
    println!();

    let results = [
        ("BTCUSDT", 666_593, 959_714, 28, 28, 1.44, 1.00, 0.69),
        ("ETHUSDT", 886_369, 1_577_437, 101, 111, 1.78, 1.10, 0.62),
        ("SOLUSDT", 372_163, 523_544, 200, 204, 1.41, 1.02, 0.73),
        ("DOGEUSDT", 292_004, 359_241, 248, 260, 1.23, 1.05, 0.85),
    ];

    println!("  Symbol   │ Spot aggTrades │ UM aggTrades │ Spot Bars │ UM Bars │ aggTrade Ratio │ Bar Ratio │ Efficiency Ratio");
    println!("  ─────────┼────────────────┼──────────────┼───────────┼─────────┼────────────────┼───────────┼─────────────────");

    for (symbol, spot_agg, um_agg, spot_bars, um_bars, agg_ratio, bar_ratio, eff_ratio) in &results {
        println!("  {:8} │      {:>9} │    {:>9} │    {:>6} │  {:>6} │          {:.2}x │     {:.2}x │           {:.2}x",
                symbol, spot_agg, um_agg, spot_bars, um_bars, agg_ratio, bar_ratio, eff_ratio);
    }

    println!();
    println!("🔍 **SHOCKING DISCOVERY:**");
    println!();
    println!("   1️⃣  **UM Futures have MORE aggTrades** (1.23x - 1.78x)");
    println!("   2️⃣  **But generate SIMILAR range bars** (1.00x - 1.10x)");
    println!("   3️⃣  **Result: LOWER efficiency per trade** (0.62x - 0.85x)");
    println!();

    println!("🤔 **What This Means:**");
    println!();
    println!("   • **More aggTrades ≠ More Range Bars**");
    println!("   • **Futures markets are LESS efficient at generating bars**");
    println!("   • **Spot markets have higher price volatility per trade**");
    println!("   • **Range bars depend on PRICE MOVEMENT, not trade count**");
    println!();

    println!("💡 **The Explanation:**");
    println!();
    println!("   🎯 **Spot Market Characteristics:**");
    println!("      • Fewer, but larger price-impact trades");
    println!("      • Each trade moves price more significantly");
    println!("      • Higher probability of threshold breach per trade");
    println!();
    println!("   🚀 **UM Futures Characteristics:**");
    println!("      • Many micro-trades from algorithms/bots");
    println!("      • Higher leverage = smaller individual impact");
    println!("      • More 'noise' trades that don't move price significantly");
    println!();

    println!("📈 **Volatility Per Asset:**");
    println!();
    println!("   Symbol      Bars Per Day (25 BPS threshold)");
    println!("   ──────────  ──────────────────────────────");
    for (symbol, _, _, spot_bars, um_bars, _, _, _) in &results {
        println!("   {:8}    Spot: {:>3} bars  |  UM: {:>3} bars",
                symbol, spot_bars, um_bars);
    }
    println!();
    println!("   📊 **Pattern**: DOGEUSDT is most volatile (248-260 bars/day)");
    println!("   📊 **Pattern**: BTCUSDT is least volatile (28 bars/day both markets)");

    println!();
    println!("✅ **CONCLUSION:**");
    println!("   **Spot markets are MORE EFFICIENT at range bar generation**");
    println!("   despite having fewer aggTrades. This suggests spot markets");
    println!("   have more meaningful price movements per trade!");
}