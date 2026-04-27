# compiler backend llvm preparation 01

## 目的

small VPS 上で backend / LLVM を root disk 既成事実にせず進めるための
docs-first current guardrail です。

## current anchors

- `/mnt/mirrorea-work`
- `target/ -> /mnt/mirrorea-work/cargo-target`
- `scripts/env/mirrorea_storage_env.sh`
- `scripts/storage/detach_prepare.sh`
- `scripts/storage/cleanup_disposable_artifacts.sh --list`
- `CARGO_HOME=/mnt/mirrorea-work/cargo-registry-cache cargo test -p mir-ast --no-run`

## current rule

- source repo は detachable storage only にしない
- heavy disposable artifact は external workdir を優先する
- `CARGO_TARGET_DIR` と `CARGO_HOME` は external workdir へ向けられるように保つ
- cleanup は explicit confirmation なしで delete しない

## stop line

- actual LLVM build
- final backend choice
- final packaging / FFI / engine adapter target

## 関連

- `plan/23-compiler-backend-llvm-guardrail-roadmap.md`
- `docs/reports/0927-compiler-backend-llvm-preparation.md`
