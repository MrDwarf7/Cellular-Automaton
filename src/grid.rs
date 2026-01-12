use crate::{
    cell::{Cell, CellType},
    presets::{Preset, PresetProvider},
    PresetT,
};
use rand::Rng;
use std::collections::HashMap;

pub struct Grid {
    pub width: u32,
    pub height: u32,
    cells: Vec<Cell>,
    next_cells: Vec<Cell>,
    // Triple buffer: stable read state for chunk boundaries
    boundary_buffer: Vec<Cell>,
}

// Chunk configuration for batched processing
pub const CHUNK_SIZE: u32 = 32;
pub const BOUNDARY_RADIUS: u32 = 6; // Radius for neighbor lookups (max interaction distance)

impl Grid {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        Grid {
            width,
            height,
            cells: vec![Cell::new(CellType::Black); size],
            next_cells: vec![Cell::new(CellType::Black); size],
            boundary_buffer: vec![Cell::new(CellType::Black); size],
        }
    }

    // pub fn initialize_random(&mut self, densities: impl PresetProvider) {
    pub fn initialize_random(&mut self, densities: &serde_json::Map<String, serde_json::Value>) {
        let mut rng = rand::thread_rng();

        // Start all cells as Black
        self.cells.fill(Cell::new(CellType::Black));

        // PERF: make this constant, similarly to how I did on the Presets (except this
        // is your 'registry' of cell types, which can be properly constant).
        // Your look-ups on this should always be O(1), it's a known set of types.
        let cell_types = [
            ("Black", CellType::Black),
            ("Green", CellType::Green),
            ("Orange", CellType::Orange),
            ("Gray", CellType::Gray),
            ("Purple", CellType::Purple),
            ("Red", CellType::Red),
            ("White", CellType::White),
            ("Blue", CellType::Blue),
            ("Brown", CellType::Brown),
            ("Tan", CellType::Tan),
            ("Gold", CellType::Gold),
            ("Lime", CellType::Lime),
            ("Crimson", CellType::Crimson),
            ("Maroon", CellType::Maroon),
            ("Coral", CellType::Coral),
            ("Pink", CellType::Pink),
            ("Magenta", CellType::Magenta),
            ("Cyan", CellType::Cyan),
            ("Yellow", CellType::Yellow),
            ("Teal", CellType::Teal),
            ("Navy", CellType::Navy),
            ("Olive", CellType::Olive),
            ("Indigo", CellType::Indigo),
            ("Khaki", CellType::Khaki),
            ("Slate", CellType::Slate),
            ("Rust", CellType::Rust),
            ("Mint", CellType::Mint),
            ("Peach", CellType::Peach),
            ("Aqua", CellType::Aqua),
            ("Silver", CellType::Silver),
            ("Violet", CellType::Violet),
            ("Amber", CellType::Amber),
            ("Pearl", CellType::Pearl),
            ("Smoke", CellType::Smoke),
            ("Glint", CellType::Glint),
            ("Tint", CellType::Tint),
            ("Shade", CellType::Shade),
        ];

        // PERF:[wtf] is going on here???? bruh...

        // Process each cell type and only update if it should be that type
        for (name, cell_type) in cell_types.iter() {
            if let Some(density_val) = densities.get(*name) {
                if let Some(density) = density_val.as_f64() {
                    let density = (density / 100.0).min(1.0).max(0.0);
                    if density > 0.0 {
                        for cell in self.cells.iter_mut() {
                            if cell.cell_type == CellType::Black && rng.gen::<f64>() < density {
                                *cell = Cell::new(*cell_type);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn get_cell(&self, x: u32, y: u32) -> Option<Cell> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let idx = (y * self.width + x) as usize;
        Some(self.cells[idx].clone())
    }

    pub fn set_cell(&mut self, x: u32, y: u32, cell_type: CellType) {
        if x >= self.width || y >= self.height {
            return;
        }
        let idx = (y * self.width + x) as usize;
        self.cells[idx] = Cell::new(cell_type);
    }

    pub fn get_next_cell(&self, x: u32, y: u32) -> Option<Cell> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let idx = (y * self.width + x) as usize;
        Some(self.next_cells[idx].clone())
    }

    pub fn set_next_cell(&mut self, x: u32, y: u32, cell: Cell) {
        if x >= self.width || y >= self.height {
            return;
        }
        let idx = (y * self.width + x) as usize;
        self.next_cells[idx] = cell;
    }

    pub fn swap_buffers(&mut self) {
        std::mem::swap(&mut self.cells, &mut self.next_cells);
    }

    /// Copy boundary region for a chunk to boundary_buffer for isolated reads
    ///
    /// This must be called sequentially before parallel chunk processing.
    /// Since chunks at (cx%2, cy%2) don't overlap, this is called in layers.
    pub fn copy_chunk_boundary(&mut self, chunk_x: u32, chunk_y: u32) {
        let start_x = (chunk_x * CHUNK_SIZE).saturating_sub(BOUNDARY_RADIUS);
        let start_y = (chunk_y * CHUNK_SIZE).saturating_sub(BOUNDARY_RADIUS);
        let end_x = ((chunk_x + 1) * CHUNK_SIZE + BOUNDARY_RADIUS).min(self.width);
        let end_y = ((chunk_y + 1) * CHUNK_SIZE + BOUNDARY_RADIUS).min(self.height);

        for y in start_y..end_y {
            for x in start_x..end_x {
                if x < self.width && y < self.height {
                    let idx = (y * self.width + x) as usize;
                    self.boundary_buffer[idx] = self.cells[idx].clone();
                }
            }
        }
    }

    /// Get cell from boundary buffer (stable read state)
    pub fn get_cell_from_boundary(&self, x: u32, y: u32) -> Option<Cell> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let idx = (y * self.width + x) as usize;
        Some(self.boundary_buffer[idx].clone())
    }

    /// Count neighbors using boundary buffer for isolation (optimized)
    #[inline]
    pub fn count_neighbors_isolated(&self, x: u32, y: u32, cell_type: CellType) -> usize {
        let mut count = 0;
        let width = self.width as usize;
        let x_usize = x as usize;
        let y_usize = y as usize;

        // Direct array access without bounds checking for interior cells
        // Much faster than calling get_cell_from_boundary 8 times
        let x_i = x as i32;
        let y_i = y as i32;

        // Check all 8 neighbors
        for dy in -1..=1i32 {
            for dx in -1..=1i32 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x_i + dx;
                let ny = y_i + dy;
                if nx >= 0 && ny >= 0 && (nx as u32) < self.width && (ny as u32) < self.height {
                    let idx = ((ny as usize) * width + (nx as usize)) as usize;
                    if self.boundary_buffer[idx].cell_type == cell_type {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    /// Count in radius using boundary buffer for isolation (optimized)
    #[inline]
    pub fn count_in_radius_isolated(
        &self,
        x: u32,
        y: u32,
        cell_type: CellType,
        radius: u32,
    ) -> usize {
        let mut count = 0;
        let x_start = if x < radius { 0 } else { x - radius };
        let x_end = (x + radius + 1).min(self.width);
        let y_start = if y < radius { 0 } else { y - radius };
        let y_end = (y + radius + 1).min(self.height);

        // Row-major iteration for cache efficiency
        for cy in y_start..y_end {
            let row_base = (cy * self.width) as usize;
            for cx in x_start..x_end {
                let idx = (row_base + cx as usize) as usize;
                if idx < self.boundary_buffer.len() {
                    if self.boundary_buffer[idx].cell_type == cell_type {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    pub fn count_neighbors(&self, x: u32, y: u32, cell_type: CellType) -> usize {
        let mut count = 0;
        for dy in -1..=1i32 {
            for dx in -1..=1i32 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = (x as i32 + dx) as u32;
                let ny = (y as i32 + dy) as u32;
                if let Some(cell) = self.get_cell(nx, ny) {
                    if cell.cell_type == cell_type {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    pub fn count_in_radius(&self, x: u32, y: u32, cell_type: CellType, radius: u32) -> usize {
        let mut count = 0;
        let x_start = if x < radius { 0 } else { x - radius };
        let x_end = (x + radius + 1).min(self.width);
        let y_start = if y < radius { 0 } else { y - radius };
        let y_end = (y + radius + 1).min(self.height);

        for cy in y_start..y_end {
            for cx in x_start..x_end {
                if let Some(cell) = self.get_cell(cx, cy) {
                    if cell.cell_type == cell_type {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.cells.iter().map(|c| c.to_u8()).collect()
    }

    pub fn to_json(&self) -> String {
        let mut map = serde_json::Map::new();
        map.insert("width".to_string(), serde_json::json!(self.width));
        map.insert("height".to_string(), serde_json::json!(self.height));
        map.insert("cells".to_string(), serde_json::json!(self.to_bytes()));
        serde_json::to_string(&map).unwrap_or_default()
    }

    pub fn get_population_counts(&self) -> String {
        let mut counts: HashMap<String, u32> = HashMap::new();

        for cell in self.cells.iter() {
            let name = match cell.cell_type {
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
            };
            *counts.entry(name.to_string()).or_insert(0) += 1;
        }

        let mut map = serde_json::Map::new();
        for (name, count) in counts {
            map.insert(name, serde_json::json!(count));
        }
        serde_json::to_string(&map).unwrap_or_default()
    }
}
