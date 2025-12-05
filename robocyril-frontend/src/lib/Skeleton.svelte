<script>
  let { width = '100%', height = '1rem', style = '' } = $props();
</script>

<div class="skeleton" style="width: {width}; height: {height}; {style}"></div>

<style>
  .skeleton {
    background: linear-gradient(
      90deg,
      var(--bg-tertiary) 25%,
      var(--border) 50%,
      var(--bg-tertiary) 75%
    );
    background-size: 200% 100%;
    animation: shimmer 1.5s infinite;
    border-radius: 0.25rem;
  }

  @keyframes shimmer {
    0% { background-position: 200% 0; }
    100% { background-position: -200% 0; }
  }
</style>
