# Update Cyril Blog (steponnopets.net/cyril)

## Changes to Deploy

The following changes from the devblog need to be applied to the Cyril blog at steponnopets.net/cyril:

### 1. Frontend Changes

**Files modified:**
- `cyril-frontend/src/App.svelte` - Add route parameter for projects
- `cyril-frontend/src/routes/Projects.svelte` - Use route params instead of hash anchors
- `cyril-frontend/src/lib/Tag.svelte` - Update project tag links to use route params

**Changes:**

#### App.svelte
Add `/projects/:id` route:
```svelte
const routes = {
  '/': Home,
  '/post/:slug': Post,
  '/projects': Projects,
  '/projects/:id': Projects,  // Add this line
  '*': NotFound
};
```

#### Projects.svelte
Replace hash-based selection with route parameter:
```svelte
let { params } = $props();  // Add this at top

// Replace the hash handling effects with:
$effect(() => {
  selectedId = params?.id || null;

  if (selectedId && projects.length > 0) {
    setTimeout(() => {
      const element = document.getElementById(selectedId);
      if (element) {
        element.scrollIntoView({ behavior: 'smooth', block: 'center' });
      }
    }, 100);
  }
});
```

Also change "GitHub →" to "Repo →" on line ~80:
```svelte
<a href={project.repo} target="_blank" rel="noopener noreferrer" class="repo-link">
  Repo &rarr;
</a>
```

#### Tag.svelte
Change the project tag link from:
```svelte
<a href="/cyril/#/projects#{projectId}" class="tag project-tag">
```
To:
```svelte
<a href="/cyril/#/projects/{projectId}" class="tag project-tag">
```

### 2. Backend/CGI Scripts

**Check for CRLF line endings:**
- Run `ssh vsprod "sudo dos2unix /usr/lib/cgi-bin/cyril-feed.cgi /usr/lib/cgi-bin/cyril-projects.cgi"`
- Verify with `file /usr/lib/cgi-bin/cyril-*.cgi` - should show "ASCII text executable" not "CRLF line terminators"

### 3. Test After Deploy

1. Build: `cd cyril-frontend && npm run build`
2. Deploy: Use the same deployment pattern as devblog
3. Test project tags: Click a `® ProjectName` tag and verify it navigates to `/cyril/#/projects/projectname`
4. Test projects API: `curl https://steponnopets.net/cyril/api/projects`
5. Test RSS feed: `curl https://steponnopets.net/cyril/feed.xml`

## Why These Changes

The original implementation tried to use two `#` symbols in the URL (`/cyril/#/projects#projectname`), which doesn't work with hash-based routing. The fix uses route parameters instead (`/cyril/#/projects/projectname`).

## Date
Created: 2025-12-08
