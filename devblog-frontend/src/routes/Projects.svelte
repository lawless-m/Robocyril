<script>
  import { link } from 'svelte-spa-router';
  import { getProjects } from '../lib/api.js';
  import Skeleton from '../lib/Skeleton.svelte';

  let projects = $state([]);
  let loading = $state(true);
  let error = $state(null);
  let selectedId = $state(null);

  $effect(() => {
    loadProjects();
  });

  async function loadProjects() {
    try {
      projects = await getProjects();
    } catch (e) {
      error = e.message;
    } finally {
      loading = false;
      // Handle hash after projects are loaded
      handleHash();
    }
  }

  function handleHash() {
    const hash = window.location.hash.slice(1);
    selectedId = hash || null;

    // Scroll to the selected project after a brief delay
    if (hash) {
      setTimeout(() => {
        const element = document.getElementById(hash);
        if (element) {
          element.scrollIntoView({ behavior: 'smooth', block: 'center' });
        }
      }, 100);
    }
  }

  $effect(() => {
    // Listen for hash changes
    const handleHashChange = () => {
      selectedId = window.location.hash.slice(1) || null;
    };
    window.addEventListener('hashchange', handleHashChange);

    return () => {
      window.removeEventListener('hashchange', handleHashChange);
    };
  });
</script>

<div class="projects-page">
  <header class="page-header">
    <a href="/" use:link class="back-link">&larr; Back</a>
    <h1>Projects</h1>
    <p class="tagline">Things I'm building, breaking, and occasionally shipping.</p>
  </header>

  {#if loading}
    <div class="projects-list">
      {#each [1, 2] as _}
        <article class="project-card skeleton-card">
          <Skeleton width="30%" height="1.5rem" />
          <Skeleton width="80%" height="1rem" style="margin-top: 0.75rem" />
        </article>
      {/each}
    </div>
  {:else if error}
    <div class="error">
      <p>Couldn't load projects. {error}</p>
      <a href="/" use:link class="back-link">&larr; Back to workshop</a>
    </div>
  {:else if projects.length === 0}
    <div class="empty">
      <p>No projects yet.</p>
      <p class="empty-subtext">The workshop is empty. Time to build something.</p>
    </div>
  {:else}
    <div class="projects-list">
      {#each projects as project}
        <article
          id={project.id}
          class="project-card"
          class:selected={selectedId === project.id}
        >
          <div class="project-header">
            <span class="project-tag">® {project.name}</span>
            <a href={project.repo} target="_blank" rel="noopener noreferrer" class="repo-link">
              GitHub &rarr;
            </a>
          </div>
          <p class="project-description">
            {selectedId === project.id ? project.description : project.short_description}
          </p>
          {#if selectedId === project.id}
            <div class="project-details">
              <a href={project.repo} target="_blank" rel="noopener noreferrer" class="full-repo-link">
                {project.repo}
              </a>
            </div>
          {/if}
        </article>
      {/each}
    </div>
  {/if}
</div>

<style>
  .projects-page {
    animation: fadeIn 0.3s ease;
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(10px); }
    to { opacity: 1; transform: translateY(0); }
  }

  .page-header {
    margin-bottom: 2rem;
    padding-bottom: 1.5rem;
    border-bottom: 1px solid var(--border);
  }

  .back-link {
    display: inline-block;
    color: var(--text-secondary);
    font-size: 0.875rem;
    margin-bottom: 1rem;
    transition: color var(--transition);
  }

  .back-link:hover {
    color: var(--accent);
  }

  h1 {
    font-size: 2rem;
    font-weight: 600;
    margin: 0;
    color: var(--text-primary);
  }

  .tagline {
    color: var(--text-secondary);
    font-size: 1rem;
    margin-top: 0.5rem;
    font-style: italic;
  }

  .projects-list {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .project-card {
    padding: 1.5rem;
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    border-radius: 0.75rem;
    transition: border-color var(--transition), transform var(--transition), box-shadow var(--transition);
  }

  .project-card:hover {
    border-color: var(--accent);
    transform: translateY(-2px);
  }

  .project-card.selected {
    border-color: var(--accent);
    border-width: 2px;
    background: var(--bg-tertiary);
    box-shadow: 0 4px 12px rgba(88, 166, 255, 0.15);
  }

  .project-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 0.75rem;
  }

  .project-tag {
    background: var(--accent);
    color: var(--bg-primary);
    padding: 0.25rem 0.75rem;
    border-radius: 0.25rem;
    font-size: 0.875rem;
    font-weight: 600;
  }

  .repo-link {
    color: var(--text-secondary);
    font-size: 0.875rem;
    transition: color var(--transition);
  }

  .repo-link:hover {
    color: var(--accent);
  }

  .project-description {
    color: var(--text-secondary);
    font-size: 0.9375rem;
    line-height: 1.6;
    margin: 0;
  }

  .project-card.selected .project-description {
    color: var(--text-primary);
    font-size: 1rem;
  }

  .project-details {
    margin-top: 1rem;
    padding-top: 1rem;
    border-top: 1px solid var(--border);
  }

  .full-repo-link {
    font-family: 'IBM Plex Mono', 'Fira Code', monospace;
    font-size: 0.8125rem;
    color: var(--accent);
    word-break: break-all;
  }

  .empty {
    text-align: center;
    padding: 3rem 1rem;
    color: var(--text-secondary);
  }

  .empty-subtext {
    margin-top: 0.5rem;
    font-size: 0.875rem;
    color: var(--text-muted);
    font-style: italic;
  }

  .error {
    text-align: center;
    padding: 3rem 1rem;
    color: var(--text-secondary);
  }

  .skeleton-card {
    pointer-events: none;
  }

  @media (max-width: 640px) {
    h1 {
      font-size: 1.5rem;
    }

    .project-header {
      flex-direction: column;
      align-items: flex-start;
      gap: 0.5rem;
    }
  }
</style>
