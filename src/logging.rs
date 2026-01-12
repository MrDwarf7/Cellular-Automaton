use chrono::Local;
use fern::Dispatch;
use log::LevelFilter;
use std::fs::OpenOptions;
use std::io;

pub fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("cellular_sim.log")?;

    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {:5}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Info)
        .level_for("cellular_sim", LevelFilter::Debug)
        // Console output
        .chain(io::stdout())
        // File output
        .chain(log_file)
        .apply()?;

    log::info!("=== Cellular Simulation Started ===");
    Ok(())
}

pub fn log_startup_info(grid_width: u32, grid_height: u32, preset: &str) {
    log::info!("Grid Size: {}x{} ({} cells)", 
        grid_width, grid_height, grid_width as u64 * grid_height as u64);
    log::info!("Preset: {}", preset);
    log::info!("Chunk Size: 32x32");
}

pub fn log_tick_performance(tick: u64, time_ms: f64, cells_processed: u64) {
    let cells_per_ms = cells_processed as f64 / time_ms;
    log::debug!("Tick {}: {:.2}ms ({:.0} cells/ms)", tick, time_ms, cells_per_ms);
}
