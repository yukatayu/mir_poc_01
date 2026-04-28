#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

source "$REPO_ROOT/scripts/env/mirrorea_storage_env.sh" >/dev/null

echo "[detach_prepare] git status"
(cd "$REPO_ROOT" && git status --short)

echo "[detach_prepare] current branch"
(cd "$REPO_ROOT" && git branch --show-current)

echo "[detach_prepare] disk usage"
df -h

echo "[detach_prepare] mounts"
findmnt

echo "[detach_prepare] block devices"
lsblk -f

echo "[detach_prepare] repo usage"
du -sh "$REPO_ROOT" 2>/dev/null || true
du -sh "$REPO_ROOT/target" "$REPO_ROOT/.git" "$REPO_ROOT/.cargo" "$REPO_ROOT/.lake" 2>/dev/null || true

if [[ -d "$MIRROREA_WORKDIR" ]]; then
  echo "[detach_prepare] external workdir usage"
  du -sh "$MIRROREA_WORKDIR" 2>/dev/null || true
  du -sh "$MIRROREA_WORKDIR"/* 2>/dev/null || true
else
  echo "[detach_prepare] external workdir missing: $MIRROREA_WORKDIR"
fi

echo "[detach_prepare] llvm staging dirs"
ls -ld \
  "$MIRROREA_WORKDIR/llvm" \
  "$MIRROREA_LLVM_SRC_DIR" \
  "$MIRROREA_LLVM_BUILD_DIR" \
  "$MIRROREA_LLVM_INSTALL_DIR" 2>/dev/null || true
if [[ -d "$MIRROREA_WORKDIR/llvm" ]]; then
  llvm_root_owner="$(stat -c '%U:%G' "$MIRROREA_WORKDIR/llvm" 2>/dev/null || echo unknown)"
  llvm_root_writable=no
  if [[ -w "$MIRROREA_WORKDIR/llvm" ]]; then
    llvm_root_writable=yes
  fi
  echo "[detach_prepare] llvm root owner: $llvm_root_owner"
  echo "[detach_prepare] llvm root writable: $llvm_root_writable"
  if [[ "$llvm_root_writable" != yes ]]; then
    echo "[detach_prepare] note: llvm/build and llvm/install cleanup remains guarded until ownership is repaired or admin setup reruns"
  fi
  echo "[detach_prepare] llvm source checkout remains intentionally outside disposable cleanup: $MIRROREA_LLVM_SRC_DIR"
fi

echo "[detach_prepare] report/dashboard files still dirty"
(cd "$REPO_ROOT" && git status --short -- progress.md tasks.md samples_progress.md docs/reports README.md Documentation.md 2>/dev/null || true)

echo "[detach_prepare] disposable candidates:"
printf '  %s\n' \
  "$CARGO_TARGET_DIR" \
  "$MIRROREA_GENERATED_ARTIFACT_DIR" \
  "$MIRROREA_CARGO_REGISTRY_CACHE" \
  "$MIRROREA_LLVM_BUILD_DIR" \
  "$MIRROREA_LLVM_INSTALL_DIR" \
  "$MIRROREA_LEAN_CACHE_DIR" \
  "$MIRROREA_TEMP_DIR" \
  "$MIRROREA_LOG_DIR"

echo "No files deleted. Re-run cleanup script with explicit confirmation if needed."
