//! Secret field-name denylist (REQ-033).
//!
//! Hard-fail when any top-level or nested **field name** matches (case-insensitive).
//! Full secret-content scanning is out of v1 scope.

/// Documented denylist (case-insensitive match on field names only).
pub const SECRET_FIELD_DENYLIST: &[&str] = &[
    "password",
    "secret",
    "token",
    "api_key",
    "private_key",
    "access_key",
    "client_secret",
];

/// Return true if `name` matches the secret field denylist (case-insensitive).
pub fn field_name_is_denied(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    SECRET_FIELD_DENYLIST.iter().any(|d| *d == lower)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn denylist_is_case_insensitive() {
        assert!(field_name_is_denied("password"));
        assert!(field_name_is_denied("PASSWORD"));
        assert!(field_name_is_denied("Api_Key"));
        assert!(field_name_is_denied("CLIENT_SECRET"));
        assert!(!field_name_is_denied("name"));
        assert!(!field_name_is_denied("description"));
    }
}
