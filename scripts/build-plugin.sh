#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
plugin_root="$repo_root/plugin"
skills_root="$plugin_root/skills"

portable_skills=(
  "cli-dx"
  "find-skills"
  "verify-gates-codex"
  "workflow-authoring-codex"
)

blocked_role_pattern='workflow_orchestrator|planner|plan_reviewer|workflow_verifier|workflow_reviewer|spark_implementer|spark_implementer_xhigh|packet_verifier|researcher|implementer|plan_reviewer'

rm -rf "$skills_root"
mkdir -p "$plugin_root/.codex-plugin" "$skills_root"

for skill in "${portable_skills[@]}"; do
  src="$repo_root/skills/$skill"
  dest="$skills_root/$skill"

  if [[ ! -d "$src" ]]; then
    echo "missing skill source: $src" >&2
    exit 1
  fi

  if rg -n "$blocked_role_pattern" "$src" >/dev/null 2>&1; then
    echo "skill is not portable without Victor-specific role config: $skill" >&2
    exit 1
  fi

  mkdir -p "$dest"
  rsync -a --delete "$src/" "$dest/"
done

cat >"$plugin_root/.codex-plugin/plugin.json" <<'EOF'
{
  "name": "codex-workflows-lite",
  "description": "Portable workflow utilities from Victor's Codex workflow stack.",
  "skills": "./skills",
  "interface": {
    "displayName": "Codex Workflows Lite",
    "shortDescription": "Portable workflow utilities for Codex",
    "longDescription": "A narrow plugin export of portable workflow utilities from Victor's Codex workflow stack. This package intentionally excludes RPIV orchestration skills that depend on custom agent-role config.",
    "developerName": "Victor Sumner",
    "category": "developer-tools",
    "capabilities": [
      "workflow",
      "verification",
      "cli"
    ],
    "defaultPrompt": [
      "Audit this CLI flow and propose concrete DX improvements.",
      "Run verification gates for this change and report evidence.",
      "Refactor this workflow surface to be smaller and more coherent."
    ],
    "brandColor": "#1f6feb"
  }
}
EOF

cat >"$plugin_root/README.md" <<'EOF'
# Codex Workflows Lite

This directory is a generated Codex plugin bundle.

- Source of truth lives in the repository root `skills/` tree.
- Rebuild with `./scripts/build-plugin.sh`.
- Do not hand-edit files under `plugin/skills/`; they will be overwritten.

Included in this bundle:
- `cli-dx`
- `find-skills`
- `verify-gates-codex`
- `workflow-authoring-codex`

Excluded on purpose:
- RPIV orchestration skills that depend on Victor-specific custom agent roles
- prompts, roles, playbooks, AGENTS, and shareable `config.toml` state
EOF

echo "Built plugin bundle at $plugin_root"
