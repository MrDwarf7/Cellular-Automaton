use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};
use std::collections::VecDeque;
use sysinfo::System;
use log::info;

#[derive(Clone, Debug)]
pub struct FrameMetrics {
    pub timestamp: Instant,
    pub fps: f64,
    pub tps: f64,
    pub cpu_percent: f32,
    pub ram_mb: f64,
    pub grid_size: u32,
}

pub struct MetricsCollector {
    start_time: Instant,
    last_frame_time: Instant,
    frame_times: VecDeque<Duration>,
    tick_times: VecDeque<Duration>,
    system: Arc<Mutex<System>>,
    history: Vec<FrameMetrics>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        let system = System::new_all();
        
        MetricsCollector {
            start_time: Instant::now(),
            last_frame_time: Instant::now(),
            frame_times: VecDeque::with_capacity(120),
            tick_times: VecDeque::with_capacity(120),
            system: Arc::new(Mutex::new(system)),
            history: Vec::new(),
        }
    }

    pub fn record_frame(&mut self) {
        let now = Instant::now();
        let frame_duration = now.duration_since(self.last_frame_time);
        
        self.frame_times.push_back(frame_duration);
        if self.frame_times.len() > 120 {
            self.frame_times.pop_front();
        }
        
        self.last_frame_time = now;
    }

    pub fn record_tick(&mut self, duration: Duration) {
        self.tick_times.push_back(duration);
        if self.tick_times.len() > 120 {
            self.tick_times.pop_front();
        }
    }

    pub fn get_fps(&self) -> f64 {
        if self.frame_times.is_empty() {
            return 0.0;
        }
        
        let sum: Duration = self.frame_times.iter().sum();
        let avg_frame_time = sum / self.frame_times.len() as u32;
        
        if avg_frame_time.as_secs_f64() > 0.0 {
            1.0 / avg_frame_time.as_secs_f64()
        } else {
            0.0
        }
    }

    pub fn get_tps(&self) -> f64 {
        if self.tick_times.is_empty() {
            return 0.0;
        }
        
        let sum: Duration = self.tick_times.iter().sum();
        let avg_tick_time = sum / self.tick_times.len() as u32;
        
        if avg_tick_time.as_secs_f64() > 0.0 {
            1.0 / avg_tick_time.as_secs_f64()
        } else {
            0.0
        }
    }

    pub fn get_cpu_usage(&self) -> f32 {
        if let Ok(mut system) = self.system.lock() {
            system.refresh_all();
            if let Some(pid) = sysinfo::get_current_pid().ok() {
                if let Some(process) = system.process(pid) {
                    return process.cpu_usage();
                }
            }
        }
        0.0
    }

    pub fn get_ram_usage_mb(&self) -> f64 {
        if let Ok(mut system) = self.system.lock() {
            system.refresh_memory();
            (system.used_memory() as f64) / 1024.0 / 1024.0
        } else {
            0.0
        }
    }

    pub fn get_avg_frame_time_ms(&self) -> f64 {
        if self.frame_times.is_empty() {
            return 0.0;
        }
        
        let sum: Duration = self.frame_times.iter().sum();
        sum.as_secs_f64() * 1000.0 / self.frame_times.len() as f64
    }

    pub fn get_avg_tick_time_ms(&self) -> f64 {
        if self.tick_times.is_empty() {
            return 0.0;
        }
        
        let sum: Duration = self.tick_times.iter().sum();
        sum.as_secs_f64() * 1000.0 / self.tick_times.len() as f64
    }

    pub fn record_metrics(&mut self, grid_size: u32) {
        let metrics = FrameMetrics {
            timestamp: Instant::now(),
            fps: self.get_fps(),
            tps: self.get_tps(),
            cpu_percent: self.get_cpu_usage(),
            ram_mb: self.get_ram_usage_mb(),
            grid_size,
        };
        
        self.history.push(metrics);
        
        // Keep last 1000 samples
        if self.history.len() > 1000 {
            self.history.remove(0);
        }
    }

    pub fn log_summary(&self, tick_count: u64) {
        let uptime = self.start_time.elapsed();
        
        info!("=== PERFORMANCE SUMMARY ===");
        info!("Uptime: {:.2}s", uptime.as_secs_f64());
        info!("Total Ticks: {}", tick_count);
        info!("FPS: {:.2}", self.get_fps());
        info!("TPS: {:.2}", self.get_tps());
        info!("Avg Frame Time: {:.2}ms", self.get_avg_frame_time_ms());
        info!("Avg Tick Time: {:.2}ms", self.get_avg_tick_time_ms());
        info!("CPU Usage: {:.1}%", self.get_cpu_usage());
        info!("RAM Usage: {:.1}MB", self.get_ram_usage_mb());
    }

    pub fn get_status_string(&self, tick_count: u64, is_running: bool) -> String {
        format!(
            "Tick: {} | FPS: {:.1} | TPS: {:.1} | CPU: {:.1}% | RAM: {:.0}MB | Avg Frame: {:.2}ms | Status: {}",
            tick_count,
            self.get_fps(),
            self.get_tps(),
            self.get_cpu_usage(),
            self.get_ram_usage_mb(),
            self.get_avg_frame_time_ms(),
            if is_running { "Running" } else { "Paused" }
        )
    }
}
