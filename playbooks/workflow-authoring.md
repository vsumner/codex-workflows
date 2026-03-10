# Workflow Authoring Playbook

Use this playbook when creating, refactoring, or integrating workflow surfaces in `~/.codex`.

The goal is to keep Victor's Codex workflow coherent:
- RPIV is the primary workflow
- `solo`, `team`, and `deep-team` are topology choices
- utilities are phase-owned subroutines
- legacy alternates are archived instead of left to drift

## First Principles

- Prefer one dominant workflow surface over many competing ones.
- Add a new workflow surface only when an existing RPIV phase or utility cannot represent the job cleanly.
- Keep user-facing entrypoints small and memorable.
- Put stable doctrine in playbooks, executable behavior in skills, and human entry in prompts.
- Do not create a role for something that is really an orchestrator.
- Prefer native Codex tools and surfaces before inventing a custom prompt, skill, role, or artifact, except where Victor intentionally standardizes on a stronger workflow surface.

## Native-First Rule

Before creating or extending a workflow surface, ask:
1. Is there already a native Codex tool for this?
2. Is there already a native slash command or UI surface for this?
3. Should the workflow reinforce the native surface instead of wrapping or replacing it?

Prefer adopting these native surfaces directly when they fit:
- `update_plan` for the live checklist
- `request_user_input` for short structured decisions
- `request_permissions` plus named permission profiles for structured filesystem/network escalation
- native thread compaction or fresh-thread resume for phase-boundary context cleanup
- native collab tools for multi-agent coordination
- native thread resume/fork/archive semantics for workflow continuity
- native plugins and `@plugin` mentions for packaged skills, MCP servers, and apps
- native `/apps` and `$` connectors for external systems

Do not create a custom workflow surface just to restate a native capability.
Create one only when you need:
- stronger doctrine
- better routing
- a repeatable multi-tool procedure
- a phase-owned specialization

Explicit exception:
- `/plan` and `/review` are not automatic native-first cases in Victor's stack.
- If the workflow surface carries the canonical planning or review doctrine, keep that workflow surface authoritative instead of routing back to the native slash command.

Native-surface notes:
- If the job is "ask the user for product or workflow direction", reach for `request_user_input`.
- If the job is "obtain more filesystem or network access", prefer named permission profiles plus `request_permissions` over inventing a workflow-specific approval wrapper.
- If the job is "cross a long or noisy phase boundary", prefer native thread compaction when available; otherwise start the next phase from artifacts in a fresh thread/context.
- If a relevant plugin already bundles the needed skill, MCP server, or app, prefer using that plugin-backed capability over creating a parallel home-level workflow surface.

## GPT-5.4 Prompt Contract Doctrine

When authoring workflow prompts, skills, or playbooks for Codex/GPT-5.x, prefer explicit contract blocks over soft prose.

Use the smallest useful subset of these blocks:
- `output_contract`
- `verbosity_controls`
- `default_follow_through_policy`
- `tool_persistence_rules`
- `dependency_checks`
- `parallel_tool_calling`
- `completeness_contract`
- `verification_loop`
- `missing_context_gating`
- `research_mode`
- `user_updates_spec`

How to apply them:
- Use `output_contract` whenever the response must follow exact sections, schemas, or tight formatting.
- Use `verbosity_controls` to keep prompts and outputs compact without dropping required evidence.
- Use `default_follow_through_policy` when the workflow must decide when to proceed, ask, or respect mid-thread task changes.
- Use `tool_persistence_rules` and `dependency_checks` when correctness depends on retrieval, prerequisite lookups, or multi-step tool use.
- Use `parallel_tool_calling` only for independent evidence gathering; do not parallelize dependency-bound steps.
- Use `completeness_contract` when the workflow spans multiple deliverables, phases, items, files, or review findings.
- Use `verification_loop` whenever the workflow can end in a wrong-but-plausible state.
- Use `missing_context_gating` when guessing would be risky and missing context may be retrievable.
- Use `research_mode` for research, review, and synthesis tasks that need multi-pass retrieval and citation discipline.
- Use `user_updates_spec` for long-running or multi-phase work so progress updates stay brief and outcome-based.

Do not cargo-cult these blocks into every prompt.
- Small deterministic prompts should stay small.
- Add blocks only when they prevent a real failure mode or make the workflow materially more reliable.

Reasoning-effort doctrine:
- Treat reasoning effort as a last-mile tuning knob, not the primary fix for weak prompting.
- Before raising reasoning effort, tighten the output contract, grounding rules, completeness rules, and verification loop.
- If you choose heavier reasoning, document why the task shape actually needs it.

## Artifact Decision Rules

Choose the smallest correct artifact:

### Prompt
Use a prompt when:
- the user needs an explicit entrypoint
- you want a slash-friendly launch surface
- the behavior is mostly routing or framing

Good examples:
- `workflow-research`
- `workflow-plan`
- `workflow-execute`
- `workflow-verify`
- `workflow-review`

### Skill
Use a skill when:
- the behavior is reusable and procedural
- the workflow coordinates multiple roles or tools
- Codex needs durable operational instructions

Good examples:
- `workflow-research-codex`
- `workflow-plan-codex`
- `workflow-execute-codex`
- `simplify-codex`

### Role
Use a role when:
- one agent has one bounded job
- the scope is narrow and stable
- output format and behavior should be tightly constrained

Good examples:
- `planner`
- `research_locator`
- `spark_implementer`
- `workflow_verifier`

Bad role candidates:
- `simplify`
- `workflow-rpiv`
- anything that mainly coordinates other agents

### Playbook
Use a playbook when:
- the rules are durable doctrine
- multiple prompts/skills/roles should reference the same guidance
- you need a canonical decision framework

Good examples:
- `personal-swarm-workflow`
- `workflow-artifacts`
- `workflow-authoring`

## Classification Rules

Before creating anything, classify it:

### Phase
Create a new phase only if it is indispensable and not already covered by:
1. Research
2. Plan
3. Execute
4. Verify
5. Review

The default answer should be no.

### Topology
`solo`, `team`, and `deep-team` are topology wrappers, not new workflows.

### Subroutine
A subroutine belongs inside a phase.

Examples:
- `workflow-learning-tests` belongs inside Research
- `simplify-codex` belongs inside Execute
- `verify-gates-codex` belongs inside Verify
- `review-workflow-codex` belongs inside Review

### Specialization
A specialization is a repo/domain-specific variant of an existing phase utility.

Example:
- `review-spacebot-codex` is a Review specialization

## Naming Rules

- Workflow prompts should usually be `workflow-*`.
- Workflow skills should usually be `workflow-*-codex`.
- Use direct names for utilities only when they are intentionally narrow and explicit:
  - `simplify-codex`
  - `verify-gates-codex`
  - `bug-scanner-autopilot-codex`
- Prefer `execute` over `implement` in public workflow language.

## Prompt / Skill Pairing

Default pattern:
1. create a skill for the operational behavior
2. create a prompt wrapper for explicit invocation
3. document it in `workflow-commands.md` if it belongs in the active surface

Exceptions:
- a role does not need a prompt wrapper
- a playbook does not need a prompt wrapper
- do not add a prompt wrapper for something that should stay internal only

## Create / Refactor Workflow Process

1. Classify the candidate:
   - phase
   - topology
   - subroutine
   - specialization
   - doctrine
2. Decide the artifact type:
   - prompt
   - skill
   - role
   - playbook
3. Check for an existing surface that already owns it.
4. If it overlaps, integrate or replace instead of duplicating.
5. If it replaces something older:
   - move the old prompt/skill out of active roots
   - update operator docs
   - remove dangling references
6. Check whether a native Codex surface should be reinforced instead of replaced.
7. Decide which prompt contract blocks are required by the workflow's failure modes.
8. Add or update verification requirements.
9. Pressure-test on one realistic task.

## Integration Rules

New utilities should be attached to a phase explicitly.

Required mapping questions:
1. Which RPIV phase owns this?
2. Is it a general subroutine or a narrow specialization?
3. Does it need a role, or is it better as a skill orchestrator?
4. Does it need a user-facing prompt wrapper?
5. What old surface should be archived?
6. Which native Codex tool or slash command should this leverage directly?

Autopilot-style behavior should normally be modeled as Execute-mode state:
- `approval_gated`
- `autonomous`
- `parallel_autonomous`

It should not create:
- a new RPIV phase
- a parallel top-level workflow
- a duplicate home-level explorer role

## Verification Rules

Every workflow surface change should verify:
- docs point to active prompts/skills only
- no archived prompt is still documented as active
- role TOMLs parse
- prompt/skill names match their documented entrypoints
- the new surface does not duplicate an existing one without a reason
- structured outputs have an explicit output contract when needed
- multi-step workflows define completion and verification conditions explicitly
- research/review/synthesis workflows define grounding and citation behavior explicitly
- user update guidance is concise and phase-change driven when the workflow is long-running

Pressure-test checklist:
1. one direct invocation through the prompt wrapper
2. one natural-language invocation path
3. one case where the workflow should not trigger

## Simplify Rule

`simplify` is:
- a skill
- with a prompt wrapper
- used as an Execute refinement subroutine

`simplify` is not:
- a role
- a phase
- a replacement for Review

## Archive Policy

Archive a workflow surface when:
- it competes with the current RPIV path
- it duplicates a newer canonical surface
- it still works technically but is no longer the chosen mental model

Do not delete by default.
Move it out of active discovery roots and keep a short archive note.

## Anti-Patterns

- recreating a native Codex tool as a custom workflow surface
- wrapping a native slash command without adding real routing or doctrine value, except when `/plan` or `/review` are intentionally being upgraded into the canonical workflow path
- using vague prose where an explicit contract block would remove ambiguity
- increasing reasoning effort instead of fixing a weak prompt contract
- creating both a prompt and a skill when only one is needed
- creating a role for a coordinator
- creating a new phase for a subroutine
- leaving a compatibility alias active after the new name is established
- keeping legacy prompts in active roots
- documenting commands that no longer resolve to active assets
- adding a utility without assigning it to an RPIV phase

## External Reference

Use the official OpenAI prompt guidance as the external prompt-quality reference:
- `https://developers.openai.com/api/docs/guides/prompt-guidance/`

## Output Contract For Authoring Work

When authoring or refactoring a workflow surface, return:

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

Verification:
- <checks run>

Residual risk:
- <remaining ambiguity>
```
