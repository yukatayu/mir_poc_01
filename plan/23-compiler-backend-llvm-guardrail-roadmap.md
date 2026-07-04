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
  - current audit on 2026-07-05: not mounted in this environment
  - historical first-cut closeout expected an external ext4 workdir, but that
    is not current live state and must not be assumed
- repo target
  - current audit on 2026-07-05: repo-local `target/` is present and about
    7.0G after broad validation
  - no current `target -> /mnt/mirrorea-work/cargo-target` symlink is assumed
- storage env
  - `scripts/env/mirrorea_storage_env.sh`
  - `MIRROREA_WORKDIR`
  - `CARGO_TARGET_DIR`
  - `MIRROREA_CARGO_REGISTRY_CACHE`
  - `CARGO_HOME`
  - `MIRROREA_WORKDIR_MOUNTED` is based on exact mountpoint detection, not on
    the filesystem containing the path
- detach / cleanup
  - `scripts/storage/detach_prepare.sh`
  - `scripts/storage/cleanup_disposable_artifacts.sh --list`
- LLVM path
  - `/mnt/mirrorea-work/llvm/src`
  - `/mnt/mirrorea-work/llvm/build`
  - `/mnt/mirrorea-work/llvm/install`
  - current audit on 2026-07-05: staging dirs are missing because the external
    workdir is not mounted
  - routine helper still does not repair ownership or create heavy directories
    under an unmounted default root

## current rules

1. source repo と committed docs/report を detachable storage only に閉じ込めない
2. heavy disposable artifact は external workdir を優先する
3. `CARGO_TARGET_DIR` と `CARGO_HOME` は external workdir 側へ逃がせるように保つ
4. cleanup は `--confirm` なしで delete しない
5. mount / format / ownership repair は routine helper に埋め込まず、明示的 setup path に残す
6. external workdir は exact mountpoint として確認する。root filesystem 上に
   directory が存在するだけでは mounted 扱いしない
7. `llvm/src` は source checkout lifecycle 未決のため disposable cleanup に含めない
8. `llvm/build` / `llvm/install` cleanup は parent staging dir が non-writable なままでは実行しない

## non-destructive probe floor

- `df -h .`
- `free -h`
- `df -h .`
- `lsblk -f`
- `findmnt /mnt/mirrorea-work` または `findmnt --mountpoint /mnt/mirrorea-work`
- `du -sh target /mnt/mirrorea-work/cargo-target`
- `bash scripts/env/mirrorea_storage_env.sh`
- `bash scripts/env/mirrorea_storage_env.sh --ensure-dirs` only after
  `MIRROREA_WORKDIR_MOUNTED=yes`
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
- cleanup of repo-local `target/` without explicit user approval

## next relation

backend guardrail の current first-cut closeout は `P17` として current snapshot に固定済みである。
The current 2026-07-05 audit supersedes any assumption that `/mnt/mirrorea-work`
is presently mounted or that repo `target/` is currently a symlink into it.
public-freeze mixed-gate 側では、この guardrail を
`toolchain adjacency inventory` としてだけ参照し、
actual LLVM build、backend choice、installed-binary packaging は
post-`P18` kept-later / true user-spec hold line に残す。

`P-OPS-08` operational backend feasibility inventory は、この guardrail を
operational suite 側へ読み替えた docs-first comparison である。
current executable path は引き続き `native host launch bundle` のみであり、
WASM client host と LLVM/native projection backend は inventory-only のまま保つ。
