# Report 0810 — Package 58 IFC checker-hint preview actualization

## Objective

Package 58 の IFC helper widening として、
source-side IFC trio `p10 / p11 / p12` を sample-local `typed_checker_hint_preview` に actualize し、
docs-first で積んでいた `family_refs[] + coverage_state` 判断を
helper-local operational CLI へ narrow に mirror する。

## Scope and assumptions

- scope は `mir-current-l2 run-source-sample` の helper-local summary に限定する。
- final public checker artifact、actual checker payload family、final typed source principal、final IFC syntax、final public verifier contract は扱わない。
- `p10 / p11` は authority-release cluster の pair として読み、各 sample 単体では `partial_cluster` に留める。
- `p12` は current source-side label-flow negative first line として `full_cluster` に置く。

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/259-current-l2-checker-cluster-fixture-evidence-attachment-ready-typed-reason-family-hint-threshold.md`
- `specs/examples/260-current-l2-typed-reason-family-hint-ready-checker-cluster-hint-bundle-shape-comparison.md`
- `specs/examples/261-current-l2-checker-cluster-hint-bundle-shape-ready-typed-family-coverage-state-threshold.md`
- `specs/examples/522-current-l2-ifc-secret-valid-invalid-foundation-and-japanese-lean-corpus-sync.md`
- `specs/examples/523-current-l2-source-side-ifc-authority-prototype-pair-and-representative-lean-sample-set-widening.md`
- `specs/examples/524-current-l2-ifc-label-flow-negative-prototype-closeout-and-representative-lean-sample-set-widening.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`

## Actions taken

1. `crates/mir-runtime/tests/current_l2_operational_cli.rs` に
   `typed_checker_hint_preview` の JSON / pretty output expectation を先に追加した。
2. 失敗テストを確認し、preview scope を sample-local helper preview に限定する実装方針を確定した。
3. `crates/mir-runtime/src/current_l2_cli.rs` に
   `typed_checker_hint_preview` summary を追加した。
4. `p10 / p11 / p12` ごとに
   `cluster_kind`、`case_label`、`family_refs[]`、`coverage_state` を固定し、
   target 外 sample は `guarded_not_reached` を返すようにした。
5. `specs/examples/529-current-l2-ifc-typed-checker-hint-preview-actualization.md` を追加し、
   current reading / stop line / retained later を source-backed に整理した。
6. `Documentation.md`、`progress.md`、`tasks.md`、`plan/`、`specs/`、`plan/90-source-traceability.md`
   を更新し、Package 58 の current reading を
   「IFC helper widening actualize 済み、残りは broader theorem-side / diagnostics widening」
   に再同期した。

### Files changed

- `crates/mir-runtime/src/current_l2_cli.rs`
- `crates/mir-runtime/tests/current_l2_operational_cli.rs`
- `specs/examples/529-current-l2-ifc-typed-checker-hint-preview-actualization.md`
- `docs/reports/0810-package58-ifc-checker-hint-preview-actualization.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`

## Evidence / outputs / test results

### Commands run

```bash
git status --short
df -h .
free -h
cargo test -p mir-runtime --test current_l2_operational_cli
cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-typed-proof-model-check/p10-typed-authorized-fingerprint-declassification.txt --format json
cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-typed-proof-model-check/p11-typed-unauthorized-fingerprint-release.txt --format json
cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-typed-proof-model-check/p12-typed-classified-fingerprint-publication-block.txt --format json
python3 scripts/validate_docs.py
git diff --check
```

### Outputs / results

- `git status --short`
  - 実装開始時点では clean だった。
- `df -h .`
  - `/` は 99G 中 82G 使用、残り 13G。
- `free -h`
  - メモリは 960Mi 中 676Mi 使用、swap 19Gi 中 1.7Gi 使用。
- `cargo test -p mir-runtime --test current_l2_operational_cli`
  - 追加後は 15 tests passed。
- `cargo run ... p10 ... --format json`
  - `verification_preview.formal_hook_status = reached`
  - `typed_checker_hint_preview.status = reached`
  - `cluster_kind = ifc_authority_release_cluster`
  - `coverage_state = partial_cluster`
- `cargo run ... p11 ... --format json`
  - `typed_checker_hint_preview.status = reached`
  - `case_label = authority_miss_negative`
  - `coverage_state = partial_cluster`
- `cargo run ... p12 ... --format json`
  - `typed_checker_hint_preview.status = reached`
  - `cluster_kind = ifc_label_flow_cluster`
  - `coverage_state = full_cluster`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 809 numbered report(s).`
- `git diff --check`
  - 差分の whitespace / conflict-marker 問題は無かった。

## What changed in understanding

- `typed_reason_family_hint` の docs-first line は、
  global checker payload family を待たずに
  sample-local helper preview へ narrow に actualize できる段階に来ていた。
- `coverage_state` は docs-only comparison で終わらせるより、
  `p10 / p11` pair を partial、`p12` を full と読むことで
  over-read を抑える役目が実際にあった。
- current IFC trio は already runnable / preview-aligned / theorem/model-check bridge-aligned なので、
  実装上の欠けは final theory ではなく helper/CLI hardening 側に移っている。

## Open questions

- `typed_checker_hint_preview` を beyond IFC trio へどこまで widen するか。
- `supported_kind_refs[]` のような stronger payload を current checker-side line に足すべきか。
- actual checker payload family をどの mixed gate で reopen するか。
- stronger typed source principal / final IFC syntax / final public verifier contract をどの順で reopen するか。

## Suggested next prompt

Package 58 の次段として、`typed_checker_hint_preview` の widening を IFC trio の外へ広げるか、
あるいは broader theorem-side sample export を 1 つ actualize して、
どちらが mixed gate をより多く潰せるかを比較しながら narrow package を 1 本進めてください。
