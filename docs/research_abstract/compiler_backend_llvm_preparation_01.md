# compiler backend llvm preparation 01

## 目的

small VPS 上で backend / LLVM を root disk 既成事実にせず進めるための
docs-first guardrail です。

## guardrail anchors

- `/mnt/mirrorea-work`
- `target/ -> /mnt/mirrorea-work/cargo-target`
- `scripts/env/mirrorea_storage_env.sh`
- `scripts/env/mirrorea_storage_env.sh --ensure-dirs`
- `scripts/storage/detach_prepare.sh`
- `scripts/storage/cleanup_disposable_artifacts.sh --list`
- `CARGO_HOME=/mnt/mirrorea-work/cargo-registry-cache cargo test -p mir-ast --no-run`
- `/mnt/mirrorea-work/llvm` owner / writable status
- `llvm/src` は staging path として保持し、disposable cleanup 対象に含めない

## guardrail rule

- source repo は detachable storage only にしない
- heavy disposable artifact は external workdir を優先する
- `CARGO_TARGET_DIR` と `CARGO_HOME` は external workdir へ向けられるように保つ
- cleanup は explicit confirmation なしで delete しない
- `llvm/build` / `llvm/install` cleanup は parent staging dir が non-writable なままでは実行しない
- ownership repair は routine helper ではなく explicit setup path に残す
- closeout で実際に通したのは list-mode と owner/writable probe までであり、cleanup refusal branch 自体は non-destructive stop line / helper implementation evidence として読む

## stop line

- actual LLVM build
- final backend choice
- final packaging / FFI / engine adapter target
- root-owned staging parent の ownership repair

## 関連

- `plan/23-compiler-backend-llvm-guardrail-roadmap.md`
- `docs/reports/0927-compiler-backend-llvm-preparation.md`
