---
name: cli-dx
description: This skill should be used when the user asks to "improve CLI UX", "design command output", "refactor terminal prompts", "audit command-line developer experience", or "improve automation-friendly CLI behavior". Covers developer experience patterns for command-line and terminal-first workflows.
---

# CLI Developer Experience

## Overview

Design CLI experiences that are fast, predictable, accessible, and automation-friendly.
Use this skill to audit existing CLIs and produce specific improvements with implementation-ready guidance.

## Workflow

1. Classify the CLI interaction model.
2. Run the DX rubric.
3. Prioritize findings and propose concrete changes.
4. Validate behavior with scenario-based checks.

### 1. Classify the Interaction Model

Pick one primary mode:
- `one-shot`: command returns output and exits
- `interactive`: wizard, prompt loop, menu, or TUI flow
- `agentic`: long-running conversational terminal session with mixed command/result blocks

Then document the operating context:
- local terminal vs. remote/web terminal
- novice vs. expert user goals
- human-only vs. automation/bot usage
- accessibility requirements (screen reader, contrast, color customization)

### 2. Run the DX Rubric

Score each area as `good`, `needs work`, or `critical`:

1. Discoverability
- Ensure `--help`, examples, and clear command names exist.
- Ensure failures suggest next actions.

2. Prompt and Input Quality
- Ensure prompts describe intent, constraints, and defaults.
- Ensure prompts avoid ambiguity and hidden side effects.
- Ensure non-interactive alternatives exist for scripted use.

3. Output Clarity
- Ensure output is scannable with stable labels and consistent formatting.
- Ensure text copy/paste preserves line integrity.
- Ensure machine-readable output is available (`--json`, stable keys).

4. Responsiveness and Rendering
- Minimize redraw flicker and noisy frame updates.
- Avoid spinner-only indicators in accessibility-sensitive flows.
- Prefer explicit status messages (`Working...`, action-specific progress text).

5. Accessibility
- Ensure screen readers can interpret prompts and progress.
- Ensure color contrast works with user-chosen backgrounds.
- Prefer ANSI role-based palettes that support 4-bit customization.

6. Safety and Trust
- Prefer non-destructive defaults.
- Confirm destructive actions.
- Show what will change before execution where possible.
- Preserve an easy rollback path.

7. Automation and Repeatability
- Ensure deterministic exit codes.
- Ensure behavior is scriptable without interactive UI assumptions.
- Ensure reproducible workflows for setup/build/deploy/test tasks.

8. Configuration and Scope Controls
- Ensure project-root detection is correct.
- Avoid monorepo-root ambiguity when command scope should be package-local.
- Support include/exclude or rule files for code-modifying tools.

### 3. Prioritize and Propose Changes

Prioritize findings as:
- `P0`: blocks usage or causes harmful/confusing behavior
- `P1`: high-friction issue with frequent impact
- `P2`: quality issue with moderate impact
- `P3`: polish improvement

For each finding, include:
- observed behavior
- impacted user/persona/context
- recommended change
- acceptance criteria
- implementation hint (API surface, output contract, prompt text, rendering strategy)

Use this compact output template:

```markdown
### [P1] Prompt text is ambiguous in `tool deploy`
Observed: Prompt asks "Proceed?" without describing target environment.
Impact: Users can deploy to the wrong environment.
Fix: Change prompt to "Deploy to `prod-us-east-1` now?" and include `--yes` bypass.
Acceptance: Prompt always includes target env and skip option is documented in `--help`.
```

### 4. Validate with Scenarios

Run checks against these scenarios:
- first-time user running command from fresh repo
- experienced user running repeated workflow quickly
- automation script parsing output and exit code
- screen reader user navigating prompts
- low-contrast and custom-color terminal themes
- remote/web terminal with limited capabilities

## Design Rules

Apply these defaults when proposing CLI DX changes:
- Prefer explicitness over cleverness in prompts and error text.
- Prefer iterative, narrowly scoped operations over giant one-shot mutations.
- Prefer stable output contracts over visually dense output.
- Prefer text accessibility over animation-heavy indicators.
- Prefer user-customizable color roles over hard-coded palette assumptions.

## Source Notes

Read `references/cli-dx-principles.md` when you need:
- terminal UX and agentic CLI interaction pitfalls
- accessibility-specific CLI guidance
- code-generation CLI workflow practices (scope, commit hygiene, rule files)
