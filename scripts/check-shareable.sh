#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
codex_home="${HOME}/.codex"
upstream_codex="${HOME}/src/github.com/openai/codex"

usage() {
  cat <<'USAGE'
Usage: ./scripts/check-shareable.sh [--codex-home PATH] [--upstream-codex PATH]

Validates the tracked shareable Codex workflow surface, compares it against
an installed Codex home tree, and optionally checks key config/command drift
against a local upstream Codex clone.
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --codex-home)
      [[ $# -ge 2 ]] || {
        echo "missing value for --codex-home" >&2
        exit 2
      }
      codex_home="$2"
      shift 2
      ;;
    --upstream-codex)
      [[ $# -ge 2 ]] || {
        echo "missing value for --upstream-codex" >&2
        exit 2
      }
      upstream_codex="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

failures=0
warnings=0
triage_file="$repo_root/playbooks/upstream-surface-triage.md"
upstream_features_file="$upstream_codex/codex-rs/features/src/lib.rs"
upstream_models_file="$upstream_codex/codex-rs/models-manager/models.json"
upstream_slash_file="$upstream_codex/codex-rs/tui/src/slash_command.rs"
upstream_role_file="$upstream_codex/codex-rs/core/src/agent/role.rs"
upstream_tools_dir="$upstream_codex/codex-rs/tools/src"
upstream_tool_registry_file="$upstream_tools_dir/tool_registry_plan.rs"

pass() {
  printf 'PASS %s\n' "$1"
}

warn() {
  warnings=$((warnings + 1))
  printf 'WARN %s\n' "$1"
}

fail() {
  failures=$((failures + 1))
  printf 'FAIL %s\n' "$1"
}

require_repo_path() {
  local rel="$1"
  if [[ -e "$repo_root/$rel" ]]; then
    pass "repo has $rel"
  else
    fail "repo is missing $rel"
  fi
}

compare_path() {
  local rel="$1"
  local repo_rel="${2:-$rel}"
  local repo_path="$repo_root/$repo_rel"
  local home_path="$codex_home/$rel"
  local diff_output
  local mismatched=0

  if [[ ! -e "$home_path" ]]; then
    fail "$home_path is missing; run ./scripts/install-shareable-to-home.sh"
    return
  fi

  if [[ -d "$repo_path" ]]; then
    if [[ "$rel" == "skills" ]]; then
      while IFS= read -r repo_child; do
        child_name="$(basename "$repo_child")"
        if [[ ! -e "$home_path/$child_name" ]]; then
          fail "$rel/$child_name is missing from $codex_home"
          mismatched=1
          continue
        fi
        diff_output="$(diff -qr "$repo_child" "$home_path/$child_name" || true)"
        if [[ -n "$diff_output" ]]; then
          fail "$rel/$child_name differs between repo and $codex_home"
          printf '%s\n' "$diff_output" | sed -n '1,20p'
          mismatched=1
        fi
      done < <(find "$repo_path" -mindepth 1 -maxdepth 1 ! -name '.system' | sort)

      if [[ "$mismatched" -eq 0 ]]; then
        pass "$rel matches $codex_home for repo-managed entries"
      fi
      return
    else
      diff_output="$(diff -qr "$repo_path" "$home_path" || true)"
    fi
  else
    diff_output="$(diff -u "$repo_path" "$home_path" || true)"
  fi

  if [[ -n "$diff_output" ]]; then
    fail "$rel differs between repo and $codex_home"
    printf '%s\n' "$diff_output" | sed -n '1,20p'
  else
    pass "$rel matches $codex_home"
  fi
}

extract_config_feature_keys() {
  python3 - "$repo_root/config.toml.example" <<'PY'
import re
import sys
from pathlib import Path

path = Path(sys.argv[1])
inside = False
keys = []
for raw_line in path.read_text().splitlines():
    line = raw_line.strip()
    if line.startswith('[') and not line.startswith('#'):
        inside = line == '[features]'
        continue
    if not inside:
        continue
    if line.startswith('# Optional named permissions profiles.'):
        break
    match = re.match(r'^(?:#\s*)?([A-Za-z0-9_]+)\s*=\s*(?:true|false)\b', line)
    if match:
        keys.append(match.group(1))
for key in sorted(set(keys)):
    print(key)
PY
}

extract_upstream_feature_keys() {
  python3 - "$upstream_features_file" <<'PY'
import re
import sys
from pathlib import Path

path = Path(sys.argv[1])
text = path.read_text()
keys = sorted(set(re.findall(r'key:\s*"([a-z0-9_]+)"', text)))
for key in keys:
    print(key)
PY
}

extract_repo_model_names() {
  python3 - "$repo_root/config.toml.example" "$repo_root/roles" <<'PY'
import re
import sys
from pathlib import Path

config_path = Path(sys.argv[1])
roles_dir = Path(sys.argv[2])
models = set()
for raw_line in config_path.read_text().splitlines():
    match = re.match(r'^(model|review_model)\s*=\s*"([^"]+)"\s*$', raw_line.strip())
    if match:
        models.add(match.group(2))
for role_file in roles_dir.glob('*.toml'):
    for raw_line in role_file.read_text().splitlines():
        match = re.match(r'^model\s*=\s*"([^"]+)"\s*$', raw_line.strip())
        if match:
            models.add(match.group(1))
for model in sorted(models):
    print(model)
PY
}

extract_upstream_model_slugs() {
  python3 - "$upstream_models_file" <<'PY'
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
obj = json.loads(path.read_text())
for model in sorted({item['slug'] for item in obj.get('models', []) if 'slug' in item}):
    print(model)
PY
}


check_upstream_models() {
  if [[ ! -f "$upstream_models_file" ]]; then
    warn "$upstream_models_file not found; skipping upstream model drift checks"
    return
  fi

  local repo_models
  local upstream_models
  repo_models="$(extract_repo_model_names)"
  upstream_models="$(extract_upstream_model_slugs)"

  if [[ -z "$repo_models" ]]; then
    fail "shareable config and roles expose no model pins"
    return
  fi

  local missing=0
  while IFS= read -r model_name; do
    [[ -n "$model_name" ]] || continue
    if grep -Fxq "$model_name" <<<"$upstream_models"; then
      pass "model exists upstream: $model_name"
    else
      warn "model not present in bundled upstream models.json: $model_name"
      missing=1
    fi
  done <<<"$repo_models"

  if [[ "$missing" -eq 0 ]]; then
    pass "all shareable model pins are recognized by upstream Codex"
  else
    warn "bundled upstream models.json is not a complete source of live model availability; verify missing pins against the runtime picker or current Codex tool surface before changing config"
  fi
}

check_upstream_feature_keys() {
  if [[ ! -f "$upstream_features_file" ]]; then
    warn "$upstream_features_file not found; skipping upstream feature-key drift checks"
    return
  fi

  local config_keys
  local upstream_keys
  config_keys="$(extract_config_feature_keys)"
  upstream_keys="$(extract_upstream_feature_keys)"

  if [[ -z "$config_keys" ]]; then
    fail "config.toml.example exposes no feature keys in [features]"
    return
  fi

  local missing=0
  while IFS= read -r key; do
    [[ -n "$key" ]] || continue
    if grep -Fxq "$key" <<<"$upstream_keys"; then
      pass "feature key exists upstream: $key"
    else
      fail "feature key missing upstream: $key"
      missing=1
    fi
  done <<<"$config_keys"

  if [[ "$missing" -eq 0 ]]; then
    pass "all shareable feature keys are recognized by upstream Codex"
  fi
}

check_upstream_native_commands() {
  if [[ ! -f "$upstream_slash_file" ]]; then
    warn "$upstream_slash_file not found; skipping upstream native-command drift checks"
    return
  fi

  while IFS='|' read -r command variant; do
    [[ -n "$command" ]] || continue
    if grep -Fq "SlashCommand::$variant" "$upstream_slash_file"; then
      pass "native command exists upstream: $command"
    else
      fail "native command missing upstream: $command"
    fi
  done <<'COMMANDS'
/plan|Plan
/collab|Collab
/agent|Agent
/fast|Fast
/review|Review
/apps|Apps
/permissions|Permissions
COMMANDS
}

extract_triaged_features() {
  python3 - "$triage_file" <<'PY'
import re
import sys
from pathlib import Path

text = Path(sys.argv[1]).read_text().splitlines()
inside = False
for raw_line in text:
    line = raw_line.strip()
    if line == "## Features":
        inside = True
        continue
    if inside and line.startswith("## "):
        break
    if not inside:
        continue
    match = re.match(r"^- `([^`]+)`: ([a-z_]+)$", line)
    if match:
        print(f"{match.group(1)}|{match.group(2)}")
PY
}

extract_triaged_commands() {
  python3 - "$triage_file" <<'PY'
import re
import sys
from pathlib import Path

text = Path(sys.argv[1]).read_text().splitlines()
inside = False
for raw_line in text:
    line = raw_line.strip()
    if line == "## Commands":
        inside = True
        continue
    if inside and line.startswith("## "):
        break
    if not inside:
        continue
    match = re.match(r"^- `([^`]+)`: ([a-z_]+)$", line)
    if match:
        print(f"{match.group(1)}|{match.group(2)}")
PY
}

extract_upstream_triage_feature_keys() {
  python3 - "$upstream_features_file" <<'PY'
import re
import sys
from pathlib import Path

text = Path(sys.argv[1]).read_text()
excluded = {
    "web_search_request",
    "web_search_cached",
    "search_tool",
    "sqlite",
    "use_linux_sandbox_bwrap",
    "request_rule",
    "experimental_windows_sandbox",
    "elevated_windows_sandbox",
    "remote_models",
    "steer",
    "collaboration_modes",
    "debug_hide_spawn_agent_metadata",
    "multi_agent_v2",
    "remote_control",
    "responses_websockets",
    "responses_websockets_v2",
    "runtime_metrics",
    "general_analytics",
    "codex_hooks",
    "codex_git_commit",
    "use_legacy_landlock",
    "enable_request_compression",
}
for key in sorted(set(re.findall(r'key:\s*"([^"]+)"', text))):
    if key in excluded:
        continue
    print(key)
PY
}

extract_upstream_command_names() {
  python3 - "$upstream_slash_file" <<'PY'
import re
import sys
from pathlib import Path

text = Path(sys.argv[1]).read_text()
body = re.search(r"pub enum SlashCommand \{(.*?)\n\}", text, re.S)
if body is None:
    raise SystemExit("could not locate SlashCommand enum")

def kebab(name: str) -> str:
    parts = re.findall(r"[A-Z][a-z0-9]*", name)
    return "-".join(part.lower() for part in parts)

pending = None
for raw_line in body.group(1).splitlines():
    stripped = raw_line.strip()
    if not stripped or stripped.startswith("//"):
        continue
    attr = re.match(r'#\[strum\((.*)\)\]', stripped)
    if attr:
        serialize = re.search(r'serialize = "([^"]+)"', attr.group(1))
        to_string = re.search(r'to_string = "([^"]+)"', attr.group(1))
        pending = to_string.group(1) if to_string else serialize.group(1) if serialize else None
        continue
    if stripped.startswith("#["):
        continue
    name = stripped.rstrip(",")
    if not re.fullmatch(r"[A-Z][A-Za-z0-9_]*", name):
        continue
    command = pending or kebab(name)
    pending = None
    print(f"/{command}")
PY
}

check_upstream_triage_coverage() {
  if [[ ! -f "$upstream_features_file" || ! -f "$upstream_slash_file" ]]; then
    warn "upstream feature or slash-command source missing; skipping triage coverage checks"
    return
  fi
  if [[ ! -f "$triage_file" ]]; then
    fail "missing upstream triage file: $triage_file"
    return
  fi

  local triaged_features
  local triaged_commands
  local upstream_features
  local upstream_commands
  triaged_features="$(extract_triaged_features)"
  triaged_commands="$(extract_triaged_commands)"
  upstream_features="$(extract_upstream_triage_feature_keys)"
  upstream_commands="$(extract_upstream_command_names)"

  if [[ -z "$triaged_features" ]]; then
    fail "upstream surface triage file has no feature entries"
    return
  fi
  if [[ -z "$triaged_commands" ]]; then
    fail "upstream surface triage file has no command entries"
    return
  fi

  while IFS= read -r key; do
    [[ -n "$key" ]] || continue
    if grep -Fq "${key}|" <<<"$triaged_features"; then
      pass "triaged upstream feature: $key"
    else
      fail "missing triage entry for upstream feature: $key"
    fi
  done <<<"$upstream_features"

  while IFS='|' read -r key _status; do
    [[ -n "$key" ]] || continue
    if grep -Fxq "$key" <<<"$upstream_features"; then
      pass "triage feature still exists upstream: $key"
    else
      fail "stale triage feature entry not found upstream: $key"
    fi
  done <<<"$triaged_features"

  while IFS= read -r command_name; do
    [[ -n "$command_name" ]] || continue
    if grep -Fq "${command_name}|" <<<"$triaged_commands"; then
      pass "triaged upstream command: $command_name"
    else
      fail "missing triage entry for upstream command: $command_name"
    fi
  done <<<"$upstream_commands"

  while IFS='|' read -r command_name _status; do
    [[ -n "$command_name" ]] || continue
    if grep -Fxq "$command_name" <<<"$upstream_commands"; then
      pass "triage command still exists upstream: $command_name"
    else
      fail "stale triage command entry not found upstream: $command_name"
    fi
  done <<<"$triaged_commands"
}

check_upstream_builtin_roles() {
  if [[ ! -f "$upstream_role_file" ]]; then
    warn "$upstream_role_file not found; skipping upstream built-in role drift checks"
    return
  fi

  while IFS= read -r role_name; do
    [[ -n "$role_name" ]] || continue
    if python3 - "$upstream_role_file" "$role_name" <<'PY'
import sys
from pathlib import Path
text = Path(sys.argv[1]).read_text()
needle = f'"{sys.argv[2]}".to_string()'
sys.exit(0 if needle in text else 1)
PY
    then
      pass "built-in role exists upstream: $role_name"
    else
      fail "built-in role missing upstream: $role_name"
    fi
  done <<'ROLES'
explorer
worker
ROLES
}

check_upstream_native_tools() {
  if [[ ! -f "$upstream_tool_registry_file" ]]; then
    warn "$upstream_tool_registry_file not found; skipping upstream native-tool drift checks"
    return
  fi

  while IFS= read -r tool_name; do
    [[ -n "$tool_name" ]] || continue
    if rg -F "\"$tool_name\"" "$upstream_tools_dir" >/dev/null 2>&1
    then
      pass "native tool exists upstream: $tool_name"
    else
      fail "native tool missing upstream: $tool_name"
    fi
done <<'TOOLS'
request_user_input
request_permissions
spawn_agent
send_input
resume_agent
wait_agent
close_agent
spawn_agents_on_csv
js_repl
js_repl_reset
TOOLS
}

check_prompt_frontmatter() {
  while IFS= read -r prompt_file; do
    [[ -n "$prompt_file" ]] || continue
    if python3 - "$prompt_file" <<'PY'
import re
import sys
from pathlib import Path

text = Path(sys.argv[1]).read_text()
match = re.match(r'^---\n(.*?)\n---\n', text, re.S)
if match is None:
    raise SystemExit(1)
if re.search(r'^description:\s+', match.group(1), re.M) is None:
    raise SystemExit(1)
PY
    then
      pass "prompt frontmatter is valid: ${prompt_file#$repo_root/}"
    else
      fail "prompt frontmatter is missing description or YAML block: ${prompt_file#$repo_root/}"
    fi
  done < <(find "$repo_root/prompts" -name '*.md' | sort)
}

check_skill_frontmatter() {
  while IFS= read -r skill_file; do
    [[ -n "$skill_file" ]] || continue
    if python3 - "$skill_file" <<'PY'
import re
import sys
from pathlib import Path

text = Path(sys.argv[1]).read_text()
match = re.match(r'^---\n(.*?)\n---\n', text, re.S)
if match is None:
    raise SystemExit(1)
frontmatter = match.group(1)
for key in ("name", "description"):
    if re.search(rf'^{key}:\s+', frontmatter, re.M) is None:
        raise SystemExit(1)
PY
    then
      pass "skill frontmatter is valid: ${skill_file#$repo_root/}"
    else
      fail "skill frontmatter is missing name/description or YAML block: ${skill_file#$repo_root/}"
    fi
  done < <(find "$repo_root/skills" -name 'SKILL.md' | sort)
}

echo "== Repo surface =="
for rel in \
  AGENTS.md \
  .agents/plugins/marketplace.json \
  config.toml.example \
  plugin \
  plugin/.codex-plugin/plugin.json \
  plugin/README.md \
  playbooks \
  playbooks/upstream-surface-triage.md \
  prompts \
  roles \
  rules \
  skills \
  scripts \
  README.md
do
  require_repo_path "$rel"
done

echo "== Config targets =="
while IFS= read -r rel; do
  [[ -n "$rel" ]] || continue
  if [[ -f "$repo_root/$rel" ]]; then
    pass "config target exists: $rel"
  else
    fail "config target missing: $rel"
  fi
done < <(sed -n 's/^config_file = "\([^"]*\)"$/\1/p' "$repo_root/config.toml.example")

echo "== Role models =="
for role_file in "$repo_root"/roles/*.toml; do
  role_name="$(basename "$role_file")"
  model="$(sed -n 's/^model = "\([^"]*\)"$/\1/p' "$role_file" | head -1)"
  if [[ -z "$model" ]]; then
    fail "$role_name has no model pin"
    continue
  fi
  pass "$role_name has model pin $model"
done

echo "== Prompt frontmatter =="
check_prompt_frontmatter

echo "== Skill frontmatter =="
check_skill_frontmatter

echo "== Relative references =="
while IFS= read -r ref_line; do
  [[ -n "$ref_line" ]] || continue
  file="${ref_line%%:*}"
  ref="${ref_line#*:}"
  target="$(python3 -c 'import os, sys; print(os.path.normpath(sys.argv[1]))' "$ref")"
  if [[ -e "$(cd "$(dirname "$file")" && pwd)/$target" ]]; then
    pass "$(basename "$file") reference exists: $ref"
  else
    fail "$file references missing path $ref"
  fi
done < <(perl -ne 'while(/`(\.\.\/\.\.\/[^`]+)`/g){ print "$ARGV:$1\n"; }' \
  "$repo_root"/prompts/*.md \
  "$repo_root"/skills/*/SKILL.md)

echo "== Upstream drift =="
if [[ -d "$upstream_codex" ]]; then
  check_upstream_feature_keys
  check_upstream_models
  check_upstream_native_commands
  check_upstream_builtin_roles
  check_upstream_native_tools
  check_upstream_triage_coverage
else
  warn "$upstream_codex does not exist; skipping upstream drift checks"
fi

echo "== Installed tree =="
if [[ -d "$codex_home" ]]; then
  compare_path AGENTS.md
  compare_path config.toml.from-repo.example config.toml.example
  compare_path playbooks
  compare_path prompts
  compare_path roles
  compare_path rules
  compare_path skills
else
  warn "$codex_home does not exist; skipping installed-tree comparison"
fi

echo "== Summary =="
if [[ "$failures" -gt 0 ]]; then
  printf 'FAIL %s checks failed; %s warnings\n' "$failures" "$warnings"
  exit 1
fi

printf 'PASS all checks succeeded; %s warnings\n' "$warnings"
