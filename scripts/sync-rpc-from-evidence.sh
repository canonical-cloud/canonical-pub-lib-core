#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat >&2 <<'EOF'
usage: sync-rpc-from-evidence.sh SOURCE_REPO CONTRACT_IR PARITY_REPORT AUDIENCE SCOPE

Public example:
  scripts/sync-rpc-from-evidence.sh ../canonical-api-server.rs /tmp/regular/contract-ir.json /tmp/regular/report.json public regular
EOF
  exit 64
}

[[ $# -eq 5 ]] || usage
source_repo="$1"
contract_ir="$2"
parity_report="$3"
audience="$4"
scope="$5"

case "$audience" in
  server|public) ;;
  *) echo "unsupported RPC audience: $audience" >&2; exit 64 ;;
esac
case "$scope" in
  regular|admin) ;;
  *) echo "unsupported RPC scope: $scope" >&2; exit 64 ;;
esac
if [[ "$scope" == "admin" && "$audience" != "server" ]]; then
  echo "admin RPC operations are server-only and may not be projected to $audience" >&2
  exit 64
fi
if [[ "$audience" == "server" ]]; then
  echo "this public client repository may only materialize the public audience" >&2
  exit 64
fi

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"
command -v ores-stack >/dev/null 2>&1 || { echo "ores-stack is required" >&2; exit 127; }
test -d "$source_repo/.git" || { echo "SOURCE_REPO must be a Git checkout: $source_repo" >&2; exit 66; }
for required in \
  "$source_repo/generated/rpc/server-operation-index.json" \
  "$source_repo/generated/rpc/server-route-map.json" \
  "$contract_ir" \
  "$parity_report"
do
  test -s "$required" || { echo "required RPC authority/evidence is missing or empty: $required" >&2; exit 66; }
done
if [[ -n "$(git -C "$source_repo" status --porcelain --untracked-files=normal)" ]]; then
  echo "SOURCE_REPO must be clean; generated clients bind to an exact committed source tree" >&2
  git -C "$source_repo" status --short >&2
  exit 65
fi
source_sha="$(git -C "$source_repo" rev-parse HEAD)"
echo "syncing public RPC projection from $source_sha ($audience/$scope)"
sync_args=(sync --root "$repo_root" --given "$source_repo" --audience "$audience" --scope "$scope" --contract-ir "$contract_ir" --parity-report "$parity_report")
ores-stack "${sync_args[@]}"
ores-stack "${sync_args[@]}" --check
