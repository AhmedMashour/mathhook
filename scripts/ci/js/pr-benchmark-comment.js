// PR Benchmark Comment Script
// Updates or creates a comment with benchmark results on PRs
// Compiled from TypeScript source
'use strict';

const MARKER = '<!-- mathhook-benchmark-comment -->';

function validateInputs(results) {
  if (!results || typeof results !== 'object') {
    throw new Error('Invalid results: expected object');
  }
  if (typeof results.rust !== 'string' || typeof results.python !== 'string' || typeof results.node !== 'string') {
    throw new Error('Invalid results: missing rust, python, or node status');
  }
}

module.exports = async ({ github, context, results }) => {
  try {
    validateInputs(results);

    const { rust, python, node } = results;
    const statusIcon = (status) => (status === 'success' ? '✅' : '⚠️');

    const body = `${MARKER}
## Benchmark Results

| Platform | Status |
|----------|--------|
| Rust | ${statusIcon(rust)} |
| Python | ${statusIcon(python)} |
| Node.js | ${statusIcon(node)} |

[View Dashboard](https://${context.repo.owner}.github.io/${context.repo.repo}/)`;

    const { data: comments } = await github.rest.issues.listComments({
      owner: context.repo.owner,
      repo: context.repo.repo,
      issue_number: context.issue.number,
    });

    const existing = comments.find(
      (c) => c.user.type === 'Bot' && c.body.includes(MARKER)
    );

    if (existing) {
      await github.rest.issues.updateComment({
        owner: context.repo.owner,
        repo: context.repo.repo,
        comment_id: existing.id,
        body,
      });
    } else {
      await github.rest.issues.createComment({
        owner: context.repo.owner,
        repo: context.repo.repo,
        issue_number: context.issue.number,
        body,
      });
    }
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    console.error(`Failed to post benchmark comment: ${message}`);
    throw error;
  }
};
