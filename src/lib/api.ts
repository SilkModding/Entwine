import { invoke } from '@tauri-apps/api/core';
import type { Mod, InstalledMod, AppStatus } from './types';

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
