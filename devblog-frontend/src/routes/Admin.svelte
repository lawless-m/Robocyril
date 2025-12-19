<script>
  import { link } from 'svelte-spa-router';

  let apiKey = $state('');
  let authenticated = $state(false);
  let activeTab = $state('posts');

  let posts = $state([]);
  let projects = $state([]);
  let loading = $state(false);
  let error = $state(null);
  let success = $state(null);

  // Edit states
  let editingPost = $state(null);
  let editingTags = $state('');
  let editingProject = $state(null);

  // Full post editor
  let editingFullPost = $state(null);
  let editTitle = $state('');
  let editContent = $state('');
  let editTagsText = $state('');

  async function authenticate() {
    loading = true;
    error = null;
    try {
      // Test the API key by trying to fetch all posts (including drafts)
      const response = await fetch('/devblog/api/posts?include_drafts=true', {
        headers: {
          'X-Cyril-Key': apiKey
        }
      });

      if (response.ok) {
        authenticated = true;
        await loadData();
      } else {
        error = 'Invalid API key';
      }
    } catch (e) {
      error = `Authentication failed: ${e.message}`;
    } finally {
      loading = false;
    }
  }

  async function loadData() {
    loading = true;
    error = null;
    try {
      // Load posts
      const postsResponse = await fetch('/devblog/api/posts?include_drafts=true');
      if (postsResponse.ok) {
        posts = await postsResponse.json();
      }

      // Load projects
      const projectsResponse = await fetch('/devblog/api/projects');
      if (projectsResponse.ok) {
        projects = await projectsResponse.json();
      }
    } catch (e) {
      error = `Failed to load data: ${e.message}`;
    } finally {
      loading = false;
    }
  }

  async function deletePost(slug) {
    if (!confirm(`Delete post "${slug}"?`)) return;

    loading = true;
    error = null;
    success = null;
    try {
      const response = await fetch(`/devblog/api/post?slug=${encodeURIComponent(slug)}`, {
        method: 'DELETE',
        headers: {
          'X-Cyril-Key': apiKey
        }
      });

      if (response.ok) {
        success = 'Post deleted successfully';
        await loadData();
      } else {
        const data = await response.json().catch(() => ({}));
        error = data.error || 'Failed to delete post';
      }
    } catch (e) {
      error = `Delete failed: ${e.message}`;
    } finally {
      loading = false;
    }
  }

  async function togglePublish(post) {
    loading = true;
    error = null;
    success = null;
    try {
      const response = await fetch(`/devblog/api/post?slug=${encodeURIComponent(post.slug)}`, {
        method: 'PATCH',
        headers: {
          'Content-Type': 'application/json',
          'X-Cyril-Key': apiKey
        },
        body: JSON.stringify({
          title: post.title,
          publish: !post.published_at
        })
      });

      if (response.ok) {
        success = post.published_at ? 'Post unpublished' : 'Post published';
        await loadData();
      } else {
        const data = await response.json().catch(() => ({}));
        error = data.error || 'Failed to update post';
      }
    } catch (e) {
      error = `Update failed: ${e.message}`;
    } finally {
      loading = false;
    }
  }

  async function syncProjects() {
    loading = true;
    error = null;
    success = null;
    try {
      // This would need a new backend endpoint to re-sync all projects
      success = 'Project sync not yet implemented';
    } catch (e) {
      error = `Sync failed: ${e.message}`;
    } finally {
      loading = false;
    }
  }

  function startEditTags(post) {
    editingPost = post;
    editingTags = post.tags.join(', ');
  }

  function cancelEditTags() {
    editingPost = null;
    editingTags = '';
  }

  async function saveTags() {
    if (!editingPost) return;

    loading = true;
    error = null;
    success = null;
    try {
      // Parse tags from comma-separated string
      const tags = editingTags.split(',').map(t => t.trim()).filter(t => t);

      const response = await fetch(`/devblog/api/post?slug=${encodeURIComponent(editingPost.slug)}`, {
        method: 'PATCH',
        headers: {
          'Content-Type': 'application/json',
          'X-Cyril-Key': apiKey
        },
        body: JSON.stringify({
          title: editingPost.title,
          tags: tags,
          publish: !!editingPost.published_at
        })
      });

      if (response.ok) {
        success = 'Tags updated successfully';
        editingPost = null;
        editingTags = '';
        await loadData();
      } else {
        const data = await response.json().catch(() => ({}));
        error = data.error || 'Failed to update tags';
      }
    } catch (e) {
      error = `Update failed: ${e.message}`;
    } finally {
      loading = false;
    }
  }

  function startEditPost(post) {
    editingFullPost = post;
    editTitle = post.title;
    editContent = post.content;
    editTagsText = post.tags.join(', ');
  }

  function cancelEditPost() {
    editingFullPost = null;
    editTitle = '';
    editContent = '';
    editTagsText = '';
  }

  async function savePost() {
    if (!editingFullPost) return;

    loading = true;
    error = null;
    success = null;
    try {
      const tags = editTagsText.split(',').map(t => t.trim()).filter(t => t);

      const response = await fetch(`/devblog/api/post?slug=${encodeURIComponent(editingFullPost.slug)}`, {
        method: 'PATCH',
        headers: {
          'Content-Type': 'application/json',
          'X-Cyril-Key': apiKey
        },
        body: JSON.stringify({
          title: editTitle,
          content: editContent,
          tags: tags,
          publish: !!editingFullPost.published_at
        })
      });

      if (response.ok) {
        success = 'Post updated successfully';
        cancelEditPost();
        await loadData();
      } else {
        const data = await response.json().catch(() => ({}));
        error = data.error || 'Failed to update post';
      }
    } catch (e) {
      error = `Update failed: ${e.message}`;
    } finally {
      loading = false;
    }
  }
</script>

<div class="admin-page">
  {#if !authenticated}
    <div class="login-form">
      <h1>Admin Panel</h1>
      <p>Enter API key to access admin functions</p>

      <input
        type="password"
        bind:value={apiKey}
        placeholder="API Key"
        onkeydown={(e) => e.key === 'Enter' && authenticate()}
      />

      <button onclick={authenticate} disabled={loading || !apiKey}>
        {loading ? 'Authenticating...' : 'Login'}
      </button>

      {#if error}
        <div class="error">{error}</div>
      {/if}
    </div>
  {:else}
    <div class="admin-container">
      <header class="admin-header">
        <h1>Admin Panel</h1>
        <a href="/" use:link class="back-link">&larr; Back to Blog</a>
      </header>

      {#if error}
        <div class="alert error">{error}</div>
      {/if}

      {#if success}
        <div class="alert success">{success}</div>
      {/if}

      <div class="tabs">
        <button
          class="tab"
          class:active={activeTab === 'posts'}
          onclick={() => activeTab = 'posts'}
        >
          Posts ({posts.length})
        </button>
        <button
          class="tab"
          class:active={activeTab === 'projects'}
          onclick={() => activeTab = 'projects'}
        >
          Projects ({projects.length})
        </button>
      </div>

      {#if editingFullPost}
        <div class="modal-overlay" onclick={cancelEditPost}>
          <div class="modal" onclick={(e) => e.stopPropagation()}>
            <div class="modal-header">
              <h2>Edit Post: {editingFullPost.slug}</h2>
              <button class="close-button" onclick={cancelEditPost}>&times;</button>
            </div>
            <div class="modal-body">
              <div class="form-group">
                <label>Title</label>
                <input
                  type="text"
                  bind:value={editTitle}
                  placeholder="Post title"
                  class="form-input"
                />
              </div>
              <div class="form-group">
                <label>Tags (comma-separated)</label>
                <input
                  type="text"
                  bind:value={editTagsText}
                  placeholder="rust, svelte, web"
                  class="form-input"
                />
              </div>
              <div class="form-group">
                <label>Content (Markdown)</label>
                <textarea
                  bind:value={editContent}
                  placeholder="# Post content here..."
                  class="form-textarea"
                  rows="20"
                ></textarea>
              </div>
            </div>
            <div class="modal-footer">
              <button class="btn-secondary" onclick={cancelEditPost}>Cancel</button>
              <button class="btn-primary" onclick={savePost} disabled={loading}>
                {loading ? 'Saving...' : 'Save Changes'}
              </button>
            </div>
          </div>
        </div>
      {/if}

      <div class="tab-content">
        {#if activeTab === 'posts'}
          <div class="posts-list">
            <div class="list-header">
              <h2>All Posts</h2>
              <button onclick={loadData} disabled={loading}>Refresh</button>
            </div>

            <table class="data-table">
              <thead>
                <tr>
                  <th>Title</th>
                  <th>Slug</th>
                  <th>Tags</th>
                  <th>Status</th>
                  <th>Created</th>
                  <th>Actions</th>
                </tr>
              </thead>
              <tbody>
                {#each posts as post}
                  <tr>
                    <td>{post.title}</td>
                    <td><code>{post.slug}</code></td>
                    <td>
                      {#if editingPost?.slug === post.slug}
                        <div class="tag-editor">
                          <input
                            type="text"
                            bind:value={editingTags}
                            placeholder="tag1, tag2, tag3"
                            class="tag-input"
                          />
                          <div class="tag-editor-actions">
                            <button
                              class="btn-small"
                              onclick={saveTags}
                              disabled={loading}
                            >
                              Save
                            </button>
                            <button
                              class="btn-small"
                              onclick={cancelEditTags}
                            >
                              Cancel
                            </button>
                          </div>
                        </div>
                      {:else}
                        <div class="tags">
                          {#each post.tags as tag}
                            <span class="tag">{tag}</span>
                          {/each}
                        </div>
                      {/if}
                    </td>
                    <td>
                      <span class="badge" class:published={post.published_at} class:draft={!post.published_at}>
                        {post.published_at ? 'Published' : 'Draft'}
                      </span>
                    </td>
                    <td>{new Date(post.created_at).toLocaleDateString()}</td>
                    <td class="actions">
                      {#if editingPost?.slug !== post.slug}
                        <button
                          class="btn-small"
                          onclick={() => startEditPost(post)}
                          disabled={loading}
                        >
                          Edit Post
                        </button>
                        <button
                          class="btn-small"
                          onclick={() => startEditTags(post)}
                          disabled={loading}
                        >
                          Edit Tags
                        </button>
                        <button
                          class="btn-small"
                          onclick={() => togglePublish(post)}
                          disabled={loading}
                        >
                          {post.published_at ? 'Unpublish' : 'Publish'}
                        </button>
                        <button
                          class="btn-small btn-danger"
                          onclick={() => deletePost(post.slug)}
                          disabled={loading}
                        >
                          Delete
                        </button>
                      {/if}
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        {:else}
          <div class="projects-list">
            <div class="list-header">
              <h2>All Projects</h2>
              <button onclick={syncProjects} disabled={loading}>Sync from Posts</button>
            </div>

            <table class="data-table">
              <thead>
                <tr>
                  <th>Name</th>
                  <th>ID</th>
                  <th>Repository</th>
                  <th>Created</th>
                </tr>
              </thead>
              <tbody>
                {#each projects as project}
                  <tr>
                    <td>® {project.name}</td>
                    <td><code>{project.id}</code></td>
                    <td><a href={project.repo} target="_blank">{project.repo}</a></td>
                    <td>{new Date(project.created_at).toLocaleDateString()}</td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>

<style>
  .admin-page {
    padding: 2rem;
    max-width: 1200px;
    margin: 0 auto;
  }

  .login-form {
    max-width: 400px;
    margin: 4rem auto;
    padding: 2rem;
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    border-radius: 0.5rem;
    text-align: center;
  }

  .login-form h1 {
    margin-bottom: 1rem;
  }

  .login-form p {
    color: var(--text-secondary);
    margin-bottom: 2rem;
  }

  .login-form input {
    width: 100%;
    padding: 0.75rem;
    margin-bottom: 1rem;
    background: var(--bg-primary);
    border: 1px solid var(--border);
    border-radius: 0.25rem;
    color: var(--text-primary);
    font-size: 1rem;
  }

  .login-form button {
    width: 100%;
    padding: 0.75rem;
    background: var(--accent);
    color: var(--bg-primary);
    border: none;
    border-radius: 0.25rem;
    font-size: 1rem;
    font-weight: 600;
    cursor: pointer;
  }

  .login-form button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .admin-container {
    animation: fadeIn 0.3s ease;
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  .admin-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
    padding-bottom: 1rem;
    border-bottom: 1px solid var(--border);
  }

  .admin-header h1 {
    margin: 0;
  }

  .back-link {
    color: var(--text-secondary);
    text-decoration: none;
    transition: color var(--transition);
  }

  .back-link:hover {
    color: var(--accent);
  }

  .alert {
    padding: 1rem;
    margin-bottom: 1rem;
    border-radius: 0.25rem;
  }

  .alert.error {
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.3);
    color: #ef4444;
  }

  .alert.success {
    background: rgba(34, 197, 94, 0.1);
    border: 1px solid rgba(34, 197, 94, 0.3);
    color: #22c55e;
  }

  .tabs {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 2rem;
    border-bottom: 1px solid var(--border);
  }

  .tab {
    padding: 0.75rem 1.5rem;
    background: none;
    border: none;
    border-bottom: 2px solid transparent;
    color: var(--text-secondary);
    cursor: pointer;
    font-size: 1rem;
    transition: all var(--transition);
  }

  .tab:hover {
    color: var(--text-primary);
  }

  .tab.active {
    color: var(--accent);
    border-bottom-color: var(--accent);
  }

  .list-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .list-header h2 {
    margin: 0;
  }

  .list-header button {
    padding: 0.5rem 1rem;
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    border-radius: 0.25rem;
    color: var(--text-primary);
    cursor: pointer;
    transition: all var(--transition);
  }

  .list-header button:hover {
    border-color: var(--accent);
  }

  .list-header button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .data-table {
    width: 100%;
    border-collapse: collapse;
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    border-radius: 0.5rem;
  }

  .data-table thead {
    background: var(--bg-tertiary);
  }

  .data-table th {
    padding: 1rem;
    text-align: left;
    font-weight: 600;
    border-bottom: 1px solid var(--border);
  }

  .data-table td {
    padding: 1rem;
    border-bottom: 1px solid var(--border);
  }

  .data-table tbody tr:last-child td {
    border-bottom: none;
  }

  .data-table tbody tr:hover {
    background: var(--bg-tertiary);
  }

  .data-table code {
    font-family: 'IBM Plex Mono', monospace;
    font-size: 0.875rem;
    color: var(--accent);
  }

  .data-table a {
    color: var(--accent);
    text-decoration: none;
  }

  .data-table a:hover {
    text-decoration: underline;
  }

  .badge {
    display: inline-block;
    padding: 0.25rem 0.75rem;
    border-radius: 1rem;
    font-size: 0.75rem;
    font-weight: 600;
  }

  .badge.published {
    background: rgba(34, 197, 94, 0.2);
    color: #22c55e;
  }

  .badge.draft {
    background: rgba(156, 163, 175, 0.2);
    color: #9ca3af;
  }

  .actions {
    display: flex;
    gap: 0.5rem;
  }

  .btn-small {
    padding: 0.25rem 0.75rem;
    background: var(--accent);
    color: var(--bg-primary);
    border: none;
    border-radius: 0.25rem;
    font-size: 0.875rem;
    cursor: pointer;
    transition: opacity var(--transition);
  }

  .btn-small:hover {
    opacity: 0.8;
  }

  .btn-small:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-danger {
    background: #ef4444;
  }

  .error {
    color: #ef4444;
    margin-top: 1rem;
  }

  .tags {
    display: flex;
    flex-wrap: wrap;
    gap: 0.25rem;
  }

  .tag {
    background: var(--bg-tertiary);
    color: var(--text-secondary);
    padding: 0.25rem 0.5rem;
    border-radius: 0.25rem;
    font-size: 0.75rem;
    border: 1px solid var(--border);
  }

  .tag-editor {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .tag-input {
    padding: 0.5rem;
    background: var(--bg-primary);
    border: 1px solid var(--border);
    border-radius: 0.25rem;
    color: var(--text-primary);
    font-size: 0.875rem;
    min-width: 300px;
  }

  .tag-input:focus {
    outline: none;
    border-color: var(--accent);
  }

  .tag-editor-actions {
    display: flex;
    gap: 0.5rem;
  }

  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    animation: fadeIn 0.2s ease;
  }

  .modal {
    background: var(--bg-primary);
    border: 1px solid var(--border);
    border-radius: 0.5rem;
    width: 90%;
    max-width: 900px;
    max-height: 90vh;
    display: flex;
    flex-direction: column;
    animation: slideIn 0.3s ease;
  }

  @keyframes slideIn {
    from {
      transform: translateY(-20px);
      opacity: 0;
    }
    to {
      transform: translateY(0);
      opacity: 1;
    }
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem;
    border-bottom: 1px solid var(--border);
  }

  .modal-header h2 {
    margin: 0;
    font-size: 1.25rem;
  }

  .close-button {
    background: none;
    border: none;
    font-size: 2rem;
    color: var(--text-secondary);
    cursor: pointer;
    line-height: 1;
    padding: 0;
    width: 2rem;
    height: 2rem;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .close-button:hover {
    color: var(--text-primary);
  }

  .modal-body {
    padding: 1.5rem;
    overflow-y: auto;
    flex: 1;
  }

  .form-group {
    margin-bottom: 1.5rem;
  }

  .form-group label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 600;
    color: var(--text-secondary);
  }

  .form-input {
    width: 100%;
    padding: 0.75rem;
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    border-radius: 0.25rem;
    color: var(--text-primary);
    font-size: 1rem;
    font-family: inherit;
  }

  .form-input:focus {
    outline: none;
    border-color: var(--accent);
  }

  .form-textarea {
    width: 100%;
    padding: 0.75rem;
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    border-radius: 0.25rem;
    color: var(--text-primary);
    font-size: 0.875rem;
    font-family: 'IBM Plex Mono', monospace;
    resize: vertical;
    min-height: 400px;
  }

  .form-textarea:focus {
    outline: none;
    border-color: var(--accent);
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 1rem;
    padding: 1.5rem;
    border-top: 1px solid var(--border);
  }

  .btn-primary {
    padding: 0.75rem 1.5rem;
    background: var(--accent);
    color: var(--bg-primary);
    border: none;
    border-radius: 0.25rem;
    font-size: 1rem;
    font-weight: 600;
    cursor: pointer;
    transition: opacity var(--transition);
  }

  .btn-primary:hover {
    opacity: 0.9;
  }

  .btn-primary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-secondary {
    padding: 0.75rem 1.5rem;
    background: var(--bg-secondary);
    color: var(--text-primary);
    border: 1px solid var(--border);
    border-radius: 0.25rem;
    font-size: 1rem;
    cursor: pointer;
    transition: border-color var(--transition);
  }

  .btn-secondary:hover {
    border-color: var(--accent);
  }
</style>
