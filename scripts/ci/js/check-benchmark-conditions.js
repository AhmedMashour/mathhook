// Check Benchmark Conditions Script
// Determines if benchmarks should run based on event type and labels
// Compiled from TypeScript source
'use strict';

module.exports = async ({ context, core }) => {
  const eventName = context.eventName;

  // Always run on push to main or manual dispatch
  if (eventName === 'push' || eventName === 'workflow_dispatch') {
    core.setOutput('run', 'true');
    core.setOutput('reason', eventName === 'push' ? 'push-to-main' : 'manual');
    return;
  }

  // For PRs, check for 'perf' label or '[benchmark]' in title
  if (eventName === 'pull_request' && context.payload.pull_request) {
    const labels = context.payload.pull_request.labels.map((l) => l.name);
    if (labels.includes('perf') || labels.includes('benchmark')) {
      core.setOutput('run', 'true');
      core.setOutput('reason', 'perf-label');
      return;
    }

    const title = context.payload.pull_request.title || '';
    if (title.includes('[benchmark]') || title.includes('[perf]')) {
      core.setOutput('run', 'true');
      core.setOutput('reason', 'commit-message');
      return;
    }

    // Skip - expensive and not requested
    core.setOutput('run', 'false');
    core.setOutput('reason', 'not-requested');
    core.notice('Benchmarks skipped. Add "perf" label or "[benchmark]" to PR title to run.');
    return;
  }

  core.setOutput('run', 'false');
  core.setOutput('reason', 'unknown-event');
};
