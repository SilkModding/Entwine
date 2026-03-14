<script lang="ts">
  import type { AppStatus, AppSettings } from '$lib/types';
  import ContentHeader from '$lib/components/ui/ContentHeader.svelte';
  import VersionManager from '$lib/components/VersionManager.svelte';
  import BepInExSettings from '$lib/components/BepInExSettings.svelte';

  interface Props {
    status: AppStatus | null;
    appSettings: AppSettings;
    logPath: string;
    installingSilk: boolean;
    onBrowsePath: () => void;
    onUninstallSilk: () => void;
    onSaveSettings: () => void;
  }

  let {
    status,
    appSettings = $bindable(),
    logPath,
    installingSilk,
    onBrowsePath,
    onUninstallSilk,
    onSaveSettings,
  }: Props = $props();
</script>

<ContentHeader title="Settings" />

<div class="content-body">
  <div class="settings-section">
    <h2>Game Installation</h2>

    <div class="setting-item">
      <div class="setting-info">
        <h3>SpiderHeck Location</h3>
        <p class="setting-value">{status?.gamePath ?? 'Not set'}</p>
      </div>
      <button class="btn btn-secondary" onclick={onBrowsePath}>Change</button>
    </div>

    <div class="setting-item">
      <div class="setting-info">
        <h3>Mods Folder</h3>
        <p class="setting-value">{status?.modsPath ?? 'Not available'}</p>
      </div>
    </div>

    <div class="setting-item">
      <div class="setting-info">
        <h3>Log File</h3>
        <p class="setting-value">{logPath ? logPath + '/entwine.log' : 'Not available'}</p>
      </div>
    </div>

    <div class="setting-item">
      <div class="setting-info">
        <h3>Launch Method</h3>
        <select
          class="setting-select"
          bind:value={appSettings.launchMethod}
          onchange={onSaveSettings}
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
        <button class="btn btn-secondary" onclick={onUninstallSilk} disabled={installingSilk}>
          {installingSilk ? 'Uninstalling...' : 'Uninstall Silk'}
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
        <a href="https://github.com/SilkModding/Silk" target="_blank">Silk on GitHub</a>
        <a href="https://silk.abstractmelon.net" target="_blank">Silk Website</a>
        <a href="https://discord.gg/GGv92crcx3" target="_blank">Discord Community</a>
      </div>
    </div>
  </div>
</div>

<style>
  .content-body {
    flex: 1;
    overflow-y: auto;
    padding: 1.5rem;
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
