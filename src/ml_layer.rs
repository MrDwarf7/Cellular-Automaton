/// ML Layer: CNN-based Rule Parameter Generator
/// 
/// This module handles inference from a trained CNN to generate dynamic rule parameters
/// that adapt to the current grid state. Parameters can come from:
/// - Trained neural network (ONNX, TensorFlow, PyTorch)
/// - Stubs/heuristics during development
/// - Hybrid approach (NN-guided + hand-crafted rules)

use serde::{Serialize, Deserialize};
use crate::cell::CellType;

/// Region-level rule parameters
/// Applied per NxN chunk of the grid
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RegionRuleParams {
    /// Multiplier for spread rates (0.5 = slower, 1.5 = faster)
    pub spread_modifier: f32,
    
    /// How aggressive disease spreads (0.1 = weak, 1.0 = full strength)
    pub infection_rate: f32,
    
    /// How aggressive predators hunt (0.1 = weak, 1.0 = full strength)
    pub predation_pressure: f32,
    
    /// Overall ecosystem health bias (-1.0 = decay, 0.0 = neutral, 1.0 = growth)
    pub ecosystem_health: f32,
    
    /// Likelihood of novel mutations (0.0 = none, 1.0 = high)
    pub mutation_rate: f32,
    
    /// Pressure toward diverse cell types (0.0 = indifferent, 1.0 = strong)
    pub diversity_pressure: f32,
    
    /// Resource abundance (0.5 = scarce, 1.5 = abundant)
    pub resource_abundance: f32,
    
    /// Stochastic chaos level (0.0 = deterministic, 1.0 = chaotic)
    pub chaos_level: f32,
}

/// Local cell-level parameters
/// Can vary per-cell or per small neighborhood
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LocalRuleParams {
    /// Boost to this cell's vitality/survival (0.0-1.0)
    pub vitality_boost: f32,
    
    /// Chance this cell reproduces (0.0-0.2)
    pub reproduction_chance: f32,
    
    /// How much genetic traits mutate in offspring (0.0-0.1)
    pub trait_mutation_rate: f32,
}

/// Global environmental modifiers
/// Affects entire grid uniformly
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GlobalRuleParams {
    /// Environmental temperature bias (-1.0 = cold/decay, 1.0 = hot/growth)
    pub temperature: f32,
    
    /// How much chaos/randomness in outcomes (0.0-1.0)
    pub chaos_level: f32,
    
    /// How quickly cells starve (0.5 = lenient, 2.0 = harsh)
    pub starvation_pressure: f32,
    
    /// Overall time scale (0.5 = slow, 2.0 = fast)
    pub simulation_speed: f32,
}

impl Default for RegionRuleParams {
    fn default() -> Self {
        RegionRuleParams {
            spread_modifier: 1.0,
            infection_rate: 1.0,
            predation_pressure: 1.0,
            ecosystem_health: 0.0,
            mutation_rate: 0.5,
            diversity_pressure: 0.5,
            resource_abundance: 1.0,
            chaos_level: 0.3,
        }
    }
}

impl Default for LocalRuleParams {
    fn default() -> Self {
        LocalRuleParams {
            vitality_boost: 0.0,
            reproduction_chance: 0.05,
            trait_mutation_rate: 0.02,
        }
    }
}

impl Default for GlobalRuleParams {
    fn default() -> Self {
        GlobalRuleParams {
            temperature: 0.0,
            chaos_level: 0.3,
            starvation_pressure: 1.0,
            simulation_speed: 1.0,
        }
    }
}

/// Main ML interface for rule generation
pub trait RuleGenerator {
    /// Given the current grid state, generate rule parameters
    /// 
    /// Input: grid cells encoded as u8 (37 cell types)
    /// Returns: (region_params, local_params, global_params)
    fn generate_rules(
        &self,
        grid: &[u8],
        width: u32,
        height: u32,
        region_size: u32,
    ) -> (
        Vec<RegionRuleParams>,  // For each region
        Vec<LocalRuleParams>,   // For each cell
        GlobalRuleParams,       // Global modifiers
    );
}

/// Stub implementation: Returns sensible defaults
/// Use this while training the real neural network
pub struct StubRuleGenerator;

impl RuleGenerator for StubRuleGenerator {
    fn generate_rules(
        &self,
        grid: &[u8],
        width: u32,
        height: u32,
        region_size: u32,
    ) -> (
        Vec<RegionRuleParams>,
        Vec<LocalRuleParams>,
        GlobalRuleParams,
    ) {
        let num_regions = (
            (width + region_size - 1) / region_size * 
            (height + region_size - 1) / region_size
        ) as usize;
        
        let num_cells = (width * height) as usize;
        
        // Simple heuristic: analyze grid to adjust parameters
        let (green_density, purple_density, orange_density) = 
            analyze_grid(grid, width, height);
        
        // Adapt rules based on ecosystem state
        let region_params = (0..num_regions)
            .map(|_| {
                let mut params = RegionRuleParams::default();
                
                // If disease is high, boost healing/immunity
                if purple_density > 0.3 {
                    params.infection_rate = 0.7;
                    params.ecosystem_health = 0.2;
                }
                
                // If vegetation is low, boost growth
                if green_density < 0.2 {
                    params.spread_modifier = 1.3;
                    params.ecosystem_health = 0.5;
                }
                
                // If orange is dying, increase diversity pressure
                if orange_density < 0.1 {
                    params.diversity_pressure = 0.8;
                    params.mutation_rate = 0.7;
                }
                
                params
            })
            .collect();
        
        let local_params = (0..num_cells)
            .map(|i| {
                let cell_type = grid[i];
                let mut params = LocalRuleParams::default();
                
                // Herbal cells get vitality boost
                if cell_type == CellType::Green.to_u8() {
                    params.vitality_boost = 0.2;
                }
                
                // Orange cells get reproduction chance
                if cell_type == CellType::Orange.to_u8() {
                    params.reproduction_chance = 0.08;
                }
                
                params
            })
            .collect();
        
        let global_params = GlobalRuleParams {
            temperature: ((green_density - 0.3) * 0.5).clamp(-1.0, 1.0) as f32,
            chaos_level: purple_density as f32 * 0.5,
            starvation_pressure: (1.0 - green_density * 0.5) as f32,
            simulation_speed: 1.0,
        };
        
        (region_params, local_params, global_params)
    }
}

/// Analyze grid densities for heuristic adaptation
fn analyze_grid(
    grid: &[u8],
    _width: u32,
    _height: u32,
) -> (f64, f64, f64) {
    let total = grid.len() as f64;
    
    let green_count = grid.iter()
        .filter(|&&cell| cell == CellType::Green.to_u8())
        .count() as f64;
    
    let purple_count = grid.iter()
        .filter(|&&cell| cell == CellType::Purple.to_u8())
        .count() as f64;
    
    let orange_count = grid.iter()
        .filter(|&&cell| cell == CellType::Orange.to_u8())
        .count() as f64;
    
    (green_count / total, purple_count / total, orange_count / total)
}

/// Helper to get region params for a specific cell
pub fn get_region_params(
    region_params: &[RegionRuleParams],
    x: u32,
    y: u32,
    width: u32,
    region_size: u32,
) -> RegionRuleParams {
    let region_x = (x / region_size) as usize;
    let region_y = (y / region_size) as usize;
    let regions_per_row = ((width + region_size - 1) / region_size) as usize;
    let idx = region_y * regions_per_row + region_x;
    
    region_params
        .get(idx)
        .copied()
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_stub_generator() {
        let grid = vec![1u8; 100 * 100]; // All green
        let generator = StubRuleGenerator;
        
        let (regions, locals, global) = 
            generator.generate_rules(&grid, 100, 100, 32);
        
        assert!(!regions.is_empty());
        assert_eq!(locals.len(), 10000);
        assert!(global.temperature > 0.0); // High green → warm
    }
}
