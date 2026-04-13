# Workflow Foundation

This is the current target architecture for `codex-workflows`. It is tailored to Victor's recent Codex and Claude work history: lots of review, verification, workflow reflection, subagent/team experiments, CI friction, and repeated "what's next?" handoff questions.

The goal is not to recreate Claudify or build a custom Codex runner. The goal is to make Codex better at Victor's work by giving it the right local context, narrow deterministic evidence tools, and durable workflow judgment without bloating every turn.

## Architecture

Use this stack:

1. Codex is the harness.
   - Do not wrap Codex in a large workflow runner unless repeated evidence proves Codex-native skills and CLIs cannot cover the job.
   - Let Codex own planning, synthesis, review judgment, and tool composition.

2. Repo-local docs are the source of truth.
   - Keep `AGENTS.md` short and table-of-contents shaped.
   - Put durable architecture, quality, verification, and workflow expectations in docs or skills.
   - Use local framework/API docs, including DocSet-style caches, before web scraping when they exist.

3. Fat skills encode judgment.
   - Skills should teach routing, process, safety boundaries, and verification expectations.
   - Prefer a small high-quality skill set over a large catalog.
   - Current core set: repo orientation, review-fix-verify, artifact gating, Codex thread search, workflow learning, environment debugging.

4. Small deterministic CLIs expose noisy state.
   - `codex-threads` is justified because raw Codex history is large, noisy, and needs bounded JSON access.
   - Claudify's `claude-threads search --json --limit N --matches N` is the preferred Claude-history evidence path; use `--verbose` only when full session payloads are necessary.
   - Reject CLIs that only aggregate `git`, `gh`, or one stable shell command.

5. Mechanical checks preserve taste.
   - Capture repeated preferences as tests, lints, schemas, stale-doc checks, quality-score docs, and remediation-rich errors when possible.
   - Use prose skills for judgment that cannot be made mechanical yet.

6. Automations graduate slowly.
   - Start with read-only reports.
   - Promote only after 3-10 representative manual runs produce useful output.
   - Require explicit gates before write/publish automations.

## Naming

Keep the technical plugin id `victor-workflows` for now.

`workflows` is cleaner, but a full rename touches plugin identity, local cache paths, memory-extension paths, install scripts, docs, and user config. That churn is not worth taking while the workflow layer is still personal and still proving its shape. Revisit the rename only if the plugin becomes less Victor-specific or if a migration plan updates all paths in one pass.

## Resolver Model

For each task type, load only the context that changes the outcome.

| Task | Load First | Avoid |
| --- | --- | --- |
| "What's next?" | `git status --short --branch`, recent commits, relevant handoff | New branch/status CLI |
| Workflow reflection | `codex-threads` patterns/search, then representative sessions | Raw `~/.codex` transcript dumps |
| Codex + Claude history review | `codex-threads`; Claudify `claude-threads` as bounded evidence | Porting Claudify orchestration |
| New workflow artifact | `artifact-gate`, existing skill/docs, validation contract | Creating machinery from vibes |
| Review-fix-verify | Current diff, review findings, targeted tests | Fixing unverified stale comments |
| Swift/macOS or Apple work | Local docs/DocSet output when available, build/test output | Web scraping before local docs |
| App/UI verification | Boot command, logs, screenshots/snapshots/video proof | Trusting code inspection alone |
| Publish readiness | `git`, `gh`/GitHub state, verification evidence | Commit/push/PR without request |

## Promotion Ladder

Use this order for workflow improvements:

1. Manual pass on 3-10 representative examples.
2. Show Victor the output shape.
3. If useful, codify the process as a skill or doc.
4. Add a CLI only for noisy deterministic state requiring bounded output, stable IDs, pagination, or safe defaults.
5. Add mechanical checks when repeated taste can be enforced deterministically.
6. Add read-only automation once the manual output is boringly useful.
7. Add write/publish automation only with idempotency, approval gates, rollback notes, and verification proof.

## Near-Term Backlog

1. Finish the memory-extension update that captures the harness-engineering lessons.
2. Update `workflow-learning` so it knows how to use Claude history as evidence without copying Claudify's architecture.
3. Add `review-fix-verify` as the next missing workflow skill from the 30-day history pass.
4. Run 3-10 manual workflow-reflection passes using Codex and Claude history, then compare the output shape.
5. Improve `codex-threads` with metrics only if those manual passes need them: tool calls, verification commands, changed files, compaction markers, publish actions, or command churn.
6. Use the local Claudify `claude-threads` bounded search output in 3-10 manual workflow-reflection passes.
7. Patch or wrap `claude-threads` only if those passes still show a noisy-state gap.
8. Prototype read-only reports: morning commit pulse, upskill draft, and quality/doc drift report.

## Deferred

Do not build these yet:

- a `codex-workflow status/next/branch` CLI;
- a custom orchestration runner;
- a large skill catalog;
- scheduled self-modifying skill/doc updates;
- green-PR fixers or Sentry-style fixers.

These may become correct later, but only after smaller loops show the exact missing deterministic primitive and the approval boundary.

## Validation Contract

Given Victor asks "look at my Codex and Claude work history with fresh eyes and improve `codex-workflows`," a fresh Codex session should:

- use `codex-threads` for bounded Codex history evidence;
- use Claudify's `claude-threads` bin as evidence input when useful;
- avoid raw transcript dumps and copied article summaries;
- recommend skills, docs, memory guidance, and mechanical checks before new orchestration;
- preserve the thin-harness/fat-skills architecture;
- produce a small ordered task list with gates;
- avoid commits, pushes, PRs, and scheduled automations unless Victor explicitly asks.
