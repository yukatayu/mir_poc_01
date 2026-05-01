# Report 1089 — storage/env entrypoint guardrail freshness

- Date: 2026-05-01 13:15 JST
- Author / agent: Codex
- Scope: storage / backend guardrail maintenance
- Decision levels touched: none; no normative `specs/` decision changed

## Objective

Verify that the repo-local storage environment entrypoints still enforce the documented small-VPS guardrails: mounted external workdir preference, cargo target/cache routing, non-destructive detach audit, cleanup list safety, and no accidental LLVM source deletion or backend/product claim.

## Scope and assumptions

- This is a focused maintenance package, not a new implementation or product-shaping package.
- The package does not mount, format, delete, or repair storage.
- The package does not perform an actual LLVM checkout/build/install, choose a backend target, adopt packaging, or claim installed-binary readiness.
- The storage audit is allowed to create or use external cargo/cache/build directories under `/mnt/mirrorea-work` through existing scripts and `cargo test --no-run`.
- Known warning under review: `/mnt/mirrorea-work/llvm` is owned by `root:root` and is not writable by the current user, while `llvm/src`, `llvm/build`, and `llvm/install` are present and user-writable.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `.docs/progress-task-axes.md`
- `.docs/continuous-task-policy.md`
- `.docs/current-l2-source-sample-authoring-policy.md`
- `AGENTS.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/04-mir-core.md`
- `specs/05-mirrorea-fabric.md`
- `specs/06-prismcascade-positioning.md`
- `specs/07-typed-effects-wiring-platform.md`
- `specs/08-cross-system-relations.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `plan/00-index.md`
- relevant `plan/01..11`, `plan/18..38`
- `samples/README.md`
- `scripts/README.md`
- `sub-agent-pro/mir_poc_01_research_handoff_2026-04-30.md`
- `docs/hands_on/compiler_backend_llvm_preparation_01.md`
- `scripts/env/mirrorea_storage_env.sh`
- `scripts/storage/detach_prepare.sh`
- `scripts/storage/cleanup_disposable_artifacts.sh`
- `docs/reports/1085-repository-wide-validation-freshness-checkpoint.md`
- `docs/reports/1088-active-entrypoint-command-freshness-audit.md`

## Actions taken

- Ran a non-destructive storage/resource audit covering disk, memory, block devices, mounts, repo size, and external workdir usage.
- Ran `scripts/env/mirrorea_storage_env.sh` with and without `--ensure-dirs` to verify the entrypoint still reports the external workdir and cargo/LLVM path exports.
- Ran `scripts/storage/detach_prepare.sh` to verify detach-readiness reporting and disposable-candidate listing without deleting files.
- Ran `scripts/storage/cleanup_disposable_artifacts.sh --list` to verify cleanup remains explicit-confirmation gated and keeps `llvm/src` excluded.
- Ran `CARGO_HOME=/mnt/mirrorea-work/cargo-registry-cache cargo test -p mir-ast --no-run` to verify the external cargo cache path remains usable without running the full Rust test floor.
- Updated snapshot docs to record the focused storage freshness checkpoint and the known LLVM parent ownership warning.
- Updated reusable storage validation anchors in `progress.md`, `tasks.md`, and `samples_progress.md` after reviewer feedback so the entrypoint, detach audit, cleanup list, and external cargo no-run probe are reproducible from snapshot docs.

## Files changed

- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/1089-storage-env-entrypoint-guardrail-freshness.md`

## Commands run

```bash
git status --short
git branch --show-current
git log -1 --oneline
date '+%Y-%m-%d %H:%M %Z'
```

```bash
df -h
free -h
lsblk -f
findmnt
du -sh .
du -sh target .git .cargo .lake 2>/dev/null || true
bash scripts/env/mirrorea_storage_env.sh
bash scripts/env/mirrorea_storage_env.sh --ensure-dirs
bash scripts/storage/detach_prepare.sh
bash scripts/storage/cleanup_disposable_artifacts.sh --list
ls -ld target /mnt/mirrorea-work/cargo-target /mnt/mirrorea-work/cargo-registry-cache /mnt/mirrorea-work/llvm /mnt/mirrorea-work/llvm/src /mnt/mirrorea-work/llvm/build /mnt/mirrorea-work/llvm/install
CARGO_HOME=/mnt/mirrorea-work/cargo-registry-cache cargo test -p mir-ast --no-run
```

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
```

## Evidence / outputs / test results

- `git status --short` before the package was clean; branch was `main`; prior commit was `932e3a6 Record entrypoint command freshness audit`.
- `df -h` showed root `/dev/vda2` at `99G` size, `64G` used, `31G` available, `68%` used.
- `df -h` and `findmnt` showed `/mnt/mirrorea-work` mounted from `/dev/vdb1`, size `196G`, used `6.1G`, available `180G`, `4%` used.
- `free -h` showed `960Mi` memory total, `323Mi` available, and `19Gi` swap total with `18Gi` free.
- `lsblk -f` showed `/dev/vdb1` as ext4 label `mirrorea-work`, mounted at `/mnt/mirrorea-work`.
- `du -sh .` reported the repo at `49M`; `du -sh target .git .cargo .lake` reported `target` as `0` because it is a symlink and `.git` as `25M`.
- `scripts/env/mirrorea_storage_env.sh` reported `MIRROREA_WORKDIR=/mnt/mirrorea-work`, mounted external workdir status, and cargo target/cache paths under that workdir.
- `scripts/env/mirrorea_storage_env.sh --ensure-dirs` completed with exit code 0 and did not fall back to root-local heavy build paths.
- `scripts/storage/detach_prepare.sh` completed with exit code 0, reported disposable candidates, and explicitly deleted no files.
- `scripts/storage/cleanup_disposable_artifacts.sh --list` completed with exit code 0, listed cleanup candidates, excluded `llvm/src`, and required `--confirm` for deletion.
- `ls -ld` confirmed `target -> /mnt/mirrorea-work/cargo-target`; `/mnt/mirrorea-work/cargo-target` and `/mnt/mirrorea-work/cargo-registry-cache` exist; `/mnt/mirrorea-work/llvm` is `root:root`; `llvm/src`, `llvm/build`, and `llvm/install` are `yukatayu:yukatayu`.
- `CARGO_HOME=/mnt/mirrorea-work/cargo-registry-cache cargo test -p mir-ast --no-run` completed with exit code 0 and produced test executables without running the full test suite.
- `python3 scripts/check_source_hierarchy.py` passed before and after reviewer follow-up: required `35`, present `35`, missing `0`.
- `python3 scripts/validate_docs.py` passed before and after reviewer follow-up and reported the documentation scaffold complete with `1087` numbered reports.
- `git diff --check` passed before and after reviewer follow-up with no whitespace errors.
- Known warning remains: `/mnt/mirrorea-work/llvm` parent is root-owned and non-writable by the current user.

## What changed in understanding

The storage guardrail remains consistent with the current docs: repo-local heavy artifacts are redirected to the mounted external workdir, cleanup remains explicit-confirmation gated, and LLVM source is not in a disposable cleanup set. The only observed storage warning is the already-known root-owned LLVM parent directory; it is not a new blocker for the non-destructive audit, but it remains relevant before any actual LLVM build or staging rewrite.

## Open questions

- `U1` still needs a user-facing choice for packaging / installed binary target, host integration target, first shipped public surface scope, and final shared-space operational catalog breadth.
- Actual LLVM build, backend choice, and installed-binary adoption remain deferred.
- Whether to repair ownership of `/mnt/mirrorea-work/llvm` parent should be decided before an actual LLVM build package; this report does not change ownership.

## Suggested next prompt

Continue autonomous maintenance with the next safe package: run another narrow freshness audit over active docs / validation anchors, or wait for `U1` if the next step should become public-product shaping.

## Plan update status

`plan/` 更新不要: no roadmap, sequencing, semantics, or long-lived comparison changed.

## progress.md update status

`progress.md` 更新済み: added the 2026-05-01 13:13 JST storage/env guardrail freshness log line.

## tasks.md update status

`tasks.md` 更新済み: added the storage/env entrypoint guardrail freshness status under current task-level status.

## samples_progress.md update status

`samples_progress.md` 更新済み: refreshed `PH0`, `STORAGE-01`, and recent validation rows for this focused storage checkpoint.

## Reviewer findings and follow-up

- Reviewer finding: `samples_progress.md` could be read as overclaiming `STORAGE-01` at `100%` while report commit/push status was still pending.
  Follow-up: kept the existing current-scope storage first-cut closeout percentage, but narrowed the row wording so `100%` refers to the current guardrail scope only, not an actual LLVM build, and made the 1089 freshness addition conditional on package commit/push. If push fails, the final user-facing package close must report local-only status and no pushed closeout claim.
- Reviewer finding: report 1089 lacked an explicit reviewer findings and follow-up section.
  Follow-up: added this section and recorded that the reviewer session was closed after completion.
- Reviewer finding: reusable storage validation anchors were incomplete relative to the 1089 evidence, especially `scripts/env/mirrorea_storage_env.sh --ensure-dirs`, `cleanup_disposable_artifacts.sh --list`, and the external cargo no-run probe.
  Follow-up: added the storage/backend guardrail bundle to `progress.md`, expanded the `tasks.md` storage/backend executable floor and validation floor, and expanded the `samples_progress.md` storage summary / matrix command anchors.
- Reviewer non-finding: no direct claim was found that an actual LLVM build, backend adoption, packaging adoption, or destructive cleanup was performed.
- Reviewer non-finding: no source-hierarchy or sample-taxonomy violation was found in the package wording.

## Skipped validations and reasons

- Full repository validation floor skipped for this package because 1085 already recorded a same-day full floor and this package only touched storage guardrail snapshot docs/report.
- Full `cargo test -p mir-ast` skipped; `cargo test -p mir-ast --no-run` was the focused check proving the external cargo cache/build routing still works without rerunning all tests.
- No destructive cleanup command was run. `cleanup_disposable_artifacts.sh --list` was used instead of `--confirm`.
- No actual LLVM build/install command was run; this package only verifies storage guardrails and entrypoint behavior.

## Commit / push status

Closeout commit/push pending at report write. The exact commit SHA is only available after staging this report and committing; the package must not be treated as closed until `git commit --no-gpg-sign` and `git push` succeed. The user-facing package closeout records the resulting SHA or push failure.

## Sub-agent session close status

Reviewer sub-agent `019de1c0-fef3-7792-96be-fbe37adf258f` completed and was closed. No implementation sub-agent was needed because this package is a focused storage audit with snapshot/report updates only.
