<script lang="ts">
  import { onMount } from 'svelte';
  import type { ModConfigFile, ConfigValue } from '$lib/types';
  import { listModConfigs, getModConfig, setModConfigValue, resetModConfig } from '$lib/api';

  interface Props {
    gamePath: string | null;
  }

  let { gamePath }: Props = $props();

  let modConfigs = $state<ModConfigFile[]>([]);
  let selectedModId = $state<string | null>(null);
  let selectedConfig = $state<Record<string, ConfigValue> | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let saving = $state(false);
  let expandedKeys = $state<Set<string>>(new Set());

  const selectedMod = $derived(
    modConfigs.find(m => m.modId === selectedModId)
  );

  onMount(() => {
    loadConfigs();
  });

  async function loadConfigs() {
    if (!gamePath) {
      error = 'No game path set';
      loading = false;
      return;
    }

    try {
      loading = true;
      error = null;
      modConfigs = await listModConfigs(gamePath);
      
      if (modConfigs.length > 0 && !selectedModId) {
        selectedModId = modConfigs[0].modId;
        await loadSelectedConfig();
      }
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to load configs';
      console.error('Failed to load configs:', e);
    } finally {
      loading = false;
    }
  }

  async function loadSelectedConfig() {
    if (!gamePath || !selectedModId) return;

    try {
      selectedConfig = await getModConfig(gamePath, selectedModId);
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to load config';
      console.error('Failed to load config:', e);
    }
  }

  async function handleModSelect(modId: string) {
    selectedModId = modId;
    expandedKeys.clear();
    await loadSelectedConfig();
  }

  async function handleValueChange(key: string, value: ConfigValue) {
    if (!gamePath || !selectedModId) return;

    try {
      saving = true;
      await setModConfigValue(gamePath, selectedModId, key, value);
      
      // Update local state
      if (selectedConfig) {
        const keys = key.split('.');
        let current: any = selectedConfig;
        for (let i = 0; i < keys.length - 1; i++) {
          current = current[keys[i]];
        }
        current[keys[keys.length - 1]] = value;
        selectedConfig = { ...selectedConfig };
      }
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to save config';
      console.error('Failed to save config:', e);
    } finally {
      saving = false;
    }
  }

  async function handleReset() {
    if (!gamePath || !selectedModId || !confirm('Are you sure you want to reset this config to defaults?')) {
      return;
    }

    try {
      await resetModConfig(gamePath, selectedModId);
      await loadConfigs();
      await loadSelectedConfig();
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to reset config';
      console.error('Failed to reset config:', e);
    }
  }

  function toggleExpanded(key: string) {
    if (expandedKeys.has(key)) {
      expandedKeys.delete(key);
    } else {
      expandedKeys.add(key);
    }
    expandedKeys = new Set(expandedKeys);
  }

  function getValueType(value: ConfigValue): string {
    if (value === null) return 'null';
    if (Array.isArray(value)) return 'array';
    return typeof value;
  }

  // Convert snake-case or kebab-case to Title Case
  function formatModName(name: string): string {
    return name
      .split(/[-_]/)
      .map(word => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
      .join(' ');
  }

  // Convert camelCase to Spaced Pascal Case
  function formatConfigKey(key: string): string {
    // Split on capital letters or numbers, keeping them
    return key
      .replace(/([A-Z])/g, ' $1')
      .replace(/([0-9]+)/g, ' $1')
      .trim()
      .split(' ')
      .map(word => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
      .join(' ');
  }

  function renderConfigFields(config: Record<string, ConfigValue>, parentKey = ''): any {
    return Object.entries(config).map(([key, value]) => {
      const fullKey = parentKey ? `${parentKey}.${key}` : key;
      const valueType = getValueType(value);
      const isExpanded = expandedKeys.has(fullKey);

      return {
        key,
        fullKey,
        value,
        valueType,
        isExpanded,
      };
    });
  }
</script>

{#if loading}
  <div class="loading-state">
    <div class="spinner-large"></div>
    <p>Loading configs...</p>
  </div>
{:else if modConfigs.length === 0}
  <div class="empty-state">
    <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
      <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
      <polyline points="14 2 14 8 20 8"></polyline>
      <line x1="12" y1="18" x2="12" y2="12"></line>
      <line x1="9" y1="15" x2="15" y2="15"></line>
    </svg>
    <p>No mod configs found</p>
    <span>Install mods with Silk to configure them here</span>
  </div>
{:else}
  <div class="config-layout">
    <aside class="mod-selector">
      {#each modConfigs as mod}
        <button
          class="mod-selector-item"
          class:active={selectedModId === mod.modId}
          onclick={() => handleModSelect(mod.modId)}
        >
          <span class="mod-name">{formatModName(mod.modName)}</span>
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="9 18 15 12 9 6"></polyline>
          </svg>
        </button>
      {/each}
    </aside>

    <main class="config-content">
      {#if selectedMod && selectedConfig}
        <div class="config-header">
          <h2>{formatModName(selectedMod.modName)}</h2>
          <button onclick={handleReset} class="btn btn-secondary">
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="23 4 23 10 17 10"></polyline>
              <path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10"></path>
            </svg>
            Reset to Defaults
          </button>
        </div>

        {#if saving}
          <div class="save-indicator">
            <div class="spinner-small"></div>
            <span>Saving...</span>
          </div>
        {/if}

        {#if error}
          <div class="error-message">
            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="12" cy="12" r="10"></circle>
              <line x1="12" y1="8" x2="12" y2="12"></line>
              <line x1="12" y1="16" x2="12.01" y2="16"></line>
            </svg>
            <span>{error}</span>
            <button onclick={() => error = null}>✕</button>
          </div>
        {/if}

        <div class="config-fields">
          {#each renderConfigFields(selectedConfig) as field}
            {@render configField(field, handleValueChange, toggleExpanded)}
          {/each}
        </div>
      {/if}
    </main>
  </div>
{/if}

{#snippet configField(field: any, onValueChange: (key: string, value: ConfigValue) => void, onToggleExpand: (key: string) => void)}
  <div class="config-field" class:nested={field.fullKey.includes('.')}>
    {#if field.valueType === 'object'}
      <button class="field-header expandable" onclick={() => onToggleExpand(field.fullKey)}>
        <div class="field-header-left">
          <svg
            class="expand-icon"
            class:expanded={field.isExpanded}
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <polyline points="9 18 15 12 9 6"></polyline>
          </svg>
          <span class="field-key">{formatConfigKey(field.key)}</span>
        </div>
        <span class="field-type">{field.valueType}</span>
      </button>
      {#if field.isExpanded}
        <div class="nested-fields">
          {#each renderConfigFields(field.value as Record<string, ConfigValue>, field.fullKey) as nestedField}
            {@render configField(nestedField, onValueChange, onToggleExpand)}
          {/each}
        </div>
      {/if}
    {:else if field.valueType === 'array'}
      <div class="field-header">
        <span class="field-key">{formatConfigKey(field.key)}</span>
        <span class="field-type">{field.valueType} ({(field.value as any[]).length} items)</span>
      </div>
      <div class="array-value">
        <pre>{JSON.stringify(field.value, null, 2)}</pre>
      </div>
    {:else if field.valueType === 'boolean'}
      <div class="field-header">
        <label class="field-key" for={field.fullKey}>{formatConfigKey(field.key)}</label>
        <label class="toggle-switch">
          <input
            id={field.fullKey}
            type="checkbox"
            checked={field.value as boolean}
            onchange={(e) => onValueChange(field.fullKey, (e.currentTarget as HTMLInputElement).checked)}
          />
          <span class="toggle-slider"></span>
        </label>
      </div>
    {:else if field.valueType === 'number'}
      <div class="field-header">
        <label class="field-key" for={field.fullKey}>{formatConfigKey(field.key)}</label>
        <input
          id={field.fullKey}
          type="number"
          value={field.value as number}
          onchange={(e) => onValueChange(field.fullKey, parseFloat((e.currentTarget as HTMLInputElement).value))}
          class="field-input"
        />
      </div>
    {:else if field.valueType === 'string'}
      <div class="field-header">
        <label class="field-key" for={field.fullKey}>{formatConfigKey(field.key)}</label>
        <input
          id={field.fullKey}
          type="text"
          value={field.value as string}
          onchange={(e) => onValueChange(field.fullKey, (e.currentTarget as HTMLInputElement).value)}
          class="field-input"
        />
      </div>
    {:else}
      <div class="field-header">
        <span class="field-key">{formatConfigKey(field.key)}</span>
        <span class="field-type">{field.valueType}</span>
      </div>
      <div class="field-value">{String(field.value)}</div>
    {/if}
  </div>
{/snippet}

<style>
  .loading-state,
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 4rem 2rem;
    color: rgba(255, 255, 255, 0.4);
    text-align: center;
  }

  .loading-state {
    gap: 1rem;
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
    to {
      transform: rotate(360deg);
    }
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
  }

  .config-layout {
    display: grid;
    grid-template-columns: 240px 1fr;
    gap: 1.5rem;
    height: 100%;
    overflow: hidden;
  }

  .mod-selector {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    overflow-y: auto;
    padding-right: 0.5rem;
  }

  .mod-selector-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.875rem 1rem;
    background: rgba(0, 0, 0, 0.3);
    border: 1px solid rgba(255, 0, 100, 0.2);
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s;
    color: rgba(255, 255, 255, 0.8);
    text-align: left;
    font-size: 0.95rem;
  }

  .mod-selector-item:hover {
    background: rgba(0, 0, 0, 0.4);
    border-color: rgba(255, 0, 100, 0.4);
    transform: translateX(4px);
  }

  .mod-selector-item.active {
    background: rgba(255, 0, 100, 0.15);
    border-color: #ff0064;
    color: white;
  }

  .mod-selector-item svg {
    opacity: 0.5;
    transition: opacity 0.2s;
  }

  .mod-selector-item:hover svg,
  .mod-selector-item.active svg {
    opacity: 1;
  }

  .config-content {
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .config-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding-bottom: 1rem;
    margin-bottom: 1rem;
    border-bottom: 1px solid rgba(255, 0, 100, 0.2);
  }

  .config-header h2 {
    margin: 0;
    font-size: 1.5rem;
    color: white;
  }

  .btn {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.6rem 1rem;
    border-radius: 8px;
    font-size: 0.875rem;
    cursor: pointer;
    transition: all 0.2s;
    border: none;
    font-weight: 500;
  }

  .btn-secondary {
    background: rgba(255, 255, 255, 0.1);
    color: #fff;
    border: 1px solid rgba(255, 255, 255, 0.2);
  }

  .btn-secondary:hover {
    background: rgba(255, 255, 255, 0.15);
    transform: translateY(-1px);
  }

  .save-indicator {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1rem;
    background: rgba(0, 255, 100, 0.1);
    border: 1px solid rgba(0, 255, 100, 0.3);
    border-radius: 8px;
    color: #00ff64;
    font-size: 0.875rem;
    margin-bottom: 1rem;
  }

  .error-message {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1rem;
    background: rgba(255, 0, 0, 0.1);
    border: 1px solid rgba(255, 0, 0, 0.3);
    border-radius: 8px;
    color: #ff4444;
    font-size: 0.875rem;
    margin-bottom: 1rem;
  }

  .error-message button {
    margin-left: auto;
    background: none;
    border: none;
    color: inherit;
    cursor: pointer;
    padding: 0.25rem 0.5rem;
    opacity: 0.7;
    transition: opacity 0.2s;
  }

  .error-message button:hover {
    opacity: 1;
  }

  .config-fields {
    flex: 1;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    padding-right: 0.5rem;
  }

  .config-field {
    background: rgba(0, 0, 0, 0.4);
    border: 1px solid rgba(255, 0, 100, 0.2);
    border-radius: 8px;
    padding: 1rem;
    backdrop-filter: blur(10px);
    transition: all 0.2s;
  }

  .config-field:hover {
    border-color: rgba(255, 0, 100, 0.4);
  }

  .config-field.nested {
    margin-left: 1.5rem;
    background: rgba(0, 0, 0, 0.5);
    border-color: rgba(255, 0, 100, 0.15);
  }

  .field-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
  }

  .field-header.expandable {
    width: 100%;
    background: none;
    border: none;
    cursor: pointer;
    padding: 0;
    text-align: left;
    color: inherit;
  }

  .field-header-left {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .expand-icon {
    transition: transform 0.2s;
    color: rgba(255, 0, 100, 0.7);
    flex-shrink: 0;
  }

  .expand-icon.expanded {
    transform: rotate(90deg);
  }

  .field-key {
    font-weight: 500;
    color: white;
    font-size: 0.95rem;
  }

  .field-type {
    font-size: 0.75rem;
    color: rgba(255, 0, 100, 0.6);
    background: rgba(255, 0, 100, 0.1);
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-family: 'Courier New', monospace;
    flex-shrink: 0;
  }

  .field-input {
    padding: 0.5rem 0.75rem;
    background: rgba(0, 0, 0, 0.4);
    border: 1px solid rgba(255, 0, 100, 0.3);
    border-radius: 6px;
    color: white;
    font-size: 0.9rem;
    transition: all 0.2s;
    width: 200px;
  }

  .field-input:focus {
    outline: none;
    border-color: #ff0064;
    background: rgba(0, 0, 0, 0.6);
  }

  .toggle-switch {
    position: relative;
    display: inline-block;
    width: 48px;
    height: 24px;
    flex-shrink: 0;
  }

  .toggle-switch input {
    opacity: 0;
    width: 0;
    height: 0;
  }

  .toggle-slider {
    position: absolute;
    cursor: pointer;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(255, 255, 255, 0.2);
    border-radius: 24px;
    transition: 0.3s;
  }

  .toggle-slider:before {
    position: absolute;
    content: "";
    height: 18px;
    width: 18px;
    left: 3px;
    bottom: 3px;
    background-color: white;
    border-radius: 50%;
    transition: 0.3s;
  }

  input:checked + .toggle-slider {
    background-color: #ff0064;
  }

  input:checked + .toggle-slider:before {
    transform: translateX(24px);
  }

  .array-value pre,
  .field-value {
    background: rgba(0, 0, 0, 0.4);
    padding: 0.75rem;
    border-radius: 6px;
    color: rgba(255, 255, 255, 0.8);
    font-size: 0.875rem;
    font-family: 'Courier New', monospace;
    overflow-x: auto;
    margin-top: 0.5rem;
  }

  .nested-fields {
    margin-top: 0.75rem;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  /* Scrollbar styling */
  .mod-selector::-webkit-scrollbar,
  .config-fields::-webkit-scrollbar {
    width: 6px;
  }

  .mod-selector::-webkit-scrollbar-track,
  .config-fields::-webkit-scrollbar-track {
    background: rgba(0, 0, 0, 0.2);
    border-radius: 3px;
  }

  .mod-selector::-webkit-scrollbar-thumb,
  .config-fields::-webkit-scrollbar-thumb {
    background: rgba(255, 0, 100, 0.5);
    border-radius: 3px;
  }

  .mod-selector::-webkit-scrollbar-thumb:hover,
  .config-fields::-webkit-scrollbar-thumb:hover {
    background: rgba(255, 0, 100, 0.7);
  }
</style>
