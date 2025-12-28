// Create Release Failure Issue Script
// Creates a GitHub issue when a release fails
// Compiled from TypeScript source
'use strict';

function validateInputs(params) {
  const required = ['refName', 'runId', 'serverUrl'];
  for (const field of required) {
    if (!params[field] || typeof params[field] !== 'string') {
      throw new Error(`Missing or invalid required field: ${field}`);
    }
  }
}

module.exports = async (params) => {
  try {
    validateInputs(params);

    const { github, context, refName, runId, serverUrl } = params;

    const title = `Release Failed: ${refName}`;
    const body = `## Release Failure

**Tag:** ${refName}
**Workflow Run:** ${serverUrl}/${context.repo.owner}/${context.repo.repo}/actions/runs/${runId}

### Required Actions

- [ ] Investigate the failure
- [ ] Fix the issue
- [ ] Re-run the release workflow or create a new tag

### Troubleshooting

1. Check the workflow logs for specific errors
2. Verify all secrets are configured correctly
3. Ensure version numbers are consistent across all package files
4. Check if the version already exists on any registry
`;

    await github.rest.issues.create({
      owner: context.repo.owner,
      repo: context.repo.repo,
      title,
      body,
      labels: ['release', 'bug', 'urgent'],
    });
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    console.error(`Failed to create release failure issue: ${message}`);
    throw error;
  }
};
