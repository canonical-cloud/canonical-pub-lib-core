use crate::OpaqueId;

/// Transport metadata that can be shared by browser, mobile, CLI, worker, and
/// server callers without depending on an HTTP implementation crate.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct RequestMetadata {
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub request_id: Option<OpaqueId>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub correlation_id: Option<OpaqueId>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub idempotency_key: Option<OpaqueId>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub trace_id: Option<OpaqueId>,
}

impl RequestMetadata {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.request_id.is_none()
            && self.correlation_id.is_none()
            && self.idempotency_key.is_none()
            && self.trace_id.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_metadata_is_empty() {
        assert!(RequestMetadata::default().is_empty());
    }
}
