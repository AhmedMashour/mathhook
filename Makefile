# MathHook Build System
# Docker-based cross-platform build, test, and benchmark pipeline

.PHONY: help setup build-all build-rust build-python build-node \
        generate-bindings generate-bindings-python generate-bindings-node \
        test bench bench-rust bench-quick bench-save bench-compare bench-dashboard bench-clean \
        publish-all release release-patch release-minor release-major \
        shell clean clean-cache docker-pull docker-build docker-push

# Docker image name (used by docker-compose via MATHHOOK_BUILDER_IMAGE)
BUILDER_IMAGE ?= ghcr.io/ahmedmashour/mathhook-builder:latest
export MATHHOOK_BUILDER_IMAGE = $(BUILDER_IMAGE)

# ─────────────────────────────────────────────────────────────────────────────
# Help
# ─────────────────────────────────────────────────────────────────────────────

help:
	@echo "MathHook Build System"
	@echo ""
	@echo "Bindings:     make generate-bindings (REQUIRED before build-python/node)"
	@echo "Build:        make build-all | build-rust | build-python | build-node"
	@echo "Test:         make test"
	@echo "Bench:        make bench | bench-rust | bench-quick"
	@echo "Baseline:     make bench-save N=name | bench-compare N=name"
	@echo "Release:      make release-patch | release-minor | release-major"
	@echo "Dev:          make setup | shell | clean"

# ─────────────────────────────────────────────────────────────────────────────
# Docker
# ─────────────────────────────────────────────────────────────────────────────

docker-ensure:
	@docker image inspect $(BUILDER_IMAGE) >/dev/null 2>&1 || \
		docker pull $(BUILDER_IMAGE) 2>/dev/null || \
		docker compose build builder

docker-pull:
	docker pull $(BUILDER_IMAGE)

docker-build:
	docker compose build builder

docker-push: docker-build
	docker push $(BUILDER_IMAGE)

# ─────────────────────────────────────────────────────────────────────────────
# Binding Generation (MUST run before building Python/Node bindings)
# ─────────────────────────────────────────────────────────────────────────────

generate-bindings:
	cargo run -p mathhook-binding-codegen -- generate --target python
	cargo run -p mathhook-binding-codegen -- generate --target node

generate-bindings-python:
	cargo run -p mathhook-binding-codegen -- generate --target python

generate-bindings-node:
	cargo run -p mathhook-binding-codegen -- generate --target node

# ─────────────────────────────────────────────────────────────────────────────
# Build
# ─────────────────────────────────────────────────────────────────────────────

build-all: docker-ensure build-rust build-python build-node

build-rust: docker-ensure
	docker compose run --rm build-rust

build-python: docker-ensure generate-bindings-python
	docker compose run --rm build-python $(if $(RESUME),--resume,)

build-node: docker-ensure generate-bindings-node
	docker compose run --rm build-node $(if $(RESUME),--resume,)

# ─────────────────────────────────────────────────────────────────────────────
# Test
# ─────────────────────────────────────────────────────────────────────────────

test: docker-ensure
	docker compose run --rm test

# ─────────────────────────────────────────────────────────────────────────────
# Benchmark
# ─────────────────────────────────────────────────────────────────────────────

bench: docker-ensure
	docker compose run --rm bench
	@echo "Dashboard: artifacts/dashboard/index.html"

bench-rust: docker-ensure
	docker compose run --rm builder cargo bench -p mathhook-benchmarks --benches

bench-quick: docker-ensure
	docker compose run --rm builder cargo bench -p mathhook-benchmarks --benches -- --sample-size 10

bench-save: docker-ensure
	@test -n "$(N)" || (echo "Usage: make bench-save N=name" && exit 1)
	docker compose run --rm builder cargo bench -p mathhook-benchmarks --benches -- --save-baseline $(N)

bench-compare: docker-ensure
	docker compose run --rm builder cargo bench -p mathhook-benchmarks --benches -- --baseline $(or $(N),main)

bench-dashboard: docker-ensure
	docker compose run --rm builder python3 scripts/ci/generate_dashboard.py /build/artifacts /build/artifacts/dashboard /build/target/criterion

bench-clean:
	rm -rf artifacts/ target/criterion/

# ─────────────────────────────────────────────────────────────────────────────
# Publish & Release
# ─────────────────────────────────────────────────────────────────────────────

publish-all: build-all
	@test -n "$(CARGO_REGISTRY_TOKEN)" || (echo "Set CARGO_REGISTRY_TOKEN" && exit 1)
	@test -n "$(PYPI_API_TOKEN)" || (echo "Set PYPI_API_TOKEN" && exit 1)
	@test -n "$(NPM_TOKEN)" || (echo "Set NPM_TOKEN" && exit 1)
	docker compose --profile publish run --rm \
		-e CARGO_REGISTRY_TOKEN -e PYPI_API_TOKEN -e NPM_TOKEN publish

release-patch release-minor release-major:
	./scripts/release.sh $(subst release-,,$@) $(if $(DOCKER),--docker) $(if $(CI),--ci)

release:
	@test -n "$(V)" || (echo "Usage: make release V=x.y.z" && exit 1)
	./scripts/release.sh $(V) $(if $(DOCKER),--docker) $(if $(CI),--ci)

# ─────────────────────────────────────────────────────────────────────────────
# Development
# ─────────────────────────────────────────────────────────────────────────────

setup: docker-ensure
	@echo "Ready. Run: make help"

shell: docker-ensure
	docker compose run --rm builder bash

clean:
	rm -rf target/wheels/ crates/mathhook-node/*.node crates/mathhook-node/npm/
	docker compose down

clean-cache: clean
	docker compose down -v
	docker volume rm mathhook-cargo-registry mathhook-cargo-git mathhook-target-cache 2>/dev/null || true
