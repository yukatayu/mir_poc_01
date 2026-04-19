# Report 0737 — model check projection prefloor and tool seam hardening

## 1. Objective

row-local `model_check_concrete_carriers`、small-cluster projection reserve、property-language seam、tool-binding seam を、
final adoption に上げずに representative runtime/static/guarded/prototype corpus で compare できる helper-local pre-floor として actualize する。

## 2. Scope and assumptions

- scope は current L2 の test/support helper と docs-first theory-lab line に限定する。
- public CLI / public verifier contract / settled property language / concrete tool binding は fixed しない。
- principal source は row-local machine-facing `model_check_concrete_carriers` に維持する。

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
- `specs/examples/415-current-l2-model-check-projection-and-property-family-reserve-inventory.md`
- `specs/examples/420-current-l2-model-check-carrier-to-projection-bridge-note.md`
- `specs/examples/427-current-l2-sample-visible-theorem-model-check-property-summary-wording.md`
- `specs/examples/435-current-l2-tool-binding-stop-line-refresh.md`
- `specs/examples/441-current-l2-model-check-small-cluster-projection-keep-drop-refresh.md`
- `specs/examples/447-current-l2-model-check-property-language-and-tool-binding-later-gate-framing-note.md`
- `specs/examples/463-current-l2-verifier-preview-alignment-prefloor-and-public-contract-mixed-gate-note.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`

## 4. Actions taken

1. preview-aligned route と existing carrier support を読み、mixed gate の principal source を row-local carrier に固定した。
2. RED として `crates/mir-runtime/tests/current_l2_model_check_projection_prefloor.rs` を追加し、未実装 import failure を確認した。
3. `crates/mir-runtime/tests/support/current_l2_source_sample_emitted_artifact_support.rs` に sample-local model-check projection pre-floor route を追加した。
4. representative corpus `e5 / p05 / p06 / p07 / p08` で、carrier / projection / property-tool seam の compare floor を test で固定した。
5. `specs/examples/464-current-l2-model-check-projection-prefloor-and-property-tool-seam-mixed-gate-note.md` を追加し、helper-local compare floor と stop line を source-backed に書き上げた。
6. `specs/12-decision-register.md` に `D-056` を追加し、mixed-gate concretization 前の compare hardening judgment として登録した。

## 5. Files changed

- Added: `crates/mir-runtime/tests/current_l2_model_check_projection_prefloor.rs`
- Added: `specs/examples/464-current-l2-model-check-projection-prefloor-and-property-tool-seam-mixed-gate-note.md`
- Modified: `crates/mir-runtime/tests/support/current_l2_source_sample_emitted_artifact_support.rs`
- Modified: `specs/12-decision-register.md`

## 6. Commands run and exact outputs

1. RED confirmation

```text
$ cargo test -p mir-runtime --test current_l2_model_check_projection_prefloor
error[E0432]: unresolved imports ... CurrentL2SourceSampleModelCheckProjectionPrefloor ... build_current_l2_source_sample_model_check_projection_prefloor
error: could not compile `mir-runtime` (test "current_l2_model_check_projection_prefloor") due to 1 previous error
```

2. GREEN confirmation

```text
$ cargo test -p mir-runtime --test current_l2_model_check_projection_prefloor
running 5 tests
test model_check_projection_prefloor_keeps_guarded_prototype_as_not_reached ... ok
test model_check_projection_prefloor_reaches_order_handoff_runtime_prototype ... ok
test model_check_projection_prefloor_reaches_static_underdeclared_sample ... ok
test model_check_projection_prefloor_reaches_stale_reconnect_runtime_prototype ... ok
test model_check_projection_prefloor_reaches_typed_runtime_prototype_without_collapsing_theorem_or_typed_lines ... ok

test result: ok. 5 passed; 0 failed
```

## 7. Evidence / findings

- helper-local pre-floor を追加しても principal source は row-local carrier に保てた。
- guarded sample `p05` は empty-success へ寄せず、`guard:projection_not_reached` のまま残せた。
- runtime/static/guarded/prototype corpus を同じ compare floor で扱えるため、property-language seam と tool-binding seam を later gate の distinct marker として機械的に維持できる。
- この package で fixed したのは compare floor だけであり、settled property language adoption と concrete tool seam adoption は依然 mixed gate のままである。

## 8. What changed in understanding

- current repo では、model-check mixed gate を reopen する前に、projection reserve と later-gate seam を helper-local compare floor まで actualize しても public widening は起きない。
- `verification_preview` / `artifact_preview` だけでは見えにくかった property-language seam / tool-binding seam の distinctness を、representative corpus 上の machine-check へ落とせることが確認できた。

## 9. Open questions

- first settled property language を actual adoption に送る mixed gate をどこに置くか。
- concrete tool brand / schema をどの line で reopen するか。
- production checker / runtime-policy contract を model-check line の principal source に近づける順序をどう切るか。

## 10. Suggested next prompt

`specs/examples/446` と current helper-local preview chain を起点に、theorem discharge / public-contract 側でも row-local review unit / discharge-entry reserve / transport-public seam を distinct な helper-local pre-floor として機械検証し、mixed gate 前の compare floor を揃えてください。
