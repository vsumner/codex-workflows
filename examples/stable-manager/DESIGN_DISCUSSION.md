# DESIGN_DISCUSSION (Example: stable-manager)

## Decision Context
- initiative: codexify bootstrap adoption
- related_learning_artifact: LEARNING.md

## Options
### Option A
- summary: Lightweight docs-only checklist.
- benefits: Fast to adopt.
- risks: Easy to bypass, weak enforcement.
- complexity: low.

### Option B
- summary: Versioned templates plus role-enforced orchestration.
- benefits: Strong consistency, reusable contracts.
- risks: Slightly higher setup overhead.
- complexity: medium.

## Selected Approach
- decision: Option B
- why_selected: Provides clear contracts and enforces completion quality.
- why_not_others: Option A fails to prevent bypass behavior.

## Interfaces Affected
- .codex role files
- template artifacts
- task/check schemas
