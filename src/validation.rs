use core::fmt;

/// A stable, machine-readable validation issue suitable for SDKs and UI clients.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ValidationIssue {
    pub code: String,
    pub path: String,
    pub message: String,
}

impl ValidationIssue {
    pub fn new(
        code: impl Into<String>,
        path: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            path: path.into(),
            message: message.into(),
        }
    }
}

/// One or more deterministic validation failures.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ValidationErrors(Vec<ValidationIssue>);

impl ValidationErrors {
    pub fn new(first: ValidationIssue) -> Self {
        Self(vec![first])
    }

    #[must_use]
    pub fn from_vec(issues: Vec<ValidationIssue>) -> Option<Self> {
        (!issues.is_empty()).then_some(Self(issues))
    }

    pub fn push(&mut self, issue: ValidationIssue) {
        self.0.push(issue);
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[must_use]
    pub fn issues(&self) -> &[ValidationIssue] {
        &self.0
    }

    #[must_use]
    pub fn into_issues(self) -> Vec<ValidationIssue> {
        self.0
    }
}

impl fmt::Display for ValidationErrors {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "validation failed with {} issue(s)", self.len())
    }
}

impl std::error::Error for ValidationErrors {}

pub type ValidationResult<T> = Result<T, ValidationErrors>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_empty_constructor_preserves_issue() {
        let issue = ValidationIssue::new("required", "/customer/id", "customer id is required");
        let errors = ValidationErrors::new(issue.clone());
        assert_eq!(errors.issues(), &[issue]);
    }

    #[test]
    fn empty_vectors_do_not_create_error_sets() {
        assert_eq!(ValidationErrors::from_vec(Vec::new()), None);
    }
}
