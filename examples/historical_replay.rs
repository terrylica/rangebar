//! Historical Range Bar Visualizer with Time-Aware Playback
//!
//! Loads 3 months of historical BTCUSDT aggTrades data and replays with
//! accelerated timing to visualize range bar formation. Uses single-line
//! terminal updates until range bars complete.

use std::io::Write;
use std::time::Duration;

use crossterm::event::{poll, read, Event, KeyCode, KeyEventKind};
//use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use tokio::signal;

use rangebar::{AggTrade};
use rangebar::range_bars::ExportRangeBarProcessor;
use rangebar::data::HistoricalDataLoader;

/// Time-aware playback engine with acceleration
struct PlaybackEngine {
    trades: Vec<AggTrade>,
    current_index: usize,
    acceleration_factor: f64,
    paused: bool,
}

impl PlaybackEngine {
    fn new(trades: Vec<AggTrade>, acceleration_factor: f64) -> Self {
        Self {
            trades,
            current_index: 0,
            acceleration_factor,
            paused: false,
        }
    }

    fn set_speed(&mut self, factor: f64) {
        self.acceleration_factor = factor;
        println!("\n🚀 Speed: {:.0}x", factor);
    }

    fn toggle_pause(&mut self) {
        self.paused = !self.paused;
        if self.paused {
            println!("\n⏸️  Paused");
        } else {
            println!("\n▶️  Resumed at {:.0}x speed", self.acceleration_factor);
        }
    }

    async fn next_trade(&mut self) -> Option<AggTrade> {
        if self.paused || self.current_index >= self.trades.len() {
            return None;
        }

        let trade = self.trades[self.current_index].clone();

        // Calculate time delta to next trade
        if self.current_index + 1 < self.trades.len() {
            let current_timestamp = self.trades[self.current_index].timestamp;
            let next_timestamp = self.trades[self.current_index + 1].timestamp;
            let delta_ms = (next_timestamp - current_timestamp) as f64;

            // Apply acceleration and sleep
            let accelerated_delay_ms = delta_ms / self.acceleration_factor;
            if accelerated_delay_ms > 0.1 {
                tokio::time::sleep(Duration::from_millis(accelerated_delay_ms as u64)).await;
            }
        }

        self.current_index += 1;
        Some(trade)
    }

    fn progress(&self) -> (usize, usize, f64) {
        let percent = (self.current_index as f64 / self.trades.len() as f64) * 100.0;
        (self.current_index, self.trades.len(), percent)
    }
}

/// Terminal display manager
struct TerminalDisplay {
    bar_count: u32,
    trade_count: u64,
    current_bar_trades: u64,
    current_bar_open: Option<f64>,
}

impl TerminalDisplay {
    fn new() -> Self {
        Self {
            bar_count: 0,
            trade_count: 0,
            current_bar_trades: 0,
            current_bar_open: None,
        }
    }

    fn update_building_bar(&mut self, price: f64) {
        self.trade_count += 1;
        self.current_bar_trades += 1;

        if self.current_bar_open.is_none() {
            self.current_bar_open = Some(price);
        }

        // Single-line update using carriage return
        print!("\rBuilding bar #{}: {} trades, current: ${:.2}, open: ${:.2}",
               self.bar_count + 1,
               self.current_bar_trades,
               price,
               self.current_bar_open.unwrap_or(price));
        std::io::stdout().flush().unwrap();
    }

    fn complete_range_bar(&mut self, open: f64, high: f64, low: f64, close: f64, volume: f64) {
        self.bar_count += 1;

        // Print completed bar on new line
        println!("\n✅ RANGE BAR #{}: OHLC = {:.2}/{:.2}/{:.2}/{:.2}, Volume = {:.6}, Trades: {}",
                self.bar_count,
                open,
                high,
                low,
                close,
                volume,
                self.current_bar_trades);

        // Reset for next bar
        self.current_bar_trades = 0;
        self.current_bar_open = None;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Historical Range Bar Visualizer - BTCUSDT (25 BPS)");
    println!("========================================================");
    println!("Controls: q=quit, +=faster, -=slower, p=pause");
    println!("");

    // Raw mode disabled for clean terminal output

    // Load 3 months of historical data
    let loader = HistoricalDataLoader::new("BTCUSDT");
    let trades = loader.load_historical_range(90).await?;

    // Initialize components
    let mut playback = PlaybackEngine::new(trades, 10000.0); // 10000x acceleration
    let mut processor = ExportRangeBarProcessor::new(25); // 25 BPS threshold
    let mut display = TerminalDisplay::new();

    println!("▶️  Starting playback at {:.0}x speed...\n", playback.acceleration_factor);

    loop {
        // Handle keyboard input (non-blocking)
        if poll(Duration::from_millis(0))? {
            if let Event::Key(key) = read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => {
                            println!("\n👋 Exiting...");
                            break;
                        }
                        KeyCode::Char('+') | KeyCode::Char('=') => {
                            let new_speed = (playback.acceleration_factor * 2.0).min(100000.0);
                            playback.set_speed(new_speed);
                        }
                        KeyCode::Char('-') => {
                            let new_speed = (playback.acceleration_factor / 2.0).max(100.0);
                            playback.set_speed(new_speed);
                        }
                        KeyCode::Char('p') => {
                            playback.toggle_pause();
                        }
                        _ => {}
                    }
                }
            }
        }

        // Handle Ctrl+C
        tokio::select! {
            _ = signal::ctrl_c() => {
                println!("\n👋 Shutting down gracefully...");
                break;
            }
            trade_opt = playback.next_trade() => {
                if let Some(trade) = trade_opt {
                    display.update_building_bar(trade.price.to_f64());

                    // Process trade using continuous processor
                    processor.process_trades_continuously(&[trade]);

                    // Get any newly completed bars
                    let completed_bars = processor.get_all_completed_bars();
                    for bar in completed_bars {
                        display.complete_range_bar(
                            bar.open.to_f64(),
                            bar.high.to_f64(),
                            bar.low.to_f64(),
                            bar.close.to_f64(),
                            bar.volume.to_f64()
                        );
                    }

                    // Progress update every 100k trades
                    if display.trade_count % 100_000 == 0 {
                        let (current, total, percent) = playback.progress();
                        println!("\n📊 Progress: {}/{} trades ({:.1}%)", current, total, percent);
                    }
                } else if !playback.paused {
                    // End of data
                    println!("\n🎉 Playback completed!");
                    break;
                }
            }
        }
    }

    println!("📈 Final stats: {} range bars formed from {} trades",
             display.bar_count, display.trade_count);

    // Terminal cleanup not needed without raw mode
    Ok(())
}