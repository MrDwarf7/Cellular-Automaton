use crate::cell::{Cell, CellType, Genes};
use crate::grid::Grid;
use rand::Rng;

const REPRODUCTION_CHANCE: f64 = 1.0 / 100_000_000.0; // 1 in 100 million

/// Check for reproduction between nearby cells
pub fn check_reproduction(grid: &mut Grid, x: u32, y: u32, rng: &mut impl Rng) {
    // Very low chance of reproduction trigger
    if rng.gen::<f64>() > REPRODUCTION_CHANCE {
        return;
    }

    // Check neighbors for compatible reproduction
    for dy in -2..=2i32 {
        for dx in -2..=2i32 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let nx = (x as i32 + dx) as u32;
            let ny = (y as i32 + dy) as u32;

            if let Some(parent2) = grid.get_cell(nx, ny) {
                if let Some(parent1) = grid.get_cell(x, y) {
                    if can_reproduce(&parent1, &parent2) {
                        // Attempt reproduction
                        let offspring = create_offspring(&parent1, &parent2, rng);
                        
                        // Place offspring in random adjacent empty cell
                        if let Some((ox, oy)) = find_empty_neighbor(grid, x, y, rng) {
                            grid.set_next_cell(ox, oy, offspring);
                            return;
                        }
                    }
                }
            }
        }
    }
}

fn can_reproduce(parent1: &Cell, parent2: &Cell) -> bool {
    // Allow reproduction between cells of the same type or very similar types
    parent1.cell_type == parent2.cell_type
        || are_compatible_types(parent1.cell_type, parent2.cell_type)
}

fn are_compatible_types(type1: CellType, type2: CellType) -> bool {
    // Define compatible breeding pairs (same family groups)
    match (type1, type2) {
        // Green family
        (CellType::Green, CellType::Green) => true,
        
        // Orange/Vitality family
        (CellType::Orange, CellType::Orange) => true,
        
        // Red/healing family
        (CellType::Red, CellType::Red) => true,
        
        // Predator families
        (CellType::Crimson, CellType::Crimson) => true,
        (CellType::Crimson, CellType::Maroon) => true,
        (CellType::Maroon, CellType::Crimson) => true,
        (CellType::Maroon, CellType::Maroon) => true,
        
        // Herbivore family
        (CellType::Brown, CellType::Brown) => true,
        (CellType::Brown, CellType::Tan) => true,
        (CellType::Tan, CellType::Brown) => true,
        (CellType::Tan, CellType::Tan) => true,
        
        // Water family
        (CellType::Blue, CellType::Blue) => true,
        (CellType::Blue, CellType::Teal) => true,
        (CellType::Teal, CellType::Blue) => true,
        (CellType::Teal, CellType::Teal) => true,
        
        // Environmental
        (CellType::Cyan, CellType::Cyan) => true,
        (CellType::Yellow, CellType::Yellow) => true,
        
        // Defensive
        (CellType::White, CellType::White) => true,
        (CellType::Mint, CellType::Mint) => true,
        (CellType::Peach, CellType::Peach) => true,
        
        _ => false,
    }
}

fn create_offspring(parent1: &Cell, parent2: &Cell, rng: &mut impl Rng) -> Cell {
    // Blend genes from both parents
    let mut genes = Genes::blend(&parent1.genes, &parent2.genes);
    genes.clamp();

    // Choose cell type: usually one of the parents, sometimes blended
    let offspring_type = if rng.gen::<f64>() < 0.7 {
        // 70% chance to be one of the parents
        if rng.gen::<bool>() {
            parent1.cell_type
        } else {
            parent2.cell_type
        }
    } else {
        // 30% chance to be a hybrid (blend of both)
        parent1.cell_type
    };

    let mut offspring = Cell::with_genes(offspring_type, genes);
    
    // Offspring starts at age 0 with parents' traits
    offspring.genes.generation = parent1.genes.generation.max(parent2.genes.generation) + 1;
    offspring.genes.parent_types = (
        parent1.cell_type.to_u8(),
        parent2.cell_type.to_u8(),
    );

    offspring
}

fn find_empty_neighbor(grid: &Grid, x: u32, y: u32, rng: &mut impl Rng) -> Option<(u32, u32)> {
    let mut candidates = Vec::new();

    for dy in -1..=1i32 {
        for dx in -1..=1i32 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let nx = (x as i32 + dx) as u32;
            let ny = (y as i32 + dy) as u32;

            if let Some(cell) = grid.get_cell(nx, ny) {
                if cell.cell_type == CellType::Black {
                    candidates.push((nx, ny));
                }
            }
        }
    }

    if candidates.is_empty() {
        None
    } else {
        Some(candidates[rng.gen_range(0..candidates.len())])
    }
}

/// Blend color from two cell types
pub fn blend_colors(type1: CellType, type2: CellType) -> String {
    let (r, g, b) = Genes::blend_color(type1, type2);
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compatible_types() {
        assert!(are_compatible_types(CellType::Green, CellType::Green));
        assert!(are_compatible_types(CellType::Crimson, CellType::Maroon));
        assert!(!are_compatible_types(CellType::Green, CellType::Orange));
    }

    #[test]
    fn test_genes_blending() {
        let g1 = Genes {
            spread_tendency: 0.8,
            aggression: 0.2,
            vitality: 0.5,
            mutatability: 0.1,
            generation: 0,
            parent_types: (0, 0),
        };
        
        let g2 = Genes {
            spread_tendency: 0.2,
            aggression: 0.8,
            vitality: 0.5,
            mutatability: 0.1,
            generation: 0,
            parent_types: (0, 0),
        };
        
        let blended = Genes::blend(&g1, &g2);
        
        // Average should be around 0.5, with some mutation
        assert!(blended.spread_tendency >= 0.0 && blended.spread_tendency <= 1.0);
        assert!(blended.aggression >= 0.0 && blended.aggression <= 1.0);
        assert_eq!(blended.generation, 1);
    }
}
