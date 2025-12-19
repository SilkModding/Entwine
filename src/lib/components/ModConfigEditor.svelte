<script lang="ts">
  import { onMount } from 'svelte';
  import type { InstalledMod, ModConfig, ConfigValue } from '$lib/types';
  import { getModConfig, saveModConfig, deleteModConfig } from '$lib/api';

  export let mod: InstalledMod;
  export let gamePath: string;
  export let onClose: () => void;

  let config: ModConfig = {};
  let loading = true;
  let error = '';
  let saving = false;

  onMount(async () => {
    try {
      config = await getModConfig(gamePath, mod.id);
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    } finally {
      loading = false;
    }
  });

  async function handleSave() {
    saving = true;
    error = '';
    try {
      await saveModConfig(gamePath, mod.id, config);
      onClose();
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    } finally {
      saving = false;
    }
  }

  async function handleDelete() {
    if (!confirm('Are you sure you want to delete this mod\'s config? This will reset it to defaults.')) {
      return;
    }
    
    try {
      await deleteModConfig(gamePath, mod.id);
      onClose();
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    }
  }

  function updateConfigValue(key: string, value: ConfigValue) {
    config[key] = value;
  }

  function renderConfigValue(key: string, value: ConfigValue): string {
    if (typeof value === 'object' && !Array.isArray(value)) {
      return JSON.stringify(value, null, 2);
    }
    return String(value);
  }
</script>

<div class="modal-backdrop" on:click={onClose}>
  <div class="modal" on:click|stopPropagation>
    <div class="modal-header">
      <h2>Config: {mod.name}</h2>
      <button class="close-btn" on:click={onClose}>×</button>
    </div>

    <div class="modal-body">
      {#if loading}
        <div class="loading">Loading config...</div>
      {:else if error}
        <div class="error">{error}</div>
      {:else if Object.keys(config).length === 0}
        <div class="no-config">
          <p>This mod has no configuration file yet.</p>
          <p>You can create one by adding values below or the mod will create one on first run.</p>
        </div>
      {:else}
        <div class="config-editor">
          {#each Object.entries(config) as [key, value]}
            <div class="config-item">
              <label for={key}>{key}</label>
              {#if typeof value === 'boolean'}
                <input
                  type="checkbox"
                  id={key}
                  checked={value}
                  on:change={(e) => updateConfigValue(key, e.currentTarget.checked)}
                />
              {:else if typeof value === 'number'}
                <input
                  type="number"
                  id={key}
                  value={value}
                  on:input={(e) => updateConfigValue(key, parseFloat(e.currentTarget.value))}
                />
              {:else if typeof value === 'string'}
                <input
                  type="text"
                  id={key}
                  value={value}
                  on:input={(e) => updateConfigValue(key, e.currentTarget.value)}
                />
              {:else}
                <textarea
                  id={key}
                  rows="4"
                  value={renderConfigValue(key, value)}
                  on:input={(e) => {
                    try {
                      updateConfigValue(key, JSON.parse(e.currentTarget.value));
                    } catch {
                      // Invalid JSON, ignore
                    }
                  }}
                />
              {/if}
            </div>
          {/each}
        </div>
      {/if}
    </div>

    <div class="modal-footer">
      <button class="btn btn-secondary danger" on:click={handleDelete}>Delete Config</button>
      <div class="button-group">
        <button class="btn" on:click={onClose}>Cancel</button>
        <button class="btn btn-primary" on:click={handleSave} disabled={saving || loading}>
          {saving ? 'Saving...' : 'Save'}
        </button>
      </div>
    </div>
  </div>
</div>

<style>
  .modal-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal {
    background: rgba(0, 0, 0, 0.85);
    border: 1px solid #ff0064;
    border-radius: 12px;
    width: 90%;
    max-width: 720px;
    max-height: 80vh;
    display: flex;
    flex-direction: column;
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.45);
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 1.25rem;
    border-bottom: 1px solid rgba(255,255,255,0.05);
  }

  .modal-header h2 {
    margin: 0;
    font-size: 1.1rem;
    color: #fff;
    font-weight: 600;
  }

  .close-btn {
    background: none;
    border: none;
    font-size: 2rem;
    color: #888;
    cursor: pointer;
    padding: 0;
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .close-btn:hover {
    color: #fff;
  }

  .modal-body {
    flex: 1;
    overflow-y: auto;
    padding: 1rem 1.25rem;
  }

  .loading,
  .no-config {
    text-align: center;
    padding: 2rem;
    color: #888;
  }

  .error {
    color: #ff6b6b;
    padding: 1rem;
    background: rgba(255, 107, 107, 0.1);
    border-radius: 4px;
  }

  .config-editor {
    display: flex;
    flex-direction: column;
    gap: 0.85rem;
  }

  .config-item {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .config-item label {
    font-weight: 600;
    color: #fff;
    font-size: 0.9rem;
  }

  .config-item input[type="text"],
  .config-item input[type="number"],
  .config-item textarea {
    background: rgba(0,0,0,0.25);
    border: 1px solid rgba(255,255,255,0.04);
    border-radius: 8px;
    padding: 0.6rem;
    color: #fff;
    font-family: monospace;
  }

  .config-item input[type="checkbox"] {
    width: 20px;
    height: 20px;
  }

  .modal-footer {
    display: flex;
    justify-content: space-between;
    padding: 1rem 1.25rem;
    border-top: 1px solid rgba(255,255,255,0.03);
  }

  .button-group {
    display: flex;
    gap: 0.5rem;
  }

  /* Buttons use global .btn classes; keep minor fallback */
  button {
    padding: 0.55rem 1rem;
    border-radius: 8px;
    border: none;
    color: #fff;
    cursor: pointer;
    font-size: 0.9rem;
  }

  button:hover:not(:disabled) {
    transform: translateY(-1px);
  }

  button:disabled {
    opacity: 0.55;
    cursor: not-allowed;
  }

  /* Use global .btn, .btn-primary, .btn-secondary classes from the app */
  .btn-secondary.danger {
    background: #d32f2f;
  }

  .btn-secondary.danger:hover:not(:disabled) {
    background: #f44336;
  }
</style>
