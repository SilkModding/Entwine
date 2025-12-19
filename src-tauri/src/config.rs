use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConfigValue {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Array(Vec<ConfigValue>),
    Object(HashMap<String, ConfigValue>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModConfig {
    pub mod_id: String,
    pub config: HashMap<String, ConfigValue>,
}

/// Get the path to the Silk config directory
pub fn get_config_path(game_path: &str) -> PathBuf {
    PathBuf::from(game_path).join("Silk").join("Config")
}

/// Get the path to a specific mod's config file
pub fn get_mod_config_path(game_path: &str, mod_id: &str) -> PathBuf {
    get_config_path(game_path)
        .join("Mods")
        .join(format!("{}.yaml", mod_id))
}

/// Load a mod's config file
pub fn load_mod_config(game_path: &str, mod_id: &str) -> Result<HashMap<String, ConfigValue>, String> {
    let config_path = get_mod_config_path(game_path, mod_id);
    
    if !config_path.exists() {
        return Ok(HashMap::new());
    }
    
    let content = fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read config file: {}", e))?;
    
    let config: HashMap<String, ConfigValue> = serde_yaml::from_str(&content)
        .map_err(|e| format!("Failed to parse config YAML: {}", e))?;
    
    Ok(config)
}

/// Save a mod's config file
pub fn save_mod_config(
    game_path: &str,
    mod_id: &str,
    config: &HashMap<String, ConfigValue>,
) -> Result<(), String> {
    let config_path = get_mod_config_path(game_path, mod_id);
    
    // Create directories if they don't exist
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }
    
    let yaml = serde_yaml::to_string(config)
        .map_err(|e| format!("Failed to serialize config to YAML: {}", e))?;
    
    fs::write(&config_path, yaml)
        .map_err(|e| format!("Failed to write config file: {}", e))?;
    
    Ok(())
}



/// Set a specific config value in a mod's config
pub fn set_config_value(
    config: &mut HashMap<String, ConfigValue>,
    key: &str,
    value: ConfigValue,
) {
    let keys: Vec<&str> = key.split('.').collect();
    
    if keys.len() == 1 {
        config.insert(keys[0].to_string(), value);
        return;
    }
    
    let mut current = config
        .entry(keys[0].to_string())
        .or_insert_with(|| ConfigValue::Object(HashMap::new()));
    
    for key in &keys[1..keys.len() - 1] {
        if let ConfigValue::Object(obj) = current {
            current = obj
                .entry(key.to_string())
                .or_insert_with(|| ConfigValue::Object(HashMap::new()));
        }
    }
    
    if let ConfigValue::Object(obj) = current {
        obj.insert(keys.last().unwrap().to_string(), value);
    }
}

/// List all mod configs
pub fn list_mod_configs(game_path: &str) -> Result<Vec<String>, String> {
    let mods_config_path = get_config_path(game_path).join("Mods");
    
    if !mods_config_path.exists() {
        return Ok(Vec::new());
    }
    
    let mut configs = Vec::new();
    
    for entry in fs::read_dir(&mods_config_path)
        .map_err(|e| format!("Failed to read mods config directory: {}", e))?
    {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();
        
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("yaml") {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                configs.push(stem.to_string());
            }
        }
    }
    
    Ok(configs)
}

/// Delete a mod's config file
pub fn delete_mod_config(game_path: &str, mod_id: &str) -> Result<(), String> {
    let config_path = get_mod_config_path(game_path, mod_id);
    
    if config_path.exists() {
        fs::remove_file(&config_path)
            .map_err(|e| format!("Failed to delete config file: {}", e))?;
    }
    
    Ok(())
}
