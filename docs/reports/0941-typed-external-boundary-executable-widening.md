# 0941 typed external boundary executable widening

## Objective

phase 9 `Typed external boundary / adapter` の docs-first ladder を、
repo-local synthetic preview helper widening としてどこまで actualize できるかを、
repo-local alpha current layer を壊さずに閉じる。

## Scope and assumptions

- current scope は **helper-local synthetic preview cut** に限る。
- final public adapter API、exact host schema、browser / network / VR host family split は固定しない。
- standard I/O は Mir core primitive にしない。
- effect boundary、transport envelope、authentication / authorization / membership / capability / witness / visualization を collapse しない。
- current synthetic preview subset は `EXT-03` / `EXT-04` に絞る。
- `EXT-01` / `EXT-02` / `EXT-05` は residual planned family に残す。

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `sub-agent-pro/mirrorea_future_plan_full_handoff_2026-04-24.md`
- `sub-agent-pro/mirrorea_phase_samples_progress_storage_handoff_2026-04-24.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/19-repository-map-and-taxonomy.md`
- `docs/research_abstract/typed_external_boundary_adapter_plan_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- reviewer / explorer input:
  - Lorentz `019dcef6-9595-7721-84b4-87a5d04489a3`
  - Boyle `019dcef6-96d8-7393-8300-cd1a14795461`

## Actions taken

1. `scripts/typed_external_boundary_samples.py` を phase 9 thin synthetic preview helper として actualize した。
   - synthetic preview IDs を `EXT-03` / `EXT-04` に絞った。
   - debug modes を `summary` / `envelopes` / `visualization` / `failures` に整理した。
   - `EXT-03` で local queue adapter lane、`EXT-04` で typed adapter failure lane を visible にした。
   - scenario label は helper-local working name であり final effect 名ではないことを payload に残した。
2. `scripts/tests/test_typed_external_boundary_samples.py` を追加・更新し、dedicated helper test suite を通した。
3. planned source family `samples/not_implemented/typed-external-boundary/` に `EXT-01..05` を揃え、
   `EXT-03` / `EXT-04` を synthetic preview helper subset として読む現在地を明示した。
5. `plan/25-typed-external-boundary-executable-roadmap.md` を新規追加し、
   repo-local synthetic preview helper widening と final public host-facing gate を分けた。
6. `docs/hands_on/typed_external_boundary_canaries_01.md` を新規追加し、
   current synthetic preview helper の実行入口を reader-facing に用意した。
7. `README.md`、`Documentation.md`、`samples/README.md`、`scripts/README.md`、
   `progress.md`、`tasks.md`、`samples_progress.md`、relevant `plan/` / `specs/` を同期した。
8. reviewer 指摘に従って、`EXT-03` / `EXT-04` を active executable sample や current E2E と読める wording を除去し、
   planned source stub を読む synthetic preview helper subset という current snapshot に揃えた。

## Files changed

- `scripts/typed_external_boundary_samples.py`
- `scripts/tests/test_typed_external_boundary_samples.py`
- `samples/not_implemented/typed-external-boundary/README.md`
- `samples/not_implemented/typed-external-boundary/01_log_text_local_console.mir`
- `samples/not_implemented/typed-external-boundary/02_show_floating_text_world_overlay.mir`
- `samples/not_implemented/typed-external-boundary/03_send_room_message_local_queue.mir`
- `samples/not_implemented/typed-external-boundary/04_adapter_failure_typed_result.mir`
- `samples/not_implemented/typed-external-boundary/05_debug_visualization_label_restriction.mir`
- `samples/README.md`
- `scripts/README.md`
- `README.md`
- `Documentation.md`
- `docs/hands_on/README.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/hands_on/typed_external_boundary_canaries_01.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/repository_layer_structure_01.md`
- `docs/research_abstract/typed_external_boundary_adapter_plan_01.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/19-repository-map-and-taxonomy.md`
- `plan/25-typed-external-boundary-executable-roadmap.md`
- `plan/90-source-traceability.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`

## Evidence / outputs / test results

Typed external helper:

```bash
python3 -m unittest scripts.tests.test_typed_external_boundary_samples
python3 scripts/typed_external_boundary_samples.py check-all --format json
python3 scripts/typed_external_boundary_samples.py closeout --format json
python3 scripts/typed_external_boundary_samples.py run EXT-03 --debug envelopes --format json
python3 scripts/typed_external_boundary_samples.py run EXT-03 --debug visualization --format json
python3 scripts/typed_external_boundary_samples.py run EXT-04 --debug failures --format json
```

- pass
- synthetic preview subset:
  `EXT-03`, `EXT-04`
- residual planned:
  `EXT-01`, `EXT-02`, `EXT-05`
- current validation は helper self-consistency と payload/debug-shape preview であり、
  phase 9 `.mir` の direct semantic execution ではない

Anchor commands:

```bash
cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes --format json
```

- pass
- `provider_boundary` と `local_queue` の current anchor が phase 9 synthetic preview helper cut と矛盾しないことを確認

Docs / hygiene:

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
```

- pass
- `python3 scripts/validate_docs.py` fresh rerun は `Found 939 numbered report(s).`
- reviewer rereview:
  Boyle `019dcef6-96d8-7393-8300-cd1a14795461` から prior findings 解消、remaining findings なし

## What changed in understanding

- phase 9 は `EXT-01..05` 全部を同時に active executable sample と読むより、
  `EXT-03` / `EXT-04` の thin synthetic preview helper cut に絞る方が current anchor と整合する。
- `EXT-01` local console は stdio builtin 誤読を誘発しやすい。
- `EXT-05` standalone visualization scenario は current helper cut では
  `EXT-03` の visualization view に吸収した方が、
  effect boundary / visualization redaction / helper-local preview の分離を保ちやすい。
- docs-first plan と executable widening を分けないと、
  final public adapter contract を早く既成事実化する risk が高い。

## Open questions

- final public adapter API / FFI をどの layer で reopen するか
- exact host schema をいつ draft するか
- `EXT-01` / `EXT-02` / `EXT-05` を phase 9 residual family のまま保つ期間
- projection / visualization lane と typed external residual family の接続点
- final public `AuthEvidence` kind と session / signature protocol

## Suggested next prompt

`Projection / placement executable widening` を進めてください。`plan/20` の validity checklist を起点に、system-wide source / place-specific projection trace の helper-local evidence floor、関連 sample / debug view、docs / progress / tasks / report 同期まで同一 package で閉じてください。
