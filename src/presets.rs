use std::{str::FromStr, sync::OnceLock};

use serde_json::{json, Map, Value};

// &str -> Concrete Preset type -> transform -> return Enum variant (tiny in program runtime)

// pub trait __ {
//  fn ____() -> ____
// }

// PERF: consider having 'PresetT' actually just be the enum lol
// pub type PresetT = Map<String, Value>;

pub type PresetT = Map<String, Value>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Preset {
    Balanced,
    DenseForest,
    PlagueOutbreak,
    PredatorHeavy,
    ScarceResources,
    Recovery,
    SparseGenesis,
    RandomFallback,
}

// impl AsRef<str> for Preset {
//     fn as_ref(&self) -> &str {
//         match self {
//             Preset::Balanced => "balanced",
//             Preset::DenseForest => "dense_forest",
//             Preset::PlagueOutbreak => "plague_outbreak",
//             Preset::PredatorHeavy => "predator_heavy",
//             Preset::ScarceResources => "scarce_resources",
//             Preset::Recovery => "recovery",
//             Preset::SparseGenesis => "sparse_genesis",
//             Preset::RandomFallback => "random_fallback",
//         }
//     }
// }

/// Kind of naive way of delegating - I could likely design a
/// better trait for this, but this works for now.
///
impl Preset {
    pub fn name(self) -> &'static str {
        match self {
            Preset::Balanced => Balanced::name(),
            Preset::DenseForest => DenseForest::name(),
            Preset::PlagueOutbreak => PlagueOutbreak::name(),
            Preset::PredatorHeavy => PredatorHeavy::name(),
            Preset::ScarceResources => ScarceResources::name(),
            Preset::Recovery => Recovery::name(),
            Preset::SparseGenesis => SparseGenesis::name(),
            Preset::RandomFallback => RandomFallback::name(),
        }
    }

    pub fn data(self) -> PresetT {
        match self {
            Preset::Balanced => Balanced::data().clone(),
            Preset::DenseForest => DenseForest::data().clone(),
            Preset::PlagueOutbreak => PlagueOutbreak::data().clone(),
            Preset::PredatorHeavy => PredatorHeavy::data().clone(),
            Preset::ScarceResources => ScarceResources::data().clone(),
            Preset::Recovery => Recovery::data().clone(),
            Preset::SparseGenesis => SparseGenesis::data().clone(),
            Preset::RandomFallback => RandomFallback::data().clone(),
        }
    }
}

impl FromStr for Preset {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "balanced" => Ok(Preset::Balanced),
            "dense_forest" => Ok(Preset::DenseForest),
            "plague_outbreak" => Ok(Preset::PlagueOutbreak),
            "predator_heavy" => Ok(Preset::PredatorHeavy),
            "scarce_resources" => Ok(Preset::ScarceResources),
            "recovery" => Ok(Preset::Recovery),
            "sparse_genesis" => Ok(Preset::SparseGenesis),
            _ => Ok(Preset::RandomFallback),
        }
    }
}

impl<F> From<F> for Preset
where
    F: AsRef<str>,
{
    fn from(f: F) -> Self {
        Preset::from_str(f.as_ref()).unwrap_or(Preset::RandomFallback)
    }
}

pub trait PresetProvider {
    fn name() -> &'static str;
    fn variant() -> Preset;
    fn data() -> &'static PresetT;
}

// Zero-sized marker structs (one per preset)

/// Balanced ecosystem with healthy populations of all types
pub struct Balanced;

/// Dense forest with heavy vegetation
pub struct DenseForest;

/// Plague outbreak scenario
pub struct PlagueOutbreak;

/// Predator-heavy ecosystem
pub struct PredatorHeavy;

/// Scarce resources scenario
pub struct ScarceResources;

/// Recovery scenario (ecosystem bouncing back)
pub struct Recovery;

/// Sparse genesis: mostly empty with minimal good/bad cells
pub struct SparseGenesis;

/// Fallback preset: random initialization
pub struct RandomFallback;

// Implementations with lazy static initialization (std::sync::OnceLock, no extra deps)
// Data is allocated once per preset the first time it's accessed and shared thereafter.
impl PresetProvider for Balanced {
    fn name() -> &'static str {
        "balanced"
    }
    fn variant() -> Preset {
        Preset::Balanced
    }
    fn data() -> &'static PresetT {
        static DATA: OnceLock<PresetT> = OnceLock::new();
        DATA.get_or_init(|| {
            json!({
                "Black": 30,
                "Green": 20,
                "Orange": 15,
                "Gray": 5,
                "Purple": 3,
                "Red": 3,
                "White": 8,
                "Blue": 5,
                "Brown": 2,
                "Cyan": 4,
                "Yellow": 2,
                "Olive": 2,
                "Lime": 1,
                "Mint": 1,
                "Peach": 1,
                "Aqua": 1,
            })
            .as_object()
            .unwrap()
            .clone()
        })
    }
}

impl PresetProvider for DenseForest {
    fn name() -> &'static str {
        "dense_forest"
    }
    fn variant() -> Preset {
        Preset::DenseForest
    }
    fn data() -> &'static PresetT {
        static DATA: OnceLock<PresetT> = OnceLock::new();
        DATA.get_or_init(|| {
            json!({
                "Black": 10,
                "Green": 50,
                "Orange": 10,
                "Gray": 2,
                "Purple": 1,
                "Red": 1,
                "White": 5,
                "Blue": 10,
                "Brown": 3,
                "Cyan": 5,
                "Yellow": 2,
                "Olive": 5,
            })
            .as_object()
            .unwrap()
            .clone()
        })
    }
}

impl PresetProvider for PlagueOutbreak {
    fn name() -> &'static str {
        "plague_outbreak"
    }
    fn variant() -> Preset {
        Preset::PlagueOutbreak
    }
    fn data() -> &'static PresetT {
        static DATA: OnceLock<PresetT> = OnceLock::new();
        DATA.get_or_init(|| {
            json!({
                "Black": 20,
                "Green": 15,
                "Orange": 10,
                "Gray": 15,
                "Purple": 20,
                "Red": 3,
                "White": 5,
                "Blue": 3,
                "Yellow": 2,
                "Peach": 3,
                "Mint": 2,
                "Aqua": 2,
            })
            .as_object()
            .unwrap()
            .clone()
        })
    }
}

impl PresetProvider for PredatorHeavy {
    fn name() -> &'static str {
        "predator_heavy"
    }
    fn variant() -> Preset {
        Preset::PredatorHeavy
    }
    fn data() -> &'static PresetT {
        static DATA: OnceLock<PresetT> = OnceLock::new();
        DATA.get_or_init(|| {
            json!({
                "Black": 25,
                "Green": 15,
                "Orange": 12,
                "Gray": 3,
                "Purple": 2,
                "Red": 2,
                "White": 5,
                "Blue": 3,
                "Brown": 5,
                "Tan": 3,
                "Crimson": 8,
                "Maroon": 2,
                "Coral": 3,
                "Cyan": 3,
            })
            .as_object()
            .unwrap()
            .clone()
        })
    }
}

impl PresetProvider for ScarceResources {
    fn name() -> &'static str {
        "scarce_resources"
    }
    fn variant() -> Preset {
        Preset::ScarceResources
    }
    fn data() -> &'static PresetT {
        static DATA: OnceLock<PresetT> = OnceLock::new();
        DATA.get_or_init(|| {
            json!({
                "Black": 60,
                "Green": 8,
                "Orange": 5,
                "Gray": 3,
                "Purple": 2,
                "Red": 2,
                "White": 3,
                "Blue": 2,
                "Brown": 2,
                "Cyan": 2,
                "Yellow": 1,
            })
            .as_object()
            .unwrap()
            .clone()
        })
    }
}

impl PresetProvider for Recovery {
    fn name() -> &'static str {
        "recovery"
    }
    fn variant() -> Preset {
        Preset::Recovery
    }
    fn data() -> &'static PresetT {
        static DATA: OnceLock<PresetT> = OnceLock::new();
        DATA.get_or_init(|| {
            json!({
                "Black": 40,
                "Green": 25,
                "Orange": 8,
                "Gray": 8,
                "Purple": 3,
                "Red": 5,
                "White": 3,
                "Blue": 5,
                "Cyan": 5,
                "Khaki": 2,
                "Rust": 2,
                "Mint": 2,
                "Yellow": 2,
            })
            .as_object()
            .unwrap()
            .clone()
        })
    }
}

impl PresetProvider for SparseGenesis {
    fn name() -> &'static str {
        "sparse_genesis"
    }
    fn variant() -> Preset {
        Preset::SparseGenesis
    }
    fn data() -> &'static PresetT {
        static DATA: OnceLock<PresetT> = OnceLock::new();
        DATA.get_or_init(|| {
            json!({
                "Black": 98,
                "Green": 0.8,
                "Orange": 0.3,
                "Blue": 0.4,
                "Purple": 0.1,
                "Gray": 0.05,
                "Cyan": 0.2,
            })
            .as_object()
            .unwrap()
            .clone()
        })
    }
}

impl PresetProvider for RandomFallback {
    fn name() -> &'static str {
        "random_fallback"
    }
    fn variant() -> Preset {
        Preset::RandomFallback
    }
    fn data() -> &'static PresetT {
        static DATA: OnceLock<PresetT> = OnceLock::new();
        DATA.get_or_init(|| {
            json!({
                "Green": 0.5,
                "Orange": 0.2,
                "Blue": 0.3,
                "Purple": 0.1,
            })
            .as_object()
            .unwrap()
            .clone()
        })
    }
}

pub fn load_preset<S: AsRef<str>>(name: S) -> Option<PresetT> {
    preset_from_name(name).map(Preset::data)

    // match name {
    //     "balanced" => Some(balanced()),
    //     "dense_forest" => Some(dense_forest()),
    //     "plague_outbreak" => Some(plague_outbreak()),
    //     "predator_heavy" => Some(predator_heavy()),
    //     "scarce_resources" => Some(scarce_resources()),
    //     "recovery" => Some(recovery()),
    //     "sparse_genesis" => Some(sparse_genesis()),
    //     _ => None,
    // }
}

pub fn preset_from_name<S: AsRef<str>>(name: S) -> Option<Preset> {
    match name.as_ref() {
        "balanced" => Some(Preset::Balanced),
        "dense_forest" => Some(Preset::DenseForest),
        "plague_outbreak" => Some(Preset::PlagueOutbreak),
        "predator_heavy" => Some(Preset::PredatorHeavy),
        "scarce_resources" => Some(Preset::ScarceResources),
        "recovery" => Some(Preset::Recovery),
        "sparse_genesis" => Some(Preset::SparseGenesis),
        _ => Some(Preset::RandomFallback),
    }
}

// /// Balanced ecosystem with healthy populations of all types
// fn balanced() -> Preset {
//     let json = json!({
//         "Black": 30,
//         "Green": 20,
//         "Orange": 15,
//         "Gray": 5,
//         "Purple": 3,
//         "Red": 3,
//         "White": 8,
//         "Blue": 5,
//         "Brown": 2,
//         "Cyan": 4,
//         "Yellow": 2,
//         "Olive": 2,
//         "Lime": 1,
//         "Mint": 1,
//         "Peach": 1,
//         "Aqua": 1,
//     });
//     json.as_object().unwrap().clone()
// }
//
// /// Dense forest with heavy vegetation
// fn dense_forest() -> Preset {
//     let json = json!({
//         "Black": 10,
//         "Green": 50,
//         "Orange": 10,
//         "Gray": 2,
//         "Purple": 1,
//         "Red": 1,
//         "White": 5,
//         "Blue": 10,
//         "Brown": 3,
//         "Cyan": 5,
//         "Yellow": 2,
//         "Olive": 5,
//     });
//     json.as_object().unwrap().clone()
// }
//
// /// Plague outbreak scenario
// fn plague_outbreak() -> Preset {
//     let json = json!({
//         "Black": 20,
//         "Green": 15,
//         "Orange": 10,
//         "Gray": 15,
//         "Purple": 20,
//         "Red": 3,
//         "White": 5,
//         "Blue": 3,
//         "Yellow": 2,
//         "Peach": 3,
//         "Mint": 2,
//         "Aqua": 2,
//     });
//     json.as_object().unwrap().clone()
// }
//
// /// Predator-heavy ecosystem
// fn predator_heavy() -> Preset {
//     let json = json!({
//         "Black": 25,
//         "Green": 15,
//         "Orange": 12,
//         "Gray": 3,
//         "Purple": 2,
//         "Red": 2,
//         "White": 5,
//         "Blue": 3,
//         "Brown": 5,
//         "Tan": 3,
//         "Crimson": 8,
//         "Maroon": 2,
//         "Coral": 3,
//         "Cyan": 3,
//     });
//     json.as_object().unwrap().clone()
// }
//
// /// Scarce resources scenario
// fn scarce_resources() -> Preset {
//     let json = json!({
//         "Black": 60,
//         "Green": 8,
//         "Orange": 5,
//         "Gray": 3,
//         "Purple": 2,
//         "Red": 2,
//         "White": 3,
//         "Blue": 2,
//         "Brown": 2,
//         "Cyan": 2,
//         "Yellow": 1,
//     });
//     json.as_object().unwrap().clone()
// }
//
// /// Recovery scenario (ecosystem bouncing back)
// fn recovery() -> Preset {
//     let json = json!({
//         "Black": 40,
//         "Green": 25,
//         "Orange": 8,
//         "Gray": 8,
//         "Purple": 3,
//         "Red": 5,
//         "White": 3,
//         "Blue": 5,
//         "Cyan": 5,
//         "Khaki": 2,
//         "Rust": 2,
//         "Mint": 2,
//         "Yellow": 2,
//     });
//     json.as_object().unwrap().clone()
// }
//
// /// Sparse genesis: mostly empty with minimal good/bad cells
// fn sparse_genesis() -> Preset {
//     let json = json!({
//         "Black": 98,
//         "Green": 0.8,
//         "Orange": 0.3,
//         "Blue": 0.4,
//         "Purple": 0.1,
//         "Gray": 0.05,
//         "Cyan": 0.2,
//     });
//     json.as_object().unwrap().clone()
// }
