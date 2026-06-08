#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

require_pattern() {
  local file="$1"
  local pattern="$2"
  local label="$3"

  if ! grep -Eq "$pattern" "$repo_root/$file"; then
    printf 'missing seam: %s\nfile: %s\npattern: %s\n' "$label" "$file" "$pattern" >&2
    exit 1
  fi
}

require_pattern \
  "codex-mcp/src/mcp/codex_apps_gate.rs" \
  "pub fn host_owned_codex_apps_enabled_for_auth_state" \
  "canonical Codex Apps auth-state gate"

require_pattern \
  "codex-mcp/src/mcp/codex_apps_gate.rs" \
  "pub fn host_owned_codex_apps_enabled_for_auth\\(" \
  "Codex Apps runtime auth helper"

require_pattern \
  "core/src/session/session.rs" \
  "apps_enabled_for_auth\\(auth\\.as_ref\\(\\)\\.is_some_and\\(\\|auth\\| auth\\.uses_codex_backend\\(\\)\\)\\)" \
  "session MCP startup keeps apps tied to runtime ChatGPT auth"

require_pattern \
  "core/src/session/turn_context.rs" \
  "AuthManager::current_auth_uses_codex_backend" \
  "turn-time tool exposure keeps apps tied to runtime auth manager"

require_pattern \
  "app-server-transport/src/transport/remote_control/auth.rs" \
  "remote control requires ChatGPT authentication; API key auth is not supported" \
  "remote control still rejects API-key-only auth"

printf 'remote apps/provider seams OK\n'
