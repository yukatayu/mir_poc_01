#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

confirm=0
list_only=0
allow_unmounted=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --confirm)
      confirm=1
      shift
      ;;
    --list)
      list_only=1
      shift
      ;;
    --allow-unmounted)
      allow_unmounted=1
      shift
      ;;
    *)
      echo "usage: $0 [--list] [--confirm] [--allow-unmounted]" >&2
      exit 2
      ;;
  esac
done

source "$REPO_ROOT/scripts/env/mirrorea_storage_env.sh" >/dev/null

mounted=no
if [[ -d "$MIRROREA_WORKDIR" ]] && findmnt -T "$MIRROREA_WORKDIR" >/dev/null 2>&1; then
  mounted=yes
fi

declare -a candidates=(
  "$CARGO_TARGET_DIR"
  "$MIRROREA_GENERATED_ARTIFACT_DIR"
  "$MIRROREA_CARGO_REGISTRY_CACHE"
  "$MIRROREA_LLVM_BUILD_DIR"
  "$MIRROREA_LLVM_INSTALL_DIR"
  "$MIRROREA_LEAN_CACHE_DIR"
  "$MIRROREA_TEMP_DIR"
  "$MIRROREA_LOG_DIR"
)

echo "[cleanup] workdir: $MIRROREA_WORKDIR"
echo "[cleanup] mounted: $mounted"
echo "[cleanup] candidates:"
printf '  %s\n' "${candidates[@]}"
llvm_root_owner=missing
llvm_root_writable=no
if [[ -d "$MIRROREA_WORKDIR/llvm" ]]; then
  llvm_root_owner="$(stat -c '%U:%G' "$MIRROREA_WORKDIR/llvm" 2>/dev/null || echo unknown)"
  if [[ -w "$MIRROREA_WORKDIR/llvm" ]]; then
    llvm_root_writable=yes
  fi
fi
echo "[cleanup] llvm root owner: $llvm_root_owner"
echo "[cleanup] llvm root writable: $llvm_root_writable"
echo "[cleanup] llvm source checkout intentionally excluded: $MIRROREA_LLVM_SRC_DIR"

if [[ "$list_only" -eq 1 ]]; then
  exit 0
fi

if [[ "$confirm" -ne 1 ]]; then
  echo "[cleanup] refusing to delete without --confirm" >&2
  exit 2
fi

if [[ "$mounted" != yes && "$allow_unmounted" -ne 1 ]]; then
  echo "[cleanup] refusing to delete from an unmounted workdir: $MIRROREA_WORKDIR" >&2
  exit 2
fi

if [[ ! -d "$MIRROREA_WORKDIR" ]]; then
  echo "[cleanup] workdir missing: $MIRROREA_WORKDIR" >&2
  exit 2
fi

workdir_abs="$(cd "$MIRROREA_WORKDIR" && pwd -P)"
if [[ -z "$workdir_abs" || "$workdir_abs" == "/" ]]; then
  echo "[cleanup] refusing unsafe workdir: $MIRROREA_WORKDIR" >&2
  exit 2
fi

for candidate in "${candidates[@]}"; do
  if [[ ! -e "$candidate" ]]; then
    echo "[cleanup] skip missing: $candidate"
    continue
  fi
  if [[ ("$candidate" == "$MIRROREA_LLVM_BUILD_DIR" || "$candidate" == "$MIRROREA_LLVM_INSTALL_DIR") && "$llvm_root_writable" != yes ]]; then
    echo "[cleanup] refusing llvm build/install cleanup while parent is not writable: $MIRROREA_WORKDIR/llvm ($llvm_root_owner)" >&2
    echo "[cleanup] use the explicit root setup path or repair ownership before deleting staged llvm build/install directories" >&2
    exit 2
  fi
  candidate_abs="$(readlink -f "$candidate")"
  case "$candidate_abs" in
    "$workdir_abs"/*)
      du -sh "$candidate" 2>/dev/null || true
      rm -rf -- "$candidate"
      echo "[cleanup] removed: $candidate_abs"
      ;;
    *)
      echo "[cleanup] refusing path outside workdir: $candidate_abs" >&2
      exit 2
      ;;
  esac
done
