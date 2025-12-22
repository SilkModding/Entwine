<script lang="ts">
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import { open } from '@tauri-apps/plugin-dialog';
  import type { Mod, InstalledMod, AppStatus, Tab, AppSettings } from '$lib/types';
  import {
    getAppStatus,
    setGamePath,
    fetchMods,
    installSilk,
    uninstallSilk,
    getInstalledMods,
    installMod,
    toggleMod,
    uninstallMod,
    getSettings,
    saveSettings,
    launchGame
  } from '$lib/api';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import ModCard from '$lib/components/ModCard.svelte';
  import InstalledModCard from '$lib/components/InstalledModCard.svelte';
  import SetupWizard from '$lib/components/SetupWizard.svelte';
  import VersionManager from '$lib/components/VersionManager.svelte';
  import BepInExSettings from '$lib/components/BepInExSettings.svelte';

  let status = $state<AppStatus | null>(null);
  let mods = $state<Mod[]>([]);
  let installedMods = $state<InstalledMod[]>([]);
  let activeTab = $state<Tab>('browse');
  let searchQuery = $state('');
  let loading = $state(true);
  let error = $state<string | null>(null);
  let installProgress = $state<string | null>(null);
  let installingSilk = $state(false);
  let installingModId = $state<string | null>(null);
  let togglingModId = $state<string | null>(null);
  let appSettings = $state<AppSettings>({ launchMethod: 'steam' });

  const filteredMods = $derived(
    mods.filter(mod =>
      mod.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
      mod.description.toLowerCase().includes(searchQuery.toLowerCase()) ||
      mod.author.toLowerCase().includes(searchQuery.toLowerCase())
    )
  );

  const installedModIds = $derived(
    new Set(installedMods.map(m => m.id))
  );

  onMount(() => {
    let unlistenFn: (() => void) | undefined;
    
    // Listen for install progress events
    listen<string>('install-progress', (event) => {
      installProgress = event.payload;
    }).then(unlisten => {
      unlistenFn = unlisten;
    });

    // Initialize app
    (async () => {
      try {
        status = await getAppStatus();
        appSettings = await getSettings();
        
        if (status.silkInstalled && status.modsPath) {
          await loadMods();
          await loadInstalledMods();
        } else if (!status.gamePath) {
          activeTab = 'settings';
        }
      } catch (e) {
        error = e instanceof Error ? e.message : 'Failed to initialize app';
      } finally {
        loading = false;
      }
    })();

    return () => {
      if (unlistenFn) unlistenFn();
    };
  });

  async function loadMods() {
    try {
      mods = await fetchMods();
    } catch (e) {
      console.error('Failed to fetch mods:', e);
    }
  }

  async function loadInstalledMods() {
    if (!status?.modsPath) return;
    try {
      installedMods = await getInstalledMods(status.modsPath);
    } catch (e) {
      console.error('Failed to load installed mods:', e);
    }
  }

  async function handleBrowsePath() {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: 'Select SpiderHeck Installation Folder'
      });

      if (selected) {
        status = await setGamePath(selected as string);
        if (status.silkInstalled && status.modsPath) {
          await loadMods();
          await loadInstalledMods();
          activeTab = 'browse';
        }
      }
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to set game path';
    }
  }

  async function handleInstallSilk() {
    if (!status?.gamePath) return;

    installingSilk = true;
    error = null;

    try {
      await installSilk(status.gamePath);
      status = await getAppStatus();
      
      if (status.silkInstalled) {
        await loadMods();
        await loadInstalledMods();
        activeTab = 'browse';
      }
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to install Silk';
    } finally {
      installingSilk = false;
      installProgress = null;
    }
  }

  async function handleUninstallSilk() {
    if (!status?.gamePath) return;
    if (!confirm('Are you sure you want to uninstall Silk? This will remove the Silk folder and injected files.')) return;

    installingSilk = true;
    error = null;

    try {
      await uninstallSilk(status.gamePath);
      status = await getAppStatus();

      if (!status.silkInstalled) {
        installedMods = [];
        activeTab = 'settings';
      }
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to uninstall Silk';
    } finally {
      installingSilk = false;
      installProgress = null;
    }
  }

  async function handleInstallMod(mod: Mod) {
    if (!status?.modsPath) return;

    installingModId = mod.id;
    error = null;

    try {
      await installMod(mod, status.modsPath);
      await loadInstalledMods();
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to install mod';
    } finally {
      installingModId = null;
      installProgress = null;
    }
  }

  async function handleToggleMod(mod: InstalledMod, enable: boolean) {
    if (!status?.modsPath) return;

    togglingModId = mod.id;

    try {
      await toggleMod(status.modsPath, mod.fileName, enable);
      await loadInstalledMods();
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to toggle mod';
    } finally {
      togglingModId = null;
    }
  }

  async function handleUninstallMod(mod: InstalledMod) {
    if (!status?.modsPath) return;

    if (!confirm(`Are you sure you want to uninstall ${mod.name}?`)) return;

    try {
      await uninstallMod(status.modsPath, mod.fileName);
      await loadInstalledMods();
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to uninstall mod';
    }
  }
  async function handleLaunchGame() {
    if (!status?.gamePath) return;
    
    error = null;
    try {
      await launchGame(status.gamePath);
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to launch game';
    }
  }

  async function handleSaveSettings() {
    try {
      await saveSettings(appSettings);
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to save settings';
    }
  }

  function handleTabChange(tab: Tab) {
    activeTab = tab;
    error = null;
  }
</script>

<main class="app">
  <Sidebar 
    {activeTab} 
    silkInstalled={status?.silkInstalled ?? false}
    onTabChange={handleTabChange}
    onLaunchGame={handleLaunchGame}
  />

  <div class="content">
    {#if loading}
      <div class="loading-state">
        <div class="spinner-large"></div>
        <p>Loading...</p>
      </div>
    {:else if !status?.silkInstalled}
      <SetupWizard 
        gamePath={status?.gamePath ?? null}
        onInstallSilk={handleInstallSilk}
        onBrowsePath={handleBrowsePath}
        installing={installingSilk}
      />
    {:else}
      <header class="content-header">
        <div class="header-left">
          <h1>
            {#if activeTab === 'browse'}
              Browse Mods
            {:else if activeTab === 'installed'}
              Installed Mods
            {:else}
              Settings
            {/if}
          </h1>
          {#if activeTab === 'browse'}
            <span class="mod-count">{mods.length} mods available</span>
          {:else if activeTab === 'installed'}
            <span class="mod-count">{installedMods.length} mods installed</span>
          {/if}
        </div>

        {#if activeTab === 'browse' || activeTab === 'installed'}
          <div class="search-box">
            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="11" cy="11" r="8"></circle>
              <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
            </svg>
            <input 
              type="text" 
              placeholder="Search mods..." 
              bind:value={searchQuery}
            />
          </div>
        {/if}
      </header>

      {#if error}
        <div class="error-banner">
          <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="10"></circle>
            <line x1="12" y1="8" x2="12" y2="12"></line>
            <line x1="12" y1="16" x2="12.01" y2="16"></line>
          </svg>
          <span>{error}</span>
          <button onclick={() => error = null}>✕</button>
        </div>
      {/if}

      {#if installProgress}
        <div class="progress-banner">
          <div class="spinner-small"></div>
          <span>{installProgress}</span>
        </div>
      {/if}

      <div class="content-body">
        {#if activeTab === 'browse'}
          {#if filteredMods.length === 0}
            <div class="empty-state">
              <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="11" cy="11" r="8"></circle>
                <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
              </svg>
              <p>No mods found</p>
              {#if searchQuery}
                <span>Try adjusting your search</span>
              {/if}
            </div>
          {:else}
            <div class="mods-grid">
              {#each filteredMods as mod (mod.id)}
                <ModCard 
                  {mod}
                  installed={installedModIds.has(mod.id)}
                  onInstall={() => handleInstallMod(mod)}
                  installing={installingModId === mod.id}
                />
              {/each}
            </div>
          {/if}
        {:else if activeTab === 'installed'}
          {#if installedMods.length === 0}
            <div class="empty-state">
              <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"></path>
              </svg>
              <p>No mods installed yet</p>
              <span>Browse the mod library to find mods to install</span>
              <button class="btn btn-primary" onclick={() => activeTab = 'browse'}>
                Browse Mods
              </button>
            </div>
          {:else}
            <div class="installed-mods-list">
              {#each installedMods as mod (mod.fileName)}
                <InstalledModCard 
                  {mod}
                  onToggle={(enable) => handleToggleMod(mod, enable)}
                  onUninstall={() => handleUninstallMod(mod)}
                  toggling={togglingModId === mod.id}
                />
              {/each}
            </div>
          {/if}
        {:else if activeTab === 'settings'}
          <div class="settings-section">
            <h2>Game Installation</h2>
            <div class="setting-item">
              <div class="setting-info">
                <h3>SpiderHeck Location</h3>
                <p class="setting-value">{status?.gamePath ?? 'Not set'}</p>
              </div>
              <button class="btn btn-secondary" onclick={handleBrowsePath}>
                Change
              </button>
            </div>

            <div class="setting-item">
              <div class="setting-info">
                <h3>Mods Folder</h3>
                <p class="setting-value">{status?.modsPath ?? 'Not available'}</p>
              </div>
            </div>

            <div class="setting-item">
              <div class="setting-info">
                <h3>Launch Method</h3>
                <select
                  class="setting-select"
                  bind:value={appSettings.launchMethod}
                  onchange={handleSaveSettings}
                >
                  <option value="steam">Steam (App ID 1329500)</option>
                  <option value="executable">Direct Executable (SpiderHeckApp.exe)</option>
                </select>
                <p class="setting-description">
                  Choose how the game should be launched when you click the "Launch Game" button.
                </p>
              </div>
            </div>

            {#if status?.silkInstalled}
              <div class="setting-item">
                <div class="setting-info">
                  <h3>Silk Installation</h3>
                  <p class="setting-value">Installed</p>
                </div>
                <button class="btn btn-secondary" onclick={handleUninstallSilk} disabled={installingSilk}>
                  {#if installingSilk}
                    Uninstalling...
                  {:else}
                    Uninstall Silk
                  {/if}
                </button>
              </div>
            {/if}
          </div>

          {#if status?.gamePath && status?.silkInstalled}
            <VersionManager gamePath={status.gamePath} />
            <BepInExSettings gamePath={status.gamePath} />
          {/if}

          <div class="settings-section">
            <h2>About</h2>
            <div class="about-content">
              <p>Entwine is a mod manager for SpiderHeck, powered by the Silk mod loader.</p>
              <div class="about-links">
                <a href="https://github.com/SilkModding/Silk" target="_blank">
                  Silk on GitHub
                </a>
                <a href="https://silk.abstractmelon.net" target="_blank">
                  Silk Website
                </a>
                <a href="https://discord.gg/GGv92crcx3" target="_blank">
                  Discord Community
                </a>
              </div>
            </div>
          </div>
        {/if}
      </div>
    {/if}
  </div>
</main>

<style>
  :global(*) {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
  }

  :global(body) {
    font-family: Arial, sans-serif;
    background: #614984;
    color: #fff;
    overflow: hidden;
  }

  .app {
    display: flex;
    height: 100vh;
    width: 100vw;
  }

  .content {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    background-image: url('/background.png');
    background-size: cover;
    background-position: center;
  }

  .loading-state {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 1rem;
    color: rgba(255, 255, 255, 0.6);
  }

  .spinner-large {
    width: 48px;
    height: 48px;
    border: 3px solid rgba(255, 0, 100, 0.2);
    border-top-color: #ff0064;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  .spinner-small {
    width: 18px;
    height: 18px;
    border: 2px solid rgba(255, 0, 100, 0.2);
    border-top-color: #ff0064;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }

  .content-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1.25rem 1.5rem;
    border-bottom: 1px solid rgba(255, 0, 100, 0.2);
    background: rgba(0, 0, 0, 0.3);
  }

  .header-left {
    display: flex;
    align-items: baseline;
    gap: 0.75rem;
  }

  .header-left h1 {
    font-size: 1.8rem;
    font-weight: 800;
    color: #ff0064;
    text-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
  }

  .mod-count {
    font-size: 0.85rem;
    color: rgba(255, 255, 255, 0.5);
  }

  .search-box {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1.25rem;
    background: rgba(0, 0, 0, 0.5);
    border: 2px solid rgba(255, 255, 255, 0.1);
    border-radius: 30px;
    color: rgba(255, 255, 255, 0.6);
    width: 280px;
    box-shadow: 0 4px 15px rgba(0, 0, 0, 0.2);
    transition: all 0.3s ease;
  }

  .search-box input {
    flex: 1;
    background: transparent;
    border: none;
    outline: none;
    color: #fff;
    font-size: 0.9rem;
  }

  .search-box input::placeholder {
    color: rgba(255, 255, 255, 0.4);
  }

  .error-banner {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem 1rem;
    background: rgba(239, 68, 68, 0.1);
    border-bottom: 1px solid rgba(239, 68, 68, 0.3);
    color: #ef4444;
    font-size: 0.9rem;
  }

  .error-banner button {
    margin-left: auto;
    background: transparent;
    border: none;
    color: #ef4444;
    cursor: pointer;
    padding: 0.25rem;
  }

  .progress-banner {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem 1rem;
    background: rgba(255, 0, 100, 0.1);
    border-bottom: 1px solid rgba(255, 0, 100, 0.3);
    color: #ff0064;
    font-size: 0.9rem;
  }

  .content-body {
    flex: 1;
    overflow-y: auto;
    padding: 1.5rem;
  }

  .mods-grid {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .installed-mods-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 4rem 2rem;
    color: rgba(255, 255, 255, 0.4);
    text-align: center;
  }

  .empty-state svg {
    margin-bottom: 1rem;
    opacity: 0.5;
  }

  .empty-state p {
    font-size: 1.1rem;
    margin-bottom: 0.5rem;
    color: rgba(255, 255, 255, 0.6);
  }

  .empty-state span {
    font-size: 0.9rem;
    margin-bottom: 1rem;
  }

  .btn {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.6rem 1.25rem;
    border-radius: 0.5rem;
    font-size: 0.9rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
    border: none;
  }

  .btn-primary {
    background: linear-gradient(135deg, #ff0064, #e60059);
    color: #fff;
    text-transform: uppercase;
    letter-spacing: 1px;
    box-shadow: 0 4px 15px rgba(255, 0, 100, 0.3);
  }

  .btn-primary:hover {
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(255, 0, 100, 0.4);
    background: linear-gradient(135deg, #ff1a75, #ff0064);
  }

  .btn-secondary {
    background: rgba(255, 255, 255, 0.1);
    color: #fff;
    border: 1px solid rgba(255, 255, 255, 0.2);
  }

  .btn-secondary:hover {
    background: rgba(255, 255, 255, 0.15);
  }

  .settings-section {
    background: rgba(0, 0, 0, 0.7);
    border: 1px solid #ff0064;
    border-radius: 12px;
    padding: 1.5rem;
    margin-bottom: 1rem;
    backdrop-filter: blur(10px);
    box-shadow: 0 8px 16px rgba(0, 0, 0, 0.3);
  }

  .settings-section h2 {
    font-size: 1.1rem;
    font-weight: 600;
    margin-bottom: 1rem;
    color: #fff;
  }

  .setting-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.75rem 0;
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  }

  .setting-item:last-child {
    border-bottom: none;
    padding-bottom: 0;
  }

  .setting-info h3 {
    font-size: 0.95rem;
    font-weight: 500;
    color: #fff;
    margin-bottom: 0.25rem;
  }

  .setting-value {
    font-size: 0.85rem;
    color: rgba(255, 255, 255, 0.5);
    word-break: break-all;
  }

  .setting-select {
    width: 100%;
    padding: 0.5rem;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 0.5rem;
    color: #fff;
    font-size: 0.9rem;
    cursor: pointer;
    margin-top: 0.5rem;
  }

  .setting-select:hover {
    border-color: rgba(255, 0, 100, 0.5);
  }

  .setting-select:focus {
    outline: none;
    border-color: #ff0064;
  }

  .setting-description {
    font-size: 0.8rem;
    color: rgba(255, 255, 255, 0.5);
    margin-top: 0.5rem;
  }

  .about-content {
    color: rgba(255, 255, 255, 0.7);
    line-height: 1.6;
  }

  .about-content p {
    margin-bottom: 1rem;
  }

  .about-links {
    display: flex;
    gap: 1.5rem;
  }

  .about-links a {
    color: #ff0064;
    text-decoration: none;
    font-size: 0.9rem;
    font-weight: 600;
  }

  .about-links a:hover {
    text-decoration: underline;
    color: #ff1a75;
  }
</style>
