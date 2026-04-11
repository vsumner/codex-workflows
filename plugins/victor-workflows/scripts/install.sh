#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
cargo install --path "$repo_root/crates/codex-threads"

plugin_cache_dir="$HOME/.codex/plugins/cache/victor-local/victor-workflows/local"
mkdir -p "$plugin_cache_dir"
cp -R "$repo_root/plugins/victor-workflows/." "$plugin_cache_dir/"

memory_extension_dir="$HOME/.codex/memories_extensions/victor-workflows"
mkdir -p "$memory_extension_dir"
cp "$repo_root/memories_extensions/victor-workflows/instructions.md" "$memory_extension_dir/instructions.md"
