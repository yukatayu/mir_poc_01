# Report 1090 — public-gate storage anchor mirror audit

- Date: 2026-05-01 13:28 JST
- Author / agent: Codex
- Scope: docs/storage-anchor maintenance
- Decision levels touched: `plan/27` repository memory only; no normative `specs/` decision changed

## Objective

Verify and repair reader-facing storage/backend guardrail anchors after report 1089 expanded the reusable storage validation bundle. The narrow goal is to ensure the public API / parser gate docs do not represent storage readiness only through the bare environment export command when the current guardrail evidence also includes `--ensure-dirs`, non-destructive detach audit, cleanup list mode, and external cargo cache no-run probing.

## Scope and assumptions

- This is a maintenance-only docs package.
- The package does not create a new public API/parser freeze, packaging decision, backend target, or installed-binary adoption claim.
- The package does not mount, format, delete, repair ownership, run destructive cleanup, or perform an actual LLVM checkout/build/install.
- `docs/hands_on/public_api_parser_gate_01.md` is a reader-facing landing page; `plan/27-public-api-parser-gate-roadmap.md` is repository memory. Both may mirror the storage guardrail bundle without changing normative semantics.
- Historical reports are not rewritten; they remain execution evidence for their original checkpoints.

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
- `plan/23-compiler-backend-llvm-guardrail-roadmap.md`
- `plan/27-public-api-parser-gate-roadmap.md`
- `samples/README.md`
- `scripts/README.md`
- `sub-agent-pro/mir_poc_01_research_handoff_2026-04-30.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/hands_on/compiler_backend_llvm_preparation_01.md`
- `docs/hands_on/public_api_parser_gate_01.md`
- `docs/research_abstract/compiler_backend_llvm_preparation_01.md`
- `docs/research_abstract/public_api_parser_gate_plan_01.md`
- `docs/reports/1089-storage-env-entrypoint-guardrail-freshness.md`

## Actions taken

- Audited storage guardrail references with `rg` across active reader-facing docs, plan memory, snapshot docs, and script taxonomy docs.
- Compared the public API / parser gate landing page and roadmap validation floor against the current storage guardrail bundle from report 1089.
- Updated `docs/hands_on/public_api_parser_gate_01.md` to include:
  `mirrorea_storage_env.sh --ensure-dirs`,
  `detach_prepare.sh`,
  `cleanup_disposable_artifacts.sh --list`,
  and external `CARGO_HOME` `cargo test -p mir-ast --no-run`.
- Updated `plan/27-public-api-parser-gate-roadmap.md` with the same command bundle and an explicit non-claim note: these commands prove storage guardrails, not LLVM build/backend/packaging adoption.
- Added the same storage-specific non-claim to `docs/hands_on/public_api_parser_gate_01.md` after reviewer feedback so the hands-on page cannot be read as backend or packaging evidence.
- Updated `progress.md`, `tasks.md`, and `samples_progress.md` to record the package and keep dashboard/report references synchronized.
- Ran the storage guardrail command bundle and docs-focused validation.

## Files changed

- `docs/hands_on/public_api_parser_gate_01.md`
- `plan/27-public-api-parser-gate-roadmap.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/1090-public-gate-storage-anchor-mirror-audit.md`

## Commands run

```bash
git status --short
git branch --show-current
git log -1 --oneline
date '+%Y-%m-%d %H:%M %Z'
```

```bash
rg -n "mirrorea_storage_env|detach_prepare|cleanup_disposable|cargo-registry-cache|cargo-target|/mnt/mirrorea-work|llvm/src|--ensure-dirs|--confirm" README.md Documentation.md scripts/README.md samples/README.md progress.md tasks.md samples_progress.md docs/hands_on docs/research_abstract plan .docs -g '*.md'
rg -n "^bash scripts/env/mirrorea_storage_env\\.sh$|mirrorea_storage_env\\.sh$" docs plan progress.md tasks.md samples_progress.md scripts/README.md README.md Documentation.md -g '*.md'
```

```bash
bash scripts/env/mirrorea_storage_env.sh
bash scripts/env/mirrorea_storage_env.sh --ensure-dirs
bash scripts/storage/detach_prepare.sh
bash scripts/storage/cleanup_disposable_artifacts.sh --list
CARGO_HOME=/mnt/mirrorea-work/cargo-registry-cache cargo test -p mir-ast --no-run
```

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
```

## Evidence / outputs / test results

- Package start state was clean; branch was `main`; prior commit was `356af6e Record storage env guardrail freshness`.
- `rg` found storage guardrail bundles already complete in `docs/hands_on/current_phase_closeout_01.md`, `docs/hands_on/compiler_backend_llvm_preparation_01.md`, `docs/research_abstract/compiler_backend_llvm_preparation_01.md`, and `plan/23-compiler-backend-llvm-guardrail-roadmap.md`.
- `rg` found that the public-gate landing page and roadmap still used `bash scripts/env/mirrorea_storage_env.sh` as the only storage command in that validation sequence.
- `scripts/README.md` did not need a validation-floor change because it is a script taxonomy document and already states the current behavior of `mirrorea_storage_env.sh`, `detach_prepare.sh`, and `cleanup_disposable_artifacts.sh`.
- `bash scripts/env/mirrorea_storage_env.sh` passed with the known warning that `/mnt/mirrorea-work/llvm` is root-owned and non-writable; it reported external workdir and cargo/LLVM staging exports under `/mnt/mirrorea-work`.
- `bash scripts/env/mirrorea_storage_env.sh --ensure-dirs` passed with the known warning that `/mnt/mirrorea-work/llvm` is root-owned and non-writable; it reported external workdir and cargo/LLVM staging exports under `/mnt/mirrorea-work`.
- `bash scripts/storage/detach_prepare.sh` passed with the same known LLVM parent ownership warning; it reported mounted `/mnt/mirrorea-work`, repo usage, external workdir usage, LLVM staging dir ownership, disposable candidates, and explicitly deleted no files.
- `bash scripts/storage/cleanup_disposable_artifacts.sh --list` passed, listed disposable candidates, excluded `llvm/src`, and did not delete files.
- `CARGO_HOME=/mnt/mirrorea-work/cargo-registry-cache cargo test -p mir-ast --no-run` passed and compiled test executables without running the full Rust test suite.
- docs_researcher sub-agent confirmed that the audited anchor sections are complete after the public-gate edits and that the only drift before this report was snapshot references to report `1090` before the report file existed.
- `python3 scripts/check_source_hierarchy.py` passed before and after reviewer follow-up: required `35`, present `35`, missing `0`.
- `python3 scripts/validate_docs.py` passed before and after reviewer follow-up and reported the documentation scaffold complete with `1088` numbered reports.
- `git diff --check` passed before and after reviewer follow-up with no whitespace errors.

## What changed in understanding

The reusable storage guardrail bundle is now mirrored consistently in the storage-specific docs, the current phase closeout guide, the public API / parser gate landing page, `plan/23`, `plan/27`, and the snapshot dashboards. This does not increase product maturity; it only improves reproducibility of the repo-side public-gate inventory.

## Open questions

- `U1` actual commitment remains open: packaging / installed binary target, host integration target, first shipped public surface scope, and final shared-space operational catalog breadth.
- Actual LLVM build, backend choice, root-owned LLVM parent ownership repair, and installed-binary adoption remain deferred.
- Public API/parser grammar freeze remains deferred.

## Suggested next prompt

Continue autonomous maintenance with another narrow freshness package, preferably one that audits a different validation anchor family without widening public API or implementation claims.

## Plan update status

`plan/` 更新済み: `plan/27-public-api-parser-gate-roadmap.md` now mirrors the full storage guardrail bundle and its non-claim boundary.

## progress.md update status

`progress.md` 更新済み: added the 2026-05-01 13:23 JST public-gate storage anchor mirror log line.

## tasks.md update status

`tasks.md` 更新済み: added the public-gate storage anchor status under current task-level status.

## samples_progress.md update status

`samples_progress.md` 更新済み: refreshed `PH0`, `STORAGE-01`, and recent validation rows for report `1090`.

## Reviewer findings and follow-up

- docs_researcher finding: snapshot docs referenced the 13:23 public-gate storage anchor mirror and report `1090` before the report existed.
  Follow-up: created this report and kept the snapshot references aligned with the report ID.
- docs_researcher non-finding: no stale or incomplete storage command bundle remained in the audited reader-facing anchor sections after the public-gate edits.
- docs_researcher non-finding: `scripts/README.md` does not need a validation-floor checklist change because it is a taxonomy document and its descriptions match current behavior.
- reviewer finding: `samples_progress.md` could be read as claiming package 1090 complete before commit/push because `STORAGE-01` remained `100`.
  Follow-up: kept the existing current-scope storage guardrail `100` value but clarified that it is not a per-package closeout claim and that package 1090 is not closed before commit/push.
- reviewer finding: the public-gate hands-on page mirrored the storage commands without the storage-specific non-claim found in `plan/27`.
  Follow-up: added a storage-specific non-claim to `docs/hands_on/public_api_parser_gate_01.md`.
- reviewer finding: the report's validation evidence omitted plain `bash scripts/env/mirrorea_storage_env.sh` even though the edited validation floor still includes it.
  Follow-up: ran the plain env export command and recorded it in this report.

## Skipped validations and reasons

- Full repository validation floor skipped because this package only updates reader-facing docs and plan memory for storage command anchors; report 1085 already recorded a same-day full floor.
- Full `cargo test -p mir-ast` skipped; the focused no-run command was used to validate external cargo cache/build routing without rerunning all tests.
- No destructive cleanup command was run. `cleanup_disposable_artifacts.sh --list` was used instead of `--confirm`.
- No actual LLVM build/install, mount, format, ownership repair, packaging adoption, or public API/parser freeze validation was attempted.

## Commit / push status

Closeout commit/push pending at report write. The exact commit SHA is only available after staging this report and committing; package 1090 must not be treated as closed until `git commit --no-gpg-sign` and `git push` succeed. The user-facing package closeout records the resulting SHA or push failure.

## Sub-agent session close status

docs_researcher sub-agent `019de1c7-93af-7251-b2b3-eb812fb06d47` completed and was closed. reviewer sub-agent `019de1cc-2510-7f70-82d5-23713b9b445e` completed and was closed.
