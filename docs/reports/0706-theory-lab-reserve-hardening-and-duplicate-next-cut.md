# 0706 — theory-lab reserve hardening and duplicate next-cut

## Objective

`tasks.md` の current self-driven queue から次の 6 package を閉じる。

1. request / predicate / `try` cluster typed-surface reserve note
2. admissible evidence contraction note
3. tool-binding stop-line refresh
4. order / handoff emitted-artifact schema reserve note
5. guarded-vs-MDTT/MTT reduction timing note
6. malformed duplicate-cluster source-sample widening comparison

加えて、current snapshot と FAQ / plan mirror を current reading に同期し、
`po` / `dep` 型の short alias を current docs から外し、snake_case relation name を principal wording に揃える。

## Scope and assumptions

- `specs/` を規範正本、`plan/` を repository memory、`docs/reports/` を時系列証跡として扱う。
- 今回は docs-first reserve / comparison package を閉じる task であり、final parser grammar、final public API、shared-space final catalog、concrete theorem/model-check tool binding は fixed しない。
- duplicate cluster については、current source-authored corpus actualization そのものではなく、next exact cut を comparison として固定する。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/examples/413...432`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `faq_004.md`
- `faq_005.md`
- `samples/current-l2/README.md`
- code / test anchor:
  - `crates/mir-semantics/tests/current_l2_proof_notebook_review_unit_support.rs`
  - `crates/mir-semantics/tests/current_l2_model_check_carrier_support.rs`
  - `crates/mir-semantics/tests/current_l2_static_gate_support.rs`
  - `crates/mir-runtime/src/current_l2.rs`
  - `scripts/current_l2_source_sample_regression.py`

## Actions taken

1. `specs/examples/433...438` を追加し、typed/theorem/model-check second-order reserve、order/handoff emitted-artifact schema reserve、guarded-vs-MDTT/MTT reduction timing、duplicate-cluster next widening cut を docs-first に固定した。
2. `Documentation.md`、`progress.md`、`tasks.md` を current snapshot に合わせて更新し、current theory-lab line を reserve checkpoint close、next reopen を third-order follow-up に更新した。
3. `plan/00`、`01`、`10`、`11`、`12`、`17`、`18`、`90` を更新し、next package order と risk/open-problem wording を新しい reserve notes に合わせた。
4. `docs/research_abstract/phase5...` と `phase6...` を current theorem-side / execution-side next line に合わせて更新した。
5. `faq_004.md` と `faq_005.md` を current near-term order と rough progress reading に合わせて再同期した。
6. `samples/current-l2/README.md` を reserve integration close 後の current line と duplicate next-cut reading に合わせて更新した。
7. `specs/00-document-map.md` と `specs/10-open-questions.md` を更新し、新 package 導線と open-question wording を current reading に揃えた。

## Files changed

- 新規:
  - `specs/examples/433-current-l2-request-predicate-try-cluster-typed-surface-reserve-note.md`
  - `specs/examples/434-current-l2-admissible-evidence-contraction-note.md`
  - `specs/examples/435-current-l2-tool-binding-stop-line-refresh.md`
  - `specs/examples/436-current-l2-order-handoff-emitted-artifact-schema-reserve-note.md`
  - `specs/examples/437-current-l2-guarded-vs-mdtt-mtt-reduction-timing-note.md`
  - `specs/examples/438-current-l2-malformed-duplicate-cluster-source-sample-widening-comparison.md`
- 更新:
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `faq_004.md`
  - `faq_005.md`
  - `samples/current-l2/README.md`
  - `specs/00-document-map.md`
  - `specs/10-open-questions.md`
  - `plan/00-index.md`
  - `plan/01-status-at-a-glance.md`
  - `plan/10-roadmap-overall.md`
  - `plan/11-roadmap-near-term.md`
  - `plan/12-open-problems-and-risks.md`
  - `plan/17-research-phases-and-autonomy-gates.md`
  - `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
  - `plan/90-source-traceability.md`
  - `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
  - `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`

## Commands run and exact outputs

```bash
date '+%Y-%m-%d %H:%M JST'
```

```text
2026-04-17 00:04 JST
```

```bash
cargo test -p mir-semantics --test current_l2_proof_notebook_review_unit_support
```

```text
running 4 tests
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

```bash
cargo test -p mir-semantics --test current_l2_model_check_carrier_support
```

```text
running 4 tests
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

```bash
cargo test -p mir-semantics --test current_l2_static_gate_support static_gate_artifact_omits_reason_codes_for_duplicate_declaration_clusters -- --exact
```

```text
running 1 test
test static_gate_artifact_omits_reason_codes_for_duplicate_declaration_clusters ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out
```

```bash
python3 scripts/current_l2_source_sample_regression.py inventory
```

```text
current L2 fixed-subset authored inventory
...
e23-malformed-try-fallback-missing-fallback-body | source-authored | malformed | not_evaluated | fixture_static_cluster | present | current authored static stop
```

## Evidence / outputs / test results

- theorem-side current floor (`formal_hook` / `proof_notebook_review_unit`) は focused test で維持されている。
- model-check current floor (`model_check_concrete_carriers`) は focused test で維持されている。
- duplicate cluster の non-promotion guard（`checked_reasons` / detached `reason_codes` に上げない）は focused test で維持されている。
- source-authored sample corpus は authored dozen のままで、`e14/e15` はまだ未昇格であることを inventory で再確認した。

## What changed in understanding

- typed/theorem/model-check line は second-order reserve package を閉じたことで、次の主眼が「first reserve note を切る」段階から「family unification / consumer threshold / projection keep-drop を切る」段階へ移った。
- order/handoff line は emitted-artifact schema reserve と reduction timing note を切ったことで、次の主眼が「schema を仮固定する」段階から「source-surface wording と modality internalization trigger を見極める」段階へ移った。
- duplicate cluster line は、comparison-first reopen の次として `e14/e15` pair-first actualization comparison を first choice に置く readingが、current repo の precedent と整合することを再確認した。

## Open questions

- capability / lineage / request / predicate / `try` cluster をどこまで shared typed family として読むか。
- notebook-first consumer threshold と discharge reserve をどこで切るか。
- small-cluster projection keep/drop をどこで narrowing するか。
- final source-surface handoff wording をどこまで docs-first に寄せるか。
- stronger modality internalization trigger をどこで認めるか。
- duplicate pair actualization を source-authored corpusへどの exact cut で入れるか。

## Suggested next prompt

次の current task map に従って、
`typed-surface family unification keep/drop note`、
`notebook-consumer threshold and discharge reserve note`、
`model-check small-cluster projection keep/drop refresh`、
`order / handoff source-surface wording reserve note`、
`modality internalization trigger note`、
`malformed duplicate-cluster source-authored static-stop pair actualization comparison`
を自走で進め、snapshot と plan mirror を同期してください。

## Update necessity notes

- `plan/` は更新した。
- `progress.md` は更新した。
- `tasks.md` は更新した。
- `Documentation.md` は更新した。
- `specs/11-roadmap-and-workstreams.md` は今回更新不要。
- `plan/13-heavy-future-workstreams.md` は今回更新不要。
- `.docs/` と `AGENTS.md` は今回更新不要。
