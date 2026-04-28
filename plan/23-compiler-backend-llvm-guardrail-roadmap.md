# plan/23 — compiler/backend/LLVM preparation current roadmap

## 目的

Macro 7 `Compiler/backend/LLVM preparation` を、
small VPS と detachable workdir 前提で安全に進めるための current guardrail roadmap を置く。

ここで固定するのは、external workdir の current truth、`CARGO_HOME` / `CARGO_TARGET_DIR` binding、
non-destructive probe、cleanup rule、stop line である。
actual LLVM checkout/build、final backend choice、packaging success criteria は固定しない。

## current anchors

- mounted workdir
  - `/mnt/mirrorea-work`
  - `/dev/vdb1` ext4 `mirrorea-work`
- repo target cutover
  - `target/ -> /mnt/mirrorea-work/cargo-target`
  - live usage:
    `cargo-target ~= 5.9G`
- storage env
  - `scripts/env/mirrorea_storage_env.sh`
  - `MIRROREA_WORKDIR`
  - `CARGO_TARGET_DIR`
  - `MIRROREA_CARGO_REGISTRY_CACHE`
  - `CARGO_HOME`
  - live cache usage:
    `cargo-registry-cache ~= 9.7M`
- detach / cleanup
  - `scripts/storage/detach_prepare.sh`
  - `scripts/storage/cleanup_disposable_artifacts.sh --list`
- LLVM path
  - `/mnt/mirrorea-work/llvm/src`
  - `/mnt/mirrorea-work/llvm/build`
  - `/mnt/mirrorea-work/llvm/install`
  - live ownership mismatch:
    `/mnt/mirrorea-work/llvm` は `root:root`、`build/install/src` は empty staging dir だが、routine helper で ownership repair はしない

## current rules

1. source repo と committed docs/report を detachable storage only に閉じ込めない
2. heavy disposable artifact は external workdir を優先する
3. `CARGO_TARGET_DIR` と `CARGO_HOME` は external workdir 側へ逃がせるように保つ
4. cleanup は `--confirm` なしで delete しない
5. mount / format / ownership repair は routine helper に埋め込まず、明示的 setup path に残す
6. `llvm/src` は source checkout lifecycle 未決のため disposable cleanup に含めない
7. `llvm/build` / `llvm/install` cleanup は parent staging dir が non-writable なままでは実行しない

## non-destructive probe floor

- `df -h .`
- `free -h`
- `df -h / /mnt/mirrorea-work`
- `findmnt /mnt/mirrorea-work`
- `du -sh target /mnt/mirrorea-work/cargo-target`
- `bash scripts/env/mirrorea_storage_env.sh`
- `bash scripts/env/mirrorea_storage_env.sh --ensure-dirs`
- `bash scripts/storage/detach_prepare.sh`
- `bash scripts/storage/cleanup_disposable_artifacts.sh --list`
- `ls -ld /mnt/mirrorea-work/llvm /mnt/mirrorea-work/llvm/src /mnt/mirrorea-work/llvm/build /mnt/mirrorea-work/llvm/install`
- `ls -ld target /mnt/mirrorea-work/cargo-target /mnt/mirrorea-work/cargo-registry-cache`
- `CARGO_HOME=/mnt/mirrorea-work/cargo-registry-cache cargo test -p mir-ast --no-run`

## current stop line

- actual LLVM checkout / configure / build
- final backend choice
- final installed-binary packaging success criteria
- final FFI / engine adapter / host deployment contract
- root-owned llvm parent の ownership repair
- source checkout retention / cleanup lifecycle

## next relation

backend guardrail の current first-cut closeout は `P17` として current snapshot に固定済みである。
public-freeze mixed-gate 側では、この guardrail を
`toolchain adjacency inventory` としてだけ参照し、
actual LLVM build、backend choice、installed-binary packaging は
post-`P18` kept-later / true user-spec hold line に残す。
