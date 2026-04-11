#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source_dir="${1:-$HOME/.codex}"

mkdir -p \
  "$repo_root/playbooks" \
  "$repo_root/prompts" \
  "$repo_root/roles" \
  "$repo_root/rules" \
  "$repo_root/skills"

rsync -a --delete "$source_dir/playbooks/" "$repo_root/playbooks/"
rsync -a --delete "$source_dir/prompts/" "$repo_root/prompts/"
rsync -a --delete "$source_dir/roles/" "$repo_root/roles/"
rsync -a --delete "$source_dir/rules/" "$repo_root/rules/"
rsync -a --delete --exclude '.system/' "$source_dir/skills/" "$repo_root/skills/"

cp "$source_dir/AGENTS.md" "$repo_root/AGENTS.md"
cp "$source_dir/rust_correctness_reviewer.toml" "$repo_root/roles/rust_correctness_reviewer.toml"

{
  printf '%s\n' '# Example shareable Codex config exported from `~/.codex`.'
  printf '%s\n' '# Secrets, per-project trust entries, and machine-specific paths are omitted.'
  printf '%s\n' '# Copy this to `~/.codex/config.toml` and then add your own `[projects."..."]` blocks locally.'
  printf '%s\n' '# Set required env vars such as `CONTEXT7_API_KEY` in your shell.'
  printf '\n'
  awk '
    function emit(line) {
      if (line == "") {
        if (last_blank) {
          return
        }
        last_blank = 1
      } else {
        last_blank = 0
      }
      print line
    }
    /^\[projects\."/ { skip_project = 1; next }
    skip_project {
      if ($0 ~ /^trust_level = /) {
        skip_project = 0
      }
      next
    }
    {
      line = $0
      gsub(/config_file = "[^"]*\/\.codex\/roles\//, "config_file = \"roles/", line)
      sub(/config_file = "[^"]*\/\.codex\/rust_correctness_reviewer\.toml"/, "config_file = \"roles/rust_correctness_reviewer.toml\"", line)
      emit(line)
    }
  ' "$source_dir/config.toml"
} > "$repo_root/config.toml.example"
