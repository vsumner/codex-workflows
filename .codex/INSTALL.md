# Installing Codexify for Codex

Install Codexify as a reusable bootstrap kit for downstream repositories.

## Prerequisites
- Git
- A target repository where you want to run the RPI loop

## Installation
1. **Clone the codexify repository:**

```bash
git clone https://github.com/vsumner/codexify.git ~/.codex/codexify
```

2. **Copy the codexify scaffold into your target repository:**

```bash
cd /path/to/your-target-repo
mkdir -p .codex
cp ~/.codex/codexify/.codex/config.toml .codex/config.toml
mkdir -p .codex/agents
cp ~/.codex/codexify/.codex/agents/*.toml .codex/agents/
mkdir -p playbooks templates schemas examples/stable-manager
cp ~/.codex/codexify/playbooks/*.md playbooks/
cp ~/.codex/codexify/templates/*.md templates/
cp ~/.codex/codexify/schemas/*.md schemas/
cp ~/.codex/codexify/examples/stable-manager/*.md examples/stable-manager/
```

3. **Restart Codex** (quit and relaunch the CLI) so role config is loaded cleanly.

## Optional: Symlink Instead of Copy
If you want updates from `~/.codex/codexify` to appear instantly in your target repo, use symlinks.

```bash
cd /path/to/your-target-repo
mkdir -p .codex
ln -s ~/.codex/codexify/.codex/config.toml .codex/config.toml
ln -s ~/.codex/codexify/.codex/agents .codex/agents
ln -s ~/.codex/codexify/playbooks playbooks
ln -s ~/.codex/codexify/templates templates
ln -s ~/.codex/codexify/schemas schemas
ln -s ~/.codex/codexify/examples examples
```

## Migrating From Earlier Manual Scaffolds
If you already copied files manually and want to align to the latest codexify:

1. **Update codexify:**

```bash
cd ~/.codex/codexify && git pull
```

2. **Reinstall into your target repo:**
- If you use copy mode, re-run the copy commands above.
- If you use symlink mode, ensure links still point to `~/.codex/codexify`.

3. **Restart Codex.**

## Verify
From your target repository:

```bash
ls -la .codex/config.toml .codex/agents
ls -la playbooks templates schemas examples/stable-manager
```

You should see:
- `.codex/config.toml`
- five role files in `.codex/agents`
- required markdown artifacts in `playbooks`, `templates`, and `schemas`

## Updating

```bash
cd ~/.codex/codexify && git pull
```

Then either:
- re-run copy commands (copy mode), or
- restart Codex (symlink mode, since files are already linked)

## Uninstalling
From your target repo, remove installed codexify artifacts:

```bash
rm -rf .codex playbooks templates schemas examples/stable-manager
```

Optionally remove the clone:

```bash
rm -rf ~/.codex/codexify
```
