#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

device="/dev/vdb"
mountpoint="/mnt/mirrorea-work"
label="mirrorea-work"
force=0
plan_only=0

default_owner="$(stat -c %U "$REPO_ROOT" 2>/dev/null || true)"
default_group="$(stat -c %G "$REPO_ROOT" 2>/dev/null || true)"
owner_name="${default_owner:-${SUDO_USER:-}}"
group_name="${default_group:-${SUDO_GID:-}}"

usage() {
  cat <<'EOF'
usage: setup_mirrorea_workdisk_root.sh [options]

options:
  --device PATH        block device to initialize (default: /dev/vdb)
  --mountpoint PATH    mountpoint to create/use (default: /mnt/mirrorea-work)
  --label LABEL        ext4 label (default: mirrorea-work)
  --owner USER         owner for created workdir tree (default: repo owner)
  --group GROUP        group for created workdir tree (default: repo group)
  --force              allow existing signatures/partitions and fstab conflicts
  --plan               print the intended plan and current device state without modifying anything
  --help               show this help
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --device)
      device="$2"
      shift 2
      ;;
    --mountpoint)
      mountpoint="$2"
      shift 2
      ;;
    --label)
      label="$2"
      shift 2
      ;;
    --owner)
      owner_name="$2"
      shift 2
      ;;
    --group)
      group_name="$2"
      shift 2
      ;;
    --force)
      force=1
      shift
      ;;
    --plan)
      plan_only=1
      shift
      ;;
    --help)
      usage
      exit 0
      ;;
    *)
      usage >&2
      exit 2
      ;;
  esac
done

partition_path() {
  local base="$1"
  if [[ "$base" =~ [0-9]$ ]]; then
    printf '%sp1\n' "$base"
  else
    printf '%s1\n' "$base"
  fi
}

print_plan() {
  echo "[setup] repo root: $REPO_ROOT"
  echo "[setup] device: $device"
  echo "[setup] mountpoint: $mountpoint"
  echo "[setup] label: $label"
  echo "[setup] owner: ${owner_name:-UNSET}"
  echo "[setup] group: ${group_name:-UNSET}"
  echo "[setup] target partition: $(partition_path "$device")"
  echo "[setup] intended layout:"
  printf '  %s\n' \
    "$mountpoint/cargo-target" \
    "$mountpoint/cargo-registry-cache" \
    "$mountpoint/generated-artifacts" \
    "$mountpoint/llvm/src" \
    "$mountpoint/llvm/build" \
    "$mountpoint/llvm/install" \
    "$mountpoint/lean-cache" \
    "$mountpoint/temp" \
    "$mountpoint/logs"
  echo "[setup] current device view:"
  lsblk -o NAME,PATH,SIZE,TYPE,FSTYPE,LABEL,UUID,MOUNTPOINT "$device" || true
  echo "[setup] current signatures:"
  if [[ $EUID -eq 0 ]]; then
    wipefs -n "$device" || true
  else
    echo "  root required to probe signatures with wipefs"
  fi
}

if [[ "$plan_only" -eq 1 ]]; then
  print_plan
  exit 0
fi

if [[ $EUID -ne 0 ]]; then
  echo "run as root or via sudo. use --plan for a non-destructive preview." >&2
  exit 2
fi

if [[ -z "${owner_name:-}" || -z "${group_name:-}" ]]; then
  echo "owner/group could not be inferred; pass --owner and --group explicitly." >&2
  exit 2
fi

if [[ ! -b "$device" ]]; then
  echo "not a block device: $device" >&2
  exit 2
fi

device_type="$(lsblk -dn -o TYPE "$device" | tr -d '[:space:]')"
if [[ "$device_type" != "disk" ]]; then
  echo "refusing non-disk device: $device ($device_type)" >&2
  exit 2
fi

if findmnt -rnS "$device" >/dev/null 2>&1; then
  echo "device is already mounted somewhere: $device" >&2
  exit 2
fi

child_count="$(lsblk -nr -o NAME "$device" | wc -l | tr -d '[:space:]')"
if [[ "$child_count" != "1" && "$force" -ne 1 ]]; then
  echo "device already has child partitions; refusing without --force: $device" >&2
  lsblk -nr -o NAME,SIZE,TYPE "$device" >&2
  exit 2
fi

signature_output="$(wipefs -n "$device" 2>/dev/null || true)"
if [[ -n "$signature_output" && "$force" -ne 1 ]]; then
  echo "device already has signatures; refusing without --force: $device" >&2
  echo "$signature_output" >&2
  exit 2
fi

if grep -Eq "^[^#].*[[:space:]]$mountpoint[[:space:]]" /etc/fstab && [[ "$force" -ne 1 ]]; then
  echo "fstab already contains mountpoint $mountpoint; refusing without --force" >&2
  exit 2
fi

if [[ -e "$mountpoint" ]] && findmnt -rnT "$mountpoint" >/dev/null 2>&1; then
  echo "mountpoint is already mounted: $mountpoint" >&2
  exit 2
fi

parted -s "$device" mklabel gpt
parted -s -a optimal "$device" mkpart primary ext4 1MiB 100%
udevadm settle

part="$(partition_path "$device")"
if [[ ! -b "$part" ]]; then
  echo "expected partition not found after partitioning: $part" >&2
  exit 2
fi

mkfs.ext4 -F -L "$label" "$part"

mkdir -p "$mountpoint"

uuid="$(blkid -s UUID -o value "$part")"
if [[ -z "$uuid" ]]; then
  echo "failed to read UUID from $part" >&2
  exit 2
fi

fstab_backup="/etc/fstab.bak.$(date +%Y%m%d%H%M%S)"
cp /etc/fstab "$fstab_backup"
echo "[setup] backed up /etc/fstab -> $fstab_backup"

fstab_line="UUID=$uuid $mountpoint ext4 defaults,nofail 0 2"
printf '%s\n' "$fstab_line" >> /etc/fstab

mount "$mountpoint"

install -d -o "$owner_name" -g "$group_name" \
  "$mountpoint/cargo-target" \
  "$mountpoint/cargo-registry-cache" \
  "$mountpoint/generated-artifacts" \
  "$mountpoint/llvm/src" \
  "$mountpoint/llvm/build" \
  "$mountpoint/llvm/install" \
  "$mountpoint/lean-cache" \
  "$mountpoint/temp" \
  "$mountpoint/logs"

chown "$owner_name:$group_name" "$mountpoint"

echo "[setup] mount complete"
findmnt "$mountpoint"
df -h "$mountpoint"
echo "[setup] created subdirectories:"
find "$mountpoint" -maxdepth 2 -mindepth 1 -type d | sort
echo "[setup] next step:"
echo "  source $REPO_ROOT/scripts/env/mirrorea_storage_env.sh"
