# canonical-pub-lib-core

Client-safe, publishable Rust core for Canonical Cloud.

This repository contains shared primitives and deterministic validation that may be linked into customer-facing clients, CLIs, SDKs, workers, WASM, and other publishable artifacts. It is intentionally small: public contracts live in `canonical-interfaces`; persistence lives in `canonical-orm-core`; private implementation code lives in `canonical-lib-code`.

## Repository split

| Repository | Responsibility | Publish to clients? |
| --- | --- | --- |
| `canonical-interfaces` | Cross-language TypeSpec / JSON Schema / wire-contract authority | generated artifacts: yes |
| `canonical-pub-lib-core` | Hand-written client-safe Rust primitives and deterministic helpers | **yes** |
| `canonical-lib-code` | Internal shared implementation code | no |
| `canonical-orm-core` | Diesel / SeaORM / SQL / persistence capability boundary | no |

Dependency direction is one-way:

```text
canonical-interfaces (contract authority)
          |
          v
client/generated bindings + canonical-pub-lib-core
          |
          +----------------------+----------------------+
          v                      v                      v
 canonical-lib-code       canonical-orm-core       client SDKs/apps
                                  |
                                  v
                         API/server persistence
```

`canonical-pub-lib-core` must never depend on `canonical-orm-core` or a database driver.

## What belongs here

- public wire/domain primitives that are not generated from TypeSpec/JSON Schema
- portable opaque IDs
- cursor pagination primitives
- transport-neutral request metadata
- canonical lowercase `x-ores-*` header helpers
- deterministic validation errors and helpers with no network/database I/O
- small compatibility adapters around generated public contracts

## What does not belong here

- Diesel, SeaORM, SQLx, Postgres clients, pools, migrations, SQL, or persistence entities
- secrets, credentials, decrypted environment files, or private connection details
- server-only authorization/admission policy
- admin-only business logic
- generated contract definitions that belong in `canonical-interfaces`

CI runs `scripts/check-public-boundary.sh` so ORM/database dependencies and persistence directories fail the build.

## Public primitives

```rust
use canonical_pub_lib_core::{
    headers::{canonical_ores_header_name, X_ORES_REQUEST_ID},
    OpaqueId, PageRequest, RequestMetadata,
};

let request_id = OpaqueId::new("audit:customer-42/request-9")?;
let metadata = RequestMetadata {
    request_id: Some(request_id),
    ..RequestMetadata::default()
};

assert!(!metadata.is_empty());
assert_eq!(
    canonical_ores_header_name("X-ORES-Request-ID").as_deref(),
    Some(X_ORES_REQUEST_ID),
);

let page = PageRequest::new(None, 100)?;
assert_eq!(page.limit, 100);
# Ok::<(), Box<dyn std::error::Error>>(())
```

Incoming HTTP header names are matched case-insensitively, but authored contracts, generated docs, fixtures, logs, and emitted names should use canonical lowercase `x-ores-*` names.

## Validation boundary

Validated types preserve their invariants during Serde deserialization. A malformed JSON payload cannot create an invalid `OpaqueId`, an out-of-range `PageRequest`, or an empty `ValidationErrors` set by bypassing constructors.

## Publishing

The crate has no git-only or private dependencies. `.zpkg.toml` defines the zed-pkg publication surface and excludes environment material, build outputs, and local vendor directories.

Before publishing:

```bash
bash scripts/check-public-boundary.sh
cargo fmt --all -- --check
cargo check --all-targets --all-features
cargo test --all-targets --all-features
cargo clippy --all-targets --all-features
```
