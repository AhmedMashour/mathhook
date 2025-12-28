// Post Regression Warning Script
// Posts a comment on PR when performance regression is detected
// Compiled from TypeScript source
'use strict';

module.exports = async ({ github, context, threshold }) => {
  const body = `## ⚠️ Performance Regression Detected

The benchmarks show a regression exceeding the ${threshold}% threshold.

Please review the benchmark results and consider:
- Reverting the performance-impacting changes
- Optimizing the new code path
- Adding the \`perf-accepted\` label if this is an acceptable tradeoff

[View Details](${context.serverUrl}/${context.repo.owner}/${context.repo.repo}/actions/runs/${context.runId})`;

  await github.rest.issues.createComment({
    owner: context.repo.owner,
    repo: context.repo.repo,
    issue_number: context.issue.number,
    body,
  });
};
