# Codex Workflows

Codex Workflows is a shareable mirror of the safe parts of `~/.codex`.

Tracked here:
- `AGENTS.md`
- `config.toml.example`
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

## Config Notes

- `config.toml.example` keeps model, feature, MCP, and agent settings.
- `[projects."..."]` trust entries are intentionally removed because they are machine-specific.
- `config_file` paths are rewritten to relative `roles/*.toml`.
- MCP env var names are preserved, but no key values are stored.
