# 0927 Compiler Backend LLVM Preparation

## Objective

phase 16 `Compiler/backend/LLVM preparation` を docs-first + non-destructive probe で repo に固定し、
mounted external workdir、`CARGO_HOME` binding、LLVM path readiness、cleanup safety、root-disk guardrail を current snapshot に同期する。

## Scope and assumptions

- current scope は guardrail docs、live but non-destructive probe、reader-facing summary の追加である。
- actual LLVM checkout/build、final backend choice、installed-binary packaging success criteria は固定しない。
- worktree には unrelated current-L2 CLI formatting diff が残っているため、この package では stage / commit に含めない。

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `scripts/env/mirrorea_storage_env.sh`
- `scripts/storage/detach_prepare.sh`
- `scripts/storage/cleanup_disposable_artifacts.sh`
- `docs/reports/0913-phase-sample-progress-storage-foundation.md`
- `docs/reports/0915-storage-mounted-and-target-cutover.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`

## Actions taken

1. `scripts/env/mirrorea_storage_env.sh` に `CARGO_HOME` binding を追加し、cargo registry cache probe の intended path を current script へ固定した。
2. `plan/23-compiler-backend-llvm-guardrail-roadmap.md` を新規追加した。
3. `docs/research_abstract/compiler_backend_llvm_preparation_01.md` を追加し、reader-facing summary を置いた。
4. live but non-destructive probe として root / workdir 使用量、mount、env export、detach/cleanup status、LLVM path、`CARGO_HOME` cargo probe を実行した。
5. README / Documentation / progress / tasks / samples_progress / specs / plan / report を、next promoted package が `hands-on docs / closeout` である current snapshot に同期した。

## Files changed

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/23-compiler-backend-llvm-guardrail-roadmap.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/compiler_backend_llvm_preparation_01.md`
- `docs/reports/0927-compiler-backend-llvm-preparation.md`
- `scripts/env/mirrorea_storage_env.sh`

## Evidence / outputs / test results

- `df -h .`
  - pass
- `free -h`
  - pass
- `df -h / /mnt/mirrorea-work`
  - pass
- `findmnt /mnt/mirrorea-work`
  - pass
- `du -sh target /mnt/mirrorea-work/cargo-target 2>/dev/null || true`
  - pass
- `bash scripts/env/mirrorea_storage_env.sh`
  - pass
- `bash scripts/storage/detach_prepare.sh`
  - pass
- `bash scripts/storage/cleanup_disposable_artifacts.sh --list`
  - pass
- `ls -ld /mnt/mirrorea-work/llvm /mnt/mirrorea-work/llvm/src /mnt/mirrorea-work/llvm/build /mnt/mirrorea-work/llvm/install`
  - pass
- `CARGO_HOME=/mnt/mirrorea-work/cargo-registry-cache cargo test -p mir-ast --no-run`
  - pass
- `python3 scripts/check_source_hierarchy.py`
  - pass
- `python3 scripts/validate_docs.py`
  - pass
- `git diff --check`
  - pass

## What changed in understanding

- `target/` externalization だけでは storage guardrail は incomplete で、`CARGO_HOME` の intended binding を current script と live probe の両方で明示して初めて “repo root を膨らませない current rule” として読める。
- LLVM preparation は actual build 前でも、path readiness、non-destructive cleanup、mount truth、memory/disk headroom の 4 点を揃えるだけで current snapshot としてかなり安定する。

## Open questions

- actual LLVM checkout / build をどの package で初めて許可するか。
- `cargo-registry-cache` naming のまま `CARGO_HOME` を担わせる current cut を後で rename するか。
- final backend choice、installed-binary success criteria、FFI / engine adapter / host deployment contract をどこで reopen するか。

## Suggested next prompt

`hands-on docs / closeout` package を進めてください。日本語 docs、`samples_progress.md`、`progress.md`、`tasks.md`、reports を再同期し、current phase closeout と remaining mixed gate / true user-spec gate を整理してください。
