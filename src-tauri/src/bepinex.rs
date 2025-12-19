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
    
    let _ = window.emit("install-progress", "Configuring BepInEx for Silk...");
    
    // Create/update doorstop_config.ini to work with Silk if needed
    let doorstop_config_path = game_dir.join("doorstop_config.ini");
    if doorstop_config_path.exists() {
        // Read existing config and preserve it if it already references Silk
        let existing = fs::read_to_string(&doorstop_config_path)
            .map_err(|e| format!("Failed to read existing doorstop config: {}", e))?;

        if existing.contains("Silk\\Silk.dll") || existing.contains("silkAssembly") || existing.contains("targetAssembly=Silk") {
            let _ = window.emit("install-progress", "Existing Silk doorstop_config.ini detected; preserving it.");
        } else {
            let doorstop_config = create_silk_doorstop_config(game_path)?;
            fs::write(&doorstop_config_path, doorstop_config)
                .map_err(|e| format!("Failed to write doorstop config: {}", e))?;
        }
    } else {
        let doorstop_config = create_silk_doorstop_config(game_path)?;
        fs::write(&doorstop_config_path, doorstop_config)
            .map_err(|e| format!("Failed to write doorstop config: {}", e))?;
    }
    
    Ok(())
}

/// Create doorstop_config.ini that works with Silk
fn create_silk_doorstop_config(game_path: &str) -> Result<String, String> {
    let game_dir = PathBuf::from(game_path);
    let silk_dll = game_dir.join("Silk").join("Silk.dll");
    
    // Check if Silk is installed
    if !silk_dll.exists() {
        return Err("Silk must be installed before installing BepInEx".to_string());
    }
    
    // Create config that loads both BepInEx and Silk
    let config = format!(
        r#"[General]
# Enable Doorstop
enabled=true

# Path to the assembly to load and execute
# For BepInEx, this is the preloader
targetAssembly=BepInEx\core\BepInEx.Preloader.dll

# If enabled, Doorstop will redirect output to a log file
redirectOutputLog=false

# If enabled, Doorstop will ignore disabled assemblies
ignoreDisableSwitch=true

[Silk]
# Silk will be loaded alongside BepInEx
silkEnabled=true
silkAssembly=Silk\Silk.dll
"#
    );
    
    Ok(config)
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
    
    // Remove doorstop DLL if it's from BepInEx (not Silk)
    // We need to be careful here - if Silk is installed, we don't want to remove winhttp.dll
    let silk_dir = game_dir.join("Silk");
    if !silk_dir.exists() {
        let doorstop_dll = game_dir.join("winhttp.dll");
        if doorstop_dll.exists() {
            fs::remove_file(&doorstop_dll)
                .map_err(|e| format!("Failed to remove doorstop DLL: {}", e))?;
        }
    }
    
    // Handle doorstop_config.ini carefully: only modify if it references BepInEx and not Silk
    let doorstop_config_path = game_dir.join("doorstop_config.ini");
    if doorstop_config_path.exists() {
        let existing = fs::read_to_string(&doorstop_config_path)
            .map_err(|e| format!("Failed to read doorstop config: {}", e))?;

        // If the file references BepInEx, we can safely modify/remove it; otherwise preserve it
        if existing.contains("BepInEx\\core\\BepInEx.Preloader.dll") {
            if silk_dir.exists() {
                // If Silk is also referenced, keep existing (it already handles both)
                if existing.contains("Silk\\Silk.dll") {
                    let _ = window.emit("install-progress", "Doorstop config contains Silk; preserving existing config.");
                } else {
                    // Replace with Silk-only config
                    let silk_config = create_silk_only_doorstop_config();
                    fs::write(&doorstop_config_path, silk_config)
                        .map_err(|e| format!("Failed to restore doorstop config: {}", e))?;
                }
            } else {
                // No Silk installed, remove doorstop config entirely
                fs::remove_file(&doorstop_config_path)
                    .map_err(|e| format!("Failed to remove doorstop config: {}", e))?;
            }
        } else {
            // File doesn't reference BepInEx: preserve it
            let _ = window.emit("install-progress", "Doorstop config does not reference BepInEx; preserving existing config.");
        }
    }
    
    let _ = window.emit("install-progress", "BepInEx uninstalled successfully!");
    
    Ok(())
}

/// Create doorstop_config.ini for Silk only
fn create_silk_only_doorstop_config() -> String {
    r#"[General]
# Enable Doorstop
enabled=true

# Path to the assembly to load and execute
targetAssembly=Silk\Silk.dll

# If enabled, Doorstop will redirect output to a log file
redirectOutputLog=false
"#
    .to_string()
}
