# Report 1170 — P-A1-12 Practical Fallback Degradation Export

- Date: 2026-05-04 15:15 JST
- Author / agent: Codex
- Scope: `P-A1-12` practical fallback degradation export widening
- Decision levels touched: L1 practical alpha-1 package sequencing, L2 practical devtools export boundary wording

## Objective

Close `P-A1-12` by widening the practical devtools lane with `VIS-A1-05` only, using exact existing `AV-A1-03` avatar fallback evidence as the source carrier.

## Scope and assumptions

- Actualize only `VIS-A1-05`.
- Keep the widened devtools floor exact-report-based and non-final.
- Do not add runtime semantics, parser/front-door widening, or new carrier kinds outside the existing practical devtools/export helper path.
- Do not claim native execution, unsupported-runtime execution success, same-session runtime attach/detach execution, full membership timeline completion, retention/on-demand completion, full product prototype completion, or final public viewer/telemetry ABI.
- Do not promote `samples/practical-alpha1/` to an active runnable root.
- Sub-agent review was not used because the current tool policy for this session does not authorize delegation without explicit user instruction.

## Start state / dirty state

The worktree started dirty with in-flight `P-A1-12` edits in `scripts/practical_alpha1_export_devtools.py`, `scripts/tests/test_practical_alpha1_export_devtools.py`, and an untracked expected bundle for `VIS-A1-05`. Snapshot docs still reflected `P-A1-11` as the latest practical package in `tasks.md` and `samples_progress.md`, while `progress.md` had partial `P-A1-12` synchronization.

## Documents consulted

- `README.md`
- `Documentation.md`
- `AGENTS.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00..03`
- `specs/09`
- `specs/18-practical-alpha1-scope.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/44-practical-alpha1-roadmap.md`
- `samples/README.md`
- `samples/practical-alpha1/README.md`
- `samples/practical-alpha1/expected/README.md`
- `scripts/README.md`
- `docs/reports/1165-p-a1-09-practical-hotplug-lifecycle-export.md`
- `docs/reports/1168-p-a1-10-practical-avatar-preview-floor.md`
- `docs/reports/1169-p-a1-11-product-preview-avatar-companion-widening.md`
- `docs/reports/TEMPLATE.md`

## 日本語要約

`P-A1-12` では、`VIS-A1-05` だけを practical devtools floor に追加しました。新しい runtime 実行は増やしておらず、既存の `AV-A1-03` exact avatar preview report をそのまま source carrier として使い、rejected source lane、degraded roles、missing host capability を維持した fallback degradation export bundle を作っただけです。つまり、これは「unsupported runtime でも実行できた」という意味ではなく、「実行できなかったことと fallback の形が devtools/export 側で正確に見える」ことだけを narrow に actualize した package です。

## Actions taken

1. Widened `scripts/practical_alpha1_export_devtools.py` so `VIS-A1-05` is emitted as `fallback_degradation_export` over exact `AV-A1-03` evidence.
2. Added focused test coverage in `scripts/tests/test_practical_alpha1_export_devtools.py` for exact avatar fallback report consumption.
3. Added committed exact expected bundle `samples/practical-alpha1/expected/vis-a1-05-fallback-degradation.expected.json`.
4. Synchronized practical alpha-1 docs, roadmap memory, and sample dashboard wording so `PA1-6` now reads as `VIS-A1-01/02/04/05/06` actualized with `VIS-A1-03/07` deferred.
5. Reran the focused practical validation floor and snapshot/document validators.

## Files changed

- `README.md`
- `Documentation.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/01-status-at-a-glance.md`
- `plan/44-practical-alpha1-roadmap.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `samples/practical-alpha1/README.md`
- `samples/practical-alpha1/expected/README.md`
- `scripts/README.md`
- `scripts/practical_alpha1_export_devtools.py`
- `scripts/tests/test_practical_alpha1_export_devtools.py`
- `samples/practical-alpha1/expected/vis-a1-05-fallback-degradation.expected.json`
- `docs/reports/1170-p-a1-12-practical-fallback-degradation-export.md`

## Commands run

```bash
date '+%Y-%m-%d %H:%M %Z'
python3 -m unittest scripts.tests.test_practical_alpha1_avatar scripts.tests.test_practical_alpha1_export_devtools scripts.tests.test_validate_docs
python3 scripts/practical_alpha1_avatar.py check-all --format json
python3 scripts/practical_alpha1_export_devtools.py check-all --format json
python3 scripts/practical_alpha1_export_devtools.py closeout --format json
python3 scripts/practical_alpha1_export_devtools.py render-html VIS-A1-05 --format json
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
git status --short
```

## Evidence / outputs / test results

- `date` returned `2026-05-04 15:15 JST`.
- `python3 -m unittest ...` passed `23` tests.
- `python3 scripts/practical_alpha1_avatar.py check-all --format json` passed `AV-A1-01/02/03` with `unsupported_runtime_fallback_present = true` and `native_execution_claimed = false`.
- `python3 scripts/practical_alpha1_export_devtools.py check-all --format json` passed `VIS-A1-01/02/04/05/06` and kept `deferred_observables = ["VIS-A1-03", "VIS-A1-07"]`.
- `python3 scripts/practical_alpha1_export_devtools.py closeout --format json` reported the implemented rows `VIS-A1-01/02/04/05/06` and preserved the non-final stop lines.
- `python3 scripts/practical_alpha1_export_devtools.py render-html VIS-A1-05 --format json` produced an HTML bundle that shows the rejected source lane, degraded roles, and missing host capability at the viewer boundary.
- `python3 scripts/check_source_hierarchy.py` passed with `73/73` required paths present.
- `python3 scripts/validate_docs.py` reported `Documentation scaffold looks complete.` and `Found 1171 numbered report(s).`
- `cargo fmt --check` and `git diff --check` were clean after the package edits.

## What changed in understanding

`VIS-A1-05` was safe to promote without a new semantics cut because the exact practical `AV-A1-03` carrier already contained the needed non-execution facts: rejected hot-plug source outcome, missing host capability, visible fallback representation, and degraded role inventory. That makes `VIS-A1-05` a consumer-side export widening rather than a runtime or avatar package widening.

## Open questions

- Whether `VIS-A1-03` membership timeline can be widened next from exact existing practical carriers without inventing new timeline semantics.
- Whether `VIS-A1-07` retention/on-demand trace can be widened next without creating a non-exact synthetic viewer lane.
- Whether broader save/load widening should precede the remaining devtools rows if exact timeline/retention carriers are still too weak.

## Suggested next prompt

Continue from `P-A1-12` and determine whether `VIS-A1-03` or `VIS-A1-07` can be actualized next from exact existing practical carriers; if neither is honest, write a blocker report and keep the practical alpha-1 queue narrow.

## Plan update status

`plan/` 更新済み: `plan/01-status-at-a-glance.md` と `plan/44-practical-alpha1-roadmap.md` を `P-A1-12` closeout wordingへ同期した。

## Documentation.md update status

`Documentation.md` 更新済み: `P-A1-12` を latest practical package として反映した。

## progress.md update status

`progress.md` 更新済み: `PA1-6` を `70%` に更新し、`P-A1-12` recent log を追加した。

## tasks.md update status

`tasks.md` 更新済み: latest practical package, next reopen point, and `PA1-6` row were synchronized to `VIS-A1-05`.

## samples_progress.md update status

`samples_progress.md` 更新済み: `VIS-A1-05` row actualization, validation timestamp, report index, and practical package snapshot were synchronized.

## Reviewer findings and follow-up

- No delegated reviewer sessions were opened because the current session policy does not authorize sub-agent delegation without explicit user instruction.
- Performed a local focused review instead:
  - checked that `VIS-A1-05` reuses only exact `AV-A1-03` evidence,
  - checked that the bundle keeps the rejected source lane explicit,
  - checked that docs do not overclaim native execution or unsupported-runtime success.

## Skipped validations and reasons

- Skipped Rust behavior tests beyond `cargo fmt --check` because `P-A1-12` changed only Python export/helper logic, expected JSON, and documentation. No Rust crate, runtime semantics, or practical Cargo-floor implementation was touched in this package.
- Skipped product-preview, transport, hot-plug, and save/load reruns because their source carriers were not modified and the focused `AV-A1-03` plus devtools export validation floor was sufficient for this exact widening.

## Commit / push status

Pending at report write.

## Sub-agent session close status

No sub-agent sessions were opened in this package.
