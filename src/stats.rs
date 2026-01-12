use crate::cell::CellType;
use crate::grid::Grid;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct EcosystemStats {
    pub populations: HashMap<String, u32>,
    pub health_score: f64,
    pub green_coverage: f64,
    pub orange_population: u32,
    pub predator_count: u32,
    pub disease_pressure: f64,
    pub diversity_index: f64,
    pub stability: f64,
}

pub fn calculate_stats(grid: &Grid) -> EcosystemStats {
    let mut populations: HashMap<String, u32> = HashMap::new();
    let total_cells = (grid.width * grid.height) as f64;
    
    for y in 0..grid.height {
        for x in 0..grid.width {
            if let Some(cell) = grid.get_cell(x, y) {
                let name = get_cell_name(cell.cell_type);
                *populations.entry(name).or_insert(0) += 1;
            }
        }
    }
    
    // Calculate health metrics
    let green = *populations.get("Green").unwrap_or(&0) as f64;
    let orange = *populations.get("Orange").unwrap_or(&0) as f64;
    let _gray = *populations.get("Gray").unwrap_or(&0) as f64;
    let purple = *populations.get("Purple").unwrap_or(&0) as f64;
    let red = *populations.get("Red").unwrap_or(&0) as f64;
    let _white = *populations.get("White").unwrap_or(&0) as f64;
    let _black = *populations.get("Black").unwrap_or(&0) as f64;
    
    // Green coverage percentage
    let green_coverage = (green / total_cells) * 100.0;
    
    // Predator count (Crimson, Maroon, Coral, Brown, Tan)
    let crimson = *populations.get("Crimson").unwrap_or(&0);
    let maroon = *populations.get("Maroon").unwrap_or(&0);
    let coral = *populations.get("Coral").unwrap_or(&0);
    let brown = *populations.get("Brown").unwrap_or(&0);
    let tan = *populations.get("Tan").unwrap_or(&0);
    let predator_count = crimson + maroon + coral + brown + tan;
    
    // Disease pressure (Purple / (Purple + Red + Orange))
    let disease_pressure = if purple + red + orange > 0.0 {
        (purple / (purple + red + orange)).min(1.0)
    } else {
        0.0
    };
    
    // Stability: measure of ecosystem not collapsing
    // Low if green or orange are very low
    let stability = if orange > 0.0 && green > 0.0 {
        ((green / total_cells) * (orange / total_cells) * 100.0).min(1.0)
    } else {
        0.0
    };
    
    // Diversity: Shannon index across top 10 populations
    let mut diversity = 0.0;
    let mut pop_list: Vec<_> = populations.iter().collect();
    pop_list.sort_by_key(|(_, count)| std::cmp::Reverse(**count));
    
    for (_, count) in pop_list.iter().take(10) {
        let p = **count as f64 / total_cells;
        if p > 0.0 {
            diversity -= p * p.log2();
        }
    }
    
    // Normalize diversity (max is log2(37))
    diversity = (diversity / 5.0).min(1.0);
    
    // Overall health score
    // Green coverage (0-50 optimal at 30-50%) + Orange (0-30) + Stability (0-20)
    let green_score = if green_coverage >= 20.0 && green_coverage <= 60.0 {
        50.0
    } else if green_coverage < 20.0 {
        green_coverage * 2.5
    } else {
        50.0 - ((green_coverage - 60.0) * 0.5)
    };
    
    let orange_score = (orange / total_cells) * 30.0 * 100.0;
    let stability_score = stability * 20.0;
    let health_score = (green_score + orange_score.min(30.0) + stability_score) / 100.0;
    
    EcosystemStats {
        populations,
        health_score: health_score.min(1.0).max(0.0),
        green_coverage,
        orange_population: orange as u32,
        predator_count,
        disease_pressure,
        diversity_index: diversity,
        stability,
    }
}

pub fn get_ecosystem_status(stats: &EcosystemStats) -> String {
    if stats.health_score > 0.7 {
        "Thriving".to_string()
    } else if stats.health_score > 0.5 {
        "Healthy".to_string()
    } else if stats.health_score > 0.3 {
        "Struggling".to_string()
    } else if stats.health_score > 0.1 {
        "Critical".to_string()
    } else {
        "Collapsed".to_string()
    }
}

fn get_cell_name(cell_type: CellType) -> String {
    match cell_type {
        CellType::Black => "Black",
        CellType::Green => "Green",
        CellType::Orange => "Orange",
        CellType::Gray => "Gray",
        CellType::Purple => "Purple",
        CellType::Red => "Red",
        CellType::White => "White",
        CellType::Blue => "Blue",
        CellType::Brown => "Brown",
        CellType::Tan => "Tan",
        CellType::Gold => "Gold",
        CellType::Lime => "Lime",
        CellType::Crimson => "Crimson",
        CellType::Maroon => "Maroon",
        CellType::Coral => "Coral",
        CellType::Pink => "Pink",
        CellType::Magenta => "Magenta",
        CellType::Cyan => "Cyan",
        CellType::Yellow => "Yellow",
        CellType::Teal => "Teal",
        CellType::Navy => "Navy",
        CellType::Olive => "Olive",
        CellType::Indigo => "Indigo",
        CellType::Khaki => "Khaki",
        CellType::Slate => "Slate",
        CellType::Rust => "Rust",
        CellType::Mint => "Mint",
        CellType::Peach => "Peach",
        CellType::Aqua => "Aqua",
        CellType::Silver => "Silver",
        CellType::Violet => "Violet",
        CellType::Amber => "Amber",
        CellType::Pearl => "Pearl",
        CellType::Smoke => "Smoke",
        CellType::Glint => "Glint",
        CellType::Tint => "Tint",
        CellType::Shade => "Shade",
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stats_calculation() {
        let grid = Grid::new(100, 100);
        let stats = calculate_stats(&grid);
        assert_eq!(stats.populations.get("Black").unwrap(), &10000);
    }
}
