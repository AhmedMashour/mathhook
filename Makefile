# MathHook Build System
# Docker-based cross-platform build and publish pipeline
#
# Quick Reference:
#   make build-all     - Build all platforms (Rust + Python + Node)
#   make publish-all   - Build and publish to all registries
#   make shell         - Debug shell in builder container
#   make clean         - Remove artifacts and caches

.PHONY: help setup build-all build-rust build-python build-node \
        publish-all test shell clean clean-cache \
        docker-pull docker-build docker-push docker-ensure \
        release release-patch release-minor release-major release-dry

# Builder image (pulls from ghcr.io by default, override for custom registry)
MATHHOOK_BUILDER_IMAGE ?= ghcr.io/ahmedmashour/mathhook-core/builder:latest

# Default target
help:
	@echo "MathHook Docker Build System"
	@echo ""
	@echo "Build Commands:"
	@echo "  make build-all      Build all platforms (Rust + Python + Node)"
	@echo "  make build-rust     Build Rust crates only"
	@echo "  make build-python   Build Python wheels (all platforms)"
	@echo "  make build-node     Build Node.js addons (all platforms)"
	@echo ""
	@echo "Release (one command):"
	@echo "  make release-patch  Bump + build + publish (auto-detects Docker/CI)"
	@echo "  make release-minor  Same for minor version"
	@echo "  make release V=x.y.z  Explicit version"
	@echo "  Options: DOCKER=1 (force local), CI=1 (force push to CI)"
	@echo ""
	@echo "Docker Image:"
	@echo "  make docker-pull    Pull pre-built builder image (fast, recommended)"
	@echo "  make docker-build   Build image locally (slow, ~30 min)"
	@echo "  make docker-push    Push image to ghcr.io (maintainers only)"
	@echo ""
	@echo "Development:"
	@echo "  make setup          First-time setup (pull image, verify)"
	@echo "  make test           Run all tests"
	@echo "  make shell          Interactive shell in builder container"
	@echo ""
	@echo "Cleanup:"
	@echo "  make clean          Remove build artifacts"
	@echo "  make clean-cache    Remove all Docker caches (nuclear option)"
	@echo ""
	@echo "Environment Variables:"
	@echo "  CARGO_REGISTRY_TOKEN     crates.io API token"
	@echo "  PYPI_API_TOKEN           PyPI API token"
	@echo "  NPM_TOKEN                npm automation token"
	@echo "  MATHHOOK_BUILDER_IMAGE   Override builder image (default: ghcr.io/ahmedmashour/mathhook-core/builder:latest)"

# Pull pre-built Docker image (recommended, fast)
docker-pull:
	@echo "=== Pulling builder image ==="
	docker pull $(MATHHOOK_BUILDER_IMAGE)

# Build Docker image locally (slow, ~30 min first time)
docker-build:
	@echo "=== Building Docker image locally ==="
	docker compose build builder

# Push Docker image to registry (maintainers only)
docker-push: docker-build
	@echo "=== Pushing builder image ==="
	docker tag mathhook-builder:latest $(MATHHOOK_BUILDER_IMAGE)
	docker push $(MATHHOOK_BUILDER_IMAGE)

# Ensure builder image exists (pull → fallback to local build)
docker-ensure:
	@docker image inspect $(MATHHOOK_BUILDER_IMAGE) > /dev/null 2>&1 || \
		(echo "Builder image not found, trying to pull..." && \
		 docker pull $(MATHHOOK_BUILDER_IMAGE) 2>/dev/null) || \
		(echo "Pull failed, building locally (this takes ~30 min first time)..." && \
		 docker compose build builder)

# Build all platforms
build-all: docker-ensure build-rust build-python build-node
	@echo "=== All builds complete ==="
	@echo "Artifacts:"
	@echo "  Python wheels: target/wheels/"
	@echo "  Node addons:   crates/mathhook-node/*.node"

# Build Rust crates
build-rust: docker-ensure
	@echo "=== Building Rust crates ==="
	docker compose run --rm build-rust

# Build Python wheels for all platforms
build-python: docker-ensure
	@echo "=== Building Python wheels ==="
	docker compose run --rm build-python

# Build Node.js addons for all platforms
build-node: docker-ensure
	@echo "=== Building Node.js addons ==="
	docker compose run --rm build-node

# Run tests
test: docker-ensure
	@echo "=== Running tests ==="
	docker compose run --rm test

# Publish to all registries
publish-all: build-all
	@echo "=== Publishing to all registries ==="
	@if [ -z "$(CARGO_REGISTRY_TOKEN)" ]; then \
		echo "ERROR: CARGO_REGISTRY_TOKEN not set"; \
		echo "Usage: CARGO_REGISTRY_TOKEN=xxx PYPI_API_TOKEN=xxx NPM_TOKEN=xxx make publish-all"; \
		exit 1; \
	fi
	@if [ -z "$(PYPI_API_TOKEN)" ]; then \
		echo "ERROR: PYPI_API_TOKEN not set"; \
		exit 1; \
	fi
	@if [ -z "$(NPM_TOKEN)" ]; then \
		echo "ERROR: NPM_TOKEN not set"; \
		exit 1; \
	fi
	docker compose --profile publish run --rm \
		-e CARGO_REGISTRY_TOKEN="$(CARGO_REGISTRY_TOKEN)" \
		-e PYPI_API_TOKEN="$(PYPI_API_TOKEN)" \
		-e NPM_TOKEN="$(NPM_TOKEN)" \
		publish

# Interactive shell for debugging
shell: docker-ensure
	@echo "=== Opening builder shell ==="
	@echo "You're now in the build container. Exit with 'exit' or Ctrl+D."
	docker compose run --rm builder bash

# Clean build artifacts
clean:
	@echo "=== Cleaning build artifacts ==="
	rm -rf target/wheels/
	rm -rf crates/mathhook-node/*.node
	rm -rf crates/mathhook-node/npm/
	docker compose down

# Nuclear option: remove all Docker caches
clean-cache: clean
	@echo "=== Removing Docker caches ==="
	docker compose down -v
	docker volume rm mathhook-cargo-registry mathhook-cargo-git mathhook-target-cache 2>/dev/null || true
	@echo "Cache cleared. Next build will be slower."

# Quick Python build (Linux only, for testing)
build-python-quick: docker-ensure
	@echo "=== Quick Python build (Linux x86_64 only) ==="
	docker compose run --rm builder bash -c '\
		cd /build/crates/mathhook-python && \
		maturin build --release --target x86_64-unknown-linux-gnu \
		  --compatibility manylinux2014 \
	'

# Quick Node build (Linux only, for testing)
build-node-quick: docker-ensure
	@echo "=== Quick Node build (Linux x86_64 only) ==="
	docker compose run --rm builder bash -c '\
		cd /build/crates/mathhook-node && \
		npm install && \
		npx napi build --platform --release --target x86_64-unknown-linux-gnu \
	'

# ============================================================================
# Release (one command)
# ============================================================================

# make release-patch          → 0.1.0 -> 0.1.1, auto-detect Docker/CI
# make release-patch DOCKER=1 → Force Docker mode
# make release-patch CI=1     → Force CI mode
# make release V=0.2.0        → Explicit version

release-patch release-minor release-major:
	@./scripts/release.sh $(subst release-,,$@) $(if $(DOCKER),--docker,) $(if $(CI),--ci,)

release:
	@if [ -z "$(V)" ]; then echo "Usage: make release V=0.2.0"; exit 1; fi
	@./scripts/release.sh $(V) $(if $(DOCKER),--docker,) $(if $(CI),--ci,)

release-dry:
	@./scripts/release.sh patch --dry-run

# ============================================================================
# Setup
# ============================================================================

setup: docker-ensure
	@echo "=== MathHook Docker Build System Ready ==="
	@echo ""
	@echo "Next steps:"
	@echo "  1. Copy tokens: cp .env.example .env && edit .env"
	@echo "  2. Release:     make release-patch"
	@echo ""
	@docker compose run --rm builder bash -c 'rustc --version && maturin --version'
