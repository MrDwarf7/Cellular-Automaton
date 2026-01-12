use crate::cell::{Cell, CellType};
use crate::grid::{Grid, CHUNK_SIZE};
use crate::genetics::check_reproduction;
use rand::Rng;

/// Apply rules with triple-buffering and chunk-based batching
/// 
/// This approach:
/// 1. Copies current state to next buffer (global state)
/// 2. Processes grid in non-overlapping 32x32 chunks sequentially
/// 3. For each chunk, copies boundary region to stable buffer BEFORE processing
/// 4. All reads use boundary_buffer (isolated state) for consistency
/// 5. All writes go to next_cells (protected by chunk isolation)
/// 6. Swaps buffers at end
/// 
/// Key benefits:
/// - Eliminates update artifacts from partially-processed neighbors
/// - 32x32 chunks fit well in L1 cache (reducing cache misses)
/// - Boundary buffer provides consistent state for all neighbor lookups
/// - Can easily parallelize later (chunks at (x%2, y%2) don't overlap)
pub fn apply_rules(grid: &mut Grid) {
    // Calculate chunk grid dimensions
    let chunks_x = (grid.width + CHUNK_SIZE - 1) / CHUNK_SIZE;
    let chunks_y = (grid.height + CHUNK_SIZE - 1) / CHUNK_SIZE;

    // Process chunks in layers: (x%2, y%2) pattern ensures no overlap
    // Layer 0: (even, even), Layer 1: (odd, even), Layer 2: (even, odd), Layer 3: (odd, odd)
    for layer in 0..4 {
        // Copy boundaries for this layer sequentially
        let chunk_coords: Vec<(u32, u32)> = (0..chunks_y)
            .flat_map(|cy| (0..chunks_x).map(move |cx| (cx, cy)))
            .filter(|(cx, cy)| (cx % 2) as usize == (layer % 2) && (cy % 2) as usize == (layer / 2))
            .collect();

        // Copy all boundaries for this layer first
        for (chunk_x, chunk_y) in &chunk_coords {
            grid.copy_chunk_boundary(*chunk_x, *chunk_y);
        }

        // Process all chunks in this layer (sequential to maintain mutation safety)
        for (chunk_x, chunk_y) in chunk_coords {
            let mut local_rng = rand::thread_rng();
            process_chunk(grid, chunk_x, chunk_y, &mut local_rng);
        }
    }

    grid.swap_buffers();
}

/// Process a single 32x32 chunk of the grid
fn process_chunk(grid: &mut Grid, chunk_x: u32, chunk_y: u32, rng: &mut impl Rng) {
    let start_x = chunk_x * CHUNK_SIZE;
    let start_y = chunk_y * CHUNK_SIZE;
    let end_x = (start_x + CHUNK_SIZE).min(grid.width);
    let end_y = (start_y + CHUNK_SIZE).min(grid.height);

    // Process all cells in this chunk
    for y in start_y..end_y {
        for x in start_x..end_x {
            if let Some(cell) = grid.get_cell(x, y) {
                apply_cell_rules(grid, x, y, &cell, rng);
            }
        }
    }
}

/// Apply rules to a single cell
fn apply_cell_rules(grid: &mut Grid, x: u32, y: u32, cell: &Cell, _rng: &mut impl Rng) {
    // Fast path: black cells are inert unless reproduction occurs
    if cell.cell_type == CellType::Black {
        check_reproduction(grid, x, y, _rng);
        return;
    }

    let mut local_rng = rand::thread_rng();

    // Check for reproduction (very rare)
    check_reproduction(grid, x, y, &mut local_rng);

    let modified = match cell.cell_type {
        CellType::Red => { apply_red_rules(grid, x, y); true },
        CellType::Purple => { apply_purple_rules(grid, x, y, &mut local_rng); true },
        CellType::Gray => { apply_gray_rules(grid, x, y, &mut local_rng); true },
        CellType::Orange => { apply_orange_rules(grid, x, y); true },
        CellType::Green => { apply_green_rules(grid, x, y, &mut local_rng); true },
        CellType::White => { apply_white_rules(grid, x, y, &mut local_rng); true },
        CellType::Blue => { apply_blue_rules(grid, x, y, &mut local_rng); true },
        CellType::Brown => { apply_brown_rules(grid, x, y, &mut local_rng); true },
        CellType::Tan => { apply_tan_rules(grid, x, y, &mut local_rng); true },
        CellType::Gold => { apply_gold_rules(grid, x, y, &mut local_rng); true },
        CellType::Lime => { apply_lime_rules(grid, x, y); true },
        CellType::Crimson => { apply_crimson_rules(grid, x, y, &mut local_rng); true },
        CellType::Maroon => { apply_maroon_rules(grid, x, y, &mut local_rng); true },
        CellType::Coral => { apply_coral_rules(grid, x, y, &mut local_rng); true },
        CellType::Pink => { apply_pink_rules(grid, x, y, &mut local_rng); true },
        CellType::Magenta => { apply_magenta_rules(grid, x, y, &mut local_rng); true },
        CellType::Cyan => { apply_cyan_rules(grid, x, y, &mut local_rng); true },
        CellType::Yellow => { apply_yellow_rules(grid, x, y, &mut local_rng); true },
        CellType::Teal => { apply_teal_rules(grid, x, y, &mut local_rng); true },
        CellType::Navy => { apply_navy_rules(grid, x, y, &mut local_rng); true },
        CellType::Olive => { apply_olive_rules(grid, x, y, &mut local_rng); true },
        CellType::Indigo => { apply_indigo_rules(grid, x, y); false },
        CellType::Khaki => { apply_khaki_rules(grid, x, y, &mut local_rng); true },
        CellType::Slate => { apply_slate_rules(grid, x, y); false },
        CellType::Rust => { apply_rust_rules(grid, x, y, &mut local_rng); true },
        CellType::Mint => { apply_mint_rules(grid, x, y, &mut local_rng); true },
        CellType::Peach => { apply_peach_rules(grid, x, y, &mut local_rng); true },
        CellType::Aqua => { apply_aqua_rules(grid, x, y, &mut local_rng); true },
        CellType::Silver => { apply_silver_rules(grid, x, y, &mut local_rng); true },
        CellType::Violet => { apply_violet_rules(grid, x, y, &mut local_rng); true },
        CellType::Amber => { apply_amber_rules(grid, x, y, &mut local_rng); true },
        CellType::Pearl => { apply_pearl_rules(grid, x, y); false },
        CellType::Smoke => { apply_smoke_rules(grid, x, y, &mut local_rng); true },
        CellType::Glint => { apply_glint_rules(grid, x, y, &mut local_rng); true },
        CellType::Tint => { apply_tint_rules(grid, x, y, &mut local_rng); true },
        CellType::Shade => { apply_shade_rules(grid, x, y, &mut local_rng); true },
        CellType::Black => false, // Handled above
    };
    
    // If no rule modified this cell, copy it to next state
    if !modified && grid.get_next_cell(x, y).is_none() {
        grid.set_next_cell(x, y, cell.clone());
    }
}

// ============================================================================
// RULE IMPLEMENTATIONS using isolated reads
// ============================================================================
// All functions use count_*_isolated() which read from boundary_buffer
// instead of the live grid. This ensures consistent reads across all neighbors.

fn apply_red_rules(grid: &mut Grid, x: u32, y: u32) {
    for dy in -1..=1i32 {
        for dx in -1..=1i32 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as i32 + dx) as u32;
            let ny = (y as i32 + dy) as u32;
            if let Some(neighbor) = grid.get_cell(nx, ny) {
                if neighbor.cell_type == CellType::Purple {
                    grid.set_next_cell(nx, ny, Cell::new(CellType::Black));
                } else if neighbor.cell_type == CellType::Gray {
                    grid.set_next_cell(nx, ny, Cell::new(CellType::Orange));
                } else if neighbor.cell_type == CellType::Pink {
                    grid.set_next_cell(nx, ny, Cell::new(CellType::Black));
                }
            }
        }
    }
}

fn apply_purple_rules(grid: &mut Grid, x: u32, y: u32, rng: &mut impl Rng) {
    const SPREAD_RATE: f64 = 0.30;
    
    for dy in -1..=1i32 {
        for dx in -1..=1i32 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as i32 + dx) as u32;
            let ny = (y as i32 + dy) as u32;
            if let Some(neighbor) = grid.get_cell(nx, ny) {
                let has_peach = grid.count_neighbors_isolated(nx, ny, CellType::Peach) > 0;
                let has_indigo = grid.count_neighbors_isolated(nx, ny, CellType::Indigo) > 0;
                let has_olive = grid.count_neighbors_isolated(nx, ny, CellType::Olive) > 0;
                
                if !has_peach && !has_indigo && !has_olive {
                    if (neighbor.cell_type == CellType::Orange || neighbor.cell_type == CellType::Gray)
                        && rng.gen::<f64>() < SPREAD_RATE
                    {
                        grid.set_next_cell(nx, ny, Cell::new(CellType::Purple));
                    }
                }
            }
        }
    }
}

fn apply_gray_rules(grid: &mut Grid, x: u32, y: u32, rng: &mut impl Rng) {
    let mut cell = grid.get_cell(x, y).unwrap();
    cell.age += 1;
    
    let num_purple = if rng.gen::<f64>() < 0.5 { 1 } else { 2 };
    let mut produced = 0;
    
    for dy in -1..=1i32 {
        for dx in -1..=1i32 {
            if dx == 0 && dy == 0 || produced >= num_purple {
                continue;
            }
            let nx = (x as i32 + dx) as u32;
            let ny = (y as i32 + dy) as u32;
            if let Some(neighbor) = grid.get_cell(nx, ny) {
                if neighbor.cell_type == CellType::Black {
                    grid.set_next_cell(nx, ny, Cell::new(CellType::Purple));
                    produced += 1;
                }
            }
        }
    }
    
    if cell.age >= 4 && rng.gen::<f64>() < 0.5 {
        grid.set_next_cell(x, y, Cell::new(CellType::Black));
        for dy in -1..=1i32 {
            for dx in -1..=1i32 {
                if rng.gen::<f64>() < 0.5 {
                    let nx = (x as i32 + dx) as u32;
                    let ny = (y as i32 + dy) as u32;
                    if let Some(neighbor) = grid.get_cell(nx, ny) {
                        if neighbor.cell_type == CellType::Black {
                            grid.set_next_cell(nx, ny, Cell::new(CellType::Green));
                        }
                    }
                }
            }
        }
    } else {
        grid.set_next_cell(x, y, cell);
    }
}

fn apply_orange_rules(grid: &mut Grid, x: u32, y: u32) {
    const SURVIVAL_RADIUS: u32 = 5;
    const SURVIVAL_THRESHOLD: usize = 3;
    
    let green_count = grid.count_in_radius_isolated(x, y, CellType::Green, SURVIVAL_RADIUS);
    let purple_neighbors = grid.count_neighbors_isolated(x, y, CellType::Purple);
    let white_neighbors = grid.count_neighbors_isolated(x, y, CellType::White);
    
    if green_count < SURVIVAL_THRESHOLD {
        grid.set_next_cell(x, y, Cell::new(CellType::Gray));
    } else if purple_neighbors > 0 || white_neighbors > 0 {
        grid.set_next_cell(x, y, Cell::new(CellType::Red));
    }
}

fn apply_green_rules(grid: &mut Grid, x: u32, y: u32, rng: &mut impl Rng) {
    let mut spread_rate: f64 = 0.40;
    
    // Check local green density to prevent explosive growth
    let local_green = grid.count_in_radius_isolated(x, y, CellType::Green, 5);
    if local_green >= 12 {
        spread_rate = 0.0; // Stop spreading if too dense
    } else if local_green >= 8 {
        spread_rate = spread_rate * 0.25; // Heavily reduce if moderately dense
    }
    
    if grid.count_neighbors_isolated(x, y, CellType::Blue) > 0 {
        spread_rate = (spread_rate * 1.75_f64).min(0.70_f64);
    }
    
    if grid.count_neighbors_isolated(x, y, CellType::Cyan) > 0 
        || grid.count_neighbors_isolated(x, y, CellType::Olive) > 0 {
        spread_rate = (spread_rate * 1.5_f64).min(0.60_f64);
    }
    
    if grid.count_neighbors_isolated(x, y, CellType::Yellow) > 0 {
        spread_rate = ((spread_rate + 0.20) as f64).min(0.90);
    }
    
    if grid.count_neighbors_isolated(x, y, CellType::Smoke) > 0 {
        spread_rate = ((spread_rate - 0.30) as f64).max(0.1);
    }
    
    for dy in -1..=1i32 {
        for dx in -1..=1i32 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as i32 + dx) as u32;
            let ny = (y as i32 + dy) as u32;
            if let Some(neighbor) = grid.get_cell(nx, ny) {
                if neighbor.cell_type == CellType::Black && rng.gen::<f64>() < spread_rate {
                    grid.set_next_cell(nx, ny, Cell::new(CellType::Green));
                }
            }
        }
    }
}

fn apply_white_rules(grid: &mut Grid, x: u32, y: u32, rng: &mut impl Rng) {
    let green_count = grid.count_in_radius_isolated(x, y, CellType::Green, 5);
    let purple_count = grid.count_in_radius_isolated(x, y, CellType::Purple, 5);
    
    if green_count < 2 && purple_count >= 4 {
        grid.set_next_cell(x, y, Cell::new(CellType::Black));
        return;
    }
    
    for dy in -1..=1i32 {
        for dx in -1..=1i32 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as i32 + dx) as u32;
            let ny = (y as i32 + dy) as u32;
            if let Some(neighbor) = grid.get_cell(nx, ny) {
                if neighbor.cell_type != CellType::White {
                    if let Some(next) = grid.get_next_cell(nx, ny) {
                        if next.cell_type == CellType::Orange || next.cell_type == CellType::Gray {
                            grid.set_next_cell(nx, ny, Cell::new(CellType::Red));
                        }
                    }
                }
            }
        }
    }
    
    for dy in -1..=1i32 {
        for dx in -1..=1i32 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as i32 + dx) as u32;
            let ny = (y as i32 + dy) as u32;
            if let Some(neighbor) = grid.get_cell(nx, ny) {
                if neighbor.cell_type == CellType::Black && rng.gen::<f64>() < 0.25 {
                    grid.set_next_cell(nx, ny, Cell::new(CellType::White));
                }
            }
        }
    }
}

fn apply_blue_rules(grid: &mut Grid, x: u32, y: u32, rng: &mut impl Rng) {
    let mut cell = grid.get_cell(x, y).unwrap();
    cell.age += 1;
    
    if cell.age >= 8 && rng.gen::<f64>() < 0.3 {
        grid.set_next_cell(x, y, Cell::new(CellType::Black));
    } else {
        grid.set_next_cell(x, y, cell);
    }
    
    for dy in -1..=1i32 {
        for dx in -1..=1i32 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as i32 + dx) as u32;
            let ny = (y as i32 + dy) as u32;
            if let Some(neighbor) = grid.get_cell(nx, ny) {
                if neighbor.cell_type == CellType::Black && rng.gen::<f64>() < 0.20 {
                    grid.set_next_cell(nx, ny, Cell::new(CellType::Green));
                }
            }
        }
    }
}

fn apply_brown_rules(grid: &mut Grid, x: u32, y: u32, rng: &mut impl Rng) {
    let green_count = grid.count_in_radius_isolated(x, y, CellType::Green, 5);
    
    for dy in -1..=1i32 {
        for dx in -1..=1i32 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as i32 + dx) as u32;
            let ny = (y as i32 + dy) as u32;
            if let Some(neighbor) = grid.get_cell(nx, ny) {
                if neighbor.cell_type == CellType::Green && rng.gen::<f64>() < 0.8 {
                    grid.set_next_cell(nx, ny, Cell::new(CellType::Black));
                }
            }
        }
    }
    
    if green_count == 0 && rng.gen::<f64>() < 0.5 {
        grid.set_next_cell(x, y, Cell::new(CellType::Gray));
    }
}

fn apply_tan_rules(grid: &mut Grid, x: u32, y: u32, rng: &mut impl Rng) {
    let green_count = grid.count_in_radius_isolated(x, y, CellType::Green, 5);
    let orange_count = grid.count_in_radius_isolated(x, y, CellType::Orange, 5);
    
    for dy in -1..=1i32 {
        for dx in -1..=1i32 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as i32 + dx) as u32;
            let ny = (y as i32 + dy) as u32;
            if let Some(neighbor) = grid.get_cell(nx, ny) {
                if (neighbor.cell_type == CellType::Green || neighbor.cell_type == CellType::Orange) 
                    && rng.gen::<f64>() < 0.7 {
                    grid.set_next_cell(nx, ny, Cell::new(CellType::Black));
                }
            }
        }
    }
    
    if green_count + orange_count < 2 {
        grid.set_next_cell(x, y, Cell::new(CellType::Gray));
    }
}

fn apply_gold_rules(grid: &mut Grid, x: u32, y: u32, rng: &mut impl Rng) {
    let gray_count = grid.count_in_radius_isolated(x, y, CellType::Gray, 5);
    
    for dy in -1..=1i32 {
        for dx in -1..=1i32 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as i32 + dx) as u32;
            let ny = (y as i32 + dy) as u32;
            if let Some(neighbor) = grid.get_cell(nx, ny) {
                if neighbor.cell_type == CellType::Gray && rng.gen::<f64>() < 0.50 {
                    grid.set_next_cell(nx, ny, Cell::new(CellType::Black));
                }
            }
        }
    }
    
    if gray_count == 0 {
        grid.set_next_cell(x, y, Cell::new(CellType::Black));
    }
}

fn apply_lime_rules(grid: &mut Grid, x: u32, y: u32) {
    let green_count = grid.count_in_radius_isolated(x, y, CellType::Green, 5);
    
    if green_count == 0 {
        grid.set_next_cell(x, y, Cell::new(CellType::Black));
    }
}

fn apply_crimson_rules(grid: &mut Grid, x: u32, y: u32, rng: &mut impl Rng) {
    let prey_count = grid.count_in_radius_isolated(x, y, CellType::Orange, 5)
        + grid.count_in_radius_isolated(x, y, CellType::Brown, 5);
    
    for dy in -1..=1i32 {
        for dx in -1..=1i32 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as i32 + dx) as u32;
            let ny = (y as i32 + dy) as u32;
            if let Some(neighbor) = grid.get_cell(nx, ny) {
                if (neighbor.cell_type == CellType::Orange || neighbor.cell_type == CellType::Brown) 
                    && rng.gen::<f64>() < 0.9 {
                    grid.set_next_cell(nx, ny, Cell::new(CellType::Black));
                }
            }
        }
    }
    
    if prey_count == 0 {
        let mut cell = grid.get_next_cell(x, y).unwrap_or_else(|| grid.get_cell(x, y).unwrap());
        cell.age += 1;
        if cell.age >= 3 {
            grid.set_next_cell(x, y, Cell::new(CellType::Black));
        } else {
            grid.set_next_cell(x, y, cell);
        }
    }
}

fn apply_maroon_rules(grid: &mut Grid, x: u32, y: u32, rng: &mut impl Rng) {
    let prey_count = grid.count_in_radius_isolated(x, y, CellType::Orange, 5)
        + grid.count_in_radius_isolated(x, y, CellType::Crimson, 5);
    
    for dy in -1..=1i32 {
        for dx in -1..=1i32 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as i32 + dx) as u32;
            let ny = (y as i32 + dy) as u32;
            if let Some(neighbor) = grid.get_cell(nx, ny) {
                if (neighbor.cell_type == CellType::Orange || neighbor.cell_type == CellType::Crimson) 
                    && rng.gen::<f64>() < 0.9 {
                    grid.set_next_cell(nx, ny, Cell::new(CellType::Black));
                }
            }
        }
    }
    
    if prey_count == 0 {
        let mut cell = grid.get_next_cell(x, y).unwrap_or_else(|| grid.get_cell(x, y).unwrap());
        cell.age += 1;
        if cell.age >= 2 {
            grid.set_next_cell(x, y, Cell::new(CellType::Black));
        } else {
            grid.set_next_cell(x, y, cell);
        }
    }
}

fn apply_coral_rules(grid: &mut Grid, x: u32, y: u32, rng: &mut impl Rng) {
    const SPREAD_RATE: f64 = 0.40;
    
    let white_neighbors = grid.count_neighbors_isolated(x, y, CellType::White);
    let red_neighbors = grid.count_neighbors_isolated(x, y, CellType::Red);
    let indigo_neighbors = grid.count_neighbors_isolated(x, y, CellType::Indigo);
    let pearl_neighbors = grid.count_neighbors_isolated(x, y, CellType::Pearl);
    
    if white_neighbors + red_neighbors + indigo_neighbors + pearl_neighbors > 0 {
        for dy in -1..=1i32 {
            for dx in -1..=1i32 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = (x as i32 + dx) as u32;
                let ny = (y as i32 + dy) as u32;
                if let Some(neighbor) = grid.get_cell(nx, ny) {
                    if neighbor.cell_type == CellType::Black && rng.gen::<f64>() < 0.1 {
                        grid.set_next_cell(nx, ny, Cell::new(CellType::Coral));
                    }
                }
            }
        }
    } else {
        for dy in -1..=1i32 {
            for dx in -1..=1i32 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = (x as i32 + dx) as u32;
                let ny = (y as i32 + dy) as u32;
                if let Some(neighbor) = grid.get_cell(nx, ny) {
                    if neighbor.cell_type == CellType::Black && rng.gen::<f64>() < SPREAD_RATE {
                        grid.set_next_cell(nx, ny, Cell::new(CellType::Coral));
                    }
                }
            }
        }
    }
}

fn apply_pink_rules(grid: &mut Grid, x: u32, y: u32, rng: &mut impl Rng) {
    let orange_count = grid.count_in_radius_isolated(x, y, CellType::Orange, 5);
    
    for dy in -1..=1i32 {
        for dx in -1..=1i32 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as i32 + dx) as u32;
            let ny = (y as i32 + dy) as u32;
            if let Some(neighbor) = grid.get_cell(nx, ny) {
                if neighbor.cell_type == CellType::Orange && rng.gen::<f64>() < 0.15 {
                    grid.set_next_cell(nx, ny, Cell::new(CellType::Pink));
                }
            }
        }
    }
    
    if orange_count == 0 {
        grid.set_next_cell(x, y, Cell::new(CellType::Black));
    }
}

fn apply_magenta_rules(grid: &mut Grid, x: u32, y: u32, rng: &mut impl Rng) {
    let same_count = grid.count_neighbors_isolated(x, y, CellType::Magenta);
    
    if same_count == 0 && rng.gen::<f64>() < 0.3 {
        grid.set_next_cell(x, y, Cell::new(CellType::Black));
        return;
    }
    
    for dy in -1..=1i32 {
        for dx in -1..=1i32 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as i32 + dx) as u32;
            let ny = (y as i32 + dy) as u32;
            if let Some(_neighbor) = grid.get_cell(nx, ny) {
                if rng.gen::<f64>() < 0.40 {
                    let rand_type = (rng.gen::<u8>() % 37) as u8;
                    if let Some(new_type) = CellType::from_u8(rand_type) {
                        grid.set_next_cell(nx, ny, Cell::new(new_type));
                    }
                }
            }
        }
    }
}

fn apply_cyan_rules(grid: &mut Grid, x: u32, y: u32, rng: &mut impl Rng) {
    const SPREAD_RATE: f64 = 0.10;
    
    for dy in -1..=1i32 {
        for dx in -1..=1i32 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as i32 + dx) as u32;
            let ny = (y as i32 + dy) as u32;
            if let Some(neighbor) = grid.get_cell(nx, ny) {
                if neighbor.cell_type == CellType::Black && rng.gen::<f64>() < SPREAD_RATE {
                    grid.set_next_cell(nx, ny, Cell::new(CellType::Cyan));
                }
            }
        }
    }
}

fn apply_yellow_rules(grid: &mut Grid, x: u32, y: u32, rng: &mut impl Rng) {
    const SPREAD_RATE: f64 = 0.15;
    
    let mut cell = grid.get_cell(x, y).unwrap();
    cell.age += 1;
    
    if cell.age >= 15 {
        let neighbor_count = grid.count_neighbors_isolated(x, y, CellType::Yellow);
        if neighbor_count == 0 && rng.gen::<f64>() < 0.3 {
            grid.set_next_cell(x, y, Cell::new(CellType::Black));
            return;
        }
    }
    
    grid.set_next_cell(x, y, cell);
    
    for dy in -1..=1i32 {
        for dx in -1..=1i32 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as i32 + dx) as u32;
            let ny = (y as i32 + dy) as u32;
            if let Some(neighbor) = grid.get_cell(nx, ny) {
                if neighbor.cell_type == CellType::Black && rng.gen::<f64>() < SPREAD_RATE {
                    grid.set_next_cell(nx, ny, Cell::new(CellType::Yellow));
                }
            }
        }
    }
}

fn apply_teal_rules(grid: &mut Grid, x: u32, y: u32, rng: &mut impl Rng) {
    let mut cell = grid.get_cell(x, y).unwrap();
    cell.age += 1;
    
    if cell.age >= 12 && rng.gen::<f64>() < 0.2 {
        grid.set_next_cell(x, y, Cell::new(CellType::Black));
    } else {
        grid.set_next_cell(x, y, cell);
    }
    
    for dy in -1..=1i32 {
        for dx in -1..=1i32 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as i32 + dx) as u32;
            let ny = (y as i32 + dy) as u32;
            if let Some(neighbor) = grid.get_cell(nx, ny) {
                if neighbor.cell_type == CellType::Black && rng.gen::<f64>() < 0.05 {
                    grid.set_next_cell(nx, ny, Cell::new(CellType::Teal));
                }
            }
        }
    }
}

fn apply_navy_rules(grid: &mut Grid, x: u32, y: u32, rng: &mut impl Rng) {
    const SPREAD_RATE: f64 = 0.05;
    
    for dy in -1..=1i32 {
        for dx in -1..=1i32 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as i32 + dx) as u32;
            let ny = (y as i32 + dy) as u32;
            if let Some(neighbor) = grid.get_cell(nx, ny) {
                if neighbor.cell_type == CellType::Blue && rng.gen::<f64>() < 0.25 {
                    grid.set_next_cell(nx, ny, Cell::new(CellType::Navy));
                } else if neighbor.cell_type == CellType::Black && rng.gen::<f64>() < SPREAD_RATE {
                    grid.set_next_cell(nx, ny, Cell::new(CellType::Navy));
                }
            }
        }
    }
}

fn apply_olive_rules(grid: &mut Grid, x: u32, y: u32, rng: &mut impl Rng) {
    let mut cell = grid.get_cell(x, y).unwrap();
    cell.age += 1;
    
    if cell.age >= 10 && rng.gen::<f64>() < 0.5 {
        grid.set_next_cell(x, y, Cell::new(CellType::Black));
        for dy in -1..=1i32 {
            for dx in -1..=1i32 {
                let nx = (x as i32 + dx) as u32;
                let ny = (y as i32 + dy) as u32;
                if let Some(neighbor) = grid.get_cell(nx, ny) {
                    if neighbor.cell_type == CellType::Black && rng.gen::<f64>() < 0.5 {
                        grid.set_next_cell(nx, ny, Cell::new(CellType::Green));
                    }
                }
            }
        }
    } else {
        grid.set_next_cell(x, y, cell);
    }
}

fn apply_indigo_rules(_grid: &mut Grid, _x: u32, _y: u32) {
    // Inert
}

fn apply_khaki_rules(grid: &mut Grid, x: u32, y: u32, rng: &mut impl Rng) {
    const SPREAD_RATE: f64 = 0.30;
    
    let green_count = grid.count_in_radius_isolated(x, y, CellType::Green, 5);
    
    if green_count > 4 {
        grid.set_next_cell(x, y, Cell::new(CellType::Black));
        return;
    }
    
    for dy in -1..=1i32 {
        for dx in -1..=1i32 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as i32 + dx) as u32;
            let ny = (y as i32 + dy) as u32;
            if let Some(neighbor) = grid.get_cell(nx, ny) {
                if (neighbor.cell_type == CellType::Gray || neighbor.cell_type == CellType::Black)
                    && rng.gen::<f64>() < SPREAD_RATE
                {
                    if neighbor.cell_type == CellType::Gray {
                        grid.set_next_cell(nx, ny, Cell::new(CellType::Cyan));
                    } else {
                        grid.set_next_cell(nx, ny, Cell::new(CellType::Khaki));
                    }
                }
            }
        }
    }
}

fn apply_slate_rules(_grid: &mut Grid, _x: u32, _y: u32) {
    // Invisible
}

fn apply_rust_rules(grid: &mut Grid, x: u32, y: u32, rng: &mut impl Rng) {
    const SPREAD_RATE: f64 = 0.25;
    
    let black_count = grid.count_in_radius_isolated(x, y, CellType::Black, 5);
    
    for dy in -1..=1i32 {
        for dx in -1..=1i32 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as i32 + dx) as u32;
            let ny = (y as i32 + dy) as u32;
            if let Some(neighbor) = grid.get_cell(nx, ny) {
                if neighbor.cell_type == CellType::Black && rng.gen::<f64>() < SPREAD_RATE {
                    grid.set_next_cell(nx, ny, Cell::new(CellType::Olive));
                }
            }
        }
    }
    
    if black_count == 0 {
        grid.set_next_cell(x, y, Cell::new(CellType::Black));
    }
}

fn apply_mint_rules(grid: &mut Grid, x: u32, y: u32, rng: &mut impl Rng) {
    const SPREAD_RATE: f64 = 0.15;
    
    for dy in -1..=1i32 {
        for dx in -1..=1i32 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as i32 + dx) as u32;
            let ny = (y as i32 + dy) as u32;
            if let Some(neighbor) = grid.get_cell(nx, ny) {
                if neighbor.cell_type == CellType::Gray && rng.gen::<f64>() < SPREAD_RATE {
                    grid.set_next_cell(nx, ny, Cell::new(CellType::Orange));
                }
            }
        }
    }
}

fn apply_peach_rules(grid: &mut Grid, x: u32, y: u32, rng: &mut impl Rng) {
    const SPREAD_RATE: f64 = 0.20;
    
    let purple_count = grid.count_in_radius_isolated(x, y, CellType::Purple, 5);
    
    if purple_count >= 5 && rng.gen::<f64>() < 0.5 {
        grid.set_next_cell(x, y, Cell::new(CellType::Black));
        return;
    }
    
    for dy in -1..=1i32 {
        for dx in -1..=1i32 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as i32 + dx) as u32;
            let ny = (y as i32 + dy) as u32;
            if let Some(neighbor) = grid.get_cell(nx, ny) {
                if neighbor.cell_type == CellType::Black && rng.gen::<f64>() < SPREAD_RATE {
                    grid.set_next_cell(nx, ny, Cell::new(CellType::Peach));
                }
            }
        }
    }
}

fn apply_aqua_rules(grid: &mut Grid, x: u32, y: u32, rng: &mut impl Rng) {
    const SPREAD_RATE: f64 = 0.12;
    
    let chaos = grid.count_neighbors_isolated(x, y, CellType::Magenta)
        + grid.count_neighbors_isolated(x, y, CellType::Crimson)
        + grid.count_neighbors_isolated(x, y, CellType::Purple);
    
    if chaos > 6 && rng.gen::<f64>() < 0.5 {
        grid.set_next_cell(x, y, Cell::new(CellType::Black));
        return;
    }
    
    for dy in -1..=1i32 {
        for dx in -1..=1i32 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as i32 + dx) as u32;
            let ny = (y as i32 + dy) as u32;
            if let Some(neighbor) = grid.get_cell(nx, ny) {
                if neighbor.cell_type == CellType::Black && rng.gen::<f64>() < SPREAD_RATE {
                    grid.set_next_cell(nx, ny, Cell::new(CellType::Aqua));
                }
            }
        }
    }
}

fn apply_silver_rules(grid: &mut Grid, x: u32, y: u32, rng: &mut impl Rng) {
    let white_neighbors = grid.count_neighbors_isolated(x, y, CellType::White);
    
    if white_neighbors < 2 && rng.gen::<f64>() < 0.1 {
        grid.set_next_cell(x, y, Cell::new(CellType::White));
        return;
    }
    
    let threats = grid.count_in_radius_isolated(x, y, CellType::Purple, 10)
        + grid.count_in_radius_isolated(x, y, CellType::Crimson, 10);
    
    if threats > 0 {
        for dy in -1..=1i32 {
            for dx in -1..=1i32 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = (x as i32 + dx) as u32;
                let ny = (y as i32 + dy) as u32;
                if let Some(neighbor) = grid.get_cell(nx, ny) {
                    if (neighbor.cell_type == CellType::Orange || neighbor.cell_type == CellType::Gray)
                        && rng.gen::<f64>() < 0.5
                    {
                        grid.set_next_cell(nx, ny, Cell::new(CellType::Red));
                    }
                }
            }
        }
    }
}

fn apply_violet_rules(grid: &mut Grid, x: u32, y: u32, rng: &mut impl Rng) {
    const SPREAD_RATE: f64 = 0.08;
    
    let pearl_count = grid.count_neighbors_isolated(x, y, CellType::Pearl);
    let white_count = grid.count_neighbors_isolated(x, y, CellType::White);
    let indigo_count = grid.count_neighbors_isolated(x, y, CellType::Indigo);
    
    if pearl_count + white_count + indigo_count > 4 && rng.gen::<f64>() < 0.5 {
        grid.set_next_cell(x, y, Cell::new(CellType::Black));
        return;
    }
    
    for dy in -1..=1i32 {
        for dx in -1..=1i32 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as i32 + dx) as u32;
            let ny = (y as i32 + dy) as u32;
            if let Some(neighbor) = grid.get_cell(nx, ny) {
                if neighbor.cell_type != CellType::Pearl && neighbor.cell_type != CellType::White
                    && neighbor.cell_type != CellType::Indigo && rng.gen::<f64>() < SPREAD_RATE
                {
                    grid.set_next_cell(nx, ny, Cell::new(CellType::Black));
                }
            }
        }
    }
}

fn apply_amber_rules(grid: &mut Grid, x: u32, y: u32, rng: &mut impl Rng) {
    let mut cell = grid.get_cell(x, y).unwrap();
    cell.age += 1;
    
    if cell.age >= 5 && rng.gen::<f64>() < 0.5 {
        grid.set_next_cell(x, y, Cell::new(CellType::Black));
        return;
    }
    
    grid.set_next_cell(x, y, cell);
}

fn apply_pearl_rules(_grid: &mut Grid, _x: u32, _y: u32) {
    // Immobile
}

fn apply_smoke_rules(grid: &mut Grid, x: u32, y: u32, rng: &mut impl Rng) {
    const SPREAD_RATE: f64 = 0.25;
    
    for dy in -1..=1i32 {
        for dx in -1..=1i32 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as i32 + dx) as u32;
            let ny = (y as i32 + dy) as u32;
            if let Some(_neighbor) = grid.get_cell(nx, ny) {
                if rng.gen::<f64>() < SPREAD_RATE {
                    grid.set_next_cell(nx, ny, Cell::new(CellType::Smoke));
                }
            }
        }
    }
    
    let yellow_count = grid.count_neighbors_isolated(x, y, CellType::Yellow);
    let red_count = grid.count_neighbors_isolated(x, y, CellType::Red);
    
    if yellow_count + red_count > 3 && rng.gen::<f64>() < 0.5 {
        grid.set_next_cell(x, y, Cell::new(CellType::Black));
    }
}

fn apply_glint_rules(grid: &mut Grid, x: u32, y: u32, rng: &mut impl Rng) {
    let mut cell = grid.get_cell(x, y).unwrap();
    cell.age += 1;
    
    if cell.age >= 2 && rng.gen::<f64>() < 0.8 {
        grid.set_next_cell(x, y, Cell::new(CellType::Black));
        return;
    }
    
    grid.set_next_cell(x, y, cell);
    
    // Reduce green spawn rate significantly and only spawn with low density constraint
    let green_count = grid.count_in_radius_isolated(x, y, CellType::Green, 5);
    if green_count >= 8 {
        return; // Don't spawn if too much green nearby
    }
    
    for dy in -2..=2i32 {
        for dx in -2..=2i32 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as i32 + dx) as u32;
            let ny = (y as i32 + dy) as u32;
            if let Some(neighbor) = grid.get_cell(nx, ny) {
                if neighbor.cell_type == CellType::Black && rng.gen::<f64>() < 0.05 {
                    grid.set_next_cell(nx, ny, Cell::new(CellType::Green));
                }
            }
        }
    }
}

fn apply_tint_rules(grid: &mut Grid, x: u32, y: u32, rng: &mut impl Rng) {
    const SPREAD_RATE: f64 = 0.30;
    
    let tint_neighbors = grid.count_neighbors_isolated(x, y, CellType::Tint);
    
    if tint_neighbors == 0 && rng.gen::<f64>() < 0.3 {
        grid.set_next_cell(x, y, Cell::new(CellType::Black));
        return;
    }
    
    let spread_rate = if tint_neighbors >= 2 { 0.40 } else { SPREAD_RATE };
    
    for dy in -1..=1i32 {
        for dx in -1..=1i32 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as i32 + dx) as u32;
            let ny = (y as i32 + dy) as u32;
            if let Some(neighbor) = grid.get_cell(nx, ny) {
                if neighbor.cell_type == CellType::Black && rng.gen::<f64>() < spread_rate {
                    grid.set_next_cell(nx, ny, Cell::new(CellType::Tint));
                }
            }
        }
    }
}

fn apply_shade_rules(grid: &mut Grid, x: u32, y: u32, rng: &mut impl Rng) {
    const SPREAD_RATE: f64 = 0.20;
    
    let green_count = grid.count_in_radius_isolated(x, y, CellType::Green, 5);
    let threat_count = grid.count_in_radius_isolated(x, y, CellType::Crimson, 5)
        + grid.count_in_radius_isolated(x, y, CellType::Purple, 5);
    
    if threat_count > 5 && green_count == 0 && rng.gen::<f64>() < 0.5 {
        grid.set_next_cell(x, y, Cell::new(CellType::Black));
        return;
    }
    
    let mut target_dirs = Vec::new();
    
    for dy in -1..=1i32 {
        for dx in -1..=1i32 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as i32 + dx) as u32;
            let ny = (y as i32 + dy) as u32;
            if let Some(neighbor) = grid.get_cell(nx, ny) {
                if neighbor.cell_type == CellType::Black {
                    let local_green = grid.count_neighbors_isolated(nx, ny, CellType::Green);
                    let local_threat = grid.count_neighbors_isolated(nx, ny, CellType::Crimson)
                        + grid.count_neighbors_isolated(nx, ny, CellType::Purple);
                    
                    if local_threat == 0 || local_green > 2 {
                        target_dirs.push((nx, ny));
                    }
                }
            }
        }
    }
    
    if !target_dirs.is_empty() && rng.gen::<f64>() < SPREAD_RATE {
        let (nx, ny) = target_dirs[rng.gen_range(0..target_dirs.len())];
        grid.set_next_cell(nx, ny, Cell::new(CellType::Shade));
    }
}
