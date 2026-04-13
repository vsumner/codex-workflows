# Victor Workflows Memory Extension

## Purpose

Interpret Codex rollout memories for Victor's workflow tooling and promote only stable, reusable guidance into memory artifacts.

This extension is for Codex workflow improvement, not for generic repo coding rules. Current-session behavior belongs in `AGENTS.md` or a skill. Memory retention guidance belongs here.

## Scope

Treat a rollout as belonging to this memory extension when any of these are true:

- `cwd` is `/Users/vsumner/src/github.com/vsumner/codex-workflows`.
- The rollout discusses Codex plugins, skills, memory extensions, local workflow CLIs, or mining `~/.codex` history.
- The task uses or changes `codex-threads`, `victor-workflows`, or future `codex-workflow` tooling.
- The task is about turning repeated Victor workflow friction into a reusable CLI, skill, plugin, or memory rule.
- The rollout discusses Codex harness engineering, agent legibility, repo-local workflow docs, local docs/DocSet caches, or Codex/Claude session-history reflection.

Keep memories project-local unless the lesson is clearly about Victor's global Codex preferences and is repeated across multiple unrelated repos.

## Promote Aggressively

Promote patterns that are stable and repeatedly useful:

- Agent-friendly CLI conventions that make Codex more effective: `--json`, bounded output, predictable errors, narrow commands, and `--help`.
- Repeated workflow friction worth turning into tooling, especially orientation, review-fix-verify, skill routing, environment debugging, and session-history mining.
- Clear ownership boundaries between CLI, plugin, skill, `AGENTS.md`, and memory extension responsibilities.
- Installation and activation steps that prevent future confusion, such as plugin cache path, config key, or session restart requirements.
- Validated search/debug paths that repeatedly lead to the right source of truth.
- Victor's preference to keep the ideal end state explicit while still choosing the smallest next artifact that advances toward it.
- The agent-legibility architecture Victor is converging on: Codex stays the harness; repo-local docs are the system of record; small deterministic tools expose evidence; fat skills encode judgment; mechanical checks enforce durable taste.
- Resolver patterns that say what context to load for a task type, such as architecture review, review-fix-verify, Swift/macOS work, app verification, project-transfer analysis, or workflow reflection.
- Feedback-loop patterns that make Codex more autonomous without bloating prompt context: per-worktree boot commands, bounded logs/metrics/traces, local documentation indexes, quality-score docs, stale-doc checks, and agent-readable lint/test failures.
- The promotion ladder for workflow improvements: run manually on 3-10 representative cases, show output, codify as a skill when useful, add a CLI only for noisy deterministic state, then consider draft/scheduled automation only after idempotency and approval gates are clear.

## Preserve As Durable Guidance

Preserve as repo or user preference guidance when the pattern is proven:

- Use a CLI for repeated large/noisy sources before adding more prompt-only workflow text.
- Use skills for routing and safety defaults around a CLI.
- Use plugins as packaging and discovery surfaces, not as the implementation by themselves.
- Use `AGENTS.md` for immediate behavior rules and memory extensions for consolidation guidance.
- Do not publish actions unless Victor explicitly asks.
- Treat `AGENTS.md` as a table of contents. Preserve the preference to move durable detail into repo-local docs, skills, or mechanical checks instead of a monolithic always-loaded instruction file.
- Prefer a small set of high-quality workflow skills over a large skill catalog: repo orientation, execution packet, review-fix-verify, workflow reflection, project transfer, docs resolution, and fresh-eyes architecture.
- Capture human taste as docs, tests, lints, quality scores, and remediation-rich error messages when possible; use prose guidance only when the judgment cannot be made mechanical yet.
- Keep local docs and DocSet-style caches as preferred sources for up-to-date API/framework context when available, especially for Apple/macOS work.
- Preserve the distinction between read-only/draft automations and write/publish automations. Morning pulse and quality/doc reports can be early read-only candidates; green-PR and Sentry-style fixers need stronger gates.

## Do Not Promote

Do not retain:

- One-off implementation details that are not stable.
- Temporary file paths, staging details, shell output noise, or transient branch state.
- Assistant proposals that were not implemented or verified.
- Session-specific counts unless they identify a repeated, useful pattern.
- Raw transcript excerpts beyond short evidence snippets needed to support a memory.
- Repo-specific rules from unrelated projects unless they recur as Victor-wide workflow preferences.
- Claims that a new workflow runner, large prompt framework, or custom orchestration layer is needed unless repeated evidence shows Codex-native skills/tools cannot cover the need.
- Large copied article summaries. Retain only the operational lesson and the source class, such as "OpenAI harness-engineering feedback loop" or "thin harness/fat skills promotion rule."

## Output Shaping

When writing or updating memory from this extension:

- Prefer concise, actionable bullets over narrative summaries.
- Include the relevant surface: CLI, plugin, skill, memory extension, `AGENTS.md`, or repo docs.
- Name the evidence class, not the whole transcript. Example: "Repeated `what's next?` orientation requests across recent Codex sessions."
- Mark uncertain or single-session observations as candidates, not durable memory.
- Avoid retaining private raw session content unless Victor explicitly asked to preserve that content.

## Useful Tags

- `project:codex-workflows`
- `workflow:codex`
- `surface:cli`
- `surface:plugin`
- `surface:skill`
- `surface:memory-extension`
- `pattern:orientation`
- `pattern:review-fix-verify`
- `pattern:skill-routing`
- `pattern:environment-debugging`
