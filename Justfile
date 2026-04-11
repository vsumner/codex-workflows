set shell := ["bash", "-euo", "pipefail", "-c"]

default:
	@just --list

build:
	cargo build

test:
	cargo test

doctor:
	cargo run -p codex-threads -- --json doctor

sync:
	cargo run -p codex-threads -- --json sync
