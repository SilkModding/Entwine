<script lang="ts">
  import { onMount } from 'svelte';
  import type { SilkVersion } from '$lib/types';
  import {
    getSilkVersion,
    checkForSilkUpdates,
    listAvailableSilkVersions,
    installSilkVersion,
  } from '$lib/api';
  import { listen } from '@tauri-apps/api/event';

  export let gamePath: string;

  let currentVersion = '';
  let latestVersion: SilkVersion | null = null;
  let availableVersions: string[] = [];
  let loading = true;
  let error = '';
  let installing = false;
  let installProgress = '';
  let selectedVersion = '';

  onMount(() => {
    let unlisten: (() => Promise<void>) | undefined;

    (async () => {
      try {
        currentVersion = await getSilkVersion(gamePath);
        const updateCheck = await checkForSilkUpdates(gamePath);
        latestVersion = updateCheck;
        availableVersions = await listAvailableSilkVersions();
      } catch (e) {
        error = e instanceof Error ? e.message : String(e);
      } finally {
        loading = false;
      }
    })();

    // Listen for install progress
    const unlisten = await listen('install-progress', (event) => {
      installProgress = event.payload as string;
    });

    return () => {
      unlisten();
    };
  });

  async function handleUpdate() {
    if (!latestVersion) return;
    
    if (!confirm(`Update Silk from v${currentVersion} to v${latestVersion.version}?`)) {
      return;
    }

    installing = true;
    error = '';
    installProgress = '';

    try {
      await installSilkVersion(latestVersion.version, gamePath);
      currentVersion = latestVersion.version;
      latestVersion = null;
      alert('Silk updated successfully! Please restart SpiderHeck.');
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    } finally {
      installing = false;
      installProgress = '';
    }
  }

  async function handleVersionSwitch() {
    if (!selectedVersion) return;
    
    if (!confirm(`Switch Silk to v${selectedVersion}? This will replace your current installation.`)) {
      return;
    }

    installing = true;
    error = '';
    installProgress = '';

    try {
      await installSilkVersion(selectedVersion, gamePath);
      currentVersion = selectedVersion;
      latestVersion = null;
      alert('Silk version changed successfully! Please restart SpiderHeck.');
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    } finally {
      installing = false;
      installProgress = '';
      selectedVersion = '';
    }
  }
</script>

<div class="version-manager">
  <h3>Silk Version Manager</h3>

  {#if loading}
    <div class="loading">Loading version info...</div>
  {:else if error}
    <div class="error">{error}</div>
  {:else}
    <div class="version-info">
      <div class="info-row">
        <span class="label">Current Version:</span>
        <span class="value">{currentVersion}</span>
      </div>

      {#if latestVersion}
        <div class="update-available">
          <div class="info-row">
            <span class="label">Latest Version:</span>
            <span class="value">{latestVersion.version}</span>
          </div>
          <button class="btn btn-primary" onclick={handleUpdate} disabled={installing}>
            {installing ? 'Updating...' : 'Update Now'}
          </button>
        </div>
      {:else}
        <div class="up-to-date">
          ✓ You're running the latest version
        </div>
      {/if}

      {#if installProgress}
        <div class="progress-message">{installProgress}</div>
      {/if}
    </div>

    <div class="version-selector">
      <h4>Switch to Different Version</h4>
      <p class="description">
        You can install a different version of Silk. This is useful for compatibility with older mods.
      </p>

      <div class="selector-row">
        <select bind:value={selectedVersion} disabled={installing}>
          <option value="">Select a version...</option>
          {#each availableVersions as version}
            <option value={version} disabled={version === currentVersion}>
              v{version} {version === currentVersion ? '(current)' : ''}
            </option>
          {/each}
        </select>
        <button
          class="btn btn-secondary"
          onclick={handleVersionSwitch}
          disabled={!selectedVersion || installing}
        >
          Install
        </button> 
      </div>
    </div>
  {/if}
</div>

<style>
  .version-manager {
    background: rgba(0, 0, 0, 0.7);
    border: 1px solid #ff0064;
    border-radius: 12px;
    padding: 1.5rem;
    margin-bottom: 1rem;
    backdrop-filter: blur(10px);
    box-shadow: 0 8px 16px rgba(0,0,0,0.3);
  }

  h3 {
    margin: 0 0 0.75rem 0;
    color: #fff;
    font-size: 1.1rem;
    font-weight: 600;
  }

  h4 {
    margin: 0 0 0.5rem 0;
    color: #fff;
    font-size: 1rem;
    font-weight: 600;
  }

  .loading {
    text-align: center;
    padding: 2rem;
    color: #888;
  }

  .error {
    color: #ff6b6b;
    padding: 1rem;
    background: rgba(255, 107, 107, 0.1);
    border-radius: 4px;
    margin-bottom: 1rem;
  }

  .version-info {
    margin-bottom: 2rem;
  }

  .info-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 0;
    border-bottom: 1px solid #333;
  }

  .label {
    color: #888;
    font-weight: 600;
  }

  .value {
    color: #fff;
    font-family: monospace;
  }

  .update-available {
    background: rgba(0, 122, 204, 0.07);
    border: 1px solid rgba(0,122,204,0.18);
    border-radius: 8px;
    padding: 0.9rem;
    margin-top: 1rem;
  }

  .update-available .info-row {
    border-bottom: none;
    margin-bottom: 1rem;
  }

  .up-to-date {
    color: #4caf50;
    padding: 0.9rem;
    text-align: center;
    background: rgba(76, 175, 80, 0.07);
    border-radius: 8px;
    margin-top: 1rem;
  }

  .progress-message {
    color: #007acc;
    padding: 0.5rem;
    text-align: center;
    font-size: 0.9rem;
    margin-top: 0.5rem;
  }

  .version-selector {
    border-top: 1px solid rgba(255,255,255,0.05);
    padding-top: 1rem;
  }

  .description {
    color: #888;
    font-size: 0.9rem;
    margin-bottom: 1rem;
  }

  .selector-row {
    display: flex;
    gap: 0.5rem;
  }

  select {
    flex: 1;
    background: rgba(0,0,0,0.22);
    border: 1px solid rgba(255,255,255,0.04);
    border-radius: 8px;
    padding: 0.6rem;
    color: #fff;
    font-size: 0.95rem;
  }

  /* Buttons use global .btn classes; keep subtle fallback */
  button {
    padding: 0.6rem 1rem;
    border-radius: 8px;
    border: none;
    color: #fff;
    cursor: pointer;
    font-size: 0.95rem;
    white-space: nowrap;
  }

  button:hover:not(:disabled) {
    transform: translateY(-1px);
  }

  button:disabled {
    opacity: 0.55;
    cursor: not-allowed;
  }

</style>
