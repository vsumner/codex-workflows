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
