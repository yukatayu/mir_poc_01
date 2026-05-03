# Report 1148 — Review P-A1-04c Detach Contract Theory Boundary

## 1. Title and identifier

- `1148-review-p-a1-04c-detach-contract-theory-boundary`

## 2. Objective

- Review `P-A1-04c` from a theory/spec perspective.
- Decide whether the practical-alpha1 detach minimal contract should be modeled primarily as `rejected` or `deferred`.
- Keep the non-claims explicit:
  - no actual detach lifecycle completion
  - no public ABI freeze
  - no transport completion
  - no save/load completion

## 3. Scope and assumptions

- Review-only task except for this required report.
- Scope is limited to:
  - `specs/18-practical-alpha1-scope.md`
  - `plan/30-attachpoint-detach-minimal-contract.md`
  - `plan/44-practical-alpha1-roadmap.md`
  - current practical hot-plug code/tests/docs
- The user prompt names `HP-A1-07`, but the current repository only reserves that row in the sample matrix; no practical fixture/expected report/script inventory currently defines it.
- The question is not whether a full detach runtime should exist now. The question is what the smallest honest contract surface is for `P-A1-04c`.

## 4. Start state / dirty state

- Working tree was already dirty in the `P-A1-04c` implementation lane.
- Pre-existing tracked changes at review end are present in:
  - `crates/mir-ast/src/practical_alpha1.rs`
  - `crates/mir-ast/src/practical_alpha1_hotplug_plan.rs`
  - `crates/mir-ast/tests/practical_alpha1_hotplug_plan.rs`
  - `crates/mir-runtime/src/practical_alpha1_hotplug.rs`
  - `crates/mir-runtime/tests/practical_alpha1_hotplug.rs`
  - `samples/practical-alpha1/expected/hp-a1-01-debug-layer-attach.expected.json`
  - `samples/practical-alpha1/expected/hp-a1-02-non-admin-debug-rejected.expected.json`
  - `samples/practical-alpha1/expected/hp-a1-03-auth-layer-contract-update.expected.json`
  - `samples/practical-alpha1/expected/hp-a1-04-ratelimit-declared-failure.expected.json`
  - `samples/practical-alpha1/expected/hp-a1-05-incompatible-patch-rejected.expected.json`
  - `scripts/practical_alpha1_attach.py`
  - `scripts/tests/test_practical_alpha1_attach.py`
- Pre-existing untracked `P-A1-04c` artifacts at review end are present in:
  - `samples/practical-alpha1/packages/hp-a1-07-detach-minimal-contract/`
  - `docs/reports/1147-p-a1-04c-sample-validation-detach-review.md`
  - `docs/reports/1147-review-pa1-04c-docs-progress-consistency.md`
- This review adds only `docs/reports/1148-review-p-a1-04c-detach-contract-theory-boundary.md`.

## 5. Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/16-runtime-package-adapter-hotplug.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/00-index.md`
- `plan/30-attachpoint-detach-minimal-contract.md`
- `plan/44-practical-alpha1-roadmap.md`
- `progress.md`
- `tasks.md`
- `sub-agent-pro/alpha-1/08-sample-matrix.md`
- `crates/mir-ast/src/practical_alpha1.rs`
- `crates/mir-ast/src/practical_alpha1_hotplug_plan.rs`
- `crates/mir-runtime/src/hotplug_runtime.rs`
- `crates/mir-runtime/src/practical_alpha1_hotplug.rs`
- `crates/mir-runtime/tests/hotplug_runtime_skeleton.rs`
- `crates/mir-runtime/tests/practical_alpha1_hotplug.rs`
- `scripts/practical_alpha1_attach.py`
- `scripts/tests/test_practical_alpha1_attach.py`
- `samples/practical-alpha1/README.md`
- `samples/practical-alpha1/expected/hp-a1-06-object-package-attach.expected.json`
- `docs/reports/1147-review-pa1-04c-docs-progress-consistency.md`
- `docs/reports/1147-p-a1-04c-sample-validation-detach-review.md`

## 6. Actions taken

1. Read the required repository front-door documents in AGENTS order.
2. Re-read the practical-alpha1 normative boundary in `specs/18` and the detach memory in `plan/30`.
3. Compared the practical sample matrix against the implemented hot-plug runner inventory.
4. Inspected the current practical hot-plug plan/report carriers and the generic hot-plug runtime skeleton.
5. Ran focused commands to confirm:
   - the public practical attach closeout still excludes detach
   - the generic runtime skeleton admits `detach + deferred`
   - the current practical hot-plug crate has unvalidated detach-path compile drift on fresh rebuild

## 7. Files changed

- `docs/reports/1148-review-p-a1-04c-detach-contract-theory-boundary.md`

## 8. Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git status --short
rg --files README.md Documentation.md progress.md tasks.md samples_progress.md specs plan sub-agent-pro
sed -n '1,260p' README.md
sed -n '1,260p' Documentation.md
sed -n '1,260p' specs/00-document-map.md
sed -n '1,260p' specs/01-charter-and-decision-levels.md
sed -n '1,260p' specs/02-system-overview.md
sed -n '1,260p' specs/03-layer-model.md
sed -n '1,260p' specs/09-invariants-and-constraints.md
sed -n '1,260p' specs/18-practical-alpha1-scope.md
sed -n '261,420p' specs/18-practical-alpha1-scope.md
sed -n '1,260p' plan/00-index.md
sed -n '1,260p' plan/30-attachpoint-detach-minimal-contract.md
sed -n '1,260p' plan/44-practical-alpha1-roadmap.md
sed -n '1,220p' sub-agent-pro/alpha-1/08-sample-matrix.md
sed -n '1,260p' crates/mir-ast/src/practical_alpha1_hotplug_plan.rs
sed -n '1,260p' crates/mir-runtime/src/practical_alpha1_hotplug.rs
sed -n '1,260p' crates/mir-runtime/tests/practical_alpha1_hotplug.rs
sed -n '1,260p' scripts/practical_alpha1_attach.py
sed -n '1,220p' scripts/tests/test_practical_alpha1_attach.py
python3 scripts/practical_alpha1_attach.py closeout --format json
cargo test -p mir-runtime --test hotplug_runtime_skeleton -- --nocapture
cargo test -p mir-runtime --test practical_alpha1_hotplug -- --nocapture
date '+%Y-%m-%d %H:%M:%S %Z'
python3 scripts/validate_docs.py
git diff --check
git status --short
```

## 9. Evidence / outputs / test results

- `specs/18` currently says two things simultaneously:
  - the current practical floor still does **not** complete detach minimal contract (`specs/18-practical-alpha1-scope.md:168-176`)
  - the package/hot-plug stage eventually requires detach minimal contract to be explicit as at least one of reject/defer/fallback (`specs/18-practical-alpha1-scope.md:185-190`)
- `plan/30` models the existing detach memory as:
  - explicit detach boundary
  - accepted `detach_request#1`
  - post-detach domain-action rejection
  - migration/rollback still deferred
  (`plan/30-attachpoint-detach-minimal-contract.md:23-29`, `46-50`, `69-72`)
- `sub-agent-pro/alpha-1/08-sample-matrix.md:34-40` reserves `HP-A1-07` as `detach-minimal reject/defer/fallback`, not as a settled reject-only row.
- The in-progress working tree already introduces `samples/practical-alpha1/packages/hp-a1-07-detach-minimal-contract/package.mir.json` with:
  - `operation_kind = detach`
  - a layer package only
  - notes stating that detach is explicitly rejected at the current practical floor
- The practical attach runner still exposes only `HP-A1-01..05`, `HP-A1-04B1`, `HP-A1-04B2`, and `HP-A1-06`; no `HP-A1-07` fixture is live in the runner inventory (`scripts/practical_alpha1_attach.py:16-65`).
- The runner closeout still reports:
  - `implemented_rows = HP-A1-01..05, HP-A1-04B1, HP-A1-04B2, HP-A1-06`
  - `limitations = no detach-minimal contract, Docker/local TCP, save/load, or final public ABI`
  - `stage_pa1_4_complete = false`
- The current practical hot-plug carrier already contains `operation_kind` with `attach|detach` in the front-door and plan surface (`crates/mir-ast/src/practical_alpha1.rs:264-291`, `crates/mir-ast/src/practical_alpha1_hotplug_plan.rs:59-77`, `223-230`).
- The in-progress diff also renames practical retained-later wording from `detach_minimal_contract` to `detach_runtime_execution` in plan/report constants, even though the review question is still whether the minimal contract itself should be modeled as reject-only or deferred.
- The generic runtime skeleton already admits detach-specific narrow states:
  - `detach_ready_for_boundary_cut`
  - `detach_rejected_before_boundary`
  - `detach_deferred_before_boundary`
  (`crates/mir-runtime/src/hotplug_runtime.rs:228-234`)
- The dedicated generic test passed:
  - `cargo test -p mir-runtime --test hotplug_runtime_skeleton -- --nocapture`
  - includes `hotplug_runtime_engine_report_maps_detach_deferred_to_boundary_state`
- Current practical hot-plug code has already hard-coded a reject-only detach branch:
  - `terminal_outcome = rejected_detach_minimal_contract`
  - `reason_family = detach_contract`
  - `rejection_reason_refs = [detach_minimal_contract_reject_only]`
  - `verdict_kind = rejected`
  (`crates/mir-runtime/src/practical_alpha1_hotplug.rs:573-587`)
- Fresh rebuild of the practical hot-plug target currently fails:
  - `cargo test -p mir-runtime --test practical_alpha1_hotplug -- --nocapture`
  - error `E0425`: missing type `LayerInsertionRuntimeReport` in `crates/mir-runtime/src/practical_alpha1_hotplug.rs:552`
  - this means the current detach-path shaping is not covered by fresh practical-hotplug validation
- `python3 scripts/validate_docs.py` passed after adding this report.
- `git diff --check` was clean after adding this report.

## 10. What changed in understanding

- The repository is no longer choosing between purely hypothetical options. A reject-only detach outcome is already partially encoded in Rust, but it is not mirrored into the sample matrix, practical runner inventory, or tests.
- The better comparison is therefore not “reject or defer in the abstract,” but “keep the hidden reject-only decision, or realign the practical row with the existing detach-boundary memory.”
- The current generic runtime substrate is already capable of expressing a `detach + deferred` boundary state without claiming completed lifecycle semantics.

## 11. Open questions

- Should `HP-A1-07` be introduced explicitly as a new practical sample row, or should the closeout remain named only as `P-A1-04c` until the sample taxonomy is updated everywhere?
- If `HP-A1-07` is added, should the first row be layer-only, mirroring the current `P-A1-04c` boundary and keeping object detach strictly later?
- Does the repository want a later second negative detach row for “active dependents force reject,” separate from the first minimal deferred boundary row?

## 12. Suggested next prompt

- `Implement P-A1-04c as a layer-only practical detach boundary row that uses operation_kind=detach and verdict_kind=deferred, add one exact HP-A1-07 fixture/expected report plus focused Rust/Python tests, remove the hidden reject-only default, and keep docs explicit that rollback/migration/public ABI/transport/save-load remain later.`

## 13. `plan/` update status

- `plan/` 更新不要

## 14. `Documentation.md` update status

- `Documentation.md` 更新不要

## 15. `progress.md` update status

- `progress.md` 更新不要

## 16. `tasks.md` update status

- `tasks.md` 更新不要

## 17. `samples_progress.md` update status

- `samples_progress.md` 更新不要

## 18. reviewer findings and follow-up

- Finding 1:
  The current repo has already made a semantic choice in the in-progress `P-A1-04c` diff, but not in the spec-facing artifacts. `specs/18` leaves detach minimal contract open as at least one of reject/defer/fallback (`specs/18-practical-alpha1-scope.md:190`), and the practical sample matrix still names `HP-A1-07` as `reject/defer/fallback` (`sub-agent-pro/alpha-1/08-sample-matrix.md:34-40`). However, `crates/mir-runtime/src/practical_alpha1_hotplug.rs:573-587` already hard-codes a reject-only detach outcome, and the untracked `hp-a1-07` fixture text mirrors that choice. This is a source-hierarchy violation in substance: an unresolved or at least undocumented choice has been silently promoted into implementation. Follow-up: either mirror reject-only everywhere and justify it explicitly, or revert this hidden choice and reopen the row honestly.

- Finding 2:
  Reject-only is the weaker theory fit than deferred for the first practical row. `plan/30` anchors detach memory as an explicit boundary plus deferred migration contract, not as “detach unsupported” (`plan/30-attachpoint-detach-minimal-contract.md:23-29`, `46-50`, `69-72`). The generic runtime skeleton already supports `detach_deferred_before_boundary` (`crates/mir-runtime/src/hotplug_runtime.rs:232-234`, `crates/mir-runtime/tests/hotplug_runtime_skeleton.rs:152-174`). A first practical `deferred` row would therefore preserve the non-claims while still representing detach as a recognized lifecycle boundary. A first practical `rejected` row instead collapses “boundary acknowledged but incomplete” into “request denied,” which throws away the detach-boundary reading the repo has already been using elsewhere.

- Finding 3:
  The practical hot-plug carrier is still too thin to justify an accepted detach lifecycle, but it is already sufficient for a deferred boundary row. The report surface has no lifecycle-state row, no post-detach action row, and no migration-contract row (`crates/mir-runtime/src/practical_alpha1_hotplug.rs:130-175`, `338-375`). That means `accepted detach` would overclaim. But the same carrier already supports:
  - `operation_kind`
  - membership/capability/witness lanes
  - non-final retained-later refs
  - runtime skeleton verdict/state projection
  So the narrowest honest move is `deferred`, not `accepted`, and not primary `rejected`.

- Finding 4:
  Current validation does not support trusting the hidden reject-only implementation. The runner inventory still ends at `HP-A1-06` (`scripts/practical_alpha1_attach.py:16-65`), Python unit tests know nothing about a detach row (`scripts/tests/test_practical_alpha1_attach.py:14-56`), Rust practical hot-plug tests know nothing about a detach row (`crates/mir-runtime/tests/practical_alpha1_hotplug.rs:27-186`), and a fresh rebuild of the practical hot-plug test target currently fails with `E0425` at `crates/mir-runtime/src/practical_alpha1_hotplug.rs:552`. Follow-up: do not treat the reject-only branch as shipped semantics until the build break and test gap are fixed.

- Finding 5:
  Naming drift remains a concrete risk. `HP-A1-07` is only reserved in the sample matrix today, while `P-A1-07` already names the later save/load package (`plan/44-practical-alpha1-roadmap.md:131-160`). If `P-A1-04c` closes with prose like “HP-A1-07 rejected detach” but without a synchronized sample/docs introduction, the repo will blur row numbering and package numbering at the exact place where it needs to stay explicit about transport/save-load non-claims.

## 19. skipped validations and reasons

- Did not run transport, save/load, devtools, or product-prototype validation because the task was a detach-contract review and no claim about those stages was under evaluation.
- Did not create or commit a practical detach fixture because the user requested review/recommendation, not implementation.

## 20. commit / push status

- No commit created.
- No push performed.

## 21. sub-agent session close status

- No sub-agent session used.
