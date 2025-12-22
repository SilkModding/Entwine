export interface Mod {
  id: string;
  name: string;
  description: string;
  version: string;
  author: string;
  fileName: string;
  filePath: string;
  fileSize: number;
  iconPath: string;
  uploadDate: string;
  downloads: number;
  lastDownloaded: string | null;
}

export interface InstalledMod {
  id: string;
  name: string;
  fileName: string;
  enabled: boolean;
  version: string;
  author: string;
  description: string;
  iconPath: string;
}

export interface AppStatus {
  silkInstalled: boolean;
  gamePath: string | null;
  modsPath: string | null;
}

export type Tab = 'browse' | 'installed' | 'settings';

// Config types
export type ConfigValue =
  | string
  | number
  | boolean
  | ConfigValue[]
  | { [key: string]: ConfigValue };

export interface ModConfig {
  [key: string]: ConfigValue;
}

// Version types
export interface SilkVersion {
  version: string;
  downloadUrl: string;
}

export interface ModVersionInfo {
  modId: string;
  version: string;
  silkVersion: string;
  minSilkVersion?: string;
  maxSilkVersion?: string;
}

// Settings types
export type LaunchMethod = 'steam' | 'executable';

export interface AppSettings {
  launchMethod: LaunchMethod;
}

export interface Settings {
  autoUpdate: boolean;
  checkModCompatibility: boolean;
  bepinexInstalled: boolean;
}

