<script lang="ts">
  interface Props {
    title: string;
    count?: string;
    searchQuery?: string;
    onSearchChange?: (value: string) => void;
  }

  let { title, count, searchQuery = '', onSearchChange }: Props = $props();
</script>

<header class="content-header">
  <div class="header-left">
    <h1>{title}</h1>
    {#if count}
      <span class="count-badge">{count}</span>
    {/if}
  </div>

  {#if onSearchChange !== undefined}
    <div class="search-box">
      <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="11" cy="11" r="8"></circle>
        <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
      </svg>
      <input
        type="text"
        placeholder="Search mods..."
        value={searchQuery}
        oninput={(e) => onSearchChange(e.currentTarget.value)}
      />
    </div>
  {/if}
</header>

<style>
  .content-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1.25rem 1.5rem;
    border-bottom: 1px solid rgba(255, 0, 100, 0.2);
    background: rgba(0, 0, 0, 0.3);
    flex-shrink: 0;
  }

  .header-left {
    display: flex;
    align-items: baseline;
    gap: 0.75rem;
  }

  .header-left h1 {
    font-size: 1.8rem;
    font-weight: 800;
    color: #ff0064;
    text-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
  }

  .count-badge {
    font-size: 0.85rem;
    color: rgba(255, 255, 255, 0.5);
  }

  .search-box {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1.25rem;
    background: rgba(0, 0, 0, 0.5);
    border: 2px solid rgba(255, 255, 255, 0.1);
    border-radius: 30px;
    color: rgba(255, 255, 255, 0.6);
    width: 280px;
    box-shadow: 0 4px 15px rgba(0, 0, 0, 0.2);
    transition: border-color 0.2s;
  }

  .search-box:focus-within {
    border-color: rgba(255, 0, 100, 0.4);
  }

  .search-box input {
    flex: 1;
    background: transparent;
    border: none;
    outline: none;
    color: #fff;
    font-size: 0.9rem;
  }

  .search-box input::placeholder {
    color: rgba(255, 255, 255, 0.4);
  }
</style>
