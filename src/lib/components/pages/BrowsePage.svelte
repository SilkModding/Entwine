<script lang="ts">
  import type { Mod } from '$lib/types';
  import ContentHeader from '$lib/components/ui/ContentHeader.svelte';
  import ModCard from '$lib/components/ModCard.svelte';

  interface Props {
    mods: Mod[];
    installedModIds: Set<string>;
    installingModId: string | null;
    searchQuery: string;
    onInstall: (mod: Mod) => void;
    onSearchChange: (value: string) => void;
  }

  let { mods, installedModIds, installingModId, searchQuery, onInstall, onSearchChange }: Props = $props();

  const filtered = $derived(
    mods.filter(m =>
      m.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
      m.description.toLowerCase().includes(searchQuery.toLowerCase()) ||
      m.author.toLowerCase().includes(searchQuery.toLowerCase())
    )
  );
</script>

<ContentHeader
  title="Browse Mods"
  count="{mods.length} mods available"
  {searchQuery}
  {onSearchChange}
/>

<div class="content-body">
  {#if filtered.length === 0}
    <div class="empty-state">
      <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="11" cy="11" r="8"></circle>
        <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
      </svg>
      <p>No mods found</p>
      {#if searchQuery}<span>Try adjusting your search</span>{/if}
    </div>
  {:else}
    <div class="mods-grid">
      {#each filtered as mod (mod.id)}
        <ModCard
          {mod}
          installed={installedModIds.has(mod.id)}
          onInstall={() => onInstall(mod)}
          installing={installingModId === mod.id}
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

  .mods-grid {
    display: flex;
    flex-direction: column;
    gap: 1rem;
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
