#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellType {
    // Primary Ecosystem (0-7)
    Black,      // 0 - Dead
    Green,      // 1 - Vegetation
    Orange,     // 2 - Vitality
    Gray,       // 3 - Sick
    Purple,     // 4 - Plague
    Red,        // 5 - Cure
    White,      // 6 - Defender
    Blue,       // 7 - Water

    // Herbivores & Consumers (8-11)
    Brown,      // 8  - Herbivore
    Tan,        // 9  - Omnivore
    Gold,       // 10 - Scavenger
    Lime,       // 11 - Symbiote

    // Predators & Aggressive (12-16)
    Crimson,    // 12 - Predator
    Maroon,     // 13 - Apex
    Coral,      // 14 - Aggressive
    Pink,       // 15 - Parasite
    Magenta,    // 16 - Mutant

    // Environmental & Resources (17-22)
    Cyan,       // 17 - Nutrient
    Yellow,     // 18 - Light
    Teal,       // 19 - Moisture
    Navy,       // 20 - Deep Water
    Olive,      // 21 - Soil
    Indigo,     // 22 - Mineral

    // Decomposers & Recyclers (23-25)
    Khaki,      // 23 - Fungus
    Slate,      // 24 - Bacteria (invisible)
    Rust,       // 25 - Decay

    // Regulatory & Protective (26-29)
    Mint,       // 26 - Healer
    Peach,      // 27 - Insulator
    Aqua,       // 28 - Stabilizer
    Silver,     // 29 - Sentinel

    // Rare & Exotic (30-34)
    Violet,     // 30 - Void
    Amber,      // 31 - Catalyst
    Pearl,      // 32 - Barrier
    Smoke,      // 33 - Toxic
    Glint,      // 34 - Spark

    // Behavioral Specialties (35-37)
    Tint,       // 35 - Generalist
    Shade,      // 36 - Strategist
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub cell_type: CellType,
    pub age: u8,              // For decay counters
    pub metadata: u8,         // For additional state
    pub rng_seed: u64,        // Embedded RNG state (updated every 20 frames)
    pub genes: Genes,         // Heritable traits
}

#[derive(Debug, Clone, Copy)]
pub struct Genes {
    pub spread_tendency: f64,     // 0.0-1.0: likelihood to spread
    pub aggression: f64,          // 0.0-1.0: how aggressive in interactions
    pub vitality: f64,            // 0.0-1.0: resistance to decay
    pub mutatability: f64,        // 0.0-1.0: chance to mutate children
    pub generation: u8,           // How many generations from origin
    pub parent_types: (u8, u8),   // IDs of parent cell types
}

impl Default for Genes {
    fn default() -> Self {
        Genes {
            spread_tendency: 0.5,
            aggression: 0.5,
            vitality: 0.5,
            mutatability: 0.1,
            generation: 0,
            parent_types: (0, 0),
        }
    }
}

impl Genes {
    pub fn blend(parent1: &Genes, parent2: &Genes) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        Genes {
            spread_tendency: (parent1.spread_tendency + parent2.spread_tendency) / 2.0 
                + (rng.gen::<f64>() - 0.5) * 0.2,
            aggression: (parent1.aggression + parent2.aggression) / 2.0 
                + (rng.gen::<f64>() - 0.5) * 0.2,
            vitality: (parent1.vitality + parent2.vitality) / 2.0 
                + (rng.gen::<f64>() - 0.5) * 0.2,
            mutatability: (parent1.mutatability + parent2.mutatability) / 2.0 
                + (rng.gen::<f64>() - 0.5) * 0.1,
            generation: parent1.generation.saturating_add(1).min(255),
            parent_types: (parent1.parent_types.0, parent2.parent_types.0),
        }
    }

    pub fn blend_color(type1: CellType, type2: CellType) -> (u8, u8, u8) {
        let (r1, g1, b1) = type1.get_color();
        let (r2, g2, b2) = type2.get_color();
        
        let r = ((r1 as u16 + r2 as u16) / 2) as u8;
        let g = ((g1 as u16 + g2 as u16) / 2) as u8;
        let b = ((b1 as u16 + b2 as u16) / 2) as u8;
        
        (r, g, b)
    }

    pub fn clamp(&mut self) {
        self.spread_tendency = self.spread_tendency.max(0.0).min(1.0);
        self.aggression = self.aggression.max(0.0).min(1.0);
        self.vitality = self.vitality.max(0.0).min(1.0);
        self.mutatability = self.mutatability.max(0.0).min(1.0);
    }
}

impl CellType {
    pub fn from_u8(n: u8) -> Option<CellType> {
        match n {
            0 => Some(CellType::Black),
            1 => Some(CellType::Green),
            2 => Some(CellType::Orange),
            3 => Some(CellType::Gray),
            4 => Some(CellType::Purple),
            5 => Some(CellType::Red),
            6 => Some(CellType::White),
            7 => Some(CellType::Blue),
            8 => Some(CellType::Brown),
            9 => Some(CellType::Tan),
            10 => Some(CellType::Gold),
            11 => Some(CellType::Lime),
            12 => Some(CellType::Crimson),
            13 => Some(CellType::Maroon),
            14 => Some(CellType::Coral),
            15 => Some(CellType::Pink),
            16 => Some(CellType::Magenta),
            17 => Some(CellType::Cyan),
            18 => Some(CellType::Yellow),
            19 => Some(CellType::Teal),
            20 => Some(CellType::Navy),
            21 => Some(CellType::Olive),
            22 => Some(CellType::Indigo),
            23 => Some(CellType::Khaki),
            24 => Some(CellType::Slate),
            25 => Some(CellType::Rust),
            26 => Some(CellType::Mint),
            27 => Some(CellType::Peach),
            28 => Some(CellType::Aqua),
            29 => Some(CellType::Silver),
            30 => Some(CellType::Violet),
            31 => Some(CellType::Amber),
            32 => Some(CellType::Pearl),
            33 => Some(CellType::Smoke),
            34 => Some(CellType::Glint),
            35 => Some(CellType::Tint),
            36 => Some(CellType::Shade),
            _ => None,
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            CellType::Black => 0,
            CellType::Green => 1,
            CellType::Orange => 2,
            CellType::Gray => 3,
            CellType::Purple => 4,
            CellType::Red => 5,
            CellType::White => 6,
            CellType::Blue => 7,
            CellType::Brown => 8,
            CellType::Tan => 9,
            CellType::Gold => 10,
            CellType::Lime => 11,
            CellType::Crimson => 12,
            CellType::Maroon => 13,
            CellType::Coral => 14,
            CellType::Pink => 15,
            CellType::Magenta => 16,
            CellType::Cyan => 17,
            CellType::Yellow => 18,
            CellType::Teal => 19,
            CellType::Navy => 20,
            CellType::Olive => 21,
            CellType::Indigo => 22,
            CellType::Khaki => 23,
            CellType::Slate => 24,
            CellType::Rust => 25,
            CellType::Mint => 26,
            CellType::Peach => 27,
            CellType::Aqua => 28,
            CellType::Silver => 29,
            CellType::Violet => 30,
            CellType::Amber => 31,
            CellType::Pearl => 32,
            CellType::Smoke => 33,
            CellType::Glint => 34,
            CellType::Tint => 35,
            CellType::Shade => 36,
        }
    }

    pub fn get_color(&self) -> (u8, u8, u8) {
        match self {
            CellType::Black => (0, 0, 0),
            CellType::Green => (0, 204, 0),
            CellType::Orange => (255, 136, 0),
            CellType::Gray => (136, 136, 136),
            CellType::Purple => (170, 0, 255),
            CellType::Red => (255, 0, 0),
            CellType::White => (255, 255, 255),
            CellType::Blue => (0, 136, 255),
            CellType::Brown => (165, 82, 0),
            CellType::Tan => (210, 180, 140),
            CellType::Gold => (255, 215, 0),
            CellType::Lime => (50, 205, 50),
            CellType::Crimson => (220, 20, 60),
            CellType::Maroon => (128, 0, 0),
            CellType::Coral => (255, 127, 80),
            CellType::Pink => (255, 192, 203),
            CellType::Magenta => (255, 0, 255),
            CellType::Cyan => (0, 255, 255),
            CellType::Yellow => (255, 255, 0),
            CellType::Teal => (0, 128, 128),
            CellType::Navy => (0, 0, 128),
            CellType::Olive => (128, 128, 0),
            CellType::Indigo => (75, 0, 130),
            CellType::Khaki => (240, 230, 200),
            CellType::Slate => (112, 128, 144),
            CellType::Rust => (183, 65, 14),
            CellType::Mint => (152, 251, 152),
            CellType::Peach => (255, 218, 185),
            CellType::Aqua => (0, 255, 255),
            CellType::Silver => (192, 192, 192),
            CellType::Violet => (238, 130, 238),
            CellType::Amber => (255, 191, 0),
            CellType::Pearl => (240, 240, 240),
            CellType::Smoke => (100, 100, 100),
            CellType::Glint => (255, 255, 150),
            CellType::Tint => (200, 200, 200),
            CellType::Shade => (64, 64, 64),
        }
    }
}

impl Cell {
    pub fn new(cell_type: CellType) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        Cell {
            cell_type,
            age: 0,
            metadata: 0,
            rng_seed: rng.gen::<u64>(),
            genes: Genes::default(),
        }
    }

    pub fn with_genes(cell_type: CellType, genes: Genes) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        Cell {
            cell_type,
            age: 0,
            metadata: 0,
            rng_seed: rng.gen::<u64>(),
            genes,
        }
    }

    pub fn to_u8(&self) -> u8 {
        self.cell_type.to_u8()
    }

    pub fn get_color(&self) -> (u8, u8, u8) {
        self.cell_type.get_color()
    }
}
