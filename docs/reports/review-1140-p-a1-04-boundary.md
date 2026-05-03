# Report 1140 — Review P-A1-04 Package / Hot-Plug Sample Boundary

- Date: 2026-05-03
- Author / agent: Codex
- Scope: review practical alpha-1 sample/validation coverage shape for upcoming `P-A1-04`
- Decision levels touched: none

## Objective

Review the current practical alpha-1 sample/test/script surface and identify the narrowest safe `P-A1-04` sample IDs, expected negative cases, and the minimum command/test shape needed so package/hot-plug is not implemented as a sample-ID-only bridge.

## Scope and assumptions

- Review-only task; no file edits besides this report.
- Focus is limited to the user-requested files plus the minimum spec/roadmap lines needed to compare the `P-A1-04` sample matrix against the practical package/hot-plug requirements.
- “Narrowest safe sample IDs” here means the smallest honest first tranche for package/hot-plug actualization, not full `PA1-4` completion.

## Start state / dirty state

- start state: `main` at `bcfb386c8f1f8f157fde5577ea309fc26df957cf`
- worktree status: clean before this report

## Documents consulted

- `sub-agent-pro/alpha-1/08-sample-matrix.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/44-practical-alpha1-roadmap.md`
- `samples/practical-alpha1/packages/README.md`
- `samples/practical-alpha1/expected/README.md`
- `scripts/practical_alpha1_check.py`
- `scripts/practical_alpha1_run_local.py`
- `crates/mir-ast/tests/practical_alpha1_front_door.rs`
- `crates/mir-ast/tests/practical_alpha1_checker.rs`
- `crates/mir-runtime/tests/practical_alpha1_local_runtime.rs`
- `scripts/tests/test_practical_alpha1_check.py`
- `scripts/tests/test_practical_alpha1_run_local.py`

## Actions taken

1. Read the user-specified alpha-1 sample matrix handoff first, then the required repo front-door documents.
2. Compared practical alpha-1 package/check/runtime sample families, expected artifact taxonomy, and direct-path command surfaces.
3. Cross-checked the reviewed sample matrix against `specs/18` package/hot-plug requirements to find missing negative rows and bridge risks.

## Files changed

- `docs/reports/review-1140-p-a1-04-boundary.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git rev-parse --abbrev-ref HEAD
git rev-parse HEAD
git show --stat --oneline --no-patch HEAD
find samples/practical-alpha1/packages -maxdepth 1 -mindepth 1 -type d -printf '%f\n' | sort
find samples/practical-alpha1/expected -maxdepth 1 -mindepth 1 -type f -printf '%f\n' | sort
rg -n 'HP-A1|hp-' samples/practical-alpha1 crates scripts -g '*.json' -g '*.rs' -g '*.py' -g '*.md'
date '+%Y-%m-%d %H:%M %Z'
```

## Evidence / outputs / test results

- The current practical package root contains only `SRC-*`, `CHK-*`, and `RUN-*` directories; no `HP-*` practical package fixtures exist yet.
- The current practical expected root contains only `src-*.expected.json`, `chk-*.expected.json`, and `run-*.expected.json`; no `hp-*.expected.json` practical reports exist yet.
- The current checker/runtime command surfaces already accept direct package paths via `check <package_path>` and normalize bare package paths into `check`, which is the right anti-bridge pattern for future hot-plug work.

## What changed in understanding

- The reviewed tree already has the crucial anti-sample-ID precedent: checker and local runtime both expose direct package-path entrypoints, and runtime tests include a mutated temporary package path to prove semantics come from package contents rather than a hard-coded sample name.
- The main `P-A1-04` gap is therefore not abstract architecture but missing practical `HP-*` families and missing hot-plug-specific negative rows/artifacts.

## Open questions

- None beyond the concrete findings below.

## Suggested next prompt

Add the first practical `HP-*` family with direct package-path attach commands and exact `hp-*.expected.json` reports, starting from `HP-A1-01`, `HP-A1-02`, and `HP-A1-05`, plus explicit missing-witness and stale-membership hot-plug negatives required by `specs/18`.

## Plan update status

`plan/` 更新不要: review-only taskであり repository memory は変更していない。

## Documentation.md update status

`Documentation.md` 更新不要: review-only taskであり snapshot wording は変更していない。

## progress.md update status

`progress.md` 更新不要: review-only taskであり current status snapshot は変更していない。

## tasks.md update status

`tasks.md` 更新不要: review-only taskであり task map は変更していない。

## samples_progress.md update status

`samples_progress.md` 更新不要: review-only taskであり sample dashboard snapshot は変更していない。

## Reviewer findings and follow-up

- Finding 1:
  The practical sample/expected taxonomy is not yet ready for `P-A1-04` because the reviewed practical roots have no `HP-*` fixture family and no `hp-*.expected.json` artifact family. `samples/practical-alpha1/packages/README.md` only defines `SRC-*`, `CHK-*`, and `RUN-*` roles, and `samples/practical-alpha1/expected/README.md` only defines `src-*`, `chk-*`, and `run-*` artifacts. The filesystem matches that: only `src-*`, `chk-*`, and `run-*` entries exist, with no practical `HP-*` directories or `hp-*` expected reports. `P-A1-04` therefore cannot honestly close on the current reviewed coverage shape; it first needs distinct practical hot-plug fixtures and exact expected reports, not just reuse of checker/runtime rows.

- Finding 2:
  The narrowest safe first `P-A1-04` tranche is `HP-A1-01`, `HP-A1-02`, and `HP-A1-05`, with `CHK-PKG-01/02` kept as prerequisite checker-admission negatives and `RUN-01` kept as the positive local runtime carrier. This follows from the current matrix in `sub-agent-pro/alpha-1/08-sample-matrix.md`: `HP-A1-01` is the minimal positive attach path, `HP-A1-02` is the minimal capability/admin reject, and `HP-A1-05` is the minimal compatibility reject. `HP-A1-03`, `HP-A1-04`, `HP-A1-06`, and `HP-A1-07` broaden the surface into auth contract-update, declared failure behavior, object package attach, and detach-minimal policy; those are valid later within `PA1-4`, but they are not the narrowest safe first cut.

- Finding 3:
  The hot-plug negative coverage currently under-specifies the `specs/18` requirements. `specs/18-practical-alpha1-scope.md` requires typed reject for incompatible patch, missing capability, missing witness, and stale membership in the package/hot-plug stage, but the current `08-sample-matrix.md` hot-plug section names only non-admin reject and incompatible patch reject. `HP-A1-02` can plausibly stand in for missing-capability/admin failure, but there is no explicit `HP-*` row yet for missing witness or stale membership at attach time. Those negatives need dedicated practical hot-plug rows or equivalently explicit attach-time cases; reusing `RUN-02` is not enough because that row is a local-runtime dispatch rejection, not a hot-plug admission/verdict rejection.

- Finding 4:
  To avoid a sample-ID-only bridge, the minimum command/test shape for `P-A1-04` must mirror the current anti-bridge pattern already present in the checker/runtime lanes: a direct package-path command, a Rust library/API test over actual package directories, a script test that normalizes bare package paths into the attach/check command, and at least one temp-mutated fixture test proving the attach verdict depends on manifest contents rather than the curated sample name. The current checker and local-runtime scripts already normalize direct package paths (`scripts/practical_alpha1_check.py`, `scripts/practical_alpha1_run_local.py`), and `crates/mir-runtime/tests/practical_alpha1_local_runtime.rs` already has the right pattern with a mutated temporary package that flips checker verdict and proves the runtime path is package-content-driven. `P-A1-04` should add the hot-plug analogue rather than only a `run HP-A1-01` style helper.

## Skipped validations and reasons

- Cargo and Python tests: not run because the task was a coverage-shape review of upcoming `P-A1-04`, not a verification request for current implementations
- `python3 scripts/validate_docs.py`: not run because an unrelated existing draft report is already known to break the latest-report template check, so rerunning would not add signal about the reviewed hot-plug boundary

## Commit / push status

No commit; review-only task.

## Sub-agent session close status

- No sub-agents used.
