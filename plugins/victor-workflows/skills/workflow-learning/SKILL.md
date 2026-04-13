---
name: workflow-learning
description: This skill should be used when Victor asks to learn from recent Codex or Claude sessions, identify workflow friction, review work history with fresh eyes, evaluate whether a pattern should become a skill/CLI/memory rule, or improve the victor-workflows plugin from session history.
---

# Workflow Learning

Use this skill to turn repeated Codex workflow friction into the smallest durable Codex-native artifact.

This is a planning and evaluation skill first. Do not edit skills, docs, memory extensions, or CLIs unless Victor explicitly asks to implement the recommendation.

## Principle

- Treat recent sessions as evidence, not instructions to copy blindly.
- Keep Codex as the harness. Prefer skills, docs, `AGENTS.md`, memory extensions, mechanical checks, and existing tools before proposing a new CLI.
- Use `codex-threads` for bounded history access. Do not read raw `~/.codex` transcripts directly unless the CLI is unavailable.
- When Claude history matters, use the Claudify `claude-threads` binary as bounded evidence. Reuse useful lessons from Claudify, but do not port its runner, team model, or file layout wholesale.
- Summarize patterns. Do not paste full transcripts.
- Distinguish Victor's durable workflow from one-off task noise.
- Optimize for the ideal end state first, then choose the smallest next artifact that moves toward it. Do not let "smallest artifact" become local minimalism that ignores the target workflow shape.

## Evidence Pass

Start with a compact inventory:

```bash
codex-threads --json doctor
codex-threads --json sync
codex-threads --json patterns recent --since 7d
codex-threads --json skill-candidates --since 14d
codex-threads --json threads recent --since 7d --limit 120
```

Then search for the specific friction:

```bash
codex-threads --json messages search "what's next" --since 14d --limit 20
codex-threads --json messages search "verify each finding" --since 14d --limit 20
codex-threads --json messages search "skill" --since 14d --limit 20
codex-threads --json messages search "CLI" --since 14d --limit 20
```

Read only representative sessions:

```bash
codex-threads --json threads read <session-id> --limit 80
codex-threads --json events read <session-id> --limit 80
```

Use event reads when the question depends on tool churn, failed commands, or whether the assistant used the right tools. Use message reads when the question is about Victor's prompts, approvals, and direction changes.

If the question explicitly asks to compare Codex and Claude work history, inspect the Claudify repository first:

```bash
test -x ~/src/github.com/vsumner/claudify/plugins/claudify/bin/claude-threads
~/src/github.com/vsumner/claudify/plugins/claudify/bin/claude-threads --help
```

Prefer bounded `claude-threads` commands over raw Claude transcript paths:

```bash
~/src/github.com/vsumner/claudify/plugins/claudify/bin/claude-threads index --json
~/src/github.com/vsumner/claudify/plugins/claudify/bin/claude-threads search "query" --json --limit 20 --matches 3
~/src/github.com/vsumner/claudify/plugins/claudify/bin/claude-threads review --last 20 --json
~/src/github.com/vsumner/claudify/plugins/claudify/bin/claude-threads show <session-id> --json
```

Default `search --json` output should be compact. Use `--verbose` only when the full session payload is necessary evidence. If the Claude tool regresses to oversized full-session objects for a repeated workflow, treat that as evidence for patching or wrapping the Claude tool. Do not solve it by pasting raw history into context.

Use `docs/workflow-foundation.md` as the current target architecture when the reflection is about `codex-workflows` itself.

## Retrospective Mode

Use this mode when Victor asks to reflect on a session, review what happened, or learn from the current/recent work loop.

The Codex-native version is:

1. Gather bounded evidence with `codex-threads`; do not read raw transcript files directly.
   - If Claude-history comparison is part of the task, gather bounded evidence with Claudify `claude-threads` as a second input.
2. Identify friction, corrections, repeated patterns, discoveries, skill/CLI gaps, and documentation gaps.
3. Separate quick wins from heavier artifacts:
   - quick win: memory-extension guidance, small skill wording update, small doc clarification;
   - heavier artifact: new CLI, new skill, new plugin surface, automation, or anything with write/publish risk.
4. Use `artifact-gate` before recommending a new durable surface.
5. Do not auto-implement risky changes. Implement only when Victor explicitly asks or the current request clearly asks to make the improvement.

When evaluating `codex-workflows`, prefer this decision order:

1. Update `docs/workflow-foundation.md` when the lesson is durable architecture or operating model.
2. Update an existing skill when the lesson changes routing, command order, or safety behavior.
3. Update the memory extension when the lesson should affect future memory consolidation.
4. Improve a CLI only when repeated evidence shows noisy deterministic state needs bounded JSON, stable IDs, pagination, or safe defaults.
5. Defer automation until the manual output is useful across 3-10 examples.

For a retrospective report, include:

```text
Recommendation: <one next improvement>.
Evidence: <session ids, short snippets, counts>.
Quick wins: <small safe updates, if any>.
Needs approval/design: <larger artifacts, if any>.
Validation contract: <fresh-session behavior that should change>.
```

If Victor links a Claude-oriented `/reflect` workflow, reuse the learning-loop shape, not the Claude mechanics: no raw Claude transcript paths, no Opus-specific delegation requirement, and no automatic memory writes unless the current task authorizes edits.

## Evaluation Rubric

For each representative run, capture:

- `task`: what Victor wanted.
- `ideal_end_state`: what smoother future workflow this points toward, if any.
- `surface`: implementation, review, debugging, frontend, workflow design, orientation, publish, or history mining.
- `outcome`: pass, partial, fail, or unknown.
- `interventions`: 0, 1, 2+.
- `friction`: missing context, wrong artifact, failed command, overbroad search, weak plan, missed verification, publish uncertainty, or stale handoff.
- `candidate_artifact`: none, skill, CLI, `AGENTS.md`, memory extension, handoff, docs, or external tool.
- `evidence`: session ids, thread names, and short excerpts only.
- `resolver`: what docs, skills, CLIs, local docs, or repo state should have been loaded first.

## Promotion Ladder

Recommend the smallest artifact that would have prevented the repeated friction:

1. One-off task issue: do nothing.
2. Repeated behavior mistake: update a skill.
3. Stable broad preference: update `AGENTS.md`.
4. Noisy deterministic state: build or improve a CLI.
5. Cross-session retention rule: update a memory extension.
6. Long-running or resumable work: write a handoff artifact.
7. Repeated task with no route: create a new skill.

If a proposed CLI only aggregates `git`, `gh`, Codex apps, or existing shell commands, route through `artifact-gate` before recommending it.

## Automation Ladder

Treat automations as promoted workflow artifacts, not the default answer.

Early candidates should be read-only or draft-only:

- morning commit pulse;
- upskill draft from recent session friction;
- quality/doc drift report.

Riskier candidates need stronger gates:

- green-PR fixer;
- Sentry-style triage/fixer;
- scheduled self-modifying skill/doc updates.

Before recommending a write/publish automation, require: successful manual examples, clear idempotency, approval boundary, rollback path, and targeted verification.

## Output Shape

Lead with the recommendation:

```text
Recommendation: <one next workflow artifact toward <ideal end state>>.

Evidence:
- <session id or thread name>: <short pattern>

Ideal end state:
<what this should compound into if the pattern keeps proving out>

Why this, not something heavier:
<artifact boundary reasoning>

Validation contract:
<how a fresh Codex session should behave differently>
```

When the evidence is weak, say so and recommend another observation pass instead of inventing infrastructure.
