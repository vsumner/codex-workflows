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

## Just Commands

```bash
just preflight
just gate-shareable
just sync-home
just install-home
```

- `just preflight` validates the tracked shareable surface, checks key native model/feature/command/role/tool drift against a local upstream Codex clone when available, and checks that the live `~/.codex` install matches it.
- `just gate-shareable` runs `preflight`, installs into a temp Codex home, and re-runs the
  checker against that temp tree so install drift shows up before handoff.
- If `just` is unavailable, run `./scripts/check-shareable.sh` directly. You can override the upstream clone path with `--upstream-codex /path/to/codex`.

## Config Notes

- `config.toml.example` keeps model, feature, MCP, and agent settings.
- `[projects."..."]` trust entries are intentionally removed because they are machine-specific.
- `config_file` paths are rewritten to relative `roles/*.toml`.
- MCP env var names are preserved, but no key values are stored.
