# Upstream Surface Triage

Use this file to keep local workflow guidance aligned with upstream `openai/codex`.
Every upstream feature flag and slash command that is relevant to the home workflow
surface should have an explicit status here so `check-shareable.sh` can flag new
upstream additions instead of letting them drift in silently.

Status meanings:
- `baseline`: upstream surface we rely on or accept without extra local workflow work
- `adopted`: explicitly surfaced in local config or workflow/operator docs
- `candidate`: likely useful next, but not enabled or standardized yet
- `watch`: interesting, but not mature or valuable enough to standardize yet
- `skip`: intentionally not part of the local workflow contract
- `internal`: upstream debug or low-level surface that should not drive local workflow docs

## Features
- `undo`: skip
- `shell_tool`: baseline
- `unified_exec`: adopted
- `shell_zsh_fork`: watch
- `shell_snapshot`: adopted
- `js_repl`: adopted
- `code_mode`: watch
- `code_mode_only`: skip
- `js_repl_tools_only`: skip
- `memories`: watch
- `child_agents_md`: adopted
- `image_detail_original`: candidate
- `apply_patch_freeform`: watch
- `exec_permission_approvals`: candidate
- `request_permissions_tool`: candidate
- `multi_agent`: adopted
- `enable_fanout`: watch
- `apps`: adopted
- `tool_search`: candidate
- `tool_suggest`: candidate
- `plugins`: adopted
- `image_generation`: skip
- `skill_mcp_dependency_install`: baseline
- `skill_env_var_dependency_prompt`: candidate
- `default_mode_request_user_input`: adopted
- `guardian_approval`: watch
- `tool_call_mcp_elicitation`: watch
- `personality`: adopted
- `artifact`: skip
- `fast_mode`: adopted
- `realtime_conversation`: watch
- `tui_app_server`: watch
- `prevent_idle_sleep`: adopted

## Commands
- `/model`: adopted
- `/fast`: adopted
- `/approvals`: baseline
- `/permissions`: adopted
- `/setup-default-sandbox`: skip
- `/sandbox-add-read-dir`: skip
- `/experimental`: adopted
- `/skills`: baseline
- `/review`: adopted
- `/rename`: baseline
- `/new`: adopted
- `/resume`: baseline
- `/fork`: baseline
- `/init`: adopted
- `/compact`: baseline
- `/plan`: adopted
- `/collab`: adopted
- `/agent`: adopted
- `/diff`: adopted
- `/copy`: baseline
- `/mention`: baseline
- `/status`: adopted
- `/debug-config`: internal
- `/title`: skip
- `/statusline`: baseline
- `/theme`: skip
- `/mcp`: adopted
- `/apps`: adopted
- `/plugins`: adopted
- `/logout`: skip
- `/quit`: baseline
- `/exit`: baseline
- `/feedback`: skip
- `/rollout`: internal
- `/ps`: internal
- `/stop`: baseline
- `/clear`: baseline
- `/personality`: adopted
- `/realtime`: watch
- `/settings`: watch
- `/test-approval`: internal
- `/subagents`: skip
- `/debug-m-drop`: internal
- `/debug-m-update`: internal
