<script lang="ts">
  import type { InstalledMod } from '$lib/types';
  import { getModIconUrl } from '$lib/api';

  interface Props {
    mod: InstalledMod;
    onToggle: (enable: boolean) => void;
    onUninstall: () => void;
    toggling: boolean;
  }

  let { mod, onToggle, onUninstall, toggling }: Props = $props();

  const iconUrl = $derived(getModIconUrl(mod.iconPath));
</script>

<div class="installed-mod-card" class:disabled={!mod.enabled}>
  <div class="mod-icon">
    <img src={iconUrl} alt={mod.name} onerror={(e) => (e.currentTarget.src = '/default-mod.png')} />
  </div>

  <div class="mod-info">
    <h3 class="mod-name">{mod.name}</h3>
    <p class="mod-meta">
      <span>by {mod.author}</span>
      <span>•</span>
      <span>v{mod.version}</span>
    </p>
    <p class="mod-description">{mod.description}</p>
  </div>

  <div class="mod-actions">
    <label class="toggle-switch">
      <input 
        type="checkbox" 
        checked={mod.enabled} 
        onchange={(e) => onToggle(e.currentTarget.checked)}
        disabled={toggling}
      />
      <span class="toggle-slider"></span>
    </label>

    <button class="btn-icon btn-delete" onclick={onUninstall} title="Uninstall mod">
      <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <polyline points="3 6 5 6 21 6"></polyline>
        <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path>
        <line x1="10" y1="11" x2="10" y2="17"></line>
        <line x1="14" y1="11" x2="14" y2="17"></line>
      </svg>
    </button>
  </div>
</div>

<style>
  .installed-mod-card {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1rem 1.25rem;
    background: linear-gradient(145deg, rgba(40, 30, 60, 0.5), rgba(20, 15, 30, 0.6));
    border: 1px solid rgba(255, 0, 100, 0.2);
    border-radius: 12px;
    transition: all 0.3s ease;
    backdrop-filter: blur(10px);
  }

  .installed-mod-card:hover {
    transform: translateY(-2px);
    border-color: #ff0064;
    box-shadow: 0 8px 20px rgba(0, 0, 0, 0.4);
  }

  .installed-mod-card.disabled {
    opacity: 0.6;
  }

  .mod-icon {
    width: 56px;
    height: 56px;
    border-radius: 0.5rem;
    overflow: hidden;
    background: rgba(255, 255, 255, 0.1);
    flex-shrink: 0;
  }

  .mod-icon img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .mod-info {
    flex: 1;
    min-width: 0;
  }

  .mod-name {
    font-size: 1rem;
    font-weight: 600;
    color: #fff;
    margin: 0 0 0.25rem;
  }

  .mod-meta {
    display: flex;
    gap: 0.5rem;
    font-size: 0.8rem;
    color: rgba(255, 255, 255, 0.5);
    margin: 0 0 0.35rem;
  }

  .mod-description {
    font-size: 0.85rem;
    color: rgba(255, 255, 255, 0.6);
    margin: 0;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .mod-actions {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    flex-shrink: 0;
  }

  .toggle-switch {
    position: relative;
    display: inline-block;
    width: 48px;
    height: 26px;
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
    background: rgba(255, 255, 255, 0.1);
    transition: 0.3s;
    border-radius: 26px;
  }

  .toggle-slider:before {
    position: absolute;
    content: "";
    height: 20px;
    width: 20px;
    left: 3px;
    bottom: 3px;
    background: #fff;
    transition: 0.3s;
    border-radius: 50%;
  }

  .toggle-switch input:checked + .toggle-slider {
    background: linear-gradient(135deg, #ff0064, #e60059);
  }

  .toggle-switch input:checked + .toggle-slider:before {
    transform: translateX(22px);
  }

  .toggle-switch input:disabled + .toggle-slider {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
    border-radius: 0.5rem;
    background: transparent;
    border: 1px solid rgba(255, 255, 255, 0.1);
    color: rgba(255, 255, 255, 0.6);
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-delete:hover {
    background: rgba(239, 68, 68, 0.2);
    border-color: rgba(239, 68, 68, 0.3);
    color: #ef4444;
  }
</style>
