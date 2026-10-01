pub(crate) fn valid_identifier(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'_' | b'-')
        })
}

pub(crate) fn valid_subject_identifier(value: &str) -> bool {
    valid_identifier(value)
        && !value
            .bytes()
            .next()
            .is_some_and(|byte| matches!(byte, b'.' | b'_' | b'-'))
}

pub(crate) fn valid_semver(value: &str) -> bool {
    value.len() <= 64
        && value.split('.').count() == 3
        && value.split('.').all(|component| {
            !component.is_empty() && component.bytes().all(|byte| byte.is_ascii_digit())
        })
}

#[cfg(test)]
mod tests {
    use super::{valid_semver, valid_subject_identifier};

    #[test]
    fn identifiers_and_versions_have_explicit_ascii_bounds() {
        assert!(valid_subject_identifier("department-daily"));
        assert!(!valid_subject_identifier("-department"));
        assert!(!valid_subject_identifier(&"x".repeat(129)));
        assert!(valid_semver("1.2.3"));
        assert!(!valid_semver("v1"));
    }
}
