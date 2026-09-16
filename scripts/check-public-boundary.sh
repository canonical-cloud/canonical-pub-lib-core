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

echo "public-boundary: ok"
