use gethostname::gethostname;

pub(super) const REMOTE_CONTROL_SERVER_NAME_ENV_VAR: &str = "CODEX_REMOTE_CONTROL_SERVER_NAME";

pub(super) fn remote_control_server_name() -> String {
    resolve_remote_control_server_name(
        std::env::var(REMOTE_CONTROL_SERVER_NAME_ENV_VAR)
            .ok()
            .as_deref(),
        &gethostname().to_string_lossy(),
    )
}

fn resolve_remote_control_server_name(override_name: Option<&str>, host_name: &str) -> String {
    override_name
        .and_then(non_empty_trimmed)
        .unwrap_or_else(|| non_empty_trimmed(host_name).unwrap_or_default())
}

fn non_empty_trimmed(value: &str) -> Option<String> {
    let value = value.trim();
    (!value.is_empty()).then(|| value.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_remote_control_server_name_uses_non_empty_override() {
        assert_eq!(
            resolve_remote_control_server_name(Some(" [Quotio]mengmotaMac "), "mengmotaMac"),
            "[Quotio]mengmotaMac"
        );
    }

    #[test]
    fn resolve_remote_control_server_name_falls_back_to_host_name() {
        let no_override = None;
        assert_eq!(
            resolve_remote_control_server_name(Some("  "), " mengmotaMac "),
            "mengmotaMac"
        );
        assert_eq!(
            resolve_remote_control_server_name(no_override, " mengmotaMac "),
            "mengmotaMac"
        );
    }
}
