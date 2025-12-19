import { invoke } from '@tauri-apps/api/core';
import type { Mod, InstalledMod, AppStatus, ModConfig, ConfigValue, SilkVersion, ModVersionInfo } from './types';

const MODS_BASE_URL = 'https://silk.abstractmelon.net';

export async function getAppStatus(): Promise<AppStatus> {
  return invoke('get_app_status');
}

export async function setGamePath(path: string): Promise<AppStatus> {
  return invoke('set_game_path', { path });
}

export async function fetchMods(): Promise<Mod[]> {
  return invoke('fetch_mods');
}

export async function installSilk(gamePath: string): Promise<void> {
  return invoke('install_silk', { gamePath });
}

export async function uninstallSilk(gamePath: string): Promise<void> {
  return invoke('uninstall_silk', { gamePath });
}

export async function getInstalledMods(modsPath: string): Promise<InstalledMod[]> {
  return invoke('get_installed_mods', { modsPath });
}

export async function installMod(modInfo: Mod, modsPath: string): Promise<void> {
  return invoke('install_mod', { modInfo, modsPath });
}

export async function toggleMod(modsPath: string, fileName: string, enable: boolean): Promise<void> {
  return invoke('toggle_mod', { modsPath, fileName, enable });
}

export async function uninstallMod(modsPath: string, fileName: string): Promise<void> {
  return invoke('uninstall_mod', { modsPath, fileName });
}

// Config Management API
export async function getModConfig(gamePath: string, modId: string): Promise<ModConfig> {
  return invoke('get_mod_config', { game_path: gamePath, mod_id: modId });
}

export async function saveModConfig(gamePath: string, modId: string, configData: ModConfig): Promise<void> {
  return invoke('save_mod_config', { game_path: gamePath, mod_id: modId, config_data: configData });
}

export async function setModConfigValue(
  gamePath: string,
  modId: string,
  key: string,
  value: ConfigValue
): Promise<void> {
  return invoke('set_mod_config_value', { game_path: gamePath, mod_id: modId, key, value });
}

export async function listModConfigs(gamePath: string): Promise<string[]> {
  return invoke('list_mod_configs', { game_path: gamePath });
}

export async function deleteModConfig(gamePath: string, modId: string): Promise<void> {
  return invoke('delete_mod_config', { game_path: gamePath, mod_id: modId });
}

// Version Management API
export async function getSilkVersion(gamePath: string): Promise<string> {
  return invoke('get_silk_version', { gamePath });
}

export async function getLatestSilkVersion(): Promise<string> {
  return invoke('get_latest_silk_version');
}

export async function checkForSilkUpdates(gamePath: string): Promise<SilkVersion | null> {
  return invoke('check_for_silk_updates', { gamePath });
}

export async function listAvailableSilkVersions(): Promise<string[]> {
  return invoke('list_available_silk_versions');
}

export async function installSilkVersion(version: string, gamePath: string): Promise<void> {
  return invoke('install_silk_version', { version, gamePath });
}

export async function checkModCompatibility(
  gamePath: string,
  modsPath: string,
  modId: string
): Promise<boolean> {
  return invoke('check_mod_compatibility', { gamePath, modsPath, modId });
}

// BepInEx API
export async function isBepInExInstalled(gamePath: string): Promise<boolean> {
  return invoke('is_bepinex_installed', { gamePath });
}

export async function getBepInExVersion(gamePath: string): Promise<string> {
  return invoke('get_bepinex_version', { gamePath });
}

export async function installBepInEx(gamePath: string): Promise<void> {
  return invoke('install_bepinex', { gamePath });
}

export async function uninstallBepInEx(gamePath: string): Promise<void> {
  return invoke('uninstall_bepinex', { gamePath });
}

export function getModIconUrl(iconPath: string): string {
  if (!iconPath) return '/default-mod.png';
  if (iconPath.startsWith('http')) return iconPath;
  return `${MODS_BASE_URL}${iconPath}`;
}

export function formatDate(dateString: string): string {
  const date = new Date(dateString);
  const now = new Date();
  const diffInSeconds = Math.floor((now.getTime() - date.getTime()) / 1000);

  if (diffInSeconds < 60) return 'Just now';
  if (diffInSeconds < 3600) return `${Math.floor(diffInSeconds / 60)}m ago`;
  if (diffInSeconds < 86400) return `${Math.floor(diffInSeconds / 3600)}h ago`;
  if (diffInSeconds < 604800) return `${Math.floor(diffInSeconds / 86400)}d ago`;

  return date.toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  });
}

export function formatFileSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
}

export function formatNumber(num: number): string {
  return num.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ',');
}

