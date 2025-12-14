use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;
use tauri::Emitter;

const SILK_DOWNLOAD_URL: &str = "https://github.com/SilkModding/Silk/releases/download/v0.6.1/Silk-v0.6.1.zip";
const MODS_API_URL: &str = "https://silk.abstractmelon.net/api/mods";
const MODS_BASE_URL: &str = "https://silk.abstractmelon.net";

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Mod {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub file_name: String,
    pub file_path: String,
    pub file_size: u64,
    pub icon_path: String,
    pub upload_date: String,
    pub downloads: u64,
    pub last_downloaded: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InstalledMod {
    pub id: String,
    pub name: String,
    pub file_name: String,
    pub enabled: bool,
    pub version: String,
    pub author: String,
    pub description: String,
    pub icon_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppStatus {
    pub silk_installed: bool,
    pub game_path: Option<String>,
    pub mods_path: Option<String>,
}

// Get common Steam library paths
fn get_steam_library_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    
    #[cfg(target_os = "linux")]
    {
        if let Some(home) = dirs::home_dir() {
            // Default Steam path
            paths.push(home.join(".steam/steam/steamapps/common"));
            paths.push(home.join(".local/share/Steam/steamapps/common"));
            // Flatpak Steam
            paths.push(home.join(".var/app/com.valvesoftware.Steam/.steam/steam/steamapps/common"));
            paths.push(home.join(".var/app/com.valvesoftware.Steam/.local/share/Steam/steamapps/common"));
        }
        
        // Check for additional library folders
        // Common external drive mount points
        for entry in fs::read_dir("/run/media").into_iter().flatten() {
            if let Ok(user_entry) = entry {
                for drive_entry in fs::read_dir(user_entry.path()).into_iter().flatten() {
                    if let Ok(drive) = drive_entry {
                        paths.push(drive.path().join("SteamLibrary/steamapps/common"));
                        paths.push(drive.path().join("steamapps/common"));
                    }
                }
            }
        }
        
        // Check /media as well
        for entry in fs::read_dir("/media").into_iter().flatten() {
            if let Ok(user_entry) = entry {
                for drive_entry in fs::read_dir(user_entry.path()).into_iter().flatten() {
                    if let Ok(drive) = drive_entry {
                        paths.push(drive.path().join("SteamLibrary/steamapps/common"));
                        paths.push(drive.path().join("steamapps/common"));
                    }
                }
            }
        }
    }
    
    #[cfg(target_os = "windows")]
    {
        paths.push(PathBuf::from("C:\\Program Files (x86)\\Steam\\steamapps\\common"));
        paths.push(PathBuf::from("C:\\Program Files\\Steam\\steamapps\\common"));
        
        // Check other drives
        for letter in 'D'..='Z' {
            paths.push(PathBuf::from(format!("{}:\\SteamLibrary\\steamapps\\common", letter)));
            paths.push(PathBuf::from(format!("{}:\\Steam\\steamapps\\common", letter)));
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        if let Some(home) = dirs::home_dir() {
            paths.push(home.join("Library/Application Support/Steam/steamapps/common"));
        }
    }
    
    paths
}

// Find SpiderHeck installation
fn find_spiderheck_path() -> Option<PathBuf> {
    for steam_path in get_steam_library_paths() {
        let spiderheck_path = steam_path.join("SpiderHeck");
        if spiderheck_path.exists() {
            return Some(spiderheck_path);
        }
    }
    None
}

#[tauri::command]
async fn get_app_status() -> Result<AppStatus, String> {
    let game_path = find_spiderheck_path();
    
    let silk_installed = game_path
        .as_ref()
        .map(|p| p.join("winhttp.dll").exists() || p.join("Silk").exists())
        .unwrap_or(false);
    
    let mods_path = game_path.as_ref().map(|p| {
        p.join("Silk/Mods").to_string_lossy().to_string()
    });
    
    Ok(AppStatus {
        silk_installed,
        game_path: game_path.map(|p| p.to_string_lossy().to_string()),
        mods_path,
    })
}

#[tauri::command]
async fn set_game_path(path: String) -> Result<AppStatus, String> {
    let game_path = PathBuf::from(&path);
    
    if !game_path.exists() {
        return Err("Path does not exist".to_string());
    }
    
    // Check if this looks like a SpiderHeck installation
    let has_exe = game_path.join("SpiderHeck.exe").exists() 
        || game_path.join("SpiderHeck.x86_64").exists()
        || game_path.join("SpiderHeck").exists();
    
    if !has_exe {
        return Err("This doesn't appear to be a SpiderHeck installation".to_string());
    }
    
    let silk_installed = game_path.join("winhttp.dll").exists() || game_path.join("Silk").exists();
    
    let mods_path = game_path.join("Silk/Mods").to_string_lossy().to_string();
    
    Ok(AppStatus {
        silk_installed,
        game_path: Some(path),
        mods_path: Some(mods_path),
    })
}

#[tauri::command]
async fn fetch_mods() -> Result<Vec<Mod>, String> {
    let client = reqwest::Client::new();
    
    let response = client
        .get(MODS_API_URL)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch mods: {}", e))?;
    
    let mods: Vec<Mod> = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse mods: {}", e))?;
    
    Ok(mods)
}

#[tauri::command]
async fn install_silk(game_path: String, window: tauri::Window) -> Result<(), String> {
    let game_dir = PathBuf::from(&game_path);
    
    if !game_dir.exists() {
        return Err("Game path does not exist".to_string());
    }
    
    let _ = window.emit("install-progress", "Downloading Silk...");
    
    // Download Silk
    let client = reqwest::Client::new();
    let response = client
        .get(SILK_DOWNLOAD_URL)
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
    
    // Create Mods directory if it doesn't exist
    let mods_dir = game_dir.join("Silk/Mods");
    fs::create_dir_all(&mods_dir)
        .map_err(|e| format!("Failed to create Mods directory: {}", e))?;
    
    let _ = window.emit("install-progress", "Silk installed successfully!");
    
    Ok(())
}

#[tauri::command]
async fn get_installed_mods(mods_path: String) -> Result<Vec<InstalledMod>, String> {
    let mods_dir = PathBuf::from(&mods_path);
    
    if !mods_dir.exists() {
        return Ok(Vec::new());
    }
    
    let mut installed_mods = Vec::new();
    let metadata_path = mods_dir.join(".entwine_metadata.json");
    
    // Load existing metadata
    let metadata: std::collections::HashMap<String, InstalledMod> = if metadata_path.exists() {
        let content = fs::read_to_string(&metadata_path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        std::collections::HashMap::new()
    };
    
    for entry in fs::read_dir(&mods_dir).map_err(|e| format!("Failed to read mods directory: {}", e))? {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let path = entry.path();
        let file_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        
        // Skip metadata file and hidden files
        if file_name.starts_with('.') {
            continue;
        }
        
        let is_enabled = file_name.to_lowercase().ends_with(".dll");
        let is_disabled = file_name.to_lowercase().ends_with(".dll.disabled");
        let is_mod_folder = path.is_dir();
        
        if is_enabled || is_disabled || is_mod_folder {
            // Try to find metadata for this mod
            let base_name = file_name
                .trim_end_matches(".disabled")
                .trim_end_matches(".dll")
                .to_string();
            
            if let Some(mod_meta) = metadata.get(&base_name) {
                installed_mods.push(InstalledMod {
                    enabled: is_enabled || is_mod_folder,
                    file_name: file_name.clone(),
                    ..mod_meta.clone()
                });
            } else {
                // Create basic metadata from filename
                installed_mods.push(InstalledMod {
                    id: base_name.clone(),
                    name: base_name.clone(),
                    file_name,
                    enabled: is_enabled || is_mod_folder,
                    version: "Unknown".to_string(),
                    author: "Unknown".to_string(),
                    description: "Locally installed mod".to_string(),
                    icon_path: String::new(),
                });
            }
        }
    }
    
    Ok(installed_mods)
}

#[tauri::command]
async fn install_mod(
    mod_info: Mod,
    mods_path: String,
    window: tauri::Window,
) -> Result<(), String> {
    let mods_dir = PathBuf::from(&mods_path);
    
    // Create mods directory if it doesn't exist
    fs::create_dir_all(&mods_dir)
        .map_err(|e| format!("Failed to create mods directory: {}", e))?;
    
    let _ = window.emit("install-progress", format!("Downloading {}...", mod_info.name));
    
    // Download the mod
    let download_url = format!("{}{}", MODS_BASE_URL, mod_info.file_path);
    
    let client = reqwest::Client::new();
    let response = client
        .get(&download_url)
        .send()
        .await
        .map_err(|e| format!("Failed to download mod: {}", e))?;
    
    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read mod download: {}", e))?;
    
    // Determine if it's a zip or dll based on filename
    let is_zip = mod_info.file_name.to_lowercase().ends_with(".zip") 
        || mod_info.file_name.to_lowercase().ends_with(".silkmod");
    
    if is_zip {
        let _ = window.emit("install-progress", format!("Extracting {}...", mod_info.name));
        
        // Extract zip to a subfolder
        let mod_folder = mods_dir.join(&mod_info.name);
        fs::create_dir_all(&mod_folder)
            .map_err(|e| format!("Failed to create mod folder: {}", e))?;
        
        let cursor = std::io::Cursor::new(&bytes);
        let mut archive = zip::ZipArchive::new(cursor)
            .map_err(|e| format!("Failed to read mod archive: {}", e))?;
        
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)
                .map_err(|e| format!("Failed to read zip entry: {}", e))?;
            
            let outpath = mod_folder.join(file.mangled_name());
            
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
    } else {
        // It's a DLL, save directly
        let dll_path = mods_dir.join(&mod_info.file_name);
        fs::write(&dll_path, &bytes)
            .map_err(|e| format!("Failed to write mod file: {}", e))?;
    }
    
    // Save metadata
    save_mod_metadata(&mods_dir, &mod_info)?;
    
    let _ = window.emit("install-progress", format!("{} installed successfully!", mod_info.name));
    
    Ok(())
}

fn save_mod_metadata(mods_dir: &PathBuf, mod_info: &Mod) -> Result<(), String> {
    let metadata_path = mods_dir.join(".entwine_metadata.json");
    
    let mut metadata: std::collections::HashMap<String, InstalledMod> = if metadata_path.exists() {
        let content = fs::read_to_string(&metadata_path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        std::collections::HashMap::new()
    };
    
    let base_name = mod_info.file_name
        .trim_end_matches(".dll")
        .trim_end_matches(".zip")
        .trim_end_matches(".silkmod")
        .to_string();
    
    metadata.insert(base_name, InstalledMod {
        id: mod_info.id.clone(),
        name: mod_info.name.clone(),
        file_name: mod_info.file_name.clone(),
        enabled: true,
        version: mod_info.version.clone(),
        author: mod_info.author.clone(),
        description: mod_info.description.clone(),
        icon_path: format!("{}{}", MODS_BASE_URL, mod_info.icon_path),
    });
    
    let json = serde_json::to_string_pretty(&metadata)
        .map_err(|e| format!("Failed to serialize metadata: {}", e))?;
    
    fs::write(&metadata_path, json)
        .map_err(|e| format!("Failed to write metadata: {}", e))?;
    
    Ok(())
}

#[tauri::command]
async fn toggle_mod(mods_path: String, file_name: String, enable: bool) -> Result<(), String> {
    let mods_dir = PathBuf::from(&mods_path);
    let current_path = mods_dir.join(&file_name);
    
    if !current_path.exists() {
        return Err("Mod file not found".to_string());
    }
    
    // Handle folders differently - we can't easily disable them, so we'd need to rename
    if current_path.is_dir() {
        if enable {
            // Already enabled as a folder
            return Ok(());
        } else {
            // Rename folder to add .disabled
            let new_path = mods_dir.join(format!("{}.disabled", file_name));
            fs::rename(&current_path, &new_path)
                .map_err(|e| format!("Failed to disable mod: {}", e))?;
        }
        return Ok(());
    }
    
    let new_name = if enable {
        // Enable: remove .disabled suffix
        file_name.trim_end_matches(".disabled").to_string()
    } else {
        // Disable: add .disabled suffix
        if file_name.ends_with(".disabled") {
            file_name.clone()
        } else {
            format!("{}.disabled", file_name)
        }
    };
    
    let new_path = mods_dir.join(&new_name);
    
    if current_path != new_path {
        fs::rename(&current_path, &new_path)
            .map_err(|e| format!("Failed to toggle mod: {}", e))?;
    }
    
    Ok(())
}

#[tauri::command]
async fn uninstall_mod(mods_path: String, file_name: String) -> Result<(), String> {
    let mods_dir = PathBuf::from(&mods_path);
    let mod_path = mods_dir.join(&file_name);
    
    if !mod_path.exists() {
        return Err("Mod file not found".to_string());
    }
    
    if mod_path.is_dir() {
        fs::remove_dir_all(&mod_path)
            .map_err(|e| format!("Failed to remove mod folder: {}", e))?;
    } else {
        fs::remove_file(&mod_path)
            .map_err(|e| format!("Failed to remove mod file: {}", e))?;
    }
    
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            get_app_status,
            set_game_path,
            fetch_mods,
            install_silk,
            get_installed_mods,
            install_mod,
            toggle_mod,
            uninstall_mod,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
