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
