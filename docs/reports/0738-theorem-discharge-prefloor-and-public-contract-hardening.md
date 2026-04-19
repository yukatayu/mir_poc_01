# Report 0738 — theorem discharge prefloor and public contract hardening

## 1. Objective

row-local `proof_notebook_review_unit`、abstract discharge-entry reserve、transport seam、public-contract seam を、
final adoption に上げずに representative runtime/static/guarded/prototype corpus で compare できる helper-local pre-floor として actualize する。

## 2. Scope and assumptions

- scope は current L2 の test/support helper と docs-first theory-lab line に限定する。
- public verifier contract / public theorem contract / concrete theorem prover binding / proof object public schema は fixed しない。
- principal source は row-local `proof_notebook_review_unit` に維持する。

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `.docs/progress-task-axes.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/419-current-l2-first-theorem-lemma-family-wording-hardening.md`
- `specs/examples/434-current-l2-admissible-evidence-contraction-note.md`
- `specs/examples/440-current-l2-notebook-consumer-threshold-and-discharge-reserve-note.md`
- `specs/examples/446-current-l2-theorem-discharge-transport-and-public-contract-later-gate-framing-note.md`
- `specs/examples/463-current-l2-verifier-preview-alignment-prefloor-and-public-contract-mixed-gate-note.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`

## 4. Actions taken

1. notebook-first threshold / discharge reserve / later-gate framing docs を読み直し、principal source を row-local review unit に固定した。
2. RED として `crates/mir-runtime/tests/current_l2_theorem_discharge_prefloor.rs` を追加し、未実装 import failure を確認した。
3. `crates/mir-runtime/tests/support/current_l2_source_sample_emitted_artifact_support.rs` に sample-local theorem discharge pre-floor route を追加した。
4. representative corpus `e5 / p05 / p06 / p07 / p08` で、review-unit / discharge-entry / transport-public seam の compare floor を test で固定した。
5. `specs/examples/465-current-l2-theorem-discharge-prefloor-and-public-contract-mixed-gate-note.md` を追加し、helper-local compare floor と stop line を source-backed に書き上げた。
6. `specs/12-decision-register.md` に `D-057` を追加し、mixed-gate concretization 前の compare hardening judgment として登録した。

## 5. Files changed

- Added: `crates/mir-runtime/tests/current_l2_theorem_discharge_prefloor.rs`
- Added: `specs/examples/465-current-l2-theorem-discharge-prefloor-and-public-contract-mixed-gate-note.md`
- Modified: `crates/mir-runtime/tests/support/current_l2_source_sample_emitted_artifact_support.rs`
- Modified: `specs/12-decision-register.md`

## 6. Commands run and exact outputs

1. RED confirmation

```text
$ cargo test -p mir-runtime --test current_l2_theorem_discharge_prefloor
error[E0432]: unresolved imports ... CurrentL2SourceSampleTheoremDischargePrefloor ... build_current_l2_source_sample_theorem_discharge_prefloor
error: could not compile `mir-runtime` (test "current_l2_theorem_discharge_prefloor") due to 1 previous error
```

2. GREEN confirmation

```text
$ cargo test -p mir-runtime --test current_l2_theorem_discharge_prefloor
running 5 tests
test theorem_discharge_prefloor_keeps_guarded_prototype_as_not_reached ... ok
test theorem_discharge_prefloor_reaches_order_handoff_runtime_prototype ... ok
test theorem_discharge_prefloor_reaches_static_underdeclared_sample ... ok
test theorem_discharge_prefloor_reaches_stale_reconnect_runtime_prototype ... ok
test theorem_discharge_prefloor_reaches_typed_runtime_prototype_without_premature_public_contract_adoption ... ok

test result: ok. 5 passed; 0 failed
```

## 7. Evidence / findings

- helper-local pre-floor を追加しても principal source は row-local review unit に保てた。
- guarded sample `p05` は theorem-success へ寄せず、`guard:discharge_not_reached` のまま残せた。
- runtime/static/guarded/prototype corpus を同じ compare floor で扱えるため、transport seam と public-contract seam を later gate の distinct marker として機械的に維持できる。
- この package で fixed したのは compare floor だけであり、actual discharge transport adoption と public theorem contract adoption は依然 mixed gate のままである。

## 8. What changed in understanding

- current repo では、theorem mixed gate を reopen する前に、review-unit floor と discharge/public-contract seam を helper-local compare floor まで actualize しても public widening は起きない。
- `verification_preview` / `artifact_preview` だけでは見えにくかった discharge-entry reserve と transport/public-contract seam の distinctness を、representative corpus 上の machine-check へ落とせることが確認できた。

## 9. Open questions

- actual discharge transport を actual adoption に送る mixed gate をどこに置くか。
- public theorem contract を public verifier / checker migration とどう切り離すか。
- concrete theorem prover binding と proof object public schema をどの順で reopen するか。

## 10. Suggested next prompt

`specs/examples/459`、`464`、`465` を起点に、stronger typed-surface actual adoption を mixed gate として reopen するか、あるいは theorem / model-check の actual adoption gate をどちらから先に切るかを docs-first / helper-local compare 付きで詰めてください。
