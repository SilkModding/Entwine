use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModConfigFile {
    pub mod_id: String,
    pub mod_name: String,
    pub config: HashMap<String, JsonValue>,
}

/// Get the path to the Silk config directory
fn get_config_path(game_path: &str) -> PathBuf {
    PathBuf::from(game_path).join("Silk").join("Config")
}

/// Get the path to the mods config directory
fn get_mods_config_path(game_path: &str) -> PathBuf {
    get_config_path(game_path).join("Mods")
}

/// List all available mod config files
pub fn list_mod_configs(game_path: &str) -> Result<Vec<ModConfigFile>, String> {
    let mods_config_path = get_mods_config_path(game_path);
    
    if !mods_config_path.exists() {
        return Ok(Vec::new());
    }

    let mut configs = Vec::new();
    
    let entries = fs::read_dir(&mods_config_path)
        .map_err(|e| format!("Failed to read mods config directory: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();
        
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("yaml") {
            if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                match load_mod_config(game_path, file_stem) {
                    Ok(config) => {
                        // Try to get a friendly name from the config, otherwise use the file name
                        let mod_name = file_stem.to_string();
                        configs.push(ModConfigFile {
                            mod_id: file_stem.to_string(),
                            mod_name,
                            config,
                        });
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to load config for {}: {}", file_stem, e);
                    }
                }
            }
        }
    }

    // Sort by mod name
    configs.sort_by(|a, b| a.mod_name.cmp(&b.mod_name));

    Ok(configs)
}

/// Load a specific mod's config
pub fn load_mod_config(game_path: &str, mod_id: &str) -> Result<HashMap<String, JsonValue>, String> {
    let config_path = get_mods_config_path(game_path).join(format!("{}.yaml", mod_id));
    
    if !config_path.exists() {
        return Err(format!("Config file not found for mod: {}", mod_id));
    }

    let yaml_content = fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read config file: {}", e))?;

    let yaml_value: serde_yaml::Value = serde_yaml::from_str(&yaml_content)
        .map_err(|e| format!("Failed to parse YAML: {}", e))?;

    // Convert YAML to JSON for easier frontend handling
    let json_value: JsonValue = serde_json::to_value(&yaml_value)
        .map_err(|e| format!("Failed to convert to JSON: {}", e))?;

    if let JsonValue::Object(map) = json_value {
        Ok(map.into_iter().collect())
    } else {
        Err("Config file root must be an object".to_string())
    }
}

/// Set a specific value in a mod's config
pub fn set_mod_config_value(
    game_path: &str,
    mod_id: &str,
    key: &str,
    value: JsonValue,
) -> Result<(), String> {
    let config_path = get_mods_config_path(game_path).join(format!("{}.yaml", mod_id));
    
    if !config_path.exists() {
        return Err(format!("Config file not found for mod: {}", mod_id));
    }

    // Load current config
    let yaml_content = fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read config file: {}", e))?;

    let mut yaml_value: serde_yaml::Value = serde_yaml::from_str(&yaml_content)
        .map_err(|e| format!("Failed to parse YAML: {}", e))?;

    // Navigate to the key and set the value
    let keys: Vec<&str> = key.split('.').collect();
    set_nested_value(&mut yaml_value, &keys, value)?;

    // Write back to file
    let yaml_string = serde_yaml::to_string(&yaml_value)
        .map_err(|e| format!("Failed to serialize YAML: {}", e))?;

    fs::write(&config_path, yaml_string)
        .map_err(|e| format!("Failed to write config file: {}", e))?;

    Ok(())
}

/// Helper function to set a nested value in YAML
fn set_nested_value(
    current: &mut serde_yaml::Value,
    keys: &[&str],
    value: JsonValue,
) -> Result<(), String> {
    if keys.is_empty() {
        return Err("Empty key path".to_string());
    }

    if keys.len() == 1 {
        // Convert JsonValue to serde_yaml::Value
        let yaml_value: serde_yaml::Value = serde_json::from_value(value)
            .map_err(|e| format!("Failed to convert value: {}", e))?;
        
        if let serde_yaml::Value::Mapping(ref mut map) = current {
            map.insert(
                serde_yaml::Value::String(keys[0].to_string()),
                yaml_value,
            );
            Ok(())
        } else {
            Err("Cannot set value on non-object".to_string())
        }
    } else {
        if let serde_yaml::Value::Mapping(ref mut map) = current {
            let key_value = serde_yaml::Value::String(keys[0].to_string());
            
            if !map.contains_key(&key_value) {
                map.insert(key_value.clone(), serde_yaml::Value::Mapping(serde_yaml::Mapping::new()));
            }
            
            if let Some(next) = map.get_mut(&key_value) {
                set_nested_value(next, &keys[1..], value)
            } else {
                Err("Failed to navigate config structure".to_string())
            }
        } else {
            Err("Cannot navigate through non-object".to_string())
        }
    }
}

/// Reset a mod's config by deleting it (will be recreated with defaults on next load)
pub fn reset_mod_config(game_path: &str, mod_id: &str) -> Result<(), String> {
    let config_path = get_mods_config_path(game_path).join(format!("{}.yaml", mod_id));
    
    if config_path.exists() {
        fs::remove_file(&config_path)
            .map_err(|e| format!("Failed to delete config file: {}", e))?;
        Ok(())
    } else {
        Err(format!("Config file not found for mod: {}", mod_id))
    }
}
