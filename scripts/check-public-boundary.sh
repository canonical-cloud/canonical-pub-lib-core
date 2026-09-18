#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

forbidden_dependency='^[[:space:]]*(diesel|sea-orm|sea_orm|sqlx|tokio-postgres|postgres|deadpool-postgres|deadpool|bb8)[[:space:]]*='
if grep -Eiq "$forbidden_dependency" Cargo.toml; then
  echo "error: ORM/database dependency found in public client crate" >&2
  exit 1
fi

if grep -REn --include='*.rs' '(diesel::|sea_orm::|sqlx::|tokio_postgres::)' src tests 2>/dev/null; then
  echo "error: ORM/database code reference found in public client source" >&2
  exit 1
fi

for forbidden_dir in migrations migration schema sql; do
  if find . -path './target' -prune -o -type d -name "$forbidden_dir" -print | grep -q .; then
    echo "error: persistence directory '$forbidden_dir' is not allowed here" >&2
    exit 1
  fi
done

for forbidden_path in .env env/dec env/decrypted; do
  if [[ -e "$forbidden_path" ]]; then
    echo "error: private environment material '$forbidden_path' is not publishable" >&2
    exit 1
  fi
done

manifest="generated/rpc/regular/manifest.json"
source_lock=".route-map-sources.lock"
test -s "$manifest" || { echo "error: missing public RPC manifest" >&2; exit 1; }
test -s "$source_lock" || { echo "error: missing public RPC source lock" >&2; exit 1; }
test ! -e generated/rpc/admin || { echo "error: admin RPC evidence is forbidden in public client" >&2; exit 1; }

node - "$manifest" "$source_lock" <<'NODE'
const fs = require('node:fs');
const [manifestPath, lockPath] = process.argv.slice(2);
const manifest = JSON.parse(fs.readFileSync(manifestPath, 'utf8'));
const lockText = fs.readFileSync(lockPath, 'utf8');
const match = /^CANONICAL_API_SERVER_SHA=([0-9a-f]{40})$/m.exec(lockText);
if (!match) throw new Error(`${lockPath}: expected one immutable CANONICAL_API_SERVER_SHA`);
const lockedSha = match[1];
const expectedOperations = [
  'canonical_cloud.user.find_user_by_id',
  'canonical_cloud.user.find_users',
  'canonical_cloud.version.get_version',
].sort();
const actualOperations = [...(manifest.operations ?? [])].sort();
if (new Set(actualOperations).size !== actualOperations.length) {
  throw new Error(`${manifestPath}: duplicate operation key`);
}
if (JSON.stringify(actualOperations) !== JSON.stringify(expectedOperations)) {
  throw new Error(`${manifestPath}: public operation inventory drift; expected ${JSON.stringify(expectedOperations)}, got ${JSON.stringify(actualOperations)}`);
}
if (manifest.schema_version !== 2) throw new Error(`${manifestPath}: unsupported schema_version ${manifest.schema_version}`);
if (manifest.generated_by !== 'ores-stack sync') throw new Error(`${manifestPath}: unexpected generator ${manifest.generated_by}`);
if (manifest.source_role !== 'api-server-handlers.rs') throw new Error(`${manifestPath}: source_role must be api-server-handlers.rs`);
if (manifest.audience !== 'public') throw new Error(`${manifestPath}: audience must be public`);
if (manifest.scope !== 'regular') throw new Error(`${manifestPath}: scope must be regular`);
if (manifest.http_endpoint !== '/v1/rpc') throw new Error(`${manifestPath}: canonical RPC endpoint must be /v1/rpc`);
if (!/^[0-9a-f]{40}$/.test(manifest.source_commit_sha ?? '')) throw new Error(`${manifestPath}: source_commit_sha is not immutable`);
if (manifest.source_commit_sha !== lockedSha) throw new Error(`${manifestPath}: source_commit_sha disagrees with ${lockPath}`);
if (!String(manifest.source_repository ?? '').includes('canonical-cloud/canonical-api-server.rs')) {
  throw new Error(`${manifestPath}: unexpected source_repository ${manifest.source_repository}`);
}
if (!/^[0-9a-f]{64}$/.test(manifest.source_route_map_sha256 ?? '')) throw new Error(`${manifestPath}: missing source route-map digest`);
if (!/^[0-9a-f]{64}$/.test(manifest.source_operation_index_sha256 ?? '')) throw new Error(`${manifestPath}: missing source operation-index digest`);
const expectedLanguages = ['dart', 'gleam', 'go', 'rust', 'typescript'];
const actualLanguages = [...(manifest.languages ?? [])].sort();
if (JSON.stringify(actualLanguages) !== JSON.stringify(expectedLanguages)) {
  throw new Error(`${manifestPath}: five-language inventory drift`);
}
NODE

if grep -R -Fq 'canonical_cloud.admin.' generated/rpc src/langs 2>/dev/null; then
  echo "error: admin RPC key leaked into public client" >&2
  exit 1
fi

required_rpc_files=(
  src/langs/rust/generated/user/find-users.rs
  src/langs/rust/generated/user/find-user-by-id.rs
  src/langs/rust/generated/version/get-version.rs
  src/langs/golang/generated/user/find-users.go
  src/langs/golang/generated/user/find-user-by-id.go
  src/langs/golang/generated/version/get-version.go
  src/langs/dart/generated/user/find-users.dart
  src/langs/dart/generated/user/find-user-by-id.dart
  src/langs/dart/generated/version/get-version.dart
  src/langs/typescript/generated/user/find-users.ts
  src/langs/typescript/generated/user/find-user-by-id.ts
  src/langs/typescript/generated/version/get-version.ts
  src/langs/gleam/generated/user/find_users.gleam
  src/langs/gleam/generated/user/find_user_by_id.gleam
  src/langs/gleam/generated/version/get_version.gleam
)
for rpc_file in "${required_rpc_files[@]}"; do
  test -s "$rpc_file" || {
    echo "error: missing public typed RPC operation $rpc_file" >&2
    exit 1
  }
done

for root in rust golang dart typescript gleam; do
  test -s "src/langs/$root/generated/README.md" || { echo "error: missing generated README for $root" >&2; exit 1; }
  test -s "src/langs/$root/generated/AGENTS.md" || { echo "error: missing generated AGENTS.md for $root" >&2; exit 1; }
done

for legacy in rust go dart typescript gleam; do
  test ! -e "generated/rpc/$legacy" || {
    echo "error: legacy generated/rpc/$legacy language tree remains" >&2
    exit 1
  }
done

if grep -REn \
  --include='*.ts' --include='*.dart' --include='*.go' --include='*.gleam' \
  'Promise<unknown>|RpcCallArgs|Future<Object\?>|dynamic\.Dynamic|out any' \
  src/langs/*/generated
then
  echo "error: stale dynamic RPC operation surface detected in public client" >&2
  exit 1
fi

echo "public-boundary: manifest, source lock, operation inventory, and five-language surface agree"
