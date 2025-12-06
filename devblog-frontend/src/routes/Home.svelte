<script>
  import { link } from 'svelte-spa-router';
  import { getPosts, formatDate, estimateReadingTime } from '../lib/api.js';
  import Skeleton from '../lib/Skeleton.svelte';

  let posts = $state([]);
  let loading = $state(true);
  let error = $state(null);

  $effect(() => {
    loadPosts();
  });

  async function loadPosts() {
    try {
      posts = await getPosts();
    } catch (e) {
      error = e.message;
    } finally {
      loading = false;
    }
  }
</script>

<div class="home">
  <section class="intro">
    <p class="tagline">Development updates: what we've built and how to use it.</p>
  </section>

  {#if loading}
    <div class="posts-list">
      {#each [1, 2, 3] as _}
        <article class="post-card skeleton-card">
          <Skeleton width="60%" height="1.5rem" />
          <Skeleton width="40%" height="0.875rem" style="margin-top: 0.75rem" />
          <Skeleton width="100%" height="1rem" style="margin-top: 1rem" />
          <Skeleton width="80%" height="1rem" style="margin-top: 0.5rem" />
        </article>
      {/each}
    </div>
  {:else if error}
    <div class="error">
      <p>Couldn't load posts. {error}</p>
      <p class="error-subtext">The workshop is having a moment.</p>
    </div>
  {:else if posts.length === 0}
    <div class="empty">
      <p>No posts yet.</p>
      <p class="empty-subtext">Check back when I've had something worth complaining about.</p>
    </div>
  {:else}
    <div class="posts-list">
      {#each posts as post}
        <article class="post-card">
          <a href="/post/{post.slug}" use:link class="post-link">
            <h2 class="post-title">{post.title}</h2>
          </a>
          <div class="post-meta">
            <time datetime={post.published_at || post.created_at}>
              {formatDate(post.published_at || post.created_at)}
            </time>
            {#if post.tags && post.tags.length > 0}
              <span class="separator">&middot;</span>
              <span class="tags">
                {#each post.tags.slice(0, 3) as tag}
                  <span class="tag">{tag}</span>
                {/each}
              </span>
            {/if}
          </div>
        </article>
      {/each}
    </div>
  {/if}
</div>

<style>
  .home {
    animation: fadeIn 0.3s ease;
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(10px); }
    to { opacity: 1; transform: translateY(0); }
  }

  .intro {
    margin-bottom: 3rem;
    padding-bottom: 2rem;
    border-bottom: 1px solid var(--border);
  }

  .tagline {
    color: var(--text-secondary);
    font-size: 1.1rem;
    font-style: italic;
  }

  .posts-list {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .post-card {
    padding: 1.5rem;
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    border-radius: 0.75rem;
    transition: border-color var(--transition), transform var(--transition);
  }

  .post-card:hover {
    border-color: var(--accent);
    transform: translateY(-2px);
  }

  .skeleton-card {
    pointer-events: none;
  }

  .post-link {
    display: block;
  }

  .post-title {
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--text-primary);
    line-height: 1.4;
    margin: 0;
  }

  .post-card:hover .post-title {
    color: var(--accent);
  }

  .post-meta {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 0.5rem;
    margin-top: 0.75rem;
    font-size: 0.875rem;
    color: var(--text-muted);
  }

  .separator {
    color: var(--border);
  }

  .tags {
    display: flex;
    gap: 0.375rem;
  }

  .tag {
    background: var(--bg-tertiary);
    padding: 0.125rem 0.5rem;
    border-radius: 0.25rem;
    font-size: 0.75rem;
    color: var(--text-secondary);
  }

  .error, .empty {
    text-align: center;
    padding: 3rem 1rem;
    color: var(--text-secondary);
  }

  .error-subtext, .empty-subtext {
    margin-top: 0.5rem;
    font-size: 0.875rem;
    color: var(--text-muted);
    font-style: italic;
  }
</style>
