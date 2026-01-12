# Performance Metrics & Logging System

## Overview

The Cellular Simulation now includes comprehensive performance metrics and logging capabilities for monitoring real-time performance and debugging.

## Features

### Real-Time Metrics Display

The UI displays live performance metrics in the metrics row:
- **Tick**: Current simulation tick number
- **FPS**: Frames per second (rendering)
- **TPS**: Ticks per second (simulation speed)
- **CPU**: CPU usage percentage for this process
- **RAM**: RAM usage in MB
- **Avg Frame**: Average frame render time in milliseconds
- **Status**: Running/Paused state

### Console Output

All metrics are printed to the console with colored output:
```
[2024-01-12 10:30:45.123  INFO] === Cellular Simulation Started ===
[2024-01-12 10:30:45.124  INFO] Grid Size: 2000x2000 (4000000 cells)
[2024-01-12 10:30:45.125  INFO] Preset: sparse_genesis
[2024-01-12 10:30:45.126  INFO] Chunk Size: 32x32
[2024-01-12 10:30:46.001 DEBUG] Tick 1: 15.23ms (262,660 cells/ms)
[2024-01-12 10:30:46.016 DEBUG] Tick 2: 14.89ms (268,519 cells/ms)
```

### File Logging

All logs are written to `cellular_sim.log` in the working directory:
- Includes timestamps with millisecond precision
- Log level indicators (DEBUG, INFO, ERROR)
- Persistent record of session performance

### Metrics Collection

The `MetricsCollector` tracks:
- **Frame Times**: Last 120 frame durations (sliding window)
- **Tick Times**: Last 120 tick durations (sliding window)
- **CPU Usage**: Per-process CPU utilization
- **Memory Usage**: Total system RAM consumed
- **History**: Last 1000 metric snapshots with timestamps

## Metrics Data Structure

Each recorded metric snapshot contains:
```rust
pub struct FrameMetrics {
    pub timestamp: Instant,    // When measurement was taken
    pub fps: f64,              // Frames per second
    pub tps: f64,              // Ticks per second
    pub cpu_percent: f32,      // CPU usage %
    pub ram_mb: f64,           // RAM usage in MB
    pub grid_size: u32,        // Grid width/height
}
```

## Performance Thresholds

- **Good**: FPS > 30, TPS > 60
- **Acceptable**: FPS > 15, TPS > 30
- **Poor**: FPS < 15, TPS < 15

For a 2000x2000 grid (4M cells):
- Target: 60+ TPS (ticks per second)
- Acceptable: 30+ TPS
- Minimum viable: 10+ TPS

## Key Metrics Explained

### FPS (Frames Per Second)
- Measures rendering performance
- Affected by: GPU, screen resolution, image encoding
- Target: 60 FPS for smooth display

### TPS (Ticks Per Second)
- Measures simulation performance
- Affected by: CPU, grid size, cell rules complexity
- One tick = one complete generation (all cells process rules)

### CPU Usage
- Per-process CPU utilization
- On multi-core systems, can exceed 100% per core
- Affected by: parallelization, grid size, rule complexity

### RAM Usage
- Total system memory consumed
- 2000x2000 grid = ~200MB for grid data
- Includes UI, logging, and system overhead

### Avg Frame/Tick Time
- Average time to complete one frame/tick
- Inverse of FPS/TPS (1000ms / avg_time = FPS)
- Useful for identifying bottlenecks

## Logging Configuration

Modify log level in `src/logging.rs`:

```rust
.level(LevelFilter::Info)                    // Default level
.level_for("cellular_sim", LevelFilter::Debug) // Simulation-specific
```

Levels:
- **Error**: Critical failures
- **Warn**: Potential issues
- **Info**: General information, startup messages
- **Debug**: Detailed tick performance, cell operations
- **Trace**: Very detailed tracing (not used currently)

## Usage

### Starting the Simulation

```
cargo run --release
```

This will:
1. Initialize logging to console and `cellular_sim.log`
2. Create a new simulation with sparse_genesis preset
3. Log startup information
4. Display live metrics in the UI

### Analyzing Performance

#### During Simulation
- Watch the metrics line for real-time performance
- Note when FPS/TPS drop (indicates bottleneck)
- Observe CPU/RAM usage trends

#### After Simulation
- Open `cellular_sim.log` to review session history
- Search for "PERFORMANCE SUMMARY" for end-of-session stats
- Analyze tick times to identify problematic generations

## Example Log Output

```
[2024-01-12 10:30:45.123  INFO] === Cellular Simulation Started ===
[2024-01-12 10:30:45.124  INFO] Grid Size: 2000x2000 (4000000 cells)
[2024-01-12 10:30:45.125  INFO] Preset: sparse_genesis
[2024-01-12 10:30:45.126  INFO] Chunk Size: 32x32
[2024-01-12 10:30:46.001 DEBUG] Tick 1: 15.23ms (262,660 cells/ms)
[2024-01-12 10:30:46.016 DEBUG] Tick 2: 14.89ms (268,519 cells/ms)
[2024-01-12 10:30:46.032 DEBUG] Tick 3: 15.12ms (264,905 cells/ms)

Tick: 3 | FPS: 58.3 | TPS: 65.2 | CPU: 35.2% | RAM: 542.3MB | Avg Frame: 17.15ms | Status: Running
```

## Performance Optimization Tips

Based on metrics:

1. **If TPS is low but CPU is low**: GPU/I/O bound, try simplifying rules
2. **If TPS is low and CPU is high**: CPU bottleneck, parallelize further
3. **If FPS is low but TPS is high**: Rendering bottleneck, increase display scale
4. **If RAM keeps growing**: Potential memory leak, check grid operations

## Future Enhancements

- [ ] Graph display of FPS/TPS/CPU/RAM over time
- [ ] Export metrics to CSV for analysis
- [ ] Performance profiling mode (per-rule statistics)
- [ ] Network metrics for distributed simulation
- [ ] Real-time performance alerts (TPS drop warnings)
