#!/usr/bin/env bash
set -euo pipefail

tmp_root="${TMPDIR:-/tmp}"
action="list"
confirm=0

usage() {
  echo "usage: $0 [--tmp-root DIR] [--list | --cleanup] [--confirm]" >&2
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --tmp-root)
      if [[ $# -lt 2 ]]; then
        usage
        exit 2
      fi
      tmp_root="$2"
      shift 2
      ;;
    --list)
      action="list"
      shift
      ;;
    --cleanup)
      action="cleanup"
      shift
      ;;
    --confirm)
      confirm=1
      shift
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      usage
      exit 2
      ;;
  esac
done

if [[ ! -d "$tmp_root" ]]; then
  echo "[tmp-artifacts] tmp root missing or not a directory: $tmp_root" >&2
  exit 2
fi

tmp_root_abs="$(cd "$tmp_root" && pwd -P)"
if [[ -z "$tmp_root_abs" || "$tmp_root_abs" == "/" ]]; then
  echo "[tmp-artifacts] refusing unsafe tmp root: $tmp_root" >&2
  exit 2
fi

declare -a candidates=()
while IFS= read -r -d '' candidate; do
  candidates+=("$candidate")
done < <(find "$tmp_root_abs" -mindepth 1 -maxdepth 1 -type d -name 'mirrorea-*' -print0 | sort -z)

total_kib=0
echo "tmp_root=$tmp_root_abs"
echo "candidate_glob=$tmp_root_abs/mirrorea-*"
for candidate in "${candidates[@]}"; do
  size_kib="$(du -sk "$candidate" 2>/dev/null | awk '{print $1}')"
  if [[ -z "$size_kib" ]]; then
    size_kib=0
  fi
  total_kib=$((total_kib + size_kib))
  echo "candidate=$candidate size_kib=$size_kib"
done
echo "candidate_count=${#candidates[@]}"
echo "total_kib=$total_kib"

if [[ "$action" == "list" ]]; then
  exit 0
fi

if [[ "$action" != "cleanup" ]]; then
  usage
  exit 2
fi

if [[ "$confirm" -ne 1 ]]; then
  echo "[tmp-artifacts] refusing cleanup without --confirm" >&2
  exit 2
fi

for candidate in "${candidates[@]}"; do
  basename="$(basename "$candidate")"
  case "$basename" in
    mirrorea-*) ;;
    *)
      echo "[tmp-artifacts] refusing unexpected candidate name: $candidate" >&2
      exit 2
      ;;
  esac

  candidate_abs="$(readlink -f "$candidate")"
  case "$candidate_abs" in
    "$tmp_root_abs"/mirrorea-*)
      rm -rf -- "$candidate_abs"
      echo "removed=$candidate_abs"
      ;;
    *)
      echo "[tmp-artifacts] refusing path outside tmp root: $candidate_abs" >&2
      exit 2
      ;;
  esac
done
