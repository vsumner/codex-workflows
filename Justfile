set shell := ["bash", "-euo", "pipefail", "-c"]

default:
	@just --list

preflight:
	./scripts/check-shareable.sh

gate-shareable:
	./scripts/check-shareable.sh
	tmp_dir="$(mktemp -d)"; \
	trap 'rm -rf "$tmp_dir"' EXIT; \
	./scripts/install-shareable-to-home.sh "$tmp_dir" >/dev/null; \
	./scripts/check-shareable.sh --codex-home "$tmp_dir"

build-plugin:
	./scripts/build-plugin.sh

sync-home:
	./scripts/sync-shareable-from-home.sh

install-home:
	./scripts/install-shareable-to-home.sh

install-home-to dest:
	./scripts/install-shareable-to-home.sh {{dest}}

check-home home:
	./scripts/check-shareable.sh --codex-home {{home}}
