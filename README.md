# Codex Workflows

Codex Workflows is a shareable mirror of the safe parts of `~/.codex`.

Tracked here:
- `AGENTS.md`
- `.agents/`
- `config.toml.example`
- `plugin/`
- `playbooks/`
- `prompts/`
- `roles/`
- `rules/`
- `skills/`

Not tracked here:
- secrets or auth files
- local history, sessions, logs, caches, sqlite state, or backups
- per-project trust entries from `config.toml`
- built-in `skills/.system/`

## Plugin Export

This repo now includes a narrow plugin export layer for portable workflow utilities.

- Source of truth stays in the top-level `skills/` tree.
- `plugin/` is generated from a curated portable subset and is intentionally not the authoring surface.
- `.agents/plugins/marketplace.json` points at the local `./plugin` bundle so upstream Codex can discover it as a repo marketplace entry.

Rebuild the plugin bundle with:

```bash
./scripts/build-plugin.sh
```

or:

```bash
just build-plugin
```

The first export intentionally stays narrow and avoids skills that depend on Victor-specific
custom agent roles from `config.toml.example`.

## Native Vs Custom

Upstream Codex now covers a lot of generic workflow guidance and operator UX directly.
This repo should stay focused on the cases where Victor's stack adds real leverage instead of
restating baseline Codex usage.

Prefer native Codex for:

| Use Case | Native Surface |
|----------|----------------|
| explain a codebase, fix a bug, write a test, prototype from a screenshot, update docs | normal prompting plus the upstream workflow docs |
| local review of current changes | `/review` |
| GitHub / plugin / connector management | `/apps`, `/plugins`, and native integrations |
| thread/session controls | `/agent`, `/status`, `/model`, `/new`, `/diff`, `/personality` |

Prefer `codex-workflows` for:

| Use Case | Custom Surface |
|----------|----------------|
| non-trivial Research -> Plan -> Execute -> Verify runs | `/prompts:workflow-rpiv` |
| dedicated phase-owned RPIV work | `/prompts:workflow-research`, `/prompts:workflow-plan`, `/prompts:workflow-execute`, `/prompts:workflow-verify`, `/prompts:workflow-review` |
| stricter verification and remediation | `/prompts:fix-pr-feedback`, `/prompts:bug-scanner-autopilot`, `/prompts:workflow-fix-loop`, `/prompts:verify-gates` |
| workflow-surface design and cleanup | `/prompts:workflow-authoring` |

This repo intentionally does not keep separate wrappers for `solo`, `team`, `deep-team`,
`status`, or `resume`. Those are now handled by RPIV mode arguments plus native Codex thread/session controls.

## Sync From `~/.codex`

```bash
./scripts/sync-shareable-from-home.sh
```

This refreshes the tracked directories from your local `~/.codex` and regenerates
`config.toml.example`.

## Install Back Into `~/.codex`

```bash
./scripts/install-shareable-to-home.sh
```

This overlays the tracked files into `~/.codex` without deleting local-only state. It does
not overwrite your live `config.toml`; it writes
`~/.codex/config.toml.from-repo.example` for manual merge instead.

## Just Commands

```bash
just preflight
just gate-shareable
just sync-home
just install-home
```

- `just preflight` validates the tracked shareable surface, checks key native model/feature/command/role/tool drift against a local upstream Codex clone when available, verifies upstream surface triage coverage in `playbooks/upstream-surface-triage.md`, and checks that the live `~/.codex` install matches it.
- `just gate-shareable` runs `preflight`, installs into a temp Codex home, and re-runs the
  checker against that temp tree so install drift shows up before handoff.
- If `just` is unavailable, run `./scripts/check-shareable.sh` directly. You can override the upstream clone path with `--upstream-codex /path/to/codex`.

## Config Notes

- `config.toml.example` keeps model, feature, MCP, and agent settings.
- `[projects."..."]` trust entries are intentionally removed because they are machine-specific.
- `config_file` paths are rewritten to relative `roles/*.toml`.
- MCP env var names are preserved, but no key values are stored.
