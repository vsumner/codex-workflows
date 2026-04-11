---
name: debug-environment
description: This skill should be used when Victor asks to debug local dev environment or toolchain failures, including "bun install", "storybook", "vite", "clippy", "cargo", "npm", "pnpm", "workspace dependency", "SIGKILL", "version mismatch", "dev server killed", or "rollback our changes".
---

# Debug Environment

Use this skill for local toolchain and development-environment failures where the risk is wasted churn: repeated blind reruns, broad rollbacks, changing dependency versions without evidence, or confusing the command's current directory with the repo root.

This is a skill, not a CLI. Use existing primitives first: shell commands, package-manager metadata, `git`, lockfiles, project scripts, and official tool diagnostics. Add a CLI only if repeated runs prove a noisy deterministic source needs search, stable IDs, pagination, or bounded JSON.

## Core Loop

Run this loop explicitly:

```text
capture -> map environment -> isolate failure class -> test one hypothesis -> fix smallest cause -> verify
```

Do not skip capture. Most environment failures are path, workspace, version, cache, or resource-state bugs disguised as application bugs.

## Capture

Before editing, record the operating facts:

- Exact command that failed.
- Exact current working directory.
- Package manager and version when relevant.
- Runtime versions: Node, Bun, pnpm, npm, Rust, Cargo, or tool-specific versions.
- Whether the process exits with a code, signal, timeout, or no output.
- Whether the failure reproduces from the repo root and from the directory where Victor ran it.
- Dirty git state and whether unrelated files are already modified.

Use narrow commands:

```bash
pwd
git status --short --branch
rg --files -g 'package.json' -g 'bun.lock' -g 'pnpm-lock.yaml' -g 'package-lock.json' -g 'Cargo.toml' -g 'rust-toolchain*'
```

Then inspect only the nearest manifests and scripts needed for the failing command.

## Classify

Classify the failure before changing files:

- **Wrong cwd**: workspace packages are not found, scripts resolve from an unexpected directory, or `package.json` differs between root and subdirectory.
- **Workspace shape mismatch**: `workspace:*` dependencies cannot resolve because the command is running outside the workspace root or the workspace glob excludes a package.
- **Version skew**: related packages are on mixed versions, especially Storybook/Vite/addon packages or Rust crates spanning a workspace.
- **Cache/install corruption**: package files cannot be read from `node_modules`, `.bun`, `.pnpm`, target cache, or generated virtual modules.
- **Config mismatch**: tool starts but fails resolving virtual imports or config paths.
- **Resource kill**: process exits by signal such as `SIGKILL`, often memory pressure or OS kill, not necessarily a code bug.
- **Review warning**: tool reports a warning or migration recommendation that may be valid but not related to the failing command.

State the class and why it fits the evidence. If multiple classes fit, pick the cheapest hypothesis to falsify first.

## Guardrails

Avoid these failure modes:

- Do not rollback all changes until the diff is inspected and the suspected files are named.
- Do not treat warning text as root cause just because it appears near the failure.
- Do not run install commands from a subdirectory unless the package manager and workspace layout support it.
- Do not delete lockfiles, caches, or `node_modules` as a first move. Explain the evidence first.
- Do not upgrade broad dependency families unless version skew is proven and the minimal aligned set is identified.
- Do not keep rerunning a killed command without checking signal, memory/resource pressure, and whether a smaller command isolates the same failure.
- Do not commit generated lockfile or install churn unless it is clearly expected for the fix.

If Victor asks "rollback our changes", first inspect `git diff --stat` and the touched files. Recommend rollback only for the files plausibly tied to the failure; keep unrelated user changes intact.

## Evidence Commands

Choose commands based on the failure class. Keep output bounded.

For JavaScript workspaces:

```bash
pwd
node --version
bun --version
pnpm --version
npm --version
find .. -maxdepth 2 -name package.json -print
sed -n '1,220p' package.json
```

For package/version skew:

```bash
rg '"@storybook/|"storybook"|"vite"|"react"|"typescript"' package.json **/package.json
bun pm ls storybook
pnpm why storybook
npm ls storybook
```

Run only the package-manager command that matches the repo.

For Rust workspaces:

```bash
cargo metadata --no-deps --format-version 1
cargo check -p <package>
cargo clippy -p <package> -- -D warnings
```

For resource kills:

```bash
dmesg | tail
ulimit -a
```

On macOS, `dmesg` may be unavailable or low-signal. Prefer rerunning the smallest failing build with less concurrency or tool-specific debug flags before changing code.

## Fix Strategy

Prefer the smallest reversible fix:

- Correct cwd or script path before editing manifests.
- Align a narrow dependency family only when version skew is proven.
- Restore a removed config import only when the error points to that config path.
- Remove or change a warning-causing option only if it is actionable and tied to the current toolchain behavior.
- Update lockfiles with the repo's package manager, not by hand.
- For generated artifacts, verify whether they are tracked and expected.

After any edit, run the smallest command that should prove the fix. Then run the broader command Victor actually cares about.

## Output Shape

Use this shape while working:

```text
Failure class: <wrong cwd | workspace mismatch | version skew | cache/install | config mismatch | resource kill | review warning>.
Evidence: <one or two concrete facts>.
Next check: <single command or inspection>.
```

Use this shape at handoff:

```text
Root cause: <short causal explanation>.
Changed: <minimal file or config set>.
Verified: <commands and outcomes>.
Risk: <remaining environment uncertainty, if any>.
```

If the root cause is not proven, say so. Give the next falsifiable check instead of presenting a guess as a fix.
