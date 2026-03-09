# CLI DX Principles

This reference distills practical CLI developer-experience guidance from:

- https://diatomenterprises.com/what-is-cli-programming/
- https://thenewstack.io/user-interfaces-in-agentic-cli-tools-what-developers-need/
- https://github.blog/engineering/user-experience/building-a-more-accessible-github-cli/
- https://www.builder.io/c/docs/cli-code-generation-best-practices

Use it to support concrete recommendations in `SKILL.md`.

## Core Principles

1. Optimize for speed and repeatability.
- Make common workflows faster than equivalent GUI flows.
- Keep resource usage low and avoid unnecessary rendering overhead.
- Support scripting and automation for repetitive workflows.

2. Preserve terminal compatibility.
- Do not assume every terminal has identical capabilities.
- Account for local, remote, and web-based terminal quirks.
- Keep copy/paste behavior reliable for multiline text.

3. Design for conversational and command workflows.
- Treat agentic CLI sessions as mixed command/response experiences.
- Keep session output organized and scannable.
- Let users move between normal shell usage and agent interactions cleanly.

4. Prefer explicit textual feedback.
- Use clear progress messages rather than visual-only spinner effects.
- Keep prompts unambiguous, including context and intent.
- Make error messages actionable and include next steps.

## Accessibility Guidance for CLIs

1. Make prompts screen-reader friendly.
- Use prompt structures that avoid confusing redraw-heavy interactions.
- Keep choice prompts explicit and linear when possible.

2. Use accessible progress reporting.
- Replace redraw-heavy animated indicators with stable textual progress.
- Include action-specific status messages when available.

3. Respect color customization and contrast.
- Assume terminal background colors vary by user preference.
- Choose ANSI color roles that remain legible across themes.
- Support 4-bit color mappings where possible for broad customization.

4. Treat terminal accessibility as first-class.
- Unlike web UIs, CLI accessibility lacks a single comprehensive standard.
- Validate behavior directly with terminal assistive technology workflows.

## Agentic CLI UX Guidance

1. Keep sessions responsive and interactive.
- Minimize flicker and noisy repainting.
- Preserve fast, accurate text selection and copying.

2. Keep output blocks understandable.
- Separate user input and tool responses clearly.
- Preserve line integrity and structural cues in plain text output.

3. Avoid voice-only assumptions.
- Keep text-first workflows complete and powerful.
- Support mixed modality, but do not degrade typed workflows.

## Code-Generating CLI Guidance

1. Commit before running generation.
- Encourage users to checkpoint before large mutations.
- Make rollback straightforward.

2. Run in the correct project scope.
- Execute from project root for correct analysis.
- Avoid monorepo root when operating on a specific package/app.

3. Use specific, incremental prompts.
- Prefer small, concrete requests over large compound asks.
- Iterate with multiple prompts for better control.

4. Support interactive and non-interactive modes.
- Offer guided mode for humans and flag-driven mode for automation.
- Document key flags (`--prompt`, `--cwd`, `--help`, etc.).

5. Support exclusion and rule files.
- Let users protect sensitive files and test files from mutation.
- Allow scoped instruction files to shape generation behavior per path.

## Practical Review Checklist

Use this checklist during CLI DX review:

- [ ] `--help` explains purpose, examples, and key flags.
- [ ] Errors suggest next steps and recovery.
- [ ] Prompts include intent and relevant context.
- [ ] Output is scannable and copy-safe.
- [ ] Machine-readable output mode exists for automation.
- [ ] Exit codes are deterministic.
- [ ] Progress messaging is text-accessible.
- [ ] Color usage remains legible across terminal themes.
- [ ] Destructive operations require explicit confirmation.
- [ ] Project scope detection prevents wrong-directory operations.
- [ ] Large operations can be executed in smaller iterative steps.
