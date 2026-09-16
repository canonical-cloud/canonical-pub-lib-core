use core::fmt;

pub const DEFAULT_PAGE_LIMIT: u16 = 50;
pub const MAX_PAGE_LIMIT: u16 = 500;

/// Cursor-based pagination request shared by client-facing APIs.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PageRequest {
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    pub cursor: Option<String>,
    pub limit: u16,
}

impl PageRequest {
    pub fn new(cursor: Option<String>, limit: u16) -> Result<Self, PageLimitError> {
        validate_limit(limit)?;
        Ok(Self { cursor, limit })
    }

    #[must_use]
    pub fn with_default_limit(cursor: Option<String>) -> Self {
        Self {
            cursor,
            limit: DEFAULT_PAGE_LIMIT,
        }
    }
}

impl Default for PageRequest {
    fn default() -> Self {
        Self::with_default_limit(None)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for PageRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct RawPageRequest {
            #[serde(default)]
            cursor: Option<String>,
            #[serde(default = "default_page_limit")]
            limit: u16,
        }

        let raw = <RawPageRequest as serde::Deserialize>::deserialize(deserializer)?;
        Self::new(raw.cursor, raw.limit).map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "serde")]
const fn default_page_limit() -> u16 {
    DEFAULT_PAGE_LIMIT
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PageLimitError {
    Zero,
    AboveMaximum { requested: u16, maximum: u16 },
}

impl fmt::Display for PageLimitError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero => formatter.write_str("page limit must be greater than zero"),
            Self::AboveMaximum { requested, maximum } => {
                write!(formatter, "page limit {requested} exceeds maximum {maximum}")
            }
        }
    }
}

impl std::error::Error for PageLimitError {}

fn validate_limit(limit: u16) -> Result<(), PageLimitError> {
    if limit == 0 {
        return Err(PageLimitError::Zero);
    }
    if limit > MAX_PAGE_LIMIT {
        return Err(PageLimitError::AboveMaximum {
            requested: limit,
            maximum: MAX_PAGE_LIMIT,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_bounded() {
        let request = PageRequest::default();
        assert_eq!(request.limit, DEFAULT_PAGE_LIMIT);
        assert!(request.limit <= MAX_PAGE_LIMIT);
    }

    #[test]
    fn rejects_invalid_limits() {
        assert_eq!(PageRequest::new(None, 0).unwrap_err(), PageLimitError::Zero);
        assert_eq!(
            PageRequest::new(None, MAX_PAGE_LIMIT + 1).unwrap_err(),
            PageLimitError::AboveMaximum {
                requested: MAX_PAGE_LIMIT + 1,
                maximum: MAX_PAGE_LIMIT,
            }
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn deserialization_preserves_page_bounds() {
        assert!(serde_json::from_str::<PageRequest>("{\"limit\":0}").is_err());
        assert!(serde_json::from_str::<PageRequest>("{\"limit\":501}").is_err());
        let defaulted = serde_json::from_str::<PageRequest>("{}").expect("default page request");
        assert_eq!(defaulted.limit, DEFAULT_PAGE_LIMIT);
    }
}
