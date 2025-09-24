#!/usr/bin/env cargo

//! Demo comparing spot vs futures market data access

fn main() {
    println!("🏪 Market Type Configuration Demo");
    println!("=================================");
    println!();

    println!("📊 **NEW DEFAULTS** (Post-Update):");
    println!();

    println!("🥇 **SPOT MARKET (Default)**:");
    println!("   cargo run --example historical_replay");
    println!("   cargo run --example historical_replay -- BTCUSDT");
    println!("   cargo run --example historical_replay -- BTCUSDT spot");
    println!();

    println!("🚀 **UM FUTURES (Optional)**:");
    println!("   cargo run --example historical_replay -- DOGEUSDT um");
    println!("   cargo run --example historical_replay -- BTCUSDT um");
    println!();

    println!("⚡ **CM FUTURES (Optional)**:");
    println!("   cargo run --example historical_replay -- BTCUSDT cm");
    println!();

    println!("📈 **RangeBar Export Examples**:");
    println!("   # Spot (default)");
    println!("   cargo run --bin rangebar-export -- BTCUSDT 2024-01-01 2024-01-02 25 ./output");
    println!("   ");
    println!("   # UM Futures");
    println!("   cargo run --bin rangebar-export -- BTCUSDT 2024-01-01 2024-01-02 25 ./output um");
    println!();

    println!("🔍 **Key Differences Observed**:");
    println!("   • **Spot**: Standard spot trading, market hours may apply");
    println!("   • **UM Futures**: Leveraged perpetual contracts, 24/7 trading");
    println!("   • **Trade Volume**: Both markets show similar daily volumes (200k-400k trades)");
    println!("   • **Data Availability**: Recent dates may have limited spot data availability");
    println!();

    println!("📁 **Data Sources**:");
    println!("   • Spot: data.binance.vision/data/spot/daily/aggTrades/");
    println!("   • UM Futures: data.binance.vision/data/futures/um/daily/aggTrades/");
    println!("   • CM Futures: data.binance.vision/data/futures/cm/daily/aggTrades/");
    println!();

    println!("✨ **Migration Complete**: Spot is now the default, UM/CM are optional!");
}
