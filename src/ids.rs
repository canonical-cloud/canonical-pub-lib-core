use core::fmt;

/// Maximum encoded length accepted for portable opaque identifiers.
pub const MAX_OPAQUE_ID_BYTES: usize = 256;

/// A transport-safe identifier whose internal format is deliberately opaque.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OpaqueId(String);

impl OpaqueId {
    /// Validates and constructs an opaque identifier.
    ///
    /// The value may use any non-control UTF-8 characters, but may not be empty,
    /// exceed 256 encoded bytes, or contain leading/trailing whitespace.
    pub fn new(value: impl Into<String>) -> Result<Self, IdError> {
        let value = value.into();
        validate_id(&value)?;
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for OpaqueId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for OpaqueId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl TryFrom<String> for OpaqueId {
    type Error = IdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for OpaqueId {
    type Error = IdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for OpaqueId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for OpaqueId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

/// Deterministic opaque-ID validation failures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IdError {
    Empty,
    TooLong,
    BoundaryWhitespace,
    ControlCharacter,
}

impl fmt::Display for IdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Empty => "identifier must not be empty",
            Self::TooLong => "identifier exceeds maximum encoded length",
            Self::BoundaryWhitespace => "identifier must not have leading or trailing whitespace",
            Self::ControlCharacter => "identifier must not contain control characters",
        })
    }
}

impl std::error::Error for IdError {}

fn validate_id(value: &str) -> Result<(), IdError> {
    if value.is_empty() {
        return Err(IdError::Empty);
    }
    if value.len() > MAX_OPAQUE_ID_BYTES {
        return Err(IdError::TooLong);
    }
    if value.trim() != value {
        return Err(IdError::BoundaryWhitespace);
    }
    if value.chars().any(char::is_control) {
        return Err(IdError::ControlCharacter);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_uuid_ulid_and_namespaced_ids() {
        for value in [
            "018f0f44-4e67-7abc-8def-0123456789ab",
            "01J8Z6X4T7B9Y6D4V0W3J8QF2A",
            "audit:customer-123/evidence-9",
        ] {
            assert!(OpaqueId::new(value).is_ok());
        }
    }

    #[test]
    fn rejects_boundary_whitespace_and_controls() {
        assert_eq!(
            OpaqueId::new(" customer").unwrap_err(),
            IdError::BoundaryWhitespace
        );
        assert_eq!(
            OpaqueId::new("customer\n9").unwrap_err(),
            IdError::ControlCharacter
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_representation_is_a_plain_string_and_preserves_validation() {
        let id = OpaqueId::new("evidence-42").expect("valid fixture");
        assert_eq!(
            serde_json::to_string(&id).expect("serialize"),
            "\"evidence-42\""
        );
        assert!(serde_json::from_str::<OpaqueId>("\" evidence-42\"").is_err());
    }
}
