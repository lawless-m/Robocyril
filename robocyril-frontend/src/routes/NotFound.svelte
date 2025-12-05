<script>
  import { link } from 'svelte-spa-router';
</script>

<div class="not-found">
  <h1>404</h1>
  <p class="message">Page not found.</p>
  <p class="subtext">Either you've typed something wrong or I've broken something. Probably me.</p>
  <a href="/" use:link class="back-link">&larr; Back to workshop</a>
</div>

<style>
  .not-found {
    text-align: center;
    padding: 4rem 1rem;
    animation: fadeIn 0.3s ease;
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(10px); }
    to { opacity: 1; transform: translateY(0); }
  }

  h1 {
    font-size: 4rem;
    font-weight: 600;
    color: var(--text-muted);
    margin-bottom: 0.5rem;
  }

  .message {
    font-size: 1.25rem;
    color: var(--text-primary);
    margin-bottom: 0.5rem;
  }

  .subtext {
    color: var(--text-secondary);
    font-style: italic;
    margin-bottom: 2rem;
  }

  .back-link {
    color: var(--accent);
    font-weight: 500;
  }

  .back-link:hover {
    color: var(--accent-hover);
  }
</style>
