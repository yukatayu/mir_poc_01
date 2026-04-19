# Report 0754 — model-check second-line concretization

- Date: 2026-04-18T11:59:00Z
- Author / agent: Codex (GPT-5)
- Scope: model-check second line を compare floor から helper-local actualization floor に上げ、queue / snapshot / docs chain を mixed-gate-only reading へ同期する
- Decision levels touched: `L2` current recommendation / actualization judgment。`L0/L1` invariant は変更していない

## 1. Objective

- model-check second-line concretization を self-driven package として close する。
- row-local property preview / brand-neutral request preflight / public-checker second reserve split を helper-local actualization に上げる。
- `p09` practical provider lineが入っても model-check line が fairness/provider family を collapse しないことを machine-check する。

## 2. Scope and assumptions

- first settled property language は扱わない。
- concrete model-check tool brand / schema は扱わない。
- actual public checker migration は扱わない。
- actual emitted verifier handoff artifact は扱わない。
- production checker / runtime-policy contract は扱わない。
- helper-local second-line manifest は final public verifier contract に昇格しない。

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/415-current-l2-model-check-projection-and-property-family-reserve-inventory.md`
- `specs/examples/420-current-l2-model-check-carrier-to-projection-bridge-note.md`
- `specs/examples/464-current-l2-model-check-projection-prefloor-and-property-tool-seam-mixed-gate-note.md`
- `specs/examples/474-current-l2-theorem-prover-experimental-binding-preflight.md`
- `specs/examples/475-current-l2-principal-theory-spine-and-lean-first-proof-roadmap.md`
- `specs/examples/477-current-l2-delegated-rng-service-practical-actualization.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`

## 4. Actions taken

1. model-check projection pre-floor、property-family reserve inventory、public-checker second reserve inventory を再読した。
2. failing test `current_l2_model_check_second_line_concretization.rs` を追加し、missing helper route 由来の RED を確認した。
3. `current_l2_source_sample_emitted_artifact_support.rs` に second-line manifest を追加した。
4. row-local property preview、brand-neutral request preflight、public-checker second reserve split を helper-local current cut に actualize した。
5. reached sample を `e5 / p06 / p07 / p08 / p09`、guard-only sample を `p05` に固定した。
6. `specs/examples/478` と snapshot docs を更新し、self-driven queue close と mixed-gate residual を同期した。

## 5. Files changed

- 新規:
  - `crates/mir-runtime/tests/current_l2_model_check_second_line_concretization.rs`
  - `specs/examples/478-current-l2-model-check-second-line-concretization.md`
  - `docs/reports/0754-model-check-second-line-concretization.md`
- 更新:
  - `crates/mir-runtime/tests/support/current_l2_source_sample_emitted_artifact_support.rs`
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `plan/00-index.md`
  - `plan/01-status-at-a-glance.md`
  - `plan/09-helper-stack-and-responsibility-map.md`
  - `plan/10-roadmap-overall.md`
  - `plan/11-roadmap-near-term.md`
  - `plan/12-open-problems-and-risks.md`
  - `plan/13-heavy-future-workstreams.md`
  - `plan/17-research-phases-and-autonomy-gates.md`
  - `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
  - `plan/90-source-traceability.md`
  - `specs/00-document-map.md`
  - `specs/10-open-questions.md`
  - `specs/11-roadmap-and-workstreams.md`
  - `specs/12-decision-register.md`

## 6. Commands run and exact outputs

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-18 20:59 JST

$ cargo test -p mir-runtime --test current_l2_model_check_second_line_concretization
error[E0432]: unresolved imports ...
no `CurrentL2SourceSampleModelCheckSecondLineConcretization`
no `build_current_l2_source_sample_model_check_second_line_concretization`

$ cargo test -p mir-runtime --test current_l2_model_check_second_line_concretization
running 6 tests
test model_check_second_line_concretization_reaches_delegated_provider_runtime_prototype_without_collapsing_fairness_line ... ok
test model_check_second_line_concretization_keeps_guarded_prototype_as_not_reached ... ok
test model_check_second_line_concretization_reaches_order_handoff_runtime_prototype ... ok
test model_check_second_line_concretization_reaches_stale_reconnect_runtime_prototype ... ok
test model_check_second_line_concretization_reaches_static_underdeclared_sample ... ok
test model_check_second_line_concretization_reaches_typed_runtime_prototype ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

$ cargo test -p mir-runtime --test current_l2_model_check_second_line_concretization --test current_l2_model_check_projection_prefloor --test current_l2_theorem_prover_binding_preflight --test current_l2_authoritative_room_vertical_slice_actualization --test current_l2_auditable_authority_witness_strengthening --test current_l2_delegated_rng_service_practical_actualization --test current_l2_source_sample_runner --test current_l2_operational_cli
running 58 tests
test result: ok. 58 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ python3 scripts/current_l2_source_sample_regression.py inventory
authored current-l2 source sample inventory confirmed

$ python3 scripts/current_l2_source_sample_regression.py regression
all regression commands passed

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 753 numbered report(s).

$ git diff --check
(no output)
```

## 7. Evidence / findings

- row-local carrier principal を崩さず、property preview / request preflight / public-checker second reserve split を helper-local actualization に上げられた。
- `e5 / p06 / p07 / p08 / p09` reached、`p05` guard-only の representative corpus は second-line cut に素直だった。
- `p09` でも provider/fairness family を excluded に保てたため、Problem 2 strengthening line と model-check second line を collapse せずに済んだ。
- current principal self-driven queue はここで close できるが、remaining work は mixed gate / reserve integration に残る。

## 8. Changes in understanding

- model-check second line は compare-floor のまま残すより、helper-local actualization まで上げた方が stop line が明確だった。
- public-checker side を second reserve refs として explicit に残すことで、actual public checker migration を premature に引き寄せずに済む。
- current queue `0` はこの package close 後なら drift ではなく、`mixed gate only` reading として書ける。

## 9. Open questions

- first settled property language をどの surface まで actualize するか
- concrete model-check tool brand / schema を theorem-first target とどう並べるか
- actual public checker migration と emitted verifier handoff artifact の reopen 順をどう切るか
- production checker / runtime-policy contract をどの mixed gate で concretize するか

## 10. Suggested next prompt

- theorem discharge public-contract actual format と settled property language / concrete tool seam を mixed-gate probe としてどこまで narrow に actualize するかを比較し、docs / helper / snapshot を mixed-gate-only reading に同期してください。
