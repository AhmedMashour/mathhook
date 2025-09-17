# MathHook Release Guide

This document explains how to release MathHook to package managers.

## Quick Release

```bash
# Bump patch version and release (0.1.0 -> 0.1.1)
./scripts/release.sh patch

# Bump minor version (0.1.0 -> 0.2.0)
./scripts/release.sh minor

# Bump major version (0.1.0 -> 1.0.0)
./scripts/release.sh major

# Set explicit version
./scripts/release.sh 0.2.0

# Pre-release version
./scripts/release.sh 0.2.0-alpha.1
```

The release script will:
1. Update version in all `Cargo.toml`, `pyproject.toml`, and `package.json` files
2. Run validation (fmt, clippy, tests)
3. Create a git commit and tag
4. Optionally push to trigger CI release

## Local Publishing

To publish directly from your machine (without CI):

```bash
# Publish to all registries
./scripts/publish-local.sh --all

# Publish only to specific registries
./scripts/publish-local.sh --crates  # crates.io only
./scripts/publish-local.sh --pypi    # PyPI only
./scripts/publish-local.sh --npm     # npm only

# Dry run (validate without publishing)
./scripts/publish-local.sh --dry-run --all
```

## Package Destinations

| Package | Registry | URL |
|---------|----------|-----|
| `mathhook` | crates.io | https://crates.io/crates/mathhook |
| `mathhook-core` | crates.io | https://crates.io/crates/mathhook-core |
| `mathhook-macros` | crates.io | https://crates.io/crates/mathhook-macros |
| `mathhook` | PyPI | https://pypi.org/project/mathhook/ |
| `mathhook-node` | npm | https://www.npmjs.com/package/mathhook-node |

## Setting Up API Keys

### 1. crates.io (Rust)

**Generate token:**
1. Go to https://crates.io/settings/tokens
2. Click "New Token"
3. Name: `mathhook-release`
4. Scopes: Select `publish-new` and `publish-update`
5. Click "Generate Token" and copy it

**Local setup:**
```bash
# Option A: Store in cargo credentials (recommended for local)
cargo login <YOUR_TOKEN>

# Option B: Use environment variable
export CARGO_REGISTRY_TOKEN=<YOUR_TOKEN>
```

**GitHub Actions setup:**
1. Go to your repo → Settings → Secrets and variables → Actions
2. Click "New repository secret"
3. Name: `CARGO_REGISTRY_TOKEN`
4. Value: Your crates.io token

### 2. PyPI (Python)

**Generate token:**
1. Go to https://pypi.org/manage/account/token/
2. Click "Add API token"
3. Token name: `mathhook-release`
4. Scope: Select "Entire account" (for first publish) or "Project: mathhook" (after first publish)
5. Click "Add token" and copy it (starts with `pypi-`)

**Local setup:**
```bash
# Option A: Create ~/.pypirc file
cat > ~/.pypirc << EOF
[pypi]
username = __token__
password = pypi-<YOUR_TOKEN>
EOF
chmod 600 ~/.pypirc

# Option B: Use environment variable
export PYPI_API_TOKEN=pypi-<YOUR_TOKEN>
```

**GitHub Actions setup:**
1. Go to your repo → Settings → Secrets and variables → Actions
2. Click "New repository secret"
3. Name: `PYPI_API_TOKEN`
4. Value: Your PyPI token (including the `pypi-` prefix)

**Alternative: Trusted Publishing (recommended for CI)**
1. Go to https://pypi.org/manage/project/mathhook/settings/publishing/
2. Add a new trusted publisher:
   - Owner: `AhmedMashour`
   - Repository: `mathhook`
   - Workflow: `release.yml`
   - Environment: `pypi`
3. No token needed in GitHub secrets!

### 3. npm (Node.js)

**Generate token:**
1. Go to https://www.npmjs.com/settings/YOUR_USERNAME/tokens
2. Click "Generate New Token"
3. Select **"Granular Access Token"** (recommended) or "Classic Token"
4. For Granular tokens:
   - Token name: `mathhook-release`
   - Expiration: Set as needed
   - Packages: Select "Only select packages" → `mathhook-node`
   - Permissions: "Read and write"
5. For Classic tokens:
   - Select **"Automation"** (NOT "Publish" - that requires 2FA)
6. Copy the token

**Local setup:**
```bash
# Option A: Login interactively
npm login

# Option B: Use environment variable
export NPM_TOKEN=<YOUR_TOKEN>

# Option C: Create .npmrc (for CI or scripted use)
echo "//registry.npmjs.org/:_authToken=<YOUR_TOKEN>" > ~/.npmrc
```

**GitHub Actions setup:**
1. Go to your repo → Settings → Secrets and variables → Actions
2. Click "New repository secret"
3. Name: `NPM_TOKEN`
4. Value: Your npm token

## GitHub Repository Secrets Summary

Add these secrets at: `https://github.com/AhmedMashour/mathhook/settings/secrets/actions`

| Secret Name | Source | Required For |
|-------------|--------|--------------|
| `CARGO_REGISTRY_TOKEN` | crates.io | Rust packages |
| `PYPI_API_TOKEN` | PyPI | Python package |
| `NPM_TOKEN` | npm | Node.js package |

## Release Workflow

### Via CI (Recommended)

1. **Prepare release:**
   ```bash
   ./scripts/release.sh 0.1.0
   ```

2. **Push triggers CI:**
   - When prompted, confirm push
   - CI workflow runs at: https://github.com/AhmedMashour/mathhook/actions

3. **CI automatically:**
   - Validates code (fmt, clippy, tests)
   - Publishes to crates.io (in order: macros → core → mathhook)
   - Builds Python wheels for all platforms
   - Publishes to PyPI
   - Builds Node.js binaries for all platforms
   - Publishes to npm
   - Creates GitHub Release with changelog

### Manual/Local

1. **Set environment variables:**
   ```bash
   export CARGO_REGISTRY_TOKEN=<token>
   export PYPI_API_TOKEN=<token>
   export NPM_TOKEN=<token>
   ```

2. **Update versions:**
   ```bash
   ./scripts/release.sh 0.1.0
   # Answer 'n' when asked to push
   ```

3. **Publish locally:**
   ```bash
   ./scripts/publish-local.sh --all
   ```

4. **Push after successful publish:**
   ```bash
   git push && git push origin v0.1.0
   ```

## Semantic Versioning

MathHook follows [Semantic Versioning](https://semver.org/):

- **MAJOR** (1.0.0): Breaking API changes
- **MINOR** (0.1.0): New features, backwards compatible
- **PATCH** (0.0.1): Bug fixes, backwards compatible

### Pre-release Versions

- `0.1.0-alpha.1` - Early development, unstable
- `0.1.0-beta.1` - Feature complete, testing
- `0.1.0-rc.1` - Release candidate

## Troubleshooting

### "crate already exists"
The crate version already exists on crates.io. Bump the version and try again.

### "403 Forbidden" on PyPI
Token doesn't have permission for this project. Generate a new token with "Entire account" scope for first publish.

### "npm ERR! 403"
Either:
- Package name is taken (change name in package.json)
- Token lacks publish permissions
- You're not logged in: `npm login`

### Build fails on CI
1. Check the Actions tab for detailed logs
2. Run locally first: `./scripts/publish-local.sh --dry-run --all`
3. Ensure all dependencies are properly declared

### Version mismatch
The release script updates all version files. If they get out of sync:
```bash
# Check current versions
grep 'version' Cargo.toml
grep 'version' crates/mathhook-python/pyproject.toml
grep '"version"' crates/mathhook-node/package.json

# Re-run release script to sync
./scripts/release.sh <version>
```

## First-Time Setup Checklist

Before your first release:

- [ ] Create crates.io account and generate token
- [ ] Create PyPI account and generate token
- [ ] Create npm account and generate token
- [ ] Add all tokens to GitHub repository secrets
- [ ] (Optional) Set up PyPI Trusted Publishing
- [ ] Run `./scripts/publish-local.sh --dry-run --all` to verify setup
- [ ] Commit and push the RELEASING.md and workflow files

## Verify Publication

After release, verify packages are available:

```bash
# Rust
cargo search mathhook

# Python
pip index versions mathhook

# Node.js
npm view mathhook-node versions
```

Or visit:
- https://crates.io/crates/mathhook
- https://pypi.org/project/mathhook/
- https://www.npmjs.com/package/mathhook-node
