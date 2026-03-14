<script lang="ts">
  import type { InstalledMod } from '$lib/types';
  import ContentHeader from '$lib/components/ui/ContentHeader.svelte';
  import InstalledModCard from '$lib/components/InstalledModCard.svelte';

  interface Props {
    mods: InstalledMod[];
    togglingModId: string | null;
    searchQuery: string;
    onToggle: (mod: InstalledMod, enable: boolean) => void;
    onUninstall: (mod: InstalledMod) => void;
    onSearchChange: (value: string) => void;
    onBrowseMods: () => void;
  }

  let { mods, togglingModId, searchQuery, onToggle, onUninstall, onSearchChange, onBrowseMods }: Props = $props();

  const filtered = $derived(
    searchQuery
      ? mods.filter(m =>
          m.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
          m.author.toLowerCase().includes(searchQuery.toLowerCase())
        )
      : mods
  );
</script>

<ContentHeader
  title="Installed Mods"
  count="{mods.length} mods installed"
  {searchQuery}
  {onSearchChange}
/>

<div class="content-body">
  {#if mods.length === 0}
    <div class="empty-state">
      <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
        <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"></path>
      </svg>
      <p>No mods installed yet</p>
      <span>Browse the mod library to find mods to install</span>
      <button class="btn btn-primary" onclick={onBrowseMods}>Browse Mods</button>
    </div>
  {:else}
    <div class="installed-list">
      {#each filtered as mod (mod.fileName)}
        <InstalledModCard
          {mod}
          onToggle={(enable) => onToggle(mod, enable)}
          onUninstall={() => onUninstall(mod)}
          toggling={togglingModId === mod.id}
        />
      {/each}
    </div>
  {/if}
</div>

<style>
  .content-body {
    flex: 1;
    overflow-y: auto;
    padding: 1.5rem;
  }

  .installed-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 4rem 2rem;
    color: rgba(255, 255, 255, 0.4);
    text-align: center;
  }

  .empty-state svg {
    opacity: 0.5;
    margin-bottom: 0.5rem;
  }

  .empty-state p {
    font-size: 1.1rem;
    color: rgba(255, 255, 255, 0.6);
  }

  .empty-state span {
    font-size: 0.9rem;
  }
</style>
