#![allow(dead_code)]

pub mod cell;
pub mod genetics;
pub mod grid;
pub mod logging;
pub mod metrics;
pub mod ml_layer;
pub mod nca;
pub mod presets;
pub mod rules;
pub mod stats;

pub use cell::{Cell, CellType, Genes};
pub use genetics::check_reproduction;
pub use grid::Grid;
pub use presets::{load_preset, PresetT};
pub use rules::apply_rules;
pub use stats::{calculate_stats, get_ecosystem_status};

pub struct Simulator {
    pub grid: Grid,
    pub tick_count: u64,
}

impl Simulator {
    pub fn new(width: u32, height: u32) -> Self {
        let w = if width == 0 { 1200 } else { width };
        let h = if height == 0 { 1200 } else { height };
        Simulator {
            grid: Grid::new(w, h),
            tick_count: 0,
        }
    }

    // pub fn initialize_random(&mut self, densities: &serde_json::Map<String, serde_json::Value>) {
    //     self.grid.initialize_random(densities);
    // }

    pub fn tick(&mut self) {
        apply_rules(&mut self.grid);
        self.tick_count += 1;
    }

    pub fn get_grid_data(&self) -> Vec<u8> {
        self.grid.to_bytes()
    }

    pub fn get_grid_json(&self) -> String {
        self.grid.to_json()
    }

    pub fn get_population_counts(&self) -> String {
        self.grid.get_population_counts()
    }

    pub fn get_tick_count(&self) -> u64 {
        self.tick_count
    }

    pub fn reset(&mut self) {
        self.grid = Grid::new(self.grid.width, self.grid.height);
        self.tick_count = 0;
    }

    pub fn width(&self) -> u32 {
        self.grid.width
    }

    pub fn height(&self) -> u32 {
        self.grid.height
    }

    pub fn get_cell(&self, x: u32, y: u32) -> Option<u8> {
        self.grid.get_cell(x, y).map(|c| c.to_u8())
    }

    pub fn set_cell(&mut self, x: u32, y: u32, cell_type: u8) {
        if let Some(ct) = cell::CellType::from_u8(cell_type) {
            self.grid.set_cell(x, y, ct);
        }
    }

    pub fn get_ecosystem_stats(&self) -> String {
        let stats = stats::calculate_stats(&self.grid);
        serde_json::to_string(&serde_json::json!({
            "health_score": stats.health_score,
            "status": stats::get_ecosystem_status(&stats),
            "green_coverage": stats.green_coverage,
            "orange_population": stats.orange_population,
            "predator_count": stats.predator_count,
            "disease_pressure": stats.disease_pressure,
            "diversity_index": stats.diversity_index,
            "stability": stats.stability,
        }))
        .unwrap_or_default()
    }

    pub fn load_preset(&mut self, preset_name: &str) -> bool {
        if let presets::Preset::RandomFallback = presets::Preset::from(preset_name) {
            return false;
        }
        return true;
    }

    pub fn list_presets() -> Vec<String> {
        vec![
            "balanced".to_string(),
            "dense_forest".to_string(),
            "plague_outbreak".to_string(),
            "predator_heavy".to_string(),
            "scarce_resources".to_string(),
            "recovery".to_string(),
        ]
    }
}
