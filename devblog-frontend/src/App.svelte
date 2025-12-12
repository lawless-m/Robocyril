<script>
  import Router from 'svelte-spa-router';
  import Header from './lib/Header.svelte';
  import Home from './routes/Home.svelte';
  import Post from './routes/Post.svelte';
  import Projects from './routes/Projects.svelte';
  import Admin from './routes/Admin.svelte';
  import NotFound from './routes/NotFound.svelte';

  const routes = {
    '/': Home,
    '/post/:slug': Post,
    '/projects': Projects,
    '/projects/:id': Projects,
    '/admin': Admin,
    '*': NotFound
  };
</script>

<Header />
<main>
  <Router {routes} />
</main>

<style>
  main {
    flex: 1;
    max-width: var(--max-width);
    width: 100%;
    margin: 0 auto;
    padding: 2rem 1.5rem;
  }

  @media (max-width: 640px) {
    main {
      padding: 1.5rem 1rem;
    }
  }
</style>
