<script>
  import { link } from 'svelte-spa-router';
  import { isProjectTag, getProjectIdFromTag } from './projects.js';

  let { tag } = $props();

  let isProject = $derived(isProjectTag(tag));
  let projectId = $derived(isProject ? getProjectIdFromTag(tag) : null);
</script>

{#if isProject && projectId}
  <a href="/projects#{projectId}" use:link class="tag project-tag">
    {tag}
  </a>
{:else}
  <span class="tag">{tag}</span>
{/if}

<style>
  .tag {
    background: var(--bg-tertiary);
    padding: 0.125rem 0.5rem;
    border-radius: 0.25rem;
    font-size: 0.75rem;
    color: var(--text-secondary);
  }

  .project-tag {
    background: var(--accent);
    color: var(--bg-primary);
    font-weight: 500;
    text-decoration: none;
    transition: background var(--transition), transform var(--transition);
  }

  .project-tag:hover {
    background: var(--accent-hover);
    transform: translateY(-1px);
  }
</style>
