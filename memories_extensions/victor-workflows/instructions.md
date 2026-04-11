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

Keep memories project-local unless the lesson is clearly about Victor's global Codex preferences and is repeated across multiple unrelated repos.

## Promote Aggressively

Promote patterns that are stable and repeatedly useful:

- Agent-friendly CLI conventions that make Codex more effective: `--json`, bounded output, predictable errors, narrow commands, and `--help`.
- Repeated workflow friction worth turning into tooling, especially orientation, review-fix-verify, skill routing, environment debugging, and session-history mining.
- Clear ownership boundaries between CLI, plugin, skill, `AGENTS.md`, and memory extension responsibilities.
- Installation and activation steps that prevent future confusion, such as plugin cache path, config key, or session restart requirements.
- Validated search/debug paths that repeatedly lead to the right source of truth.

## Preserve As Durable Guidance

Preserve as repo or user preference guidance when the pattern is proven:

- Use a CLI for repeated large/noisy sources before adding more prompt-only workflow text.
- Use skills for routing and safety defaults around a CLI.
- Use plugins as packaging and discovery surfaces, not as the implementation by themselves.
- Use `AGENTS.md` for immediate behavior rules and memory extensions for consolidation guidance.
- Do not publish actions unless Victor explicitly asks.

## Do Not Promote

Do not retain:

- One-off implementation details that are not stable.
- Temporary file paths, staging details, shell output noise, or transient branch state.
- Assistant proposals that were not implemented or verified.
- Session-specific counts unless they identify a repeated, useful pattern.
- Raw transcript excerpts beyond short evidence snippets needed to support a memory.
- Repo-specific rules from unrelated projects unless they recur as Victor-wide workflow preferences.

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
