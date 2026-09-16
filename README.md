# canonical-pub-lib-core

Client-safe, publishable Rust core for Canonical Cloud.

This repository contains shared domain primitives and deterministic validation that may be linked into customer-facing clients, CLIs, SDKs, workers, WASM, and other publishable artifacts.

## Boundary

Allowed here:

- public wire/domain primitives
- identifiers, pagination, and request metadata
- canonical `x-ores-*` header helpers
- deterministic validation with no I/O
- re-exports/adapters for public interface contracts

Not allowed here:

- Diesel, SeaORM, SQLx, database pools, migrations, or SQL
- persistence entities or repository implementations
- secrets, credentials, private infrastructure configuration
- server-only authorization/admission policy
- admin-only domain logic

ORM and persistence code belongs in [`canonical-orm-core`](https://github.com/canonical-cloud/canonical-orm-core). Internal implementation code belongs in `canonical-lib-code` (renamed from `canonical-lib`). Machine-readable cross-language contracts remain authoritative in [`canonical-interfaces`](https://github.com/canonical-cloud/canonical-interfaces).

## Design rule

`canonical-pub-lib-core` may depend on public contract crates, but `canonical-orm-core` and private server crates may depend on `canonical-pub-lib-core`, never the reverse.
