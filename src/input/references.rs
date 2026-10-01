use crate::diagnostics::diagnostic;
use crate::identifiers::valid_subject_identifier;
use crate::Diagnostic;

pub(crate) fn identifier(value: &str, path: &str, diagnostics: &mut Vec<Diagnostic>) {
    if !valid_subject_identifier(value) {
        diagnostics.push(diagnostic(
            "constillo.input.invalid-identifier",
            path,
            "identifier must start with a lowercase letter or digit and use bounded ASCII",
        ));
    }
}

pub(crate) fn opaque(value: &str, path: &str, diagnostics: &mut Vec<Diagnostic>) {
    let basic_valid = !value.is_empty()
        && value.len() <= 256
        && !value.bytes().any(|byte| byte.is_ascii_whitespace())
        && !value.contains('\\')
        && !value.starts_with('/')
        && !value.starts_with("./")
        && !value.starts_with("../");
    let namespaced = value.split_once(':').is_some_and(|(prefix, reference)| {
        valid_subject_identifier(prefix)
            && !reference.is_empty()
            && !matches!(
                prefix,
                "file" | "http" | "https" | "data" | "env" | "secret" | "credential"
            )
    });
    let lower = value.to_ascii_lowercase();
    let credential = [
        "password=",
        "passwd=",
        "token=",
        "api_key=",
        "apikey=",
        "bearer=",
    ]
    .iter()
    .any(|marker| lower.contains(marker));
    if !basic_valid || !namespaced || credential {
        diagnostics.push(diagnostic(
            "constillo.input.invalid-opaque-ref",
            path,
            "reference must be opaque, namespaced, bounded, and contain no locator or credential",
        ));
    }
}

pub(crate) fn sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

pub(crate) fn prefixed_sha256(value: &str) -> bool {
    value.strip_prefix("sha256:").is_some_and(sha256)
}

#[cfg(test)]
mod tests {
    use super::{opaque, prefixed_sha256};

    #[test]
    fn references_exclude_paths_urls_and_secret_locators() {
        for value in [
            "/local/report",
            "https://example.invalid",
            "secret:token=no",
        ] {
            let mut diagnostics = Vec::new();
            opaque(value, "$.ref", &mut diagnostics);
            assert!(!diagnostics.is_empty(), "accepted {value}");
        }
        assert!(prefixed_sha256(&format!("sha256:{}", "a".repeat(64))));
    }
}
