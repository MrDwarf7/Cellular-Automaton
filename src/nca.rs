/// Neural Cellular Automaton (NCA) Layer
/// 
/// Predicts the next cell state based on:
/// - Current cell type and genes
/// - Local neighborhood state
/// - Rule parameters from CNN
/// - Stochastic outputs for diversity

use crate::cell::{Cell, CellType};
use crate::ml_layer::{RegionRuleParams, LocalRuleParams, GlobalRuleParams};
use rand::Rng;

/// Embedding for a cell in the NCA input
#[derive(Debug, Clone)]
pub struct CellEmbedding {
    /// One-hot encoding of cell type (37 dims)
    pub cell_type_encoding: Vec<f32>,
    
    /// Neighborhood cell types (8×37 = 296 dims flattened)
    pub neighborhood_encoding: Vec<f32>,
    
    /// Cell's genetic traits (4 dims: spread, aggression, vitality, mutatability)
    pub genetic_traits: [f32; 4],
    
    /// Local cell type density (10 dims: counts of major types)
    pub local_density: [f32; 10],
}

/// Output prediction from NCA for a single cell
#[derive(Debug, Clone)]
pub struct NCAPrediction {
    /// Predicted next cell type (37 logits, then argmax)
    pub next_cell_logits: Vec<f32>,
    
    /// Changes to genetic traits
    pub trait_deltas: [f32; 4],
    
    /// Alternative mutation options (indices of top-K cell types)
    pub mutation_alternatives: Vec<(u8, f32)>, // (cell_type_idx, probability)
    
    /// Probability to use main prediction vs alternative (0.0-1.0)
    pub stochastic_confidence: f32,
}

/// Main NCA interface
pub trait CellularAutomaton {
    /// Predict next state for a single cell
    fn predict(
        &self,
        embedding: &CellEmbedding,
        region_params: &RegionRuleParams,
        local_params: &LocalRuleParams,
        global_params: &GlobalRuleParams,
    ) -> NCAPrediction;
}

/// Stub NCA: Uses rule-based heuristics
/// Respects rule parameters but doesn't learn
pub struct StubNCA;

impl CellularAutomaton for StubNCA {
    fn predict(
        &self,
        embedding: &CellEmbedding,
        region_params: &RegionRuleParams,
        local_params: &LocalRuleParams,
        global_params: &GlobalRuleParams,
    ) -> NCAPrediction {
        let current_type_idx = embedding.cell_type_encoding.iter()
            .position(|&x| x > 0.5)
            .unwrap_or(0);
        
        let current_type = CellType::from_u8(current_type_idx as u8).unwrap_or(CellType::Black);
        
        // Rule-based prediction respecting parameters
        let next_cell_logits = predict_next_type(
            current_type,
            &embedding.neighborhood_encoding,
            region_params,
            local_params,
            global_params,
        );
        
        let trait_deltas = predict_trait_changes(
            current_type,
            &embedding.genetic_traits,
            region_params,
        );
        
        let mutation_alternatives = get_mutation_alternatives(
            current_type,
            &next_cell_logits,
            region_params,
        );
        
        let stochastic_confidence = get_confidence(
            &next_cell_logits,
            global_params.chaos_level,
        );
        
        NCAPrediction {
            next_cell_logits,
            trait_deltas,
            mutation_alternatives,
            stochastic_confidence,
        }
    }
}

/// Predict next cell type based on rules and parameters
fn predict_next_type(
    current_type: CellType,
    _neighborhood: &[f32],
    region_params: &RegionRuleParams,
    _local_params: &LocalRuleParams,
    global_params: &GlobalRuleParams,
) -> Vec<f32> {
    let mut logits = vec![0.0f32; 37];
    
    // Set baseline logits for current type (persistence)
    logits[current_type.to_u8() as usize] = 1.0;
    
    match current_type {
        CellType::Green => {
            // Green persists or spreads based on spread_modifier
            logits[CellType::Green.to_u8() as usize] += region_params.spread_modifier * 0.5;
        },
        
        CellType::Orange => {
            // Orange responds to ecosystem health
            if region_params.ecosystem_health > 0.5 {
                logits[CellType::Orange.to_u8() as usize] += 0.5;
            } else if region_params.ecosystem_health < -0.5 {
                logits[CellType::Gray.to_u8() as usize] += 0.5;
            }
        },
        
        CellType::Purple => {
            // Purple spreads based on infection_rate
            logits[CellType::Purple.to_u8() as usize] += region_params.infection_rate * 0.8;
            // Can be healed by red/other factors
            logits[CellType::Black.to_u8() as usize] += (1.0 - region_params.infection_rate) * 0.3;
        },
        
        CellType::Gray => {
            // Gray decays if ecosystem health is bad
            if region_params.ecosystem_health < 0.0 {
                logits[CellType::Black.to_u8() as usize] += 0.5;
            } else {
                logits[CellType::Orange.to_u8() as usize] += 0.3;
            }
        },
        
        _ => {
            // Other types: apply basic persistence
            logits[current_type.to_u8() as usize] += 0.3;
        },
    }
    
    // Temperature influences growth vs decay
    if global_params.temperature > 0.5 {
        // Hot: favor growth types
        logits[CellType::Green.to_u8() as usize] += 0.2;
        logits[CellType::Orange.to_u8() as usize] += 0.1;
    } else if global_params.temperature < -0.5 {
        // Cold: favor consolidation/decay
        logits[CellType::Black.to_u8() as usize] += 0.3;
        logits[CellType::Gray.to_u8() as usize] += 0.2;
    }
    
    // Apply diversity pressure
    if region_params.diversity_pressure > 0.7 {
        // Spread logits more evenly (avoid monoculture)
        for logit in &mut logits {
            *logit *= 0.9;
            *logit += 0.1; // Small baseline for all types
        }
    }
    
    // Ensure all logits are non-negative
    for logit in &mut logits {
        *logit = logit.max(0.0);
    }
    
    logits
}

/// Predict how genetic traits change in offspring
fn predict_trait_changes(
    _current_type: CellType,
    _parent_traits: &[f32; 4],
    region_params: &RegionRuleParams,
) -> [f32; 4] {
    let mut deltas = [0.0f32; 4]; // [spread, aggression, vitality, mutatability]
    
    // Ecosystem health biases trait evolution
    if region_params.ecosystem_health > 0.5 {
        // Good conditions: favor resilience
        deltas[2] = 0.05; // vitality increases
        deltas[3] -= 0.02; // mutatability decreases (stable)
    } else if region_params.ecosystem_health < -0.5 {
        // Harsh conditions: favor adaptability
        deltas[3] = 0.05; // mutatability increases
        deltas[2] -= 0.03; // vitality decreases (high-risk)
    }
    
    // Predation pressure influences aggression genes
    if region_params.predation_pressure > 0.8 {
        deltas[1] = 0.03; // aggression increases
    }
    
    // Mutation rate influences mutatability
    deltas[3] = (region_params.mutation_rate * 0.1 - 0.05).clamp(-0.1, 0.1);
    
    deltas
}

/// Get alternative cell types that could spawn via mutation
fn get_mutation_alternatives(
    current_type: CellType,
    logits: &[f32],
    region_params: &RegionRuleParams,
) -> Vec<(u8, f32)> {
    let mut alternatives = Vec::new();
    
    // If mutation rate is high, offer diverse alternatives
    if region_params.mutation_rate > 0.6 {
        // Find top-3 cell types by logits
        let mut indexed: Vec<(usize, f32)> = logits.iter()
            .enumerate()
            .map(|(i, &l)| (i, l))
            .collect();
        
        indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        for (idx, logit) in indexed.iter().take(3) {
            if *idx != current_type.to_u8() as usize {
                alternatives.push((*idx as u8, logit.max(0.1)));
            }
        }
    } else {
        // Conservative: only offer one close alternative
        let alt_idx = (current_type.to_u8() as usize + 1) % 37;
        alternatives.push((alt_idx as u8, 0.3));
    }
    
    alternatives
}

/// Get confidence in the main prediction (stochasticity)
fn get_confidence(
    logits: &[f32],
    chaos_level: f32,
) -> f32 {
    // Find max logit
    let max_logit = logits.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    
    // Normalize as a probability
    let sum: f32 = logits.iter().sum();
    let confidence = if sum > 0.0 {
        max_logit / sum
    } else {
        0.5
    };
    
    // Chaos reduces confidence (makes predictions more stochastic)
    let adjusted = (confidence - chaos_level * 0.3).clamp(0.0, 1.0);
    adjusted
}

/// Apply NCA prediction to a cell
pub fn apply_nca_prediction(
    cell: &Cell,
    prediction: &NCAPrediction,
    rng: &mut impl Rng,
    _region_params: &RegionRuleParams,
) -> Cell {
    use rand::distributions::WeightedIndex;
    use rand::distributions::Distribution;
    
    // Decide: use main prediction or alternative?
    let use_main = rng.gen::<f32>() < prediction.stochastic_confidence;
    
    let next_type = if use_main {
        // Argmax the logits
        let next_idx = prediction.next_cell_logits.iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(idx, _)| idx)
            .unwrap_or(0);
        
        CellType::from_u8(next_idx as u8).unwrap_or(CellType::Black)
    } else {
        // Pick random alternative
        if !prediction.mutation_alternatives.is_empty() {
            let weights: Vec<f32> = prediction.mutation_alternatives.iter()
                .map(|(_, p)| *p)
                .collect();
            
            if let Ok(dist) = WeightedIndex::new(&weights) {
                let alt_idx = dist.sample(rng);
                let (cell_type_u8, _) = prediction.mutation_alternatives[alt_idx];
                CellType::from_u8(cell_type_u8).unwrap_or(CellType::Black)
            } else {
                CellType::Black
            }
        } else {
            CellType::Black
        }
    };
    
    // Apply genetic trait evolution
    let mut new_genes = cell.genes;
    new_genes.spread_tendency = (new_genes.spread_tendency + prediction.trait_deltas[0] as f64)
        .clamp(0.0, 1.0);
    new_genes.aggression = (new_genes.aggression + prediction.trait_deltas[1] as f64)
        .clamp(0.0, 1.0);
    new_genes.vitality = (new_genes.vitality + prediction.trait_deltas[2] as f64)
        .clamp(0.0, 1.0);
    new_genes.mutatability = (new_genes.mutatability + prediction.trait_deltas[3] as f64)
        .clamp(0.0, 1.0);
    
    Cell::with_genes(next_type, new_genes)
}

/// Create embedding for a cell (used as NCA input)
pub fn create_embedding(
    cell: &Cell,
    neighborhood: &[Cell],
    _rng: &mut impl Rng,
) -> CellEmbedding {
    // Cell type one-hot encoding
    let mut cell_type_encoding = vec![0.0f32; 37];
    cell_type_encoding[cell.cell_type.to_u8() as usize] = 1.0;
    
    // Neighborhood encoding (8 neighbors × 37 types = 296 dims)
    let mut neighborhood_encoding = vec![0.0f32; 296];
    for (i, neighbor) in neighborhood.iter().enumerate().take(8) {
        let base_idx = i * 37;
        neighborhood_encoding[base_idx + neighbor.cell_type.to_u8() as usize] = 1.0;
    }
    
    // Genetic traits
    let genetic_traits = [
        cell.genes.spread_tendency as f32,
        cell.genes.aggression as f32,
        cell.genes.vitality as f32,
        cell.genes.mutatability as f32,
    ];
    
    // Local density (simplified: just count major types)
    let mut local_density = [0.0f32; 10];
    let major_types = [
        CellType::Green,
        CellType::Orange,
        CellType::Purple,
        CellType::Red,
        CellType::White,
        CellType::Blue,
        CellType::Gray,
        CellType::Black,
        CellType::Brown,
        CellType::Cyan,
    ];
    
    for (i, &cell_type) in major_types.iter().enumerate() {
        let count = neighborhood.iter()
            .filter(|n| n.cell_type == cell_type)
            .count() as f32;
        local_density[i] = count / 8.0;
    }
    
    CellEmbedding {
        cell_type_encoding,
        neighborhood_encoding,
        genetic_traits,
        local_density,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_stub_nca() {
        let nca = StubNCA;
        
        let mut embedding = CellEmbedding {
            cell_type_encoding: vec![0.0; 37],
            neighborhood_encoding: vec![0.0; 296],
            genetic_traits: [0.5; 4],
            local_density: [0.1; 10],
        };
        embedding.cell_type_encoding[1] = 1.0; // Green
        
        let region_params = RegionRuleParams::default();
        let local_params = LocalRuleParams::default();
        let global_params = GlobalRuleParams::default();
        
        let prediction = nca.predict(&embedding, &region_params, &local_params, &global_params);
        
        assert_eq!(prediction.next_cell_logits.len(), 37);
        assert_eq!(prediction.trait_deltas.len(), 4);
    }
}
