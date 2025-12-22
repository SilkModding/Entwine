use std::fs;
use std::io;
use std::path::PathBuf;
use tauri::Emitter;

const BEPINEX_VERSION: &str = "5.4.23.4";
const BEPINEX_DOWNLOAD_URL: &str = "https://github.com/BepInEx/BepInEx/releases/download/v5.4.23.4/BepInEx_win_x64_5.4.23.4.zip";

/// Check if BepInEx is installed
pub fn is_bepinex_installed(game_path: &str) -> bool {
    let game_dir = PathBuf::from(game_path);
    let bepinex_dir = game_dir.join("BepInEx");
    let doorstop_dll = game_dir.join("winhttp.dll");
    
    bepinex_dir.exists() && doorstop_dll.exists()
}

/// Get BepInEx version info
pub fn get_bepinex_version(game_path: &str) -> Result<String, String> {
    if is_bepinex_installed(game_path) {
        Ok(BEPINEX_VERSION.to_string())
    } else {
        Err("BepInEx is not installed".to_string())
    }
}

/// Install BepInEx with Silk's doorstop config
pub async fn install_bepinex(game_path: &str, window: tauri::Window) -> Result<(), String> {
    let game_dir = PathBuf::from(game_path);
    
    if !game_dir.exists() {
        return Err("Game path does not exist".to_string());
    }
    
    let _ = window.emit("install-progress", "Downloading BepInEx...");
    
    // Download BepInEx
    let client = reqwest::Client::new();
    let response = client
        .get(BEPINEX_DOWNLOAD_URL)
        .send()
        .await
        .map_err(|e| format!("Failed to download BepInEx: {}", e))?;
    
    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read BepInEx download: {}", e))?;
    
    let _ = window.emit("install-progress", "Extracting BepInEx...");
    
    // Create a temp file for the zip
    let temp_dir = std::env::temp_dir();
    let zip_path = temp_dir.join("bepinex_download.zip");
    
    fs::write(&zip_path, &bytes)
        .map_err(|e| format!("Failed to write temp file: {}", e))?;
    
    // Extract the zip to game directory
    let file = fs::File::open(&zip_path)
        .map_err(|e| format!("Failed to open zip file: {}", e))?;
    
    let mut archive = zip::ZipArchive::new(file)
        .map_err(|e| format!("Failed to read zip archive: {}", e))?;
    
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .map_err(|e| format!("Failed to read zip entry: {}", e))?;
        
        let file_path = file.mangled_name();
        let file_path_str = file_path.to_string_lossy();
        
        // Only extract files from the BepInEx folder (skip root directory files)
        if !file_path_str.starts_with("BepInEx/") && !file_path_str.starts_with("BepInEx\\") {
            continue;
        }
        
        let outpath = game_dir.join(&file_path);
        
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
    
    let _ = window.emit("install-progress", "BepInEx installed successfully!");
    
    Ok(())
}

/// Uninstall BepInEx
pub async fn uninstall_bepinex(game_path: &str, window: tauri::Window) -> Result<(), String> {
    let game_dir = PathBuf::from(game_path);
    
    if !game_dir.exists() {
        return Err("Game path does not exist".to_string());
    }
    
    let _ = window.emit("install-progress", "Uninstalling BepInEx...");
    
    // Remove BepInEx directory
    let bepinex_dir = game_dir.join("BepInEx");
    if bepinex_dir.exists() {
        fs::remove_dir_all(&bepinex_dir)
            .map_err(|e| format!("Failed to remove BepInEx directory: {}", e))?;
    }
    
    let _ = window.emit("install-progress", "BepInEx uninstalled successfully!");
    
    Ok(())
}
