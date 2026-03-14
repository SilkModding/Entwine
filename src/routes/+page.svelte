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
    launchGame,
    getLogPath,
  } from '$lib/api';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import SetupWizard from '$lib/components/SetupWizard.svelte';
  import ErrorBanner from '$lib/components/ui/ErrorBanner.svelte';
  import BrowsePage from '$lib/components/pages/BrowsePage.svelte';
  import InstalledPage from '$lib/components/pages/InstalledPage.svelte';
  import ConfigPage from '$lib/components/pages/ConfigPage.svelte';
  import SettingsPage from '$lib/components/pages/SettingsPage.svelte';

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
  let logPath = $state('');

  const installedModIds = $derived(new Set(installedMods.map(m => m.id)));

  onMount(() => {
    let unlistenFn: (() => void) | undefined;
    listen<string>('install-progress', (event) => {
      installProgress = event.payload;
    }).then(fn => { unlistenFn = fn; });

    (async () => {
      try {
        status = await getAppStatus();
        appSettings = await getSettings();
        getLogPath().then(p => { logPath = p; }).catch(() => {});
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

    return () => { if (unlistenFn) unlistenFn(); };
  });

  async function loadMods() {
    try { mods = await fetchMods(); }
    catch (e) { console.error('Failed to fetch mods:', e); }
  }

  async function loadInstalledMods() {
    if (!status?.modsPath) return;
    try { installedMods = await getInstalledMods(status.modsPath); }
    catch (e) { console.error('Failed to load installed mods:', e); }
  }

  async function handleBrowsePath() {
    try {
      const selected = await open({ directory: true, multiple: false, title: 'Select SpiderHeck Installation Folder' });
      if (selected) {
        status = await setGamePath(selected as string);
        if (status.silkInstalled && status.modsPath) {
          await loadMods();
          await loadInstalledMods();
          activeTab = 'browse';
        }
      }
    } catch (e) { error = e instanceof Error ? e.message : 'Failed to set game path'; }
  }

  async function handleInstallSilk() {
    if (!status?.gamePath) return;
    installingSilk = true;
    error = null;
    try {
      await installSilk(status.gamePath);
      status = await getAppStatus();
      if (status.silkInstalled) { await loadMods(); await loadInstalledMods(); activeTab = 'browse'; }
    } catch (e) { error = e instanceof Error ? e.message : 'Failed to install Silk'; }
    finally { installingSilk = false; installProgress = null; }
  }

  async function handleUninstallSilk() {
    if (!status?.gamePath) return;
    if (!confirm('Are you sure you want to uninstall Silk? This will remove the Silk folder and injected files.')) return;
    installingSilk = true;
    error = null;
    try {
      await uninstallSilk(status.gamePath);
      status = await getAppStatus();
      if (!status.silkInstalled) { installedMods = []; activeTab = 'settings'; }
    } catch (e) { error = e instanceof Error ? e.message : 'Failed to uninstall Silk'; }
    finally { installingSilk = false; installProgress = null; }
  }

  async function handleInstallMod(mod: Mod) {
    if (!status?.modsPath) return;
    installingModId = mod.id;
    error = null;
    try { await installMod(mod, status.modsPath); await loadInstalledMods(); }
    catch (e) { error = e instanceof Error ? e.message : 'Failed to install mod'; }
    finally { installingModId = null; installProgress = null; }
  }

  async function handleToggleMod(mod: InstalledMod, enable: boolean) {
    if (!status?.modsPath) return;
    togglingModId = mod.id;
    try { await toggleMod(status.modsPath, mod.fileName, enable); await loadInstalledMods(); }
    catch (e) { error = e instanceof Error ? e.message : 'Failed to toggle mod'; }
    finally { togglingModId = null; }
  }

  async function handleUninstallMod(mod: InstalledMod) {
    if (!status?.modsPath) return;
    try { await uninstallMod(status.modsPath, mod.fileName); await loadInstalledMods(); }
    catch (e) { error = e instanceof Error ? e.message : 'Failed to uninstall mod'; }
  }

  async function handleLaunchGame() {
    if (!status?.gamePath) return;
    error = null;
    try { await launchGame(status.gamePath); }
    catch (e) { error = e instanceof Error ? e.message : 'Failed to launch game'; }
  }

  async function handleSaveSettings() {
    try { await saveSettings(appSettings); }
    catch (e) { error = e instanceof Error ? e.message : 'Failed to save settings'; }
  }

  function handleTabChange(tab: Tab) { activeTab = tab; error = null; }
</script>

<div class="app">
  <Sidebar
    {activeTab}
    silkInstalled={status?.silkInstalled ?? false}
    onTabChange={handleTabChange}
    onLaunchGame={handleLaunchGame}
  />

  <main class="content">
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
      {#if error}
        <ErrorBanner message={error} onDismiss={() => (error = null)} />
      {/if}

      {#if activeTab === 'browse'}
        <BrowsePage
          {mods}
          {installedModIds}
          {installingModId}
          {searchQuery}
          onInstall={handleInstallMod}
          onSearchChange={(v) => (searchQuery = v)}
        />
      {:else if activeTab === 'installed'}
        <InstalledPage
          mods={installedMods}
          {togglingModId}
          {searchQuery}
          onToggle={handleToggleMod}
          onUninstall={handleUninstallMod}
          onSearchChange={(v) => (searchQuery = v)}
          onBrowseMods={() => handleTabChange('browse')}
        />
      {:else if activeTab === 'config'}
        <ConfigPage gamePath={status?.gamePath ?? null} />
      {:else if activeTab === 'settings'}
        <SettingsPage
          {status}
          bind:appSettings
          {logPath}
          {installingSilk}
          onBrowsePath={handleBrowsePath}
          onUninstallSilk={handleUninstallSilk}
          onSaveSettings={handleSaveSettings}
        />
      {/if}
    {/if}
  </main>
</div>

{#if installProgress}
  <div class="progress-banner">
    <div class="spinner-small"></div>
    <span>{installProgress}</span>
  </div>
{/if}

<style>
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

  .progress-banner {
    position: fixed;
    bottom: 1.25rem;
    right: 1.25rem;
    z-index: 9999;
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.65rem 1rem;
    background: rgba(20, 10, 35, 0.95);
    border: 1px solid rgba(255, 0, 100, 0.45);
    border-radius: 0.5rem;
    color: #ff0064;
    font-size: 0.875rem;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.5);
    backdrop-filter: blur(8px);
    pointer-events: none;
  }
</style>

