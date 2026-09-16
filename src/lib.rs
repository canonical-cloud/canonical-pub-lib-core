#![forbid(unsafe_code)]
//! Publishable, client-safe shared primitives for Canonical Cloud.
//!
//! This crate is intentionally I/O-free and persistence-free. Database/ORM
//! concerns belong in `canonical-orm-core`; private implementation concerns
//! belong in `canonical-lib-code`.

pub mod headers;
pub mod ids;
pub mod pagination;
pub mod request;
pub mod validation;

pub use ids::{IdError, OpaqueId};
pub use pagination::{PageLimitError, PageRequest, DEFAULT_PAGE_LIMIT, MAX_PAGE_LIMIT};
pub use request::RequestMetadata;
pub use validation::{ValidationErrors, ValidationIssue, ValidationResult};

/// Version of the public Rust API exposed by this crate.
pub const PUBLIC_API_VERSION: &str = env!("CARGO_PKG_VERSION");
