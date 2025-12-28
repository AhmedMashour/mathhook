// Post Benchmark Skip Notice Script
// Informs users when benchmarks are skipped to save CI minutes
// Compiled from TypeScript source
'use strict';

const MARKER = '<!-- mathhook-benchmark-skip -->';

module.exports = async ({ github, context }) => {
  const body = `${MARKER}
## ⏭️ Benchmarks Skipped

Benchmarks are expensive and run only when requested. To run benchmarks:
- Add the \`perf\` label to this PR, OR
- Include \`[benchmark]\` in the PR title

*This saves GitHub Actions minutes for the project.*`;

  // Check if we already posted this
  const { data: comments } = await github.rest.issues.listComments({
    owner: context.repo.owner,
    repo: context.repo.repo,
    issue_number: context.issue.number,
  });

  const existing = comments.find((c) => c.body.includes(MARKER));
  if (!existing) {
    await github.rest.issues.createComment({
      owner: context.repo.owner,
      repo: context.repo.repo,
      issue_number: context.issue.number,
      body,
    });
  }
};
