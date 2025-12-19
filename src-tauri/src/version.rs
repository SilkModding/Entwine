use semver::Version;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;
use tauri::Emitter;

const SILK_VERSION_URL: &str = "https://raw.githubusercontent.com/SilkModding/Silk/master/version";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SilkVersion {
    pub version: String,
    pub download_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModVersionInfo {
    pub mod_id: String,
    pub version: String,
    pub silk_version: String,
    pub min_silk_version: Option<String>,
    pub max_silk_version: Option<String>,
}

/// Get the currently installed Silk version
pub fn get_installed_silk_version(game_path: &str) -> Result<String, String> {
    let version_file = PathBuf::from(game_path).join("Silk").join("version.txt");
    
    if !version_file.exists() {
        return Err("Silk version file not found".to_string());
    }
    
    fs::read_to_string(&version_file)
        .map(|s| s.trim().to_string())
        .map_err(|e| format!("Failed to read version file: {}", e))
}

/// Get the latest Silk version from GitHub
pub async fn get_latest_silk_version() -> Result<String, String> {
    let client = reqwest::Client::new();
    
    let response = client
        .get(SILK_VERSION_URL)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch latest version: {}", e))?;
    
    let version = response
        .text()
        .await
        .map_err(|e| format!("Failed to read version: {}", e))?;
    
    Ok(version.trim().to_string())
}

/// Check if an update is available
pub async fn check_for_updates(game_path: &str) -> Result<Option<SilkVersion>, String> {
    let current_version = get_installed_silk_version(game_path)?;
    let latest_version = get_latest_silk_version().await?;
    
    if is_newer_version(&latest_version, &current_version) {
        let download_url = format!("https://github.com/SilkModding/Silk/releases/download/v{}/Silk-v{}.zip", latest_version, latest_version);
        
        Ok(Some(SilkVersion {
            version: latest_version,
            download_url,
        }))
    } else {
        Ok(None)
    }
}

/// Check if version_a is newer than version_b
pub fn is_newer_version(version_a: &str, version_b: &str) -> bool {
    match (Version::parse(version_a), Version::parse(version_b)) {
        (Ok(a), Ok(b)) => a > b,
        _ => false,
    }
}

/// List available Silk versions (for version swapping)
pub async fn list_available_versions() -> Result<Vec<String>, String> {
    // For now, we'll return a hardcoded list of known stable versions
    // In the future, this could scrape GitHub releases
    Ok(vec![
        "0.6.1".to_string(),
        "0.6.0".to_string(),
        "0.5.0".to_string(),
    ])
}

/// Download a specific Silk version
pub async fn download_silk_version(
    version: &str,
    game_path: &str,
    window: tauri::Window,
) -> Result<(), String> {
    let game_dir = PathBuf::from(game_path);
    let download_url = format!("https://github.com/SilkModding/Silk/releases/download/v{}/Silk-v{}.zip", version, version);
    
    let _ = window.emit("install-progress", format!("Downloading Silk v{}...", version));
    
    // Download Silk
    let client = reqwest::Client::new();
    let response = client
        .get(&download_url)
        .send()
        .await
        .map_err(|e| format!("Failed to download Silk: {}", e))?;
    
    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read Silk download: {}", e))?;
    
    let _ = window.emit("install-progress", "Extracting Silk...");
    
    // Create a temp file for the zip
    let temp_dir = std::env::temp_dir();
    let zip_path = temp_dir.join("silk_download.zip");
    
    fs::write(&zip_path, &bytes)
        .map_err(|e| format!("Failed to write temp file: {}", e))?;
    
    // Extract the zip directly to game directory
    let file = fs::File::open(&zip_path)
        .map_err(|e| format!("Failed to open zip file: {}", e))?;
    
    let mut archive = zip::ZipArchive::new(file)
        .map_err(|e| format!("Failed to read zip archive: {}", e))?;
    
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .map_err(|e| format!("Failed to read zip entry: {}", e))?;
        
        let outpath = game_dir.join(file.mangled_name());
        
        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create parent directory: {}", e))?;
            }
            
            let mut outfile = fs::File::create(&outpath)
                .map_err(|e| format!("Failed to create file: {}", e))?;
            
            io::copy(&mut file, &mut outfile)
                .map_err(|e| format!("Failed to write file: {}", e))?;
        }
    }
    
    // Clean up temp file
    let _ = fs::remove_file(&zip_path);
    
    // Write version file
    let version_file = game_dir.join("Silk").join("version.txt");
    fs::write(&version_file, version)
        .map_err(|e| format!("Failed to write version file: {}", e))?;
    
    let _ = window.emit("install-progress", format!("Silk v{} installed successfully!", version));
    
    Ok(())
}

/// Check if a mod is compatible with the installed Silk version
pub fn check_mod_compatibility(
    installed_silk_version: &str,
    mod_version_info: &ModVersionInfo,
) -> Result<bool, String> {
    let silk_version = Version::parse(installed_silk_version)
        .map_err(|e| format!("Invalid Silk version: {}", e))?;
    
    // Check minimum version
    if let Some(min_ver) = &mod_version_info.min_silk_version {
        let min_version = Version::parse(min_ver)
            .map_err(|e| format!("Invalid minimum version: {}", e))?;
        
        if silk_version < min_version {
            return Ok(false);
        }
    }
    
    // Check maximum version
    if let Some(max_ver) = &mod_version_info.max_silk_version {
        let max_version = Version::parse(max_ver)
            .map_err(|e| format!("Invalid maximum version: {}", e))?;
        
        if silk_version > max_version {
            return Ok(false);
        }
    }
    
    Ok(true)
}

/// Get mod version info from metadata
pub fn get_mod_version_info(mod_id: &str, mods_path: &str) -> Result<ModVersionInfo, String> {
    let metadata_path = PathBuf::from(mods_path).join(".entwine_metadata.json");
    
    if !metadata_path.exists() {
        return Err("Metadata file not found".to_string());
    }
    
    let content = fs::read_to_string(&metadata_path)
        .map_err(|e| format!("Failed to read metadata: {}", e))?;
    
    let metadata: std::collections::HashMap<String, serde_json::Value> =
        serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse metadata: {}", e))?;
    
    let mod_data = metadata
        .get(mod_id)
        .ok_or_else(|| format!("Mod {} not found in metadata", mod_id))?;
    
    let version = mod_data
        .get("version")
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown")
        .to_string();
    
    let silk_version = mod_data
        .get("silkVersion")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    
    let min_silk_version = mod_data
        .get("minSilkVersion")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    
    let max_silk_version = mod_data
        .get("maxSilkVersion")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    
    Ok(ModVersionInfo {
        mod_id: mod_id.to_string(),
        version,
        silk_version: silk_version.unwrap_or_else(|| "Unknown".to_string()),
        min_silk_version,
        max_silk_version,
    })
}
