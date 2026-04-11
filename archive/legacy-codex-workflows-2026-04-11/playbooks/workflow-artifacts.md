# Workflow Artifacts

Use repo-local artifacts for every non-trivial RPIV run.
Infer the lightest artifact weight that keeps the run resumable and reviewable.

## Default Layout

Use one slugged directory per run:

```text
.codex-workflow/{slug}/
  proposal.md
  research.md
  research.json
  plan.md
  features.json
  validation-contract.md
  validation-state.json
  execution-summary.md
  verification-report.md
  handoffs/
```

`proposal.md` is optional.

Artifact weight:
- `lightweight`: compact `research.md`, `research.json`, `plan.md`, and phase-local verification notes
- `standard`: adds `validation-contract.md`, `validation-state.json`, `execution-summary.md`, and `verification-report.md`
- `graph`: adds `features.json` and the full resumable graph state

Everything else is recommended for medium and large work.

## Slug Rules
- Keep one slug across all artifacts for the same initiative.
- Prefer `{YYYY-MM-DD}-{short-task-name}`.
- Resume existing work by reusing the slug instead of creating a new directory.

## Research Outputs

Research should produce both:
1. a human-readable synthesis (`research.md`)
2. a machine-friendly sidecar (`research.json`)

### `research.md`

```md
# Research: {title}

## Metadata
- Slug: {slug}
- Mode: quick|medium|deep
- Scope: {scope}
- Intent: {intent}

## File Locations
| File | Relevance | Notes |
|------|-----------|-------|

## Current Control Flow
{key paths, boundaries, and interactions}

## Key Patterns
- {file:line} - {pattern and why it matters}

## Dependencies
- Internal:
- External:

## External Context
- {docs, API behavior, ecosystem findings}

## Verified Assumptions
- {fact backed by evidence}

## Unverified Assumptions
- {assumption + blast radius}

## AI-Slop Risks
- Risk:
  Mitigation:

## Directives For Planner
- MUST:
- MUST NOT:
- SHOULD:

## Suggested Verification Targets
- {assertions or commands to preserve}
```

### `research.json`

```json
{
  "slug": "2026-03-07-example",
  "mode": "deep",
  "intent": "feature|refactor|bugfix|architecture|integration",
  "scope": "short summary",
  "fileLocations": [
    {
      "path": "src/example.ts",
      "relevance": "core",
      "notes": "why it matters"
    }
  ],
  "controlFlow": [
    {
      "entry": "src/a.ts:functionA",
      "steps": ["src/b.ts:functionB", "src/c.ts:functionC"]
    }
  ],
  "patterns": [
    {
      "reference": "src/example.ts:42",
      "reason": "pattern to follow"
    }
  ],
  "dependencies": {
    "internal": ["src/a.ts", "src/b.ts"],
    "external": ["service-x", "sdk-y"]
  },
  "externalContext": [
    {
      "source": "official docs",
      "summary": "key fact"
    }
  ],
  "verifiedAssumptions": ["fact with evidence"],
  "unverifiedAssumptions": [
    {
      "assumption": "runtime behavior X",
      "blastRadius": "high",
      "needsLearningTest": true
    }
  ],
  "aiSlopRisks": [
    {
      "risk": "scope inflation",
      "mitigation": "limit files"
    }
  ],
  "plannerDirectives": {
    "must": ["keep API stable"],
    "mustNot": ["no broad refactor"],
    "should": ["reuse existing validation pattern"]
  },
  "suggestedVerificationTargets": [
    "cargo test -p example",
    "manual API behavior check"
  ]
}
```

## Plan Outputs

Plan first, graph second.
Emit `features.json` only when execution or resumability benefits from a graph.

### `plan.md`

```md
# Plan: {title}

## Overview
{intent and approach}

## Constraints
- {constraint}

## Non-Goals
- {non-goal}

## MUST / MUST NOT / SHOULD
- MUST:
- MUST NOT:
- SHOULD:

## Architecture
- Current state:
- Proposed changes:
- Trade-offs:

## Critical Files
- {path} - {reason}

## Phases
### P1: {phase name}
- Objective:
- Files:
- Acceptance criteria:
- Verification:

## Stop Conditions
- planning changes are incremental
- review passes stop finding substantive gaps
- graph can be compiled without material ambiguity
```

### `features.json`

Executable graph compiled from the plan.
Required for `graph` weight. Optional for `standard`. Usually omitted for `lightweight`.

```json
{
  "slug": "2026-03-07-example",
  "topology": "solo|team|deep-team",
  "phase": "execute|verify|fix-loop",
  "features": [
    {
      "id": "F1",
      "title": "execute auth endpoint",
      "status": "pending|in_progress|done|blocked|failed",
      "dependsOn": [],
      "ownedFiles": ["src/auth.rs"],
      "acceptanceCriteria": [
        "endpoint exists",
        "tests pass"
      ],
      "verificationPlan": [
        "cargo test -p auth"
      ],
      "recommendedAgent": "spark_implementer",
      "handoff": null
    }
  ]
}
```

`features[].status = done` means the packet is execution-complete and ready for Verify.
It does not mean the overall initiative is verified. Verification truth lives in
`validation-state.json` and `verification-report.md`.

## Validation Outputs

### `validation-contract.md`

Required for `standard` and `graph`. Optional for `lightweight` when the validation strategy stays inline in `plan.md`.

```md
# Validation Contract: {title}

## Scrutiny Assertions
- A1: {assertion}
  Evidence: {command or review output}

## User-Surface Assertions
- U1: {assertion}
  Evidence: {manual, browser, API, screenshot, logs}

## Required Commands
- {command}

## Environment / Fixtures
- {what must exist}

## Completion Rule
- All required assertions must be `passed` in `validation-state.json`.
```

### `validation-state.json`

Recommended once Verify begins. Required for `standard` and `graph`.
If Verify starts and the file does not exist yet, bootstrap it from
`validation-contract.md` before running checks.

```json
{
  "slug": "2026-03-07-example",
  "verificationScope": "slice|initiative|both",
  "activeFeatureIds": ["F1"],
  "overall": "pending|passed|failed|blocked",
  "initiativeOverall": "pending|passed|failed|blocked",
  "assertions": [
    {
      "id": "A1",
      "kind": "scrutiny|user_flow",
      "status": "pending|passed|failed|blocked",
      "evidence": "short summary",
      "lastUpdatedBy": "scrutiny_validator",
      "retryCount": 0
    }
  ]
}
```

For graph runs, Verify should scope explicitly:
- use `activeFeatureIds` for the slice being proved now
- use `initiativeOverall` when the full initiative is still broader than the current slice
- mark assertions `blocked` when they depend on pending features, missing fixtures, or timed-out evidence gathering rather than pretending they failed for the wrong reason

### `verification-report.md`

Recommended once Verify begins. Required for `standard` and `graph`.

```md
# Verification Report: {title}

## Summary
- Verification scope:
- Active features:
- Overall:
- Initiative overall:
- Passed assertions:
- Failed assertions:
- Blocked assertions:

## Completeness
- what is done
- what is not done

## Correctness
- behavior and edge cases

## Coherence
- matches plan and architecture

## Recommendations
- Critical:
- Warnings:
- Suggestions:
- Next step:
```

## Execution Outputs

### `execution-summary.md`

Recommended once Execute begins. Required for `standard` and `graph`.

```md
# Execution Summary: {title}

## Features
- F1: done
- F2: blocked

## Handoffs
- F1: {summary}

## Verification Evidence
- {packet-level evidence}

## Deviations
- {what changed vs plan}

## Next Step
- {verify | fix-loop | done}
```

## Rules
- Research artifacts feed the planner.
- Plan artifacts feed implementation and verification.
- Verification reads plan, research, execution summary, and validation contract.
- For `standard` and `graph` runs, Verify must leave behind both `validation-state.json` and
  `verification-report.md` even when the verdict is `failed` or `blocked`.
- If delegated validators time out or evidence gathering stalls, write the current blocked state
  and one explicit next step instead of leaving Verify half-finished.
- Resume reads artifacts first.
- Do not let multiple plan files become competing sources of truth.
- Do not emit graph artifacts just because the schema exists. Emit them because coordination, resumability, or risk justifies them.
- Do not use RPIV artifacts as a substitute for the native `update_plan` checklist.
- Use artifacts for durable state; use `update_plan` for the live checklist.
- Do not use RPIV artifacts as a substitute for native `request_user_input`.
- Use `request_user_input` for short structured decisions only; record the durable consequence in artifacts.
