#!/usr/bin/env bash
set -euo pipefail

ensure_dirs=0
allow_unmounted=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --ensure-dirs)
      ensure_dirs=1
      shift
      ;;
    --allow-unmounted)
      allow_unmounted=1
      shift
      ;;
    *)
      echo "usage: $0 [--ensure-dirs] [--allow-unmounted]" >&2
      exit 2
      ;;
  esac
done

export MIRROREA_WORKDIR="${MIRROREA_WORKDIR:-/mnt/mirrorea-work}"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$MIRROREA_WORKDIR/cargo-target}"
export MIRROREA_GENERATED_ARTIFACT_DIR="${MIRROREA_GENERATED_ARTIFACT_DIR:-$MIRROREA_WORKDIR/generated-artifacts}"
export MIRROREA_CARGO_REGISTRY_CACHE="${MIRROREA_CARGO_REGISTRY_CACHE:-$MIRROREA_WORKDIR/cargo-registry-cache}"
export CARGO_HOME="${CARGO_HOME:-$MIRROREA_CARGO_REGISTRY_CACHE}"
export MIRROREA_LLVM_SRC_DIR="${MIRROREA_LLVM_SRC_DIR:-$MIRROREA_WORKDIR/llvm/src}"
export MIRROREA_LLVM_BUILD_DIR="${MIRROREA_LLVM_BUILD_DIR:-$MIRROREA_WORKDIR/llvm/build}"
export MIRROREA_LLVM_INSTALL_DIR="${MIRROREA_LLVM_INSTALL_DIR:-$MIRROREA_WORKDIR/llvm/install}"
export MIRROREA_LEAN_CACHE_DIR="${MIRROREA_LEAN_CACHE_DIR:-$MIRROREA_WORKDIR/lean-cache}"
export MIRROREA_TEMP_DIR="${MIRROREA_TEMP_DIR:-$MIRROREA_WORKDIR/temp}"
export MIRROREA_LOG_DIR="${MIRROREA_LOG_DIR:-$MIRROREA_WORKDIR/logs}"

mounted=no
if [[ -d "$MIRROREA_WORKDIR" ]] && findmnt -T "$MIRROREA_WORKDIR" >/dev/null 2>&1; then
  mounted=yes
fi

describe_path() {
  local prefix="$1"
  local path="$2"
  local status=missing
  local owner=missing
  local writable=no
  if [[ -e "$path" ]]; then
    status=present
    owner="$(stat -c '%U:%G' "$path" 2>/dev/null || echo unknown)"
    if [[ -w "$path" ]]; then
      writable=yes
    fi
  fi
  echo "${prefix}_PATH=$path"
  echo "${prefix}_STATUS=$status"
  echo "${prefix}_OWNER=$owner"
  echo "${prefix}_WRITABLE=$writable"
}

if [[ "$ensure_dirs" -eq 1 ]]; then
  if [[ "$mounted" == yes || "$allow_unmounted" -eq 1 ]]; then
    mkdir -p \
      "$CARGO_TARGET_DIR" \
      "$MIRROREA_GENERATED_ARTIFACT_DIR" \
      "$MIRROREA_CARGO_REGISTRY_CACHE" \
      "$MIRROREA_LLVM_SRC_DIR" \
      "$MIRROREA_LLVM_BUILD_DIR" \
      "$MIRROREA_LLVM_INSTALL_DIR" \
      "$MIRROREA_LEAN_CACHE_DIR" \
      "$MIRROREA_TEMP_DIR" \
      "$MIRROREA_LOG_DIR"
  else
    echo "refusing to create heavy workdir paths under an unmounted default root: $MIRROREA_WORKDIR" >&2
    echo "mount external storage first or re-run with --allow-unmounted if you really intend to use root disk" >&2
    exit 2
  fi
fi

if [[ -d "$MIRROREA_WORKDIR/llvm" ]] && [[ ! -w "$MIRROREA_WORKDIR/llvm" ]]; then
  echo "warning: llvm staging parent is not writable by the current user: $MIRROREA_WORKDIR/llvm" >&2
  echo "cleanup of llvm/build or llvm/install can make later recreation fail until ownership is repaired via the explicit setup path" >&2
fi

echo "MIRROREA_WORKDIR=$MIRROREA_WORKDIR"
echo "CARGO_TARGET_DIR=$CARGO_TARGET_DIR"
echo "MIRROREA_GENERATED_ARTIFACT_DIR=$MIRROREA_GENERATED_ARTIFACT_DIR"
echo "MIRROREA_CARGO_REGISTRY_CACHE=$MIRROREA_CARGO_REGISTRY_CACHE"
echo "CARGO_HOME=$CARGO_HOME"
echo "MIRROREA_LLVM_SRC_DIR=$MIRROREA_LLVM_SRC_DIR"
echo "MIRROREA_LLVM_BUILD_DIR=$MIRROREA_LLVM_BUILD_DIR"
echo "MIRROREA_LLVM_INSTALL_DIR=$MIRROREA_LLVM_INSTALL_DIR"
echo "MIRROREA_LEAN_CACHE_DIR=$MIRROREA_LEAN_CACHE_DIR"
echo "MIRROREA_TEMP_DIR=$MIRROREA_TEMP_DIR"
echo "MIRROREA_LOG_DIR=$MIRROREA_LOG_DIR"
echo "MIRROREA_WORKDIR_MOUNTED=$mounted"
describe_path "MIRROREA_LLVM_ROOT" "$MIRROREA_WORKDIR/llvm"
describe_path "MIRROREA_LLVM_SRC" "$MIRROREA_LLVM_SRC_DIR"
describe_path "MIRROREA_LLVM_BUILD" "$MIRROREA_LLVM_BUILD_DIR"
describe_path "MIRROREA_LLVM_INSTALL" "$MIRROREA_LLVM_INSTALL_DIR"
