---
description: Create or refactor Codex workflow surfaces using the RPIV-first authoring rules.
argument-hint: "[task_or_surface_to_create_or_refactor]"
---
Run the `workflow-authoring-codex` workflow.

Scope: $ARGUMENTS

Requirements:
- Classify the work first:
1. phase
2. topology
3. subroutine
4. specialization
5. doctrine
- Check native Codex tools and slash surfaces before creating anything custom.
- Treat `/plan` and `/review` as explicit exceptions when Victor's canonical workflow surface is stronger.
- Decide the right artifact type:
1. prompt
2. skill
3. role
4. playbook
- Treat autopilot-like behavior as Execute-mode state (`approval_gated`, `autonomous`, `parallel_autonomous`) unless there is evidence it needs a different abstraction.
- Do not create a duplicate home-level explorer role when the built-in `explorer` already fits.
- Prefer integrating into existing RPIV phases over adding a new workflow.
- Prefer reinforcing native Codex behavior over wrapping it without added value.
- Do not use the native-first rule to collapse Victor's canonical Plan or Review workflow surfaces back to `/plan` or `/review`.
- Decide which explicit prompt contract blocks the workflow needs:
1. `output_contract`
2. `default_follow_through_policy`
3. `tool_persistence_rules`
4. `dependency_checks`
5. `parallel_tool_calling`
6. `completeness_contract`
7. `verification_loop`
8. `missing_context_gating`
9. `research_mode`
10. `user_updates_spec`
- Prefer explicit contracts over vague prose when correctness, grounding, or completion depend on them.
- Treat reasoning effort as a last-mile knob; tighten prompt contracts and verification rules before asking for heavier reasoning.
- Archive competing legacy surfaces when the new one replaces them.
- Update operator docs if the active public surface changes.
- Return the decision, created/updated/archived assets, verification, and residual risk.
