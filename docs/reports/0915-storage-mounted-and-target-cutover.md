# 0915 Storage Mounted And Target Cutover

## Objective

- user 実行の root setup 後に、追加 SSD の mount / persistence 状態を確認する。
- repo の heavy disposable artifact の第一弾として `target/` を SSD 側へ移し、通常の cargo workflow がそのまま使える状態にする。

## Scope and assumptions

- `/dev/vdb` の format / partition / mount / `/etc/fstab` 編集自体は user が root 権限で実行した。
- この task では、その結果確認、repo-local cutover、docs / report / dashboard 同期を行う。
- repo source 自体は root disk に残し、disposable artifact だけを SSD 側へ出す。

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `docs/reports/0914-storage-mount-setup-prepared.md`
- `scripts/env/mirrorea_storage_env.sh`
- `scripts/storage/setup_mirrorea_workdisk_root.sh`

## Actions taken

### Mount and persistence verification

- confirmed `/dev/vdb1` exists
- confirmed filesystem is `ext4`
- confirmed label is `mirrorea-work`
- confirmed UUID is `a87650a8-e3e9-4977-8940-6c293a0ee23c`
- confirmed mountpoint is `/mnt/mirrorea-work`
- confirmed `/etc/fstab` contains:
  - `UUID=a87650a8-e3e9-4977-8940-6c293a0ee23c /mnt/mirrorea-work ext4 defaults,nofail 0 2`

### Workdir layout verification

- confirmed these paths exist and are writable in normal use:
  - `/mnt/mirrorea-work/cargo-target`
  - `/mnt/mirrorea-work/cargo-registry-cache`
  - `/mnt/mirrorea-work/generated-artifacts`
  - `/mnt/mirrorea-work/llvm/src`
  - `/mnt/mirrorea-work/llvm/build`
  - `/mnt/mirrorea-work/llvm/install`
  - `/mnt/mirrorea-work/temp`
  - `/mnt/mirrorea-work/logs`

### Target cutover

- repo-local `target/` was still a real directory on root disk and occupied about `5.2G`
- moved it to `/mnt/mirrorea-work/cargo-target`
- replaced repo-local `target/` with a symlink back to that mounted directory
- root free space increased from about `27G` to about `32G`

### Validation after cutover

- ran `bash scripts/env/mirrorea_storage_env.sh`
  - mounted flag became `yes`
- ran `cargo test -p mir-ast --no-run`
  - succeeded with the symlinked `target/`

### Future helper improvement

- updated `scripts/storage/setup_mirrorea_workdisk_root.sh` so future runs:
  - call `systemctl daemon-reload` after editing `fstab` when available
  - correct ownership of the `llvm/` parent directory as well

## Files changed

- created:
  - `docs/reports/0915-storage-mounted-and-target-cutover.md`
- updated:
  - `scripts/storage/setup_mirrorea_workdisk_root.sh`
  - `plan/01-status-at-a-glance.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`

## Evidence / outputs / test results

### Commands run

```bash
lsblk -o NAME,PATH,SIZE,TYPE,FSTYPE,LABEL,UUID,MOUNTPOINT /dev/vdb
findmnt /mnt/mirrorea-work
grep -n '/mnt/mirrorea-work' /etc/fstab || true
findmnt --verify --tab-file /etc/fstab
ls -ld /mnt/mirrorea-work /mnt/mirrorea-work/cargo-target /mnt/mirrorea-work/generated-artifacts /mnt/mirrorea-work/llvm /mnt/mirrorea-work/logs
df -h / /mnt/mirrorea-work
du -sh target /mnt/mirrorea-work/cargo-target 2>/dev/null || true
mv target /mnt/mirrorea-work/cargo-target
ln -s /mnt/mirrorea-work/cargo-target target
df -h / /mnt/mirrorea-work
du -sh target /mnt/mirrorea-work/cargo-target 2>/dev/null
bash scripts/env/mirrorea_storage_env.sh
cargo test -p mir-ast --no-run
python3 scripts/validate_docs.py
python3 scripts/check_source_hierarchy.py
git diff --check
```

### Results

- mount verification: pass
- `fstab` entry presence: pass
- root free-space improvement: pass
- `target/` cutover to SSD: pass
- env script mounted detection: pass
- cargo lightweight validation on externalized target: pass
- docs validation: pass
- source hierarchy: pass
- diff whitespace check: pass

### Warnings / residual follow-up

- `findmnt --verify --tab-file /etc/fstab` reported no parse errors, but warned that current systemd session may still use the old `fstab` snapshot until `daemon-reload`
- cargo registry cache and LLVM build paths are prepared but not yet exercised by a dedicated package

## What changed in understanding

- storage package は policy-only ではなく、もう **mounted and partially actualized** な段階に入った。
- root disk pressure の最大要因だった `target/` は、repo source を壊さず SSD 側へ逃がせることが確認できた。
- next storage-related work は mount そのものではなく、cargo registry cache / LLVM / generated artifacts の actual probe に移る。

## Open questions

- cargo registry cache を repo-local にどう actualize するか。
- first LLVM / backend probe package でどこまで external workdir actualization を進めるか。

## Suggested next prompt

`Sugoroku sample progress alignment` を進めてください。storage は mounted / target cutover まで確認済みとして扱い、以後の heavy package では `/mnt/mirrorea-work` を前提にしつつ、必要なら package 11 `Compiler/backend/LLVM preparation guardrail` で cargo registry cache / LLVM actual probe を追加してください。
