use codex_login::CodexAuth;

pub fn host_owned_codex_apps_enabled_for_auth(
    apps_enabled: bool,
    auth: Option<&CodexAuth>,
) -> bool {
    host_owned_codex_apps_enabled_for_auth_state(
        apps_enabled,
        auth.is_some_and(CodexAuth::uses_codex_backend),
    )
}

pub fn host_owned_codex_apps_enabled_for_auth_state(
    apps_enabled: bool,
    uses_codex_backend_auth: bool,
) -> bool {
    apps_enabled && uses_codex_backend_auth
}
