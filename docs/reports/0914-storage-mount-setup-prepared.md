# 0914 Storage Mount Setup Prepared

## Objective

- 追加 200GB SSD の認識状態を再確認し、`/dev/vdb` を Mirrorea 用 workdir として安全に mount する準備を行う。
- 永続化 (`/etc/fstab`) を含む setup 方針を root 実行可能な script に落とし、current session で root 権限が無い場合でも次手を明確にする。

## Scope and assumptions

- user は `/dev/vdb` を空ディスクとして format してよいと明示した。
- ただし current session では `sudo` が password-required であり、raw block device 操作と `/etc/fstab` 編集は未実行である。
- この report は **mount 実行完了 report ではなく、認識確認と root-setup 準備 report** である。

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `docs/reports/0913-phase-sample-progress-storage-foundation.md`
- `scripts/env/mirrorea_storage_env.sh`
- `scripts/storage/detach_prepare.sh`
- `scripts/storage/cleanup_disposable_artifacts.sh`

## Actions taken

### SSD recognition audit

- `lsblk -o NAME,PATH,SIZE,TYPE,FSTYPE,LABEL,UUID,MODEL,SERIAL,PARTLABEL,MOUNTPOINT /dev/vdb`
  - `/dev/vdb`
  - size `200G`
  - type `disk`
  - no filesystem
  - no mountpoint
- `findmnt`
  - extra workdir mount is still absent
- `blkid`
  - `/dev/vdb` does not appear because it has no filesystem signature
- `/etc/fstab`
  - `/dev/vdb` or `/mnt/mirrorea-work` entry does not exist

### Privilege boundary audit

- current user is `yukatayu`
- `sudo -n true` failed with `sudo: a password is required`
- `wipefs -n /dev/vdb`, `parted -s /dev/vdb print`, `fdisk -l /dev/vdb` all failed without root access

### Prepared root setup helper

- added `scripts/storage/setup_mirrorea_workdisk_root.sh`
- default plan:
  - target device: `/dev/vdb`
  - partitioning: GPT + single primary partition
  - filesystem: `ext4`
  - mountpoint: `/mnt/mirrorea-work`
  - persistence: UUID-based `/etc/fstab` entry with `defaults,nofail`
  - owned directories:
    - `cargo-target/`
    - `cargo-registry-cache/`
    - `generated-artifacts/`
    - `llvm/src`
    - `llvm/build`
    - `llvm/install`
    - `lean-cache/`
    - `temp/`
    - `logs/`
- safety behavior:
  - refuses non-root execution except `--plan`
  - refuses non-disk device
  - refuses existing child partitions/signatures unless `--force`
  - refuses existing conflicting `fstab` mountpoint entry unless `--force`
  - backs up `/etc/fstab` before editing

## Files changed

- created:
  - `scripts/storage/setup_mirrorea_workdisk_root.sh`
  - `docs/reports/0914-storage-mount-setup-prepared.md`
- updated:
  - `plan/09-helper-stack-and-responsibility-map.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`

## Evidence / outputs / test results

### Commands run

```bash
lsblk -o NAME,PATH,SIZE,TYPE,FSTYPE,LABEL,UUID,MODEL,SERIAL,PARTLABEL,MOUNTPOINT /dev/vdb
findmnt
sudo blkid 2>/dev/null || blkid 2>/dev/null || true
grep -nEv '^\s*(#|$)' /etc/fstab || true
sudo wipefs -n /dev/vdb 2>/dev/null || wipefs -n /dev/vdb
sudo parted -s /dev/vdb print 2>/dev/null || parted -s /dev/vdb print
sudo fdisk -l /dev/vdb 2>/dev/null || fdisk -l /dev/vdb
id
sudo -n true
bash -n scripts/storage/setup_mirrorea_workdisk_root.sh
bash scripts/storage/setup_mirrorea_workdisk_root.sh --plan
python3 scripts/validate_docs.py
python3 scripts/check_source_hierarchy.py
git diff --check
```

### Results

- SSD recognition: pass
  - `/dev/vdb` is visible as a `200G` disk
- filesystem / mount / persistence: not yet executed
  - blocked by root privilege
- root helper syntax check: pass
- root helper `--plan`: pass
- docs validation: pass
- source hierarchy: pass
- diff whitespace check: pass

## What changed in understanding

- current machine では SSD 自体は問題なく見えている。問題は device recognition ではなく **privileged execution path** だけである。
- したがって storage package の次手は「設計」ではなく、「root で setup script を 1 回流す」ことである。

## Open questions

- current session から `sudo` を使える root execution path をどう確保するか。
- mount 実行後に repo-local `target/` をどのタイミングで external workdir へ切り替えるか。

## Suggested next prompt

root 権限が使える shell で `sudo bash scripts/storage/setup_mirrorea_workdisk_root.sh` を実行してください。実行後にこちらへ戻れば、mount 状態確認、`samples_progress.md` / `progress.md` の storage row 更新、`target/` / generated artifact の cutover 方針まで続けます。
