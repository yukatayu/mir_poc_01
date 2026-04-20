# 0856 — Package 99/100 problem residual bundle matrices

## 目的

Package 99 と Package 100 をまとめて進め、Problem 1 / Problem 2 の residual mixed gate を representative sample bundle 単位で `scripts/current_l2_guided_samples.py` から読めるようにする。

## Scope and assumptions

- sample corpus 自体は増やさず、existing corrected prototype と existing helper summary を再利用する。
- helper は repo-local / non-production cut に留め、final public CLI や final public contract には上げない。
- Problem 1 では theorem/model-check public seam residual、Problem 2 では witness/provider/public-shape residual を狙う。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `samples/prototype/current-l2-typed-proof-model-check/README.md`
- `samples/prototype/current-l2-order-handoff/README.md`

## Actions taken

1. `scripts/current_l2_guided_samples.py` に `matrix` subcommand を追加し、Problem 1 / Problem 2 の residual bundle を sample 単位で pretty/json 表示できるようにした。
2. Problem 1 については、`p06` を public-seam representative、`p10 / p11 / p12 / p15 / p16` を checker-adjacent / bridge-floor bundle として読む matrix を actualize した。
3. Problem 2 については、`p07 / p08` を first-line representative、`p09` を reserve practical route、`p13 / p14` を negative static-stop pair として読む matrix を actualize した。
4. `scripts/tests/test_current_l2_guided_samples.py` を拡張し、Problem 1 / Problem 2 の residual row classification と matrix text を unit test で固定した。
5. `samples/prototype/current-l2-typed-proof-model-check/README.md` と `samples/prototype/current-l2-order-handoff/README.md` に matrix command と読み方を追記した。
6. `specs/examples/573` と `specs/examples/574` を追加し、Package 99 / 100 の source-backed note を作成した。
7. `Documentation.md`、`progress.md`、`tasks.md`、`plan/01`、`plan/11`、`plan/16`、`plan/17`、`plan/18`、`plan/90`、`specs/00`、`specs/11`、`specs/12` を residual bundle matrix closeout と next queue reading に同期した。

## Files changed

- `scripts/current_l2_guided_samples.py`
- `scripts/tests/test_current_l2_guided_samples.py`
- `samples/prototype/current-l2-typed-proof-model-check/README.md`
- `samples/prototype/current-l2-order-handoff/README.md`
- `specs/examples/573-current-l2-problem1-public-seam-residual-bundle-matrix.md`
- `specs/examples/574-current-l2-problem2-public-shape-residual-bundle-matrix.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `specs/00-document-map.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`

## Commands run

- `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
- `python3 scripts/current_l2_guided_samples.py matrix problem1`
- `python3 scripts/current_l2_guided_samples.py matrix problem1 --format json`
- `python3 scripts/current_l2_guided_samples.py matrix problem2`
- `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-typed-proof-model-check/p06-typed-proof-owner-handoff.txt --format json`
- `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-typed-proof-model-check/p11-typed-unauthorized-fingerprint-release.txt --format json`

## Evidence / outputs / test results

- `matrix problem1` は `p06` を `public-seam representative`、`p10 / p11 / p12 / p15 / p16` を `checker-adjacent bridge-floor` として表示した。
- `matrix problem1 --format json` は six-row residual bundle を JSON で出し、Problem 1 residual mixed gate を sample-facing に見えるようにした。
- `matrix problem2` は `p07 / p08` を `first-line representative`、`p09` を `reserve practical route`、`p13 / p14` を `negative static-stop` として表示した。
- unit test は 9 tests 全て green。
- direct `run-source-sample` probe により、Problem 1 typed bundle が theorem/model-check public seam では bridge-only、typed checker hint では reached という current cut を機械的に再確認した。

## What changed in understanding

- Problem 1 / Problem 2 とも、remaining residual mixed gate は compare doc を増やさなくても representative sample bundle matrix だけでかなり明瞭に読める。
- Problem 1 では `p06` representative と first strong typing quintet の役割差が helper 上で明確になり、premature public-seam widening を避けやすくなった。
- Problem 2 では first completion line / reserve practical route / negative static-stop pair を 1 表で見せることで、public-shape residual を final public contract と混同しにくくなった。

## Open questions

- theorem-first pilot を repo-local emitted artifact / Lean sample corpus / residual matrix まで含めた one-bundle guide にどこまで寄せるか。
- authoritative-room first scenario を residual matrix の先で repo-local scenario bundle としてどこまでまとめるか。
- final public theorem/model-check/witness-provider contract、final parser/public API、packaging/FFI/engine adapter は still later である。

## Suggested next prompt

theorem-first pilot bundle、または authoritative-room scenario bundle のどちらかを next self-driven package として actualize してください。
