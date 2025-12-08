// Helper functions for project tags (® symbol)
// Project tags format: "® ProjectName"
// Projects are stored in SQLite and fetched via API

// Check if a tag is a project tag (starts with ®)
export function isProjectTag(tag) {
  return tag.startsWith('®');
}

// Get the project ID from a tag (for URL hash)
// "® Robocyril" → "robocyril"
export function getProjectIdFromTag(tag) {
  if (!isProjectTag(tag)) return null;
  // Remove the ® prefix and any leading space, then lowercase
  return tag.replace(/^®\s*/, '').toLowerCase();
}

// Generate the tag string from a project
// { id: 'robocyril', name: 'Robocyril' } → "® Robocyril"
export function getTagFromProject(project) {
  return `® ${project.name}`;
}
