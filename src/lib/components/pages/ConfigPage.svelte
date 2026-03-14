<script lang="ts">
  import ContentHeader from '$lib/components/ui/ContentHeader.svelte';
  import ConfigEditor from '$lib/components/ConfigEditor.svelte';

  interface Props {
    gamePath: string | null;
  }

  let { gamePath }: Props = $props();
</script>

<ContentHeader title="Mod Config" />

<div class="config-body">
  <ConfigEditor {gamePath} />
</div>

<style>
  .config-body {
    flex: 1;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }
</style>
