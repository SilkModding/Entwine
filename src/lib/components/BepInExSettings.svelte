<script lang="ts">
  import { onMount } from 'svelte';
  import { isBepInExInstalled, getBepInExVersion, installBepInEx, uninstallBepInEx } from '$lib/api';
  import { listen } from '@tauri-apps/api/event';

  export let gamePath: string;

  let installed = false;
  let version = '';
  let loading = true;
  let error = '';
  let processing = false;
  let progress = '';

  onMount(async () => {
    await checkInstallation();

    // Listen for install progress
    const unlisten = await listen('install-progress', (event) => {
      progress = event.payload as string;
    });

    return () => {
      unlisten();
    };
  });

  async function checkInstallation() {
    loading = true;
    error = '';
    try {
      installed = await isBepInExInstalled(gamePath);
      if (installed) {
        version = await getBepInExVersion(gamePath);
      }
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    } finally {
      loading = false;
    }
  }

  async function handleInstall() {
    if (!confirm('Install BepInEx? This will be configured to work alongside Silk.')) {
      return;
    }

    processing = true;
    error = '';
    progress = '';

    try {
      await installBepInEx(gamePath);
      await checkInstallation();
      alert('BepInEx installed successfully!');
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    } finally {
      processing = false;
      progress = '';
    }
  }

  async function handleUninstall() {
    if (!confirm('Uninstall BepInEx? This will not affect your Silk installation.')) {
      return;
    }

    processing = true;
    error = '';
    progress = '';

    try {
      await uninstallBepInEx(gamePath);
      await checkInstallation();
      alert('BepInEx uninstalled successfully!');
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    } finally {
      processing = false;
      progress = '';
    }
  }
</script>

<div class="bepinex-settings">
  <h3>BepInEx Integration</h3>
  <p class="description">
    BepInEx is a popular modding framework. You can install it alongside Silk to use mods that
    require BepInEx.
  </p>

  {#if loading}
    <div class="loading">Checking BepInEx installation...</div>
  {:else if error}
    <div class="error">{error}</div>
  {:else}
    <div class="status-section">
      <div class="status-row">
        <span class="label">Status:</span>
        <span class="status {installed ? 'installed' : 'not-installed'}">
          {installed ? 'Installed' : 'Not Installed'}
        </span>
      </div>

      {#if installed}
        <div class="status-row">
          <span class="label">Version:</span>
          <span class="value">{version}</span>
        </div>
      {/if}

      {#if progress}
        <div class="progress-message">{progress}</div>
      {/if}
    </div>

    <div class="actions">
      {#if installed}
        <button class="btn btn-secondary" on:click={handleUninstall} disabled={processing}>
          {processing ? 'Uninstalling...' : 'Uninstall BepInEx'}
        </button>
      {:else}
        <button class="btn btn-primary" on:click={handleInstall} disabled={processing}>
          {processing ? 'Installing...' : 'Install BepInEx'}
        </button>
      {/if}
    </div>

    <div class="info-box">
      <strong>Note:</strong> BepInEx will be configured to work alongside Silk using Silk's doorstop
      configuration. Both Silk and BepInEx mods will be loaded together.
    </div>
  {/if}
</div>

<style>
  .bepinex-settings {
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

  .description {
    color: #bdbdbd;
    font-size: 0.95rem;
    margin-bottom: 1rem;
    line-height: 1.45;
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

  .status-section {
    margin-bottom: 1rem;
  }

  .status-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 0;
    border-bottom: 1px solid rgba(255,255,255,0.03);
  }

  .label {
    color: #888;
    font-weight: 600;
  }

  .value {
    color: #fff;
    font-family: monospace;
  }

  .status {
    font-weight: 600;
    padding: 0.25rem 0.75rem;
    border-radius: 4px;
  }

  .status.installed {
    color: #4caf50;
    background: rgba(76, 175, 80, 0.1);
  }

  .status.not-installed {
    color: #ff9800;
    background: rgba(255, 152, 0, 0.1);
  }

  .progress-message {
    color: #9ad0ff;
    padding: 0.5rem;
    text-align: center;
    font-size: 0.9rem;
    margin-top: 0.5rem;
  }

  .actions {
    margin-bottom: 0.75rem;
  }

  /* Buttons use global .btn classes; provide fallback */
  button {
    width: 100%;
    padding: 0.75rem;
    border-radius: 8px;
    border: none;
    color: #fff;
    cursor: pointer;
    font-size: 0.95rem;
    font-weight: 600;
  }

  button:hover:not(:disabled) {
    transform: translateY(-1px);
  }

  button:disabled {
    opacity: 0.55;
    cursor: not-allowed;
  }

  .info-box {
    background: rgba(0, 122, 204, 0.1);
    border: 1px solid #007acc;
    border-radius: 4px;
    padding: 1rem;
    color: #888;
    font-size: 0.85rem;
    line-height: 1.5;
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

  .info-box strong {
    color: #007acc;
  }
</style>
