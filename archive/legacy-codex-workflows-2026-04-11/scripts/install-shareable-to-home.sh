#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
dest_dir="${1:-$HOME/.codex}"

mkdir -p \
  "$dest_dir/playbooks" \
  "$dest_dir/prompts" \
  "$dest_dir/roles" \
  "$dest_dir/rules" \
  "$dest_dir/skills"

rsync -a "$repo_root/playbooks/" "$dest_dir/playbooks/"
rsync -a "$repo_root/prompts/" "$dest_dir/prompts/"
rsync -a "$repo_root/roles/" "$dest_dir/roles/"
rsync -a "$repo_root/rules/" "$dest_dir/rules/"
rsync -a "$repo_root/skills/" "$dest_dir/skills/"

cp "$repo_root/AGENTS.md" "$dest_dir/AGENTS.md"
cp "$repo_root/config.toml.example" "$dest_dir/config.toml.from-repo.example"

printf '%s\n' "Wrote $dest_dir/config.toml.from-repo.example"
printf '%s\n' "Merge it into $dest_dir/config.toml manually if you want those settings."
