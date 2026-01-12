use serde_json::{json, Map, Value};

pub type Preset = Map<String, Value>;

pub fn load_preset(name: &str) -> Option<Preset> {
    match name {
        "balanced" => Some(balanced()),
        "dense_forest" => Some(dense_forest()),
        "plague_outbreak" => Some(plague_outbreak()),
        "predator_heavy" => Some(predator_heavy()),
        "scarce_resources" => Some(scarce_resources()),
        "recovery" => Some(recovery()),
        "sparse_genesis" => Some(sparse_genesis()),
        _ => None,
    }
}

/// Balanced ecosystem with healthy populations of all types
fn balanced() -> Preset {
    let json = json!({
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
    });
    json.as_object().unwrap().clone()
}

/// Dense forest with heavy vegetation
fn dense_forest() -> Preset {
    let json = json!({
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
    });
    json.as_object().unwrap().clone()
}

/// Plague outbreak scenario
fn plague_outbreak() -> Preset {
    let json = json!({
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
    });
    json.as_object().unwrap().clone()
}

/// Predator-heavy ecosystem
fn predator_heavy() -> Preset {
    let json = json!({
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
    });
    json.as_object().unwrap().clone()
}

/// Scarce resources scenario
fn scarce_resources() -> Preset {
    let json = json!({
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
    });
    json.as_object().unwrap().clone()
}

/// Recovery scenario (ecosystem bouncing back)
fn recovery() -> Preset {
    let json = json!({
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
    });
    json.as_object().unwrap().clone()
}

/// Sparse genesis: mostly empty with minimal good/bad cells
fn sparse_genesis() -> Preset {
    let json = json!({
        "Black": 98,
        "Green": 0.8,
        "Orange": 0.3,
        "Blue": 0.4,
        "Purple": 0.1,
        "Gray": 0.05,
        "Cyan": 0.2,
    });
    json.as_object().unwrap().clone()
}
