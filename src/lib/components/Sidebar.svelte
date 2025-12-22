<script lang="ts">
  import type { Tab } from '$lib/types';

  interface Props {
    activeTab: Tab;
    silkInstalled: boolean;
    onTabChange: (tab: Tab) => void;
    onLaunchGame: () => void;
  }

  let { activeTab, silkInstalled, onTabChange, onLaunchGame }: Props = $props();
</script>

<aside class="sidebar">
  <div class="logo">
    <span>Entwine</span>
  </div>

  <nav class="nav-menu">
      <button
      class="nav-item launch-button"
      onclick={onLaunchGame}
      disabled={!silkInstalled}
    >
      <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <polygon points="5 3 19 12 5 21 5 3"></polygon>
      </svg>
      <span>Launch Game</span>
    </button>

    <button
      class="nav-item"
      class:active={activeTab === 'browse'}
      onclick={() => onTabChange('browse')}
      disabled={!silkInstalled}
    >
      <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="11" cy="11" r="8"></circle>
        <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
      </svg>
      <span>Browse Mods</span>
    </button>

    <button
      class="nav-item"
      class:active={activeTab === 'installed'}
      onclick={() => onTabChange('installed')}
      disabled={!silkInstalled}
    >
      <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"></path>
        <polyline points="3.27 6.96 12 12.01 20.73 6.96"></polyline>
        <line x1="12" y1="22.08" x2="12" y2="12"></line>
      </svg>
      <span>Installed Mods</span>
    </button>

    <button
      class="nav-item"
      class:active={activeTab === 'config'}
      onclick={() => onTabChange('config')}
      disabled={!silkInstalled}
    >
      <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"></path>
        <circle cx="12" cy="12" r="3"></circle>
      </svg>
      <span>Mod Config</span>
    </button>

    <button
      class="nav-item"
      class:active={activeTab === 'settings'}
      onclick={() => onTabChange('settings')}
    >
      <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="12" cy="12" r="3"></circle>
        <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path>
      </svg>
      <span>Settings</span>
    </button>
  </nav>

  <div class="sidebar-footer">
    <div class="status-indicator" class:installed={silkInstalled}>
      <span class="status-dot"></span>
      <span>{silkInstalled ? 'Silk Installed' : 'Silk Not Installed'}</span>
    </div>
  </div>
</aside>

<style>
  .sidebar {
    width: 240px;
    background: linear-gradient(180deg, rgba(30, 20, 50) 0%, rgba(10, 5, 20) 100%);
    border-right: 1px solid rgba(255, 0, 100, 0.2);
    display: flex;
    flex-direction: column;
    padding: 1rem 0;
    backdrop-filter: blur(10px);
  }

  .logo {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.5rem 1.25rem 1.5rem;
    border-bottom: 1px solid rgba(255, 0, 100, 0.2);
    margin-bottom: 1rem;
  }

  .logo span {
    font-size: 1.8rem;
    font-weight: 800;
    color: #ff0064;
    text-shadow: 0 2px 4px rgba(0, 0, 0, 0.5);
    letter-spacing: 1px;
  }

  .nav-menu {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    padding: 0 0.75rem;
    flex: 1;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem 1rem;
    border-radius: 0.5rem;
    background: transparent;
    border: none;
    color: rgba(255, 255, 255, 0.7);
    cursor: pointer;
    transition: all 0.2s ease;
    font-size: 0.9rem;
    width: 100%;
    text-align: left;
  }

  .nav-item:hover:not(:disabled) {
    background: rgba(255, 255, 255, 0.05);
    color: #fff;
  }

  .nav-item.active {
    background: linear-gradient(135deg, #ff0064, #e60059);
    color: #fff;
    border: none;
    box-shadow: 0 4px 15px rgba(255, 0, 100, 0.3);
  }

  .nav-item.active:hover:not(:disabled) {
    background: linear-gradient(135deg, #c80050, #a60040);
    box-shadow: 0 4px 15px rgba(255, 0, 100, 0.3);
  }

  .nav-item:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .launch-button {
    background: linear-gradient(135deg, rgba(34, 197, 94, 0.2), rgba(22, 163, 74, 0.2));
    border: 1px solid rgba(34, 197, 94, 0.3);
    margin-top: 0.5rem;
  }

  .launch-button:hover:not(:disabled) {
    background: linear-gradient(135deg, rgba(34, 197, 94, 0.3), rgba(22, 163, 74, 0.3));
    border-color: rgba(34, 197, 94, 0.5);
  }

  .sidebar-footer {
    padding: 1rem 1.25rem;
    border-top: 1px solid rgba(255, 255, 255, 0.1);
    margin-top: auto;
  }

  .status-indicator {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.8rem;
    color: rgba(255, 255, 255, 0.6);
  }

  .status-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: #ef4444;
  }

  .status-indicator.installed .status-dot {
    background: #22c55e;
  }
</style>
