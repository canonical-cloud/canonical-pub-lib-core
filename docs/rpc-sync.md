# Public generated RPC clients

This repository is the public/browser target for Canonical Cloud regular RPC operations. `handlers.rs` in `canonical-api-server.rs` remains authoritative.

```sh
ores-stack sync --given ../canonical-api-server.rs --audience public --scope regular
ores-stack sync --given ../canonical-api-server.rs --audience public --scope regular --check
```

Generation is fail-closed: an operation must declare the browser audience in authoritative handler metadata. Admin scope is forbidden here; `generated/rpc/admin` must not exist.
