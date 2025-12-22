use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LaunchMethod {
    Steam,
    Executable,
}

impl Default for LaunchMethod {
    fn default() -> Self {
        LaunchMethod::Steam
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub launch_method: LaunchMethod,
}

/// Get the path to the settings file
fn get_settings_path() -> Result<PathBuf, String> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| "Failed to get config directory".to_string())?;
    
    let app_config_dir = config_dir.join("entwine");
    fs::create_dir_all(&app_config_dir)
        .map_err(|e| format!("Failed to create config directory: {}", e))?;
    
    Ok(app_config_dir.join("settings.json"))
}

/// Load app settings
pub fn load_settings() -> Result<AppSettings, String> {
    let settings_path = get_settings_path()?;
    
    if !settings_path.exists() {
        return Ok(AppSettings::default());
    }
    
    let content = fs::read_to_string(&settings_path)
        .map_err(|e| format!("Failed to read settings file: {}", e))?;
    
    let settings: AppSettings = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse settings JSON: {}", e))?;
    
    Ok(settings)
}

/// Save app settings
pub fn save_settings(settings: &AppSettings) -> Result<(), String> {
    let settings_path = get_settings_path()?;
    
    let content = serde_json::to_string_pretty(settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;
    
    fs::write(&settings_path, content)
        .map_err(|e| format!("Failed to write settings file: {}", e))?;
    
    Ok(())
}

/// Launch the game
pub fn launch_game(game_path: &str, launch_method: &LaunchMethod) -> Result<(), String> {
    match launch_method {
        LaunchMethod::Steam => {
            // Launch via Steam with App ID 1329500
            launch_via_steam()
        }
        LaunchMethod::Executable => {
            // Launch via direct executable
            launch_via_executable(game_path)
        }
    }
}

fn launch_via_steam() -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg("steam://rungameid/1329500")
            .spawn()
            .map_err(|e| format!("Failed to launch game via Steam: {}", e))?;
    }
    
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(&["/C", "start", "steam://rungameid/1329500"])
            .spawn()
            .map_err(|e| format!("Failed to launch game via Steam: {}", e))?;
    }
    
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg("steam://rungameid/1329500")
            .spawn()
            .map_err(|e| format!("Failed to launch game via Steam: {}", e))?;
    }
    
    Ok(())
}

fn launch_via_executable(game_path: &str) -> Result<(), String> {
    let game_path = PathBuf::from(game_path);
    let exe_path = game_path.join("SpiderHeckApp.exe");
    
    if !exe_path.exists() {
        return Err(format!("Game executable not found at: {}", exe_path.display()));
    }
    
    #[cfg(target_os = "linux")]
    {
        // On Linux, we might need to use Wine or Proton
        // For now, just try to execute it directly
        std::process::Command::new(&exe_path)
            .current_dir(&game_path)
            .spawn()
            .map_err(|e| format!("Failed to launch game executable: {}", e))?;
    }
    
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new(&exe_path)
            .current_dir(&game_path)
            .spawn()
            .map_err(|e| format!("Failed to launch game executable: {}", e))?;
    }
    
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&exe_path)
            .spawn()
            .map_err(|e| format!("Failed to launch game executable: {}", e))?;
    }
    
    Ok(())
}
