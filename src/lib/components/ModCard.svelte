<script lang="ts">
  import type { Mod, InstalledMod } from '$lib/types';
  import { getModIconUrl, formatDate, formatNumber } from '$lib/api';

  interface Props {
    mod: Mod;
    installed: boolean;
    onInstall: () => void;
    installing: boolean;
  }

  let { mod, installed, onInstall, installing }: Props = $props();

  const iconUrl = $derived(getModIconUrl(mod.iconPath));
</script>

<div class="mod-card">
  <div class="mod-icon">
    <img src={iconUrl} alt={mod.name} onerror={(e) => (e.currentTarget.src = '/default-mod.png')} />
  </div>

  <div class="mod-info">
    <h3 class="mod-name">{mod.name}</h3>
    <p class="mod-description">{mod.description}</p>

    <div class="mod-meta">
      <span class="meta-item">
        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path>
          <circle cx="12" cy="7" r="4"></circle>
        </svg>
        {mod.author}
      </span>
      <span class="meta-item">
        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
          <polyline points="7 10 12 15 17 10"></polyline>
          <line x1="12" y1="15" x2="12" y2="3"></line>
        </svg>
        {formatNumber(mod.downloads)}
      </span>
      <span class="meta-item">
        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <line x1="6" y1="3" x2="6" y2="15"></line>
          <circle cx="18" cy="6" r="3"></circle>
          <circle cx="6" cy="18" r="3"></circle>
          <path d="M18 9a9 9 0 0 1-9 9"></path>
        </svg>
        v{mod.version}
      </span>
      <span class="meta-item">
        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <rect x="3" y="4" width="18" height="18" rx="2" ry="2"></rect>
          <line x1="16" y1="2" x2="16" y2="6"></line>
          <line x1="8" y1="2" x2="8" y2="6"></line>
          <line x1="3" y1="10" x2="21" y2="10"></line>
        </svg>
        {formatDate(mod.uploadDate)}
      </span>
    </div>
  </div>

  <div class="mod-actions">
    {#if installed}
      <button class="btn btn-installed" disabled>
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="20 6 9 17 4 12"></polyline>
        </svg>
        Installed
      </button>
    {:else}
      <button class="btn btn-install" onclick={onInstall} disabled={installing}>
        {#if installing}
          <svg class="spinner" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="12" y1="2" x2="12" y2="6"></line>
            <line x1="12" y1="18" x2="12" y2="22"></line>
            <line x1="4.93" y1="4.93" x2="7.76" y2="7.76"></line>
            <line x1="16.24" y1="16.24" x2="19.07" y2="19.07"></line>
            <line x1="2" y1="12" x2="6" y2="12"></line>
            <line x1="18" y1="12" x2="22" y2="12"></line>
            <line x1="4.93" y1="19.07" x2="7.76" y2="16.24"></line>
            <line x1="16.24" y1="7.76" x2="19.07" y2="4.93"></line>
          </svg>
          Installing...
        {:else}
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
            <polyline points="7 10 12 15 17 10"></polyline>
            <line x1="12" y1="15" x2="12" y2="3"></line>
          </svg>
          Install
        {/if}
      </button>
    {/if}
  </div>
</div>

<style>
  .mod-card {
    display: flex;
    align-items: flex-start;
    gap: 1rem;
    padding: 1.25rem;
    background: linear-gradient(145deg, rgba(40, 30, 60, 0.5), rgba(20, 15, 30, 0.6));
    border: 1px solid rgba(255, 0, 100, 0.2);
    border-radius: 16px;
    transition: all 0.4s cubic-bezier(0.25, 0.46, 0.45, 0.94);
    backdrop-filter: blur(10px);
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.4);
  }

  .mod-card:hover {
    transform: translateY(-8px);
    border-color: #ff0064;
    box-shadow: 0 15px 35px rgba(0, 0, 0, 0.5);
  }

  .mod-icon {
    width: 80px;
    height: 80px;
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
    font-size: 1.1rem;
    font-weight: 600;
    color: #fff;
    margin: 0 0 0.5rem;
  }

  .mod-description {
    font-size: 0.9rem;
    color: rgba(255, 255, 255, 0.6);
    margin: 0 0 0.75rem;
    line-height: 1.4;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .mod-meta {
    display: flex;
    flex-wrap: wrap;
    gap: 1rem;
  }

  .meta-item {
    display: flex;
    align-items: center;
    gap: 0.35rem;
    font-size: 0.8rem;
    color: rgba(255, 255, 255, 0.5);
  }

  .mod-actions {
    flex-shrink: 0;
  }

  .btn {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.6rem 1.25rem;
    border-radius: 0.5rem;
    font-size: 0.85rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
    border: none;
  }

  .btn-install {
    background: linear-gradient(135deg, #ff0064, #e60059);
    color: #fff;
    text-transform: uppercase;
    letter-spacing: 1px;
    box-shadow: 0 4px 15px rgba(255, 0, 100, 0.3);
  }

  .btn-install:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(255, 0, 100, 0.4);
    background: linear-gradient(135deg, #ff1a75, #ff0064);
  }

  .btn-install:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }

  .btn-installed {
    background: rgba(34, 197, 94, 0.2);
    color: #22c55e;
    border: 1px solid rgba(34, 197, 94, 0.3);
    cursor: default;
  }

  .spinner {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }
</style>
