// Create Rollback Tracking Issue Script
// Creates a GitHub issue to track rollback actions
// Compiled from TypeScript source
'use strict';

function validateInputs(params) {
  const required = ['version', 'reason', 'targets', 'actor', 'runId', 'serverUrl'];
  for (const field of required) {
    if (!params[field] || typeof params[field] !== 'string') {
      throw new Error(`Missing or invalid required field: ${field}`);
    }
  }
}

function escapeMarkdown(text) {
  return text
    .replace(/\|/g, '\\|')
    .replace(/`/g, '\\`')
    .replace(/\*/g, '\\*')
    .replace(/_/g, '\\_');
}

module.exports = async (params) => {
  try {
    validateInputs(params);

    const {
      github,
      context,
      version,
      reason,
      targets,
      actor,
      runId,
      serverUrl,
      cratesResult,
      npmResult,
      pypiResult,
    } = params;

    const safeReason = escapeMarkdown(reason);

    const title = `Release Rollback: v${version}`;
    const body = `## Release Rollback

**Version:** ${version}
**Reason:** ${safeReason}
**Targets:** ${targets}
**Triggered by:** @${actor}
**Workflow run:** ${serverUrl}/${context.repo.owner}/${context.repo.repo}/actions/runs/${runId}

### Rollback Status

| Registry | Status |
|----------|--------|
| crates.io | ${cratesResult || 'skipped'} |
| npm | ${npmResult || 'skipped'} |
| PyPI | ${pypiResult || 'skipped'} |

### Required Actions

- [ ] Investigate root cause
- [ ] Prepare fix
- [ ] Test fix thoroughly
- [ ] Release patched version
- [ ] Close this issue
`;

    await github.rest.issues.create({
      owner: context.repo.owner,
      repo: context.repo.repo,
      title,
      body,
      labels: ['release', 'rollback', 'urgent'],
    });
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    console.error(`Failed to create rollback issue: ${message}`);
    throw error;
  }
};
