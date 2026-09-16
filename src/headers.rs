//! Canonical ORESoftware extension-header helpers.
//!
//! Authored contracts and emitted metadata should use lowercase names. Incoming
//! HTTP header names are matched case-insensitively, as required by HTTP.

/// Reserved prefix for ORESoftware extension headers.
pub const ORES_HEADER_PREFIX: &str = "x-ores-";
pub const X_ORES_REQUEST_ID: &str = "x-ores-request-id";
pub const X_ORES_CORRELATION_ID: &str = "x-ores-correlation-id";
pub const X_ORES_IDEMPOTENCY_KEY: &str = "x-ores-idempotency-key";
pub const X_ORES_TRACE_ID: &str = "x-ores-trace-id";

/// Returns a canonical lowercase `x-ores-*` header name when `name` is a valid
/// HTTP token inside the reserved namespace.
#[must_use]
pub fn canonical_ores_header_name(name: &str) -> Option<String> {
    if name.is_empty() || !name.is_ascii() || !name.bytes().all(is_http_token_byte) {
        return None;
    }

    let lowercase = name.to_ascii_lowercase();
    lowercase
        .starts_with(ORES_HEADER_PREFIX)
        .then_some(lowercase)
}

/// Returns whether `name` is a syntactically valid member of the reserved
/// `x-ores-*` namespace.
#[must_use]
pub fn is_ores_header(name: &str) -> bool {
    canonical_ores_header_name(name).is_some()
}

/// HTTP header-name comparison is ASCII case-insensitive.
#[must_use]
pub fn header_name_eq(left: &str, right: &str) -> bool {
    left.eq_ignore_ascii_case(right)
}

const fn is_http_token_byte(byte: u8) -> bool {
    matches!(
        byte,
        b'A'..=b'Z'
            | b'a'..=b'z'
            | b'0'..=b'9'
            | b'!'
            | b'#'
            | b'$'
            | b'%'
            | b'&'
            | b'\''
            | b'*'
            | b'+'
            | b'-'
            | b'.'
            | b'^'
            | b'_'
            | b'`'
            | b'|'
            | b'~'
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonicalizes_incoming_header_case() {
        assert_eq!(
            canonical_ores_header_name("X-ORES-Request-ID").as_deref(),
            Some(X_ORES_REQUEST_ID)
        );
    }

    #[test]
    fn rejects_non_ores_and_invalid_headers() {
        assert_eq!(canonical_ores_header_name("authorization"), None);
        assert_eq!(canonical_ores_header_name("x-ores bad"), None);
        assert_eq!(canonical_ores_header_name("x-örès-id"), None);
    }

    #[test]
    fn compares_names_case_insensitively() {
        assert!(header_name_eq("x-ores-trace-id", "X-ORES-TRACE-ID"));
    }
}
