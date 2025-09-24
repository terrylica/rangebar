//! Historical Range Bar Visualizer with Time-Aware Playback
//!
//! Loads 3 months of historical BTCUSDT aggTrades data and replays with
//! accelerated timing to visualize range bar formation. Uses single-line
//! terminal updates until range bars complete.

use std::io::Write;
use std::time::{Duration, Instant};

use crossterm::event::{poll, read, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use tokio::signal;

use rangebar::{AggTrade};
use rangebar::range_bars::ExportRangeBarProcessor;
use rangebar::data::HistoricalDataLoader;

/// Format duration in milliseconds to human-readable string
fn format_duration(duration_ms: i64) -> String {
    if duration_ms < 0 {
        return "0s".to_string();
    }

    let total_seconds = duration_ms / 1000;
    let milliseconds = duration_ms % 1000;

    if total_seconds == 0 {
        return format!("{}ms", duration_ms);
    }

    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    let mut parts = Vec::new();

    if hours > 0 {
        parts.push(format!("{}h", hours));
    }
    if minutes > 0 {
        parts.push(format!("{}m", minutes));
    }
    if seconds > 0 || parts.is_empty() {
        if milliseconds > 0 && parts.is_empty() && seconds < 10 {
            // Show decimals for short durations
            parts.push(format!("{:.1}s", seconds as f64 + milliseconds as f64 / 1000.0));
        } else {
            parts.push(format!("{}s", seconds));
        }
    }

    parts.join(" ")
}

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

    fn set_speed(&mut self, factor: f64) -> String {
        self.acceleration_factor = factor;
        format!("🚀 Speed: {:.0}x", factor)
    }

    fn toggle_pause(&mut self) -> String {
        self.paused = !self.paused;
        if self.paused {
            "⏸️  Paused".to_string()
        } else {
            format!("▶️  Resumed at {:.0}x speed", self.acceleration_factor)
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

/// Terminal display manager with rate limiting
struct TerminalDisplay {
    bar_count: u32,
    trade_count: u64,
    current_bar_trades: u64,
    current_bar_open: Option<f64>,
    last_update: Instant,
    last_price: f64,
    update_throttle: Duration,
    pending_updates: bool,
}

impl TerminalDisplay {
    fn new() -> Self {
        Self {
            bar_count: 0,
            trade_count: 0,
            current_bar_trades: 0,
            current_bar_open: None,
            last_update: Instant::now(),
            last_price: 0.0,
            update_throttle: Duration::from_millis(50), // Max 20 updates/sec
            pending_updates: false,
        }
    }

    fn update_building_bar(&mut self, price: f64) {
        self.trade_count += 1;
        self.current_bar_trades += 1;
        self.last_price = price;

        if self.current_bar_open.is_none() {
            self.current_bar_open = Some(price);
        }

        // Rate limit terminal updates to prevent formatting issues
        let now = Instant::now();
        if now.duration_since(self.last_update) >= self.update_throttle {
            self.render_building_bar();
            self.last_update = now;
            self.pending_updates = false;
        } else {
            self.pending_updates = true;
        }
    }

    fn render_building_bar(&self) {
        // Clear current line completely before writing new content
        print!("\r\x1b[K"); // Clear entire line
        print!("Building bar #{}: {} trades, current: ${:.2}, open: ${:.2}",
               self.bar_count + 1,
               self.current_bar_trades,
               self.last_price,
               self.current_bar_open.unwrap_or(self.last_price));
        std::io::stdout().flush().unwrap();
    }

    fn force_update(&mut self) {
        if self.pending_updates {
            self.render_building_bar();
            self.last_update = Instant::now();
            self.pending_updates = false;
        }
    }

    fn complete_range_bar(&mut self, open: f64, high: f64, low: f64, close: f64, volume: f64, open_time: i64, close_time: i64) {
        // Ensure any pending updates are rendered first
        self.force_update();

        self.bar_count += 1;

        // Calculate duration
        let duration_ms = close_time - open_time;
        let duration_str = format_duration(duration_ms);

        // Clear current line and print completed bar with duration
        println!("\r\x1b[K✅ RANGE BAR #{}: OHLC = {:.4}/{:.4}/{:.4}/{:.4}, Volume = {:.6}, Trades: {}, Duration: {}",
                self.bar_count,
                open,
                high,
                low,
                close,
                volume,
                self.current_bar_trades,
                duration_str);

        // Reset for next bar
        self.current_bar_trades = 0;
        self.current_bar_open = None;
        self.last_update = Instant::now();
        self.pending_updates = false;
    }

    fn print_message(&mut self, message: &str) {
        // Clear current line and print message on new line
        self.force_update();
        println!("\r\x1b[K\n{}", message);
        self.last_update = Instant::now();
        self.pending_updates = false;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Historical Range Bar Visualizer - DOGEUSDT (25 BPS)");
    println!("========================================================");
    println!("Controls: q=quit, +=faster, -=slower, p=pause");
    println!("");

    // Try to enable raw mode for keyboard input (optional for speed controls)
    let raw_mode_enabled = enable_raw_mode().is_ok();
    if !raw_mode_enabled {
        println!("⚠️ Keyboard controls disabled (terminal not interactive)");
    }

    // Load 2 days back of historical data for testing
    let loader = HistoricalDataLoader::new("DOGEUSDT");
    let trades = loader.load_historical_range(2).await?;

    // Initialize components
    let mut playback = PlaybackEngine::new(trades, 10000.0); // 10000x acceleration
    let mut processor = ExportRangeBarProcessor::new(25); // 25 BPS threshold
    let mut display = TerminalDisplay::new();

    println!("▶️  Starting playback at {:.0}x speed...\n", playback.acceleration_factor);

    loop {
        // Handle keyboard input (non-blocking) - only if raw mode is enabled
        if raw_mode_enabled && poll(Duration::from_millis(0)).unwrap_or(false) {
            if let Ok(Event::Key(key)) = read() {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => {
                            display.print_message("👋 Exiting...");
                            if raw_mode_enabled {
                                disable_raw_mode().ok();
                            }
                            break;
                        }
                        KeyCode::Char('+') | KeyCode::Char('=') => {
                            let new_speed = (playback.acceleration_factor * 2.0).min(100000.0);
                            let message = playback.set_speed(new_speed);
                            display.print_message(&message);
                        }
                        KeyCode::Char('-') => {
                            let new_speed = (playback.acceleration_factor / 2.0).max(1.0);
                            let message = playback.set_speed(new_speed);
                            display.print_message(&message);
                        }
                        KeyCode::Char('p') => {
                            let message = playback.toggle_pause();
                            display.print_message(&message);
                        }
                        _ => {}
                    }
                }
            }
        }

        // Handle Ctrl+C
        tokio::select! {
            _ = signal::ctrl_c() => {
                display.print_message("👋 Shutting down gracefully...");
                if raw_mode_enabled {
                    disable_raw_mode().ok();
                }
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
                            bar.volume.to_f64(),
                            bar.open_time,
                            bar.close_time
                        );
                    }

                    // Progress update every 100k trades
                    if display.trade_count % 100_000 == 0 {
                        let (current, total, percent) = playback.progress();
                        let progress_msg = format!("📊 Progress: {}/{} trades ({:.1}%)", current, total, percent);
                        display.print_message(&progress_msg);
                    }
                } else if !playback.paused {
                    // End of data - use display system for clean output
                    display.print_message("🎉 Playback completed!");
                    break;
                }
            }
        }
    }

    let final_stats = format!("📈 Final stats: {} range bars formed from {} trades",
                              display.bar_count, display.trade_count);
    display.print_message(&final_stats);

    // Clean up terminal raw mode if it was enabled
    if raw_mode_enabled {
        disable_raw_mode().ok();
    }
    Ok(())
}