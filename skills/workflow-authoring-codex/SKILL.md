---
name: workflow-authoring-codex
description: This skill should be used when the user asks to "create a workflow", "refactor this workflow", "add a Codex workflow skill", "integrate this utility into RPIV", "clean up workflow sprawl", or "decide if this should be a prompt, skill, role, or playbook".
---

# Workflow Authoring (Codex)

Create or refactor workflow surfaces for Victor's `~/.codex` stack.

Use this skill to make the workflow system more coherent, not more elaborate.

## Goal

- Keep RPIV as the dominant workflow model.
- Place new behavior at the right layer:
  - prompt
  - skill
  - role
  - playbook
- Integrate new utilities into an existing phase when possible.
- Archive competing legacy surfaces instead of leaving duplicates active.
- Prefer native Codex tools and surfaces before inventing custom workflow assets, except where Victor intentionally uses stronger workflow surfaces as the canonical path.

## Required Decisions

Before editing files, decide all of these:
1. classification:
   - phase
   - topology
   - subroutine
   - specialization
   - doctrine
2. artifact type:
   - prompt
   - skill
   - role
   - playbook
3. owning RPIV phase:
   - Research
   - Plan
   - Execute
   - Verify
   - Review
   - n/a
4. whether an existing surface already owns the behavior
5. whether any older surface should be archived
6. whether a native Codex tool or slash command should be used directly instead, or intentionally deferred to an existing workflow surface

## Default Rules

- Prefer integrating into an existing phase over creating a new workflow.
- Prefer a skill when the behavior coordinates multiple roles or tools.
- Prefer a role only for one bounded worker job.
- Prefer a prompt wrapper for explicit user invocation of a skill.
- Prefer a playbook for durable doctrine and decision rules.
- Prefer native Codex capabilities when they already solve the job cleanly.
- Treat `/plan` and `/review` as explicit exceptions when Victor's workflow surfaces add stronger doctrine, routing, or review behavior.

## Prompt Contract Rules

When creating or refactoring a workflow prompt, skill, or playbook, encode prompt behavior as explicit contracts instead of relying on implied prose.

Default contract blocks to consider:
1. `output_contract`
2. `verbosity_controls`
3. `default_follow_through_policy`
4. `tool_persistence_rules`
5. `dependency_checks`
6. `parallel_tool_calling`
7. `completeness_contract`
8. `verification_loop`
9. `missing_context_gating`
10. `research_mode`
11. `user_updates_spec`

Apply them selectively:
- Always define an explicit output contract when the workflow expects a schema, ordered sections, or compact output.
- Add follow-through and missing-context rules when the workflow may face ambiguity, user redirection, or risky assumptions.
- Add tool-persistence, dependency, and parallelism rules when correctness depends on retrieval, lookup, multi-tool work, or agent delegation.
- Add completeness and verification rules for any multi-step workflow, especially RPIV phase work.
- Add research mode plus citation/grounding rules for research, review, or synthesis workflows.
- Add user-updates rules for long-running or multi-phase workflows.
- Do not add every block mechanically to tiny deterministic prompts; use the smallest contract that prevents a measured failure mode.

## Specific Rules

- Do not create a new phase unless the existing RPIV phases cannot represent the job cleanly.
- Treat autopilot-like behavior as Execute-mode state (`approval_gated`, `autonomous`, `parallel_autonomous`) unless there is proof it needs a different abstraction.
- Do not create a role for an orchestrator behavior.
- Do not create a home-level explorer role when the built-in `explorer` already fits the job.
- Treat named permission profiles and native `request_permissions` as first-class native surfaces, not as a cue to invent a workflow-specific approval abstraction.
- Treat plugins as native capability bundles; if a plugin already supplies the needed skill, MCP server, or app, prefer that route over creating a parallel home-level workflow surface.
- Do not leave both old and new public names active unless compatibility is temporarily necessary.
- If compatibility is needed, mark the old surface as an alias and remove it once the new model sticks.
- If a surface is obsolete, move it out of active discovery roots and update docs.
- Do not create a custom workflow surface that merely restates `update_plan`, `request_user_input`, native thread lifecycle, or native collab controls.
- Do not use the native-first rule to collapse Victor's canonical Plan or Review workflow surfaces back to `/plan` or `/review` when the workflow surface adds real doctrine or behavior.
- Do not rely on vague statements like "be thorough", "do research", or "verify before done" when a compact contract block would make the behavior explicit.
- Do not reach for heavier reasoning as the primary fix for weak prompting; improve contracts, grounding, and verification first.

## Workflow

1. Read:
   - `../../playbooks/workflow-authoring.md`
   - `../../playbooks/personal-swarm-workflow.md`
   - `../../playbooks/workflow-commands.md`
2. Inspect the current prompt/skill/role surface.
3. Check native Codex tools and slash surfaces first.
4. Decide the correct abstraction.
5. Create or update only the necessary assets.
6. Archive competing legacy assets when appropriate.
7. Update operator docs if the active public surface changes.
8. Verify:
   - docs do not point at archived assets
   - role TOMLs parse if changed
   - the final surface is smaller or clearer, not just larger
   - prompt contracts are explicit where correctness depends on them
   - reasoning effort is justified by task shape, not used as a prompt-quality crutch

## Special Cases

### New Utility
Treat it as a phase-owned subroutine first.

Ask:
1. Does Execute own this?
2. Does Verify own this?
3. Does Review own this?

Only if the answer is clearly no should you consider a brand-new workflow.

### Simplify-Type Utility
If it orchestrates multiple reviewers, it should be:
- a skill
- with a prompt wrapper
- attached to Execute or Review

It should not be:
- a role
- a phase

### Repo-Specific Variant
Make it a specialization of an existing utility, not a new top-level workflow.

## Output

Return this structure:

```md
Decision:
- type: prompt|skill|role|playbook
- classification: phase|topology|subroutine|specialization|doctrine
- owning phase: <Research|Plan|Execute|Verify|Review|n/a>

Changes:
- created:
- updated:
- archived:

Why:
- <short rationale>
- <which native surfaces were reused>

Verification:
- <checks run>

Residual risk:
- <remaining ambiguity>
```

## References

- `../../playbooks/workflow-authoring.md`
- `../../playbooks/personal-swarm-workflow.md`
- `../../playbooks/workflow-commands.md`
- `https://developers.openai.com/api/docs/guides/prompt-guidance/`
