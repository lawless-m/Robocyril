<script>
  import { link, push } from 'svelte-spa-router';
  import { marked } from 'marked';
  import { getPost, formatDate, estimateReadingTime } from '../lib/api.js';
  import Skeleton from '../lib/Skeleton.svelte';
  import Tag from '../lib/Tag.svelte';

  let { params } = $props();

  let post = $state(null);
  let loading = $state(true);
  let error = $state(null);
  let renderedContent = $state('');

  // Configure marked
  marked.setOptions({
    gfm: true,
    breaks: false
  });

  $effect(() => {
    if (params?.slug) {
      loadPost(params.slug);
    }
  });

  async function loadPost(slug) {
    loading = true;
    error = null;
    try {
      post = await getPost(slug);
      if (post) {
        renderedContent = marked(post.content);
      }
    } catch (e) {
      error = e.message;
    } finally {
      loading = false;
    }
  }
</script>

<article class="post">
  {#if loading}
    <div class="post-header">
      <Skeleton width="80%" height="2rem" />
      <Skeleton width="30%" height="1rem" style="margin-top: 1rem" />
    </div>
    <div class="post-content">
      <Skeleton width="100%" height="1rem" style="margin-top: 2rem" />
      <Skeleton width="95%" height="1rem" style="margin-top: 0.75rem" />
      <Skeleton width="100%" height="1rem" style="margin-top: 0.75rem" />
      <Skeleton width="60%" height="1rem" style="margin-top: 0.75rem" />
    </div>
  {:else if error}
    <div class="error">
      <p>Couldn't load post. {error}</p>
      <a href="/" use:link class="back-link">&larr; Back to workshop</a>
    </div>
  {:else if !post}
    <div class="not-found">
      <h1>Post not found</h1>
      <p>Either it never existed or I deleted it in a fit of pique.</p>
      <a href="/" use:link class="back-link">&larr; Back to workshop</a>
    </div>
  {:else}
    <header class="post-header">
      <a href="/" use:link class="back-link">&larr; Back</a>
      <h1 class="post-title">{post.title}</h1>
      <div class="post-meta">
        <time datetime={post.published_at || post.created_at}>
          {formatDate(post.published_at || post.created_at)}
        </time>
        <span class="separator">&middot;</span>
        <span class="reading-time">{estimateReadingTime(post.content)}</span>
        {#if post.tags && post.tags.length > 0}
          <span class="separator">&middot;</span>
          <span class="tags">
            {#each post.tags as tag}
              <Tag {tag} />
            {/each}
          </span>
        {/if}
      </div>
    </header>

    <div class="post-content prose">
      {@html renderedContent}
    </div>

    {#if post.repo}
      <footer class="post-footer">
        <a href={post.repo} target="_blank" rel="noopener noreferrer" class="repo-link">
          View on GitHub &rarr;
        </a>
      </footer>
    {/if}
  {/if}
</article>

<style>
  .post {
    animation: fadeIn 0.3s ease;
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(10px); }
    to { opacity: 1; transform: translateY(0); }
  }

  .back-link {
    display: inline-block;
    color: var(--text-secondary);
    font-size: 0.875rem;
    margin-bottom: 1.5rem;
    transition: color var(--transition);
  }

  .back-link:hover {
    color: var(--accent);
  }

  .post-header {
    margin-bottom: 2rem;
    padding-bottom: 1.5rem;
    border-bottom: 1px solid var(--border);
  }

  .post-title {
    font-size: 2rem;
    font-weight: 600;
    line-height: 1.3;
    margin: 0;
    color: var(--text-primary);
  }

  .post-meta {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 0.5rem;
    margin-top: 1rem;
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

  /* Prose styles for markdown content */
  .prose {
    font-size: 1.0625rem;
    line-height: 1.75;
    color: var(--text-primary);
  }

  .prose :global(h1),
  .prose :global(h2),
  .prose :global(h3),
  .prose :global(h4) {
    margin-top: 2rem;
    margin-bottom: 1rem;
    font-weight: 600;
    line-height: 1.3;
    color: var(--text-primary);
  }

  .prose :global(h2) { font-size: 1.5rem; }
  .prose :global(h3) { font-size: 1.25rem; }
  .prose :global(h4) { font-size: 1.125rem; }

  .prose :global(p) {
    margin-bottom: 1.25rem;
  }

  .prose :global(a) {
    color: var(--accent);
    text-decoration: underline;
    text-underline-offset: 2px;
  }

  .prose :global(a:hover) {
    color: var(--accent-hover);
  }

  .prose :global(strong) {
    font-weight: 600;
    color: var(--text-primary);
  }

  .prose :global(code) {
    background: var(--code-bg);
    padding: 0.2em 0.4em;
    border-radius: 0.25rem;
    font-size: 0.875em;
    color: var(--accent);
  }

  .prose :global(pre) {
    background: var(--code-bg);
    padding: 1rem;
    border-radius: 0.5rem;
    overflow-x: auto;
    margin: 1.5rem 0;
    border: 1px solid var(--border);
  }

  .prose :global(pre code) {
    background: none;
    padding: 0;
    color: var(--text-primary);
    font-size: 0.875rem;
    line-height: 1.6;
  }

  .prose :global(blockquote) {
    border-left: 3px solid var(--accent);
    padding-left: 1rem;
    margin: 1.5rem 0;
    color: var(--text-secondary);
    font-style: italic;
  }

  .prose :global(ul),
  .prose :global(ol) {
    margin: 1rem 0;
    padding-left: 1.5rem;
  }

  .prose :global(li) {
    margin-bottom: 0.5rem;
  }

  .prose :global(hr) {
    border: none;
    border-top: 1px solid var(--border);
    margin: 2rem 0;
  }

  .prose :global(img) {
    max-width: 100%;
    height: auto;
    border-radius: 0.5rem;
    margin: 1.5rem 0;
  }

  .post-footer {
    margin-top: 3rem;
    padding-top: 1.5rem;
    border-top: 1px solid var(--border);
  }

  .repo-link {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    color: var(--text-secondary);
    font-size: 0.875rem;
    transition: color var(--transition);
  }

  .repo-link:hover {
    color: var(--accent);
  }

  .error, .not-found {
    text-align: center;
    padding: 3rem 1rem;
    color: var(--text-secondary);
  }

  .not-found h1 {
    font-size: 1.5rem;
    margin-bottom: 0.5rem;
    color: var(--text-primary);
  }

  .error .back-link,
  .not-found .back-link {
    display: inline-block;
    margin-top: 1.5rem;
  }

  @media (max-width: 640px) {
    .post-title {
      font-size: 1.5rem;
    }

    .prose {
      font-size: 1rem;
    }
  }
</style>
