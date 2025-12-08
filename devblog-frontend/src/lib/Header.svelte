<script>
  import { link } from 'svelte-spa-router';

  let isDark = $state(true);

  function toggleTheme() {
    isDark = !isDark;
    document.documentElement.setAttribute('data-theme', isDark ? 'dark' : 'light');
  }
</script>

<header>
  <div class="header-content">
    <a href="/" use:link class="logo">
      <span class="logo-icon">&#128187;</span>
      <span class="logo-text">Devblog</span>
    </a>
    <nav>
      <a href="/projects" use:link class="nav-link">Projects</a>
      <button class="theme-toggle" onclick={toggleTheme} aria-label="Toggle theme">
        {#if isDark}
          <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="5"></circle>
            <line x1="12" y1="1" x2="12" y2="3"></line>
            <line x1="12" y1="21" x2="12" y2="23"></line>
            <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line>
            <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line>
            <line x1="1" y1="12" x2="3" y2="12"></line>
            <line x1="21" y1="12" x2="23" y2="12"></line>
            <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line>
            <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line>
          </svg>
        {:else}
          <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
          </svg>
        {/if}
      </button>
    </nav>
  </div>
</header>

<style>
  header {
    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border);
    position: sticky;
    top: 0;
    z-index: 100;
    backdrop-filter: blur(10px);
  }

  .header-content {
    max-width: var(--max-width);
    margin: 0 auto;
    padding: 1rem 1.5rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .logo {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-weight: 600;
    font-size: 1.1rem;
    color: var(--text-primary);
  }

  .logo:hover {
    color: var(--text-primary);
  }

  .logo-icon {
    font-size: 1.4rem;
  }

  nav {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .nav-link {
    color: var(--text-secondary);
    font-weight: 500;
    font-size: 0.95rem;
    padding: 0.5rem 0.75rem;
    border-radius: 0.5rem;
    transition: background var(--transition), color var(--transition);
  }

  .nav-link:hover {
    background: var(--bg-tertiary);
    color: var(--text-primary);
  }

  .theme-toggle {
    background: none;
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    padding: 0.5rem;
    border-radius: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background var(--transition), color var(--transition);
  }

  .theme-toggle:hover {
    background: var(--bg-tertiary);
    color: var(--text-primary);
  }

  @media (max-width: 640px) {
    .header-content {
      padding: 0.75rem 1rem;
    }

    .logo-text {
      display: none;
    }
  }
</style>
