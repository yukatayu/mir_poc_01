# plan/13 — 重い将来 workstream

## 目的

この文書は、今すぐ着手しないが計画には明示的に入れておくべき重い workstream を整理する。
current L2 parser-free PoC の延長だけでは扱いきれない論点を、将来の独立した workstream として切り出す。

## なぜ今 plan に含めるのか

- 後で急に現れる論点ではないため
- 現在の semantics / helper / notation の設計が、将来の型・解析・証明可能性にどの程度影響するかを意識しておく必要があるため
- current L2 でまだ扱わない理由を明文化し、scope creep を防ぐため

## workstream 1. 型システムの強さ

### 主題

- ownership / lifetime / contract / effect / capability をどこまで型レベルへ持ち上げるか
- inference と annotation のバランス
- linearity / monotonicity をどこまで型規則で強制するか

### entry criteria

- current L2 semantics の failure space と fallback 読みが十分安定していること
- parser 境界の最小 shape が見えていること

### current inventory note

- `place` / `try-fallback` / `perform on` / `perform via` / statement-local `require` / `ensure` / option declaration core / option-local `admit` / explicit edge-row family までは first parser cut 候補として inventory 化してよい。
- exact lexical polish と richer predicate grammar はまだ companion / OPEN に残す。
- same-lineage static evidence floor、malformed / underdeclared rejection、minimal capability strengthening prohibition、request-local / option-local clause attachment、minimal predicate fragment、`try` / rollback locality の structural floor までは first checker cut 候補として inventory 化してよい。
- current static-only corpus baseline では、same-lineage floor `4`、capability floor `2`、missing-option structure floor `3` まで source-backed な regression baseline が見え始めているため、heavy workstream 前の checker-boundary整理は次段 mainline に戻してよい。
- same-lineage floor については helper-local first checker spike を入れてよいが、heavy workstream に入る前の current 段階では final checker API へはまだ固定しない。
- missing-option structure floor についても helper-local second checker spike を入れてよいが、same-lineage / missing-option を共通 helper に寄せるかどうかは heavy workstream 前にもう一段 narrow に比較する。
- capability strengthening floor についても helper-local third checker spike を入れてよいが、same-lineage / missing-option / capability を共通 helper に寄せるかどうかは heavy workstream 前にもう一段 narrow に比較する。

### 今すぐやらない理由

- 現在は syntax と semantics の companion 整理段階であり、型規則を先に固定すると全体が早期凍結しやすい

### 将来 deliverable 候補

- typing judgment の最小スケッチ
- representative programs に対する typing walk-through
- annotation burden / inference burden の比較表

## workstream 2. 静的解析可能性

### 主題

- lineage、fallback admissibility、capability mismatch、underdeclared case を静的にどこまで検出できるか
- parser-free fixture の expectation を、どこまで static analysis に移せるか

### entry criteria

- AST / syntax boundary が一定以上安定していること
- current helper stack の public behavior と thin delegation が整理されていること

### 今すぐやらない理由

- 解析対象の surface と IR がまだ companion notation 段階である

### 将来 deliverable 候補

- static checker の prototype
- representative fixture に対する static-only 判定の formalization
- false positive / false negative trade-off の整理

## workstream 3. 定理証明可能性

### 主題

- current L2 invariants を theorem prover に送れる形へ落とせるか
- canonical normalization、no re-promotion、rollback-cut non-interference などの性質をどう証明するか

### entry criteria

- semantics が current L2 で十分安定していること
- syntax ではなく semantic core を対象にできること

### 今すぐやらない理由

- まだ proof object を支える core formalization が薄い
- 先に parser-free PoC と docs mirror の drift を抑える方が費用対効果が高い

### 将来 deliverable 候補

- proof sketch
- theorem prover 向け core relation の encoding 案
- どの invariant が machine-checked proof に向くかの棚卸し

## workstream 4. 決定可能性

### 主題

- predicate sublanguage、fallback chain validation、contract check、effect wiring のどこが decidable か
- undecidable / semi-decidable な論点をどこで切るか

### entry criteria

- current L2 の範囲が一定程度閉じていること
- static analysis の土台が見えていること

### 今すぐやらない理由

- 現時点では semantics の companion 整理が優先であり、decision procedure 設計はまだ早い

### 将来 deliverable 候補

- decidability matrix
- complexity note
- language core と external verifier 境界の提案

## workstream 5. 実装可能性 / complexity

### 主題

- parser、checker、runtime、host interface、scheduler を実装したときの複雑さ
- current semantics が implementation complexity をどこまで押し上げるか

### entry criteria

- parser 前提の syntax / AST / runtime boundary がもう少し安定していること

### 今すぐやらない理由

- まだ production runtime を設計する段階ではない

### 将来 deliverable 候補

- subsystem ごとの complexity inventory
- prototype 実装比較
- benchmark の最小設計

## workstream 6. 非同期制御 / scheduler / memory-model boundary

### 主題

- `atomic_cut` のような local cut と、higher-level async-control / ordering / fairness semantics をどこで分けるか
- event-tree / authority-serial transition / witness-aware commit のような高位制御族を language / runtime / external verifier のどこに置くか
- low-level memory-order 語彙を採るべきか、それとも higher-level room / authority / consistency family に留めるべきか

### entry criteria

- shared-space / membership / authority / consistency / fairness の docs-first boundary がある程度揃っていること
- small decidable core inventory が見えており、local cut と global ordering の切り分けを語れること

### 今すぐやらない理由

- current repo で source-backed なのは `atomic_cut` の place-local cut と rollback frontier line までであり、scheduler / fairness / hardware-memory-like semantics まで immediate に固定する段階ではない
- low-level memory-order を早く入れると parser / checker / runtime / proof のすべてが同時に重くなりやすい

### current inventory note

- `specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md` では、current first package として
  - `core_static_checker`
  - `theorem_prover_boundary`
  - `protocol_verifier_boundary`
  - `runtime_policy_boundary`
  の 4-way split を整理済みである。
- `specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md` と `specs/examples/128-current-l2-handoff-artifact-threshold-comparison.md` により、proof-obligation matrix は docs 正本、external handoff artifact は mixed row bundle を current default に維持し、boundary-specific split と actual emitter は concrete consumer pressure が出たときだけ reopen する threshold まで切れている。
- `specs/examples/129-current-l2-first-external-consumer-pressure-comparison.md` により、first concrete external consumer pressure の current first practical candidate は `theorem_prover_boundary` に置く。
- `specs/examples/130-current-l2-theorem-line-narrow-actualization-comparison.md` により、theorem line の current first cut は actual emitter ではなく docs-only projection bundle に留める。
- `specs/examples/131-current-l2-theorem-line-evidence-ref-family-comparison.md` により、theorem-side `evidence_refs` は typed symbolic ref family を current first choice に置き、actual path / URI は later reopen に残す。
- `specs/examples/132-current-l2-theorem-line-public-checker-migration-threshold.md` により、current phase では theorem-side projection bundle を docs-only bridge に留め、public checker migration は concrete theorem consumer pressure が出たときだけ reopen する threshold まで固定した。
- `specs/examples/133-current-l2-theorem-line-minimum-contract-row-comparison.md` により、minimum contract row core は `obligation_kind + evidence_refs` に留め、`goal_text` / `proof_hint` / `consumer_hint` は later consumer-specific attachment に残す current first choice まで固定した。
- `specs/examples/134-current-l2-theorem-line-consumer-class-comparison.md` により、first practical consumer class は `proof_notebook` に置き、`proof_assistant_adapter` は second practical candidate、`theorem_export_checker` は later candidate に留める current first choice まで固定した。
- `specs/examples/135-current-l2-theorem-line-notebook-attachment-family-comparison.md` により、`proof_notebook` first bridge の current attachment は `goal_text` に留め、`proof_hint` / `consumer_hint` は later reopen に残す current first choice まで固定した。
- `specs/examples/136-current-l2-theorem-line-notebook-bridge-artifact-threshold.md` により、current phase では `proof_notebook` first bridge を docs-only derived view に留め、named notebook bridge sketch と actual emitted notebook artifact は concrete notebook workflow pressure が出たときだけ reopen 候補に残す current first choice まで固定した。
- `specs/examples/137-current-l2-theorem-line-next-consumer-pressure-comparison.md` により、current next practical reopen は concrete notebook workflow pressure comparison を first choice、`proof_assistant_adapter` consumer pressure comparison を second practical candidate に置く current order まで固定した。
- `specs/examples/138-current-l2-theorem-line-concrete-notebook-workflow-pressure-comparison.md` により、concrete notebook workflow pressure の current first threshold は human review checklist / walkthrough pressure に置く current first choice まで固定した。
- `specs/examples/139-current-l2-theorem-line-notebook-review-unit-named-bundle-threshold.md` により、current first cut は review checklist / walkthrough unit を docs-only named bundle に寄せるところまでに留め、stronger notebook bridge sketch は later reopen に残す current first choice まで固定した。
- `specs/examples/140-current-l2-theorem-line-review-unit-to-bridge-sketch-comparison.md` により、current first cut は named review unit を docs-only notebook bridge sketch に寄せるところまでに留め、compare / bless-like review flow metadata は later reopen に残す current first choice まで固定した。
- `specs/examples/141-current-l2-theorem-line-bridge-sketch-compare-metadata-threshold.md` により、current first cut は bridge sketch に compare basis refs までを足し、bless decision state は later reopen に残す current first choice まで固定した。
- `specs/examples/142-current-l2-theorem-line-compare-ready-bridge-bless-decision-threshold.md` により、current first cut は compare-ready bridge sketch に bless decision state までを足し、review-session metadata は later reopen に残す current first choice まで固定した。
- `specs/examples/143-current-l2-theorem-line-bless-ready-bridge-review-session-threshold.md` により、current first cut は bless-ready bridge sketch に review-note refs までを足し、retained notebook path / review session lifecycle は later reopen に残す current first choice まで固定した。
- `specs/examples/144-current-l2-theorem-line-review-linked-bless-bridge-retained-notebook-threshold.md` により、current first cut は review-linked bless bridge に retained-notebook ref までを足し、actual retained path policy は later reopen に残す current first choice まで固定した。
- `specs/examples/145-current-l2-theorem-line-retained-bridge-review-session-link-threshold.md` により、current first cut は retained-ready bless bridge に review-session ref までを足し、actor / timestamp / lifecycle state は later reopen に残す current first choice まで固定した。
- `specs/examples/146-current-l2-theorem-line-session-linked-retained-bridge-review-observation-threshold.md` により、current first cut は session-linked retained bridge に `reviewed_by_ref + reviewed_at_ref` までを足し、session lifecycle state は later reopen に残す current first choice まで固定した。
- `specs/examples/147-current-l2-theorem-line-observed-session-lifecycle-threshold.md` により、current first cut は observed session-linked retained bridge に `review_session_state` までを足し、retention state / actual retained path policy は later reopen に残す current first choice まで固定した。
- `specs/examples/148-current-l2-theorem-line-lifecycle-ready-retention-state-threshold.md` により、current first cut は lifecycle-ready retained bridge に `retention_state` までを足し、actual retained path policy / emitted artifact は later reopen に残す current first choice まで固定した。
- `specs/examples/149-current-l2-theorem-line-retention-ready-path-policy-threshold.md` により、current first cut は retention-ready retained bridge に `retained_path_policy_ref` までを足し、actual emitted notebook artifact は later reopen に残す current first choice まで固定した。
- `specs/examples/150-current-l2-theorem-line-path-ready-emitted-artifact-threshold.md` により、current first cut は path-ready retained bridge に `emitted_artifact_ref` までを足し、actual handoff emitter / consumer adapter policy は later reopen に残す current first choice まで固定した。
- `specs/examples/151-current-l2-theorem-line-emitted-ready-handoff-emitter-threshold.md` により、current first cut は emitted-ready retained bridge に `handoff_emitter_ref` までを足し、actual consumer adapter / notebook exchange rule は later reopen に残す current first choice まで固定した。
- `specs/examples/152-current-l2-theorem-line-emitter-linked-consumer-adapter-threshold.md` により、current first cut は emitter-linked retained bridge に `consumer_adapter_ref` までを足し、actual notebook exchange rule / adapter-local validation は later reopen に残す current first choice まで固定した。
- `specs/examples/153-current-l2-theorem-line-adapter-linked-exchange-rule-threshold.md` により、current first cut は adapter-linked retained bridge に `exchange_rule_ref` までを足し、adapter-local validation / environment-specific invocation surface は later reopen に残す current first choice まで固定した。
- `specs/examples/154-current-l2-theorem-line-exchange-ready-adapter-validation-threshold.md` により、current first cut は exchange-ready retained bridge に `adapter_validation_ref` までを足し、actual notebook exchange rule body / environment-specific invocation surface は later reopen に残す current first choice まで固定した。
- `specs/examples/155-current-l2-theorem-line-validation-ready-invocation-surface-threshold.md` により、current first cut は validation-ready retained bridge に `consumer_invocation_surface_ref` までを足し、actual notebook exchange body / concrete runtime coupling は later reopen に残す current first choice まで固定した。
- `specs/examples/156-current-l2-theorem-line-invocation-ready-exchange-body-threshold.md` により、current first cut は invocation-ready retained bridge に `exchange_rule_body_ref` までを足し、concrete runtime coupling / transport protocol / failure body は later reopen に残す current first choice まで固定した。
- `specs/examples/157-current-l2-theorem-line-exchange-body-ready-runtime-coupling-threshold.md` により、current first cut は exchange-body-ready retained bridge に `runtime_coupling_ref` までを足し、concrete transport protocol / failure body は later reopen に残す current first choice まで固定した。
- `specs/examples/158-current-l2-theorem-line-runtime-coupling-ready-transport-protocol-threshold.md` により、current first cut は runtime-coupling-ready retained bridge に `transport_protocol_ref` までを足し、concrete failure body は later reopen に残す current first choice まで固定した。
- `specs/examples/159-current-l2-theorem-line-transport-ready-failure-body-threshold.md` により、current first cut は transport-ready retained bridge に `failure_body_ref` までを足し、actual runtime invocation protocol / host binding / failure wording は later reopen に残す current first choice まで固定した。
- `specs/examples/160-current-l2-theorem-line-failure-ready-actual-invocation-protocol-threshold.md` により、current first cut は failure-ready retained bridge に `actual_invocation_protocol_ref` までを足し、consumer-host-specific binding / failure wording は later reopen に残す current first choice まで固定した。
- `specs/examples/161-current-l2-theorem-line-invocation-ready-host-binding-threshold.md` により、current first cut は invocation-ready retained bridge に `consumer_host_binding_ref` までを足し、failure wording と actual notebook runtime handoff actualization は later reopen に残す current first choice まで固定した。
- `specs/examples/162-current-l2-theorem-line-binding-ready-failure-wording-threshold.md` により、current first cut は binding-ready retained bridge に `failure_wording_ref` までを足し、actual notebook runtime handoff actualization / emitted invocation receipt / runtime transcript family は later reopen に残す current first choice まで固定した。
- `specs/examples/163-current-l2-theorem-line-wording-ready-runtime-handoff-threshold.md`、`164-current-l2-theorem-line-runtime-handoff-ready-invocation-receipt-threshold.md`、`165-current-l2-theorem-line-invocation-receipt-ready-runtime-transcript-threshold.md`、`166-current-l2-theorem-line-transcript-ready-materialized-runtime-handoff-threshold.md`、`167-current-l2-theorem-line-materialized-ready-concrete-payload-threshold.md`、`168-current-l2-theorem-line-payload-ready-concrete-transcript-threshold.md`、`169-current-l2-theorem-line-transcript-body-ready-serialized-channel-body-threshold.md`、`170-current-l2-theorem-line-serialized-ready-emitted-attachment-body-threshold.md`、`171-current-l2-theorem-line-attachment-body-ready-emitted-attachment-blob-threshold.md`、`172-current-l2-theorem-line-attachment-blob-ready-retained-file-body-threshold.md`、`173-current-l2-theorem-line-retained-file-body-ready-archive-materialization-threshold.md`、`174-current-l2-theorem-line-archive-materialization-ready-archive-body-bundle-threshold.md`、`175-current-l2-theorem-line-archive-body-ready-archive-bundle-threshold.md`、`176-current-l2-theorem-line-archive-bundle-ready-archive-manifest-threshold.md`、`177-current-l2-theorem-line-archive-manifest-ready-archive-member-family-threshold.md`、`178-current-l2-theorem-line-archive-member-family-ready-archive-member-body-compare-threshold.md`、`179-current-l2-theorem-line-archive-member-body-compare-ready-archive-bless-update-policy-threshold.md`、`180-current-l2-theorem-line-archive-bless-update-policy-ready-retained-archive-payload-threshold.md`、`181-current-l2-theorem-line-retained-archive-payload-ready-archive-retention-layout-threshold.md`、`182-current-l2-theorem-line-archive-retention-layout-ready-retained-archive-payload-body-family-threshold.md`、`183-current-l2-theorem-line-retained-archive-payload-body-family-ready-retained-payload-materialization-family-threshold.md`、`184-current-l2-theorem-line-retained-payload-materialization-family-ready-retained-payload-body-materialization-detail-threshold.md`、`185-current-l2-theorem-line-retained-payload-body-materialization-detail-ready-retained-payload-body-materialization-payload-threshold.md`、`186-current-l2-theorem-line-retained-payload-body-materialization-payload-ready-retained-payload-body-materialization-carrier-detail-threshold.md`、`187-current-l2-theorem-line-retained-payload-body-materialization-carrier-detail-ready-retained-payload-body-materialization-carrier-payload-threshold.md`、`188-current-l2-theorem-line-retained-payload-body-materialization-carrier-payload-ready-retained-payload-bless-update-split-threshold.md`、`189-current-l2-theorem-line-retained-payload-bless-update-split-ready-retained-payload-bless-update-row-pair-threshold.md`、`190-current-l2-theorem-line-retained-payload-bless-update-row-pair-ready-retained-payload-bless-update-row-ref-family-threshold.md`、`191-current-l2-theorem-line-retained-payload-bless-update-row-ref-family-ready-retained-payload-bless-update-dual-ref-bundle-threshold.md`、`192-current-l2-theorem-line-retained-payload-bless-update-dual-ref-bundle-ready-retained-payload-bless-update-strict-dual-field-threshold.md`、`193-current-l2-theorem-line-retained-payload-bless-update-strict-dual-field-ready-consumer-visible-pair-threshold.md`、`194-current-l2-theorem-line-consumer-visible-pair-ready-boundary-specific-handoff-pair-threshold.md`、`195-current-l2-theorem-line-boundary-specific-handoff-pair-ready-actual-handoff-pair-shape-threshold.md`、`196-current-l2-theorem-line-actual-handoff-pair-shape-ready-boundary-specific-handoff-artifact-row-threshold.md`、`197-current-l2-theorem-line-boundary-specific-handoff-artifact-row-ready-external-contract-facing-handoff-row-threshold.md`、`198-current-l2-theorem-line-external-contract-facing-handoff-row-ready-actual-external-contract-threshold.md`、`199-current-l2-theorem-line-actual-external-contract-ready-consumer-specific-external-contract-payload-threshold.md`、`200-current-l2-theorem-line-external-contract-payload-ready-proof-hint-threshold.md`、`201-current-l2-theorem-line-proof-hint-ready-consumer-hint-threshold.md`、`202-current-l2-theorem-line-consumer-hint-ready-second-consumer-pressure-threshold.md`、`203-current-l2-theorem-line-second-consumer-pressure-ready-proof-assistant-adapter-contract-threshold.md`、`204-current-l2-theorem-line-proof-assistant-adapter-contract-ready-theorem-export-checker-pressure-threshold.md`、`205-current-l2-theorem-line-theorem-export-checker-pressure-ready-checker-facing-contract-threshold.md`、`206-current-l2-theorem-line-theorem-export-checker-contract-ready-exported-checker-payload-pressure-threshold.md`、`207-current-l2-theorem-line-theorem-export-checker-payload-pressure-ready-actual-exported-checker-payload-threshold.md`、`208-current-l2-theorem-line-actual-exported-checker-payload-ready-checker-result-materialization-family-threshold.md`、`209-current-l2-theorem-line-checker-result-materialization-family-ready-actual-checker-result-payload-threshold.md`、`210-current-l2-theorem-line-actual-checker-result-payload-ready-checker-verdict-carrier-detail-threshold.md`、`211-current-l2-theorem-line-checker-verdict-carrier-detail-ready-checker-verdict-payload-family-threshold.md`、`212-current-l2-theorem-line-checker-verdict-payload-family-ready-checker-verdict-witness-family-threshold.md`、`213-current-l2-theorem-line-checker-verdict-witness-family-ready-checker-verdict-transport-family-threshold.md`、`214-current-l2-theorem-line-checker-verdict-transport-family-ready-checker-verdict-transport-carrier-detail-threshold.md`、`215-current-l2-theorem-line-checker-verdict-transport-carrier-detail-ready-checker-verdict-transport-payload-threshold.md`、`216-current-l2-theorem-line-checker-verdict-transport-payload-ready-checker-verdict-transport-receipt-threshold.md`、`217-current-l2-theorem-line-checker-verdict-transport-receipt-ready-checker-verdict-transport-channel-body-threshold.md` により、current first cut は `actual_runtime_handoff_ref`、`emitted_invocation_receipt_ref`、`runtime_transcript_ref`、`materialized_runtime_handoff_ref`、`concrete_payload_ref`、`concrete_transcript_body_ref`、`serialized_channel_body_ref`、`emitted_attachment_body_ref`、`emitted_attachment_blob_ref`、`retained_file_body_ref`、`archive_materialization_ref`、`archive_body_ref`、`archive_bundle_ref`、`archive_bundle_manifest_ref`、`archive_bundle_member_family_ref`、`archive_member_body_compare_ref`、`archive_bless_update_policy_ref`、`retained_archive_payload_ref`、`archive_retention_layout_ref`、`retained_archive_payload_body_family_ref`、`retained_payload_materialization_family_ref`、`retained_payload_body_materialization_detail_ref`、`retained_payload_body_materialization_payload_ref`、`retained_payload_body_materialization_carrier_detail_ref`、`retained_payload_body_materialization_carrier_payload_ref`、`retained_payload_body_materialization_bless_update_split_ref`、`retained_payload_body_materialization_bless_update_row_pair_ref`、`retained_payload_body_materialization_bless_update_row_ref_family_ref`、`retained_payload_body_materialization_bless_update_dual_ref_bundle_ref`、`retained_payload_body_materialization_bless_side_row_ref`、`retained_payload_body_materialization_update_side_row_ref`、`retained_payload_body_materialization_bless_update_pair`、`retained_payload_body_materialization_boundary_handoff_pair_ref`、`retained_payload_body_materialization_boundary_handoff_pair`、`retained_payload_body_materialization_boundary_handoff_artifact_row`、`retained_payload_body_materialization_external_handoff_row`、`retained_payload_body_materialization_external_contract`、`retained_payload_body_materialization_external_contract_payload`、`retained_payload_body_materialization_external_contract_proof_hint`、`retained_payload_body_materialization_external_contract_consumer_hint`、`retained_payload_body_materialization_external_contract_second_consumer_pressure`、`retained_payload_body_materialization_proof_assistant_adapter_contract`、`retained_payload_body_materialization_theorem_export_checker_pressure`、`retained_payload_body_materialization_theorem_export_checker_contract`、`retained_payload_body_materialization_theorem_export_checker_payload_pressure`、`retained_payload_body_materialization_theorem_export_checker_payload`、`retained_payload_body_materialization_theorem_export_checker_result_materialization_family`、`retained_payload_body_materialization_theorem_export_checker_result_payload`、`retained_payload_body_materialization_theorem_export_checker_verdict_carrier_detail`、`retained_payload_body_materialization_theorem_export_checker_verdict_payload_family`、`retained_payload_body_materialization_theorem_export_checker_verdict_witness_family`、`retained_payload_body_materialization_theorem_export_checker_verdict_transport_family`、`retained_payload_body_materialization_theorem_export_checker_verdict_transport_carrier_detail`、`retained_payload_body_materialization_theorem_export_checker_verdict_transport_payload`、`retained_payload_body_materialization_theorem_export_checker_verdict_transport_receipt`、`retained_payload_body_materialization_theorem_export_checker_verdict_transport_channel_body` までは theorem-side retained bridge に段階的に足し、low-level memory-order family は later reopen に残す current first choice まで固定した。
- `specs/examples/218-current-l2-theorem-line-checker-verdict-transport-channel-body-ready-low-level-memory-order-family-threshold.md` により、current first choice は theorem-line retained bridge を `retained_payload_body_materialization_theorem_export_checker_verdict_transport_channel_body` で止め、low-level memory-order family を current bridge に入れないことである。
- `specs/examples/219-current-l2-theorem-line-checker-verdict-transport-channel-body-ready-higher-level-async-control-family-comparison.md` と `220-current-l2-theorem-line-higher-level-async-control-family-ready-authority-serial-transition-family-threshold.md` により、higher-level async-control family の current first cut は `authority_serial_transition_family` であり、`witness_aware_commit_family` は second candidate、`event_tree_execution_view` は derived execution / debug view として later candidate に残す。
- `specs/examples/221-current-l2-theorem-line-authority-serial-transition-family-ready-authority-serial-transition-contract-comparison.md`、`222-current-l2-theorem-line-authority-serial-transition-contract-ready-minimal-authority-serial-contract-threshold.md`、`223-current-l2-theorem-line-minimal-authority-serial-contract-ready-authority-serial-row-detail-comparison.md`、`224-current-l2-theorem-line-authority-serial-row-detail-ready-authority-owner-ref-threshold.md` により、その next cut は minimal contract row と owner-slot detail までを theorem-side retained bridge に足す line に進んでいる。
- `specs/examples/225-current-l2-theorem-line-authority-owner-ref-ready-authority-transition-stage-family-comparison.md` と `226-current-l2-theorem-line-authority-transition-stage-family-ready-minimal-authority-transition-stage-family-threshold.md` により、その次 cut は symbolic stage family までを theorem-side retained bridge に足す line に進んでいる。
- `specs/examples/227-current-l2-theorem-line-minimal-authority-transition-stage-family-ready-authority-transition-stage-sequence-shape-comparison.md`、`228-current-l2-theorem-line-authority-transition-stage-sequence-shape-ready-minimal-authority-transition-stage-sequence-threshold.md`、`229-current-l2-theorem-line-minimal-authority-transition-stage-sequence-ready-stage-local-obligation-family-comparison.md`、`230-current-l2-theorem-line-stage-local-obligation-family-ready-minimal-authority-stage-local-obligation-family-threshold.md`、`231-current-l2-theorem-line-minimal-authority-stage-local-obligation-family-ready-stage-local-obligation-row-detail-comparison.md`、`232-current-l2-theorem-line-stage-local-obligation-row-detail-ready-minimal-authority-stage-local-obligation-row-detail-threshold.md`、`233-current-l2-theorem-line-minimal-authority-stage-local-obligation-row-detail-ready-authority-handoff-epoch-ref-comparison.md`、`234-current-l2-theorem-line-authority-handoff-epoch-ref-ready-minimal-authority-handoff-epoch-ref-threshold.md`、`235-current-l2-theorem-line-minimal-authority-handoff-epoch-ref-ready-witness-aware-handoff-family-comparison.md`、`236-current-l2-theorem-line-witness-aware-handoff-family-ready-minimal-witness-aware-handoff-family-threshold.md`、`237-current-l2-theorem-line-minimal-witness-aware-handoff-family-ready-handoff-witness-row-detail-comparison.md`、`238-current-l2-theorem-line-handoff-witness-row-detail-ready-minimal-handoff-witness-row-detail-threshold.md`、`239-current-l2-theorem-line-minimal-handoff-witness-row-detail-ready-replay-attachment-ref-comparison.md`、`240-current-l2-theorem-line-replay-attachment-ref-ready-minimal-replay-attachment-ref-threshold.md`、`241-current-l2-theorem-line-minimal-replay-attachment-ref-ready-handoff-payload-ref-comparison.md`、`242-current-l2-theorem-line-handoff-payload-ref-ready-minimal-handoff-payload-ref-threshold.md` により、その次 cut は actual handoff witness row detail、symbolic replay attachment ref、symbolic handoff payload ref までを theorem-side retained bridge に足す line に進んでいる。
- `specs/examples/255-current-l2-theorem-line-minimal-handoff-transport-channel-body-ready-low-level-memory-order-family-threshold.md` により、handoff side でも current first choice は theorem-line retained bridge を `retained_payload_body_materialization_theorem_export_handoff_transport_channel_body` で止め、low-level memory-order family を current bridge に入れないことである。
- `specs/examples/256-current-l2-small-decidable-core-ready-checker-cluster-matrix-comparison.md`、`257-current-l2-checker-cluster-matrix-ready-minimal-checker-cluster-row-threshold.md`、`258-current-l2-minimal-checker-cluster-row-ready-checker-cluster-fixture-evidence-attachment-comparison.md` により、current next pressure は theorem-line を low-level order family に further actualize することではなく、small decidable core / checker-cluster line で minimal row core と `fixture_evidence_refs` attachment を docs-first に切ることである。
- `specs/examples/259-current-l2-checker-cluster-fixture-evidence-attachment-ready-typed-reason-family-hint-threshold.md`、`260-current-l2-typed-reason-family-hint-ready-checker-cluster-hint-bundle-shape-comparison.md`、`261-current-l2-checker-cluster-hint-bundle-shape-ready-typed-family-coverage-state-threshold.md` により、その current checker-side line は optional `typed_reason_family_hint` attachment、`family_refs[]` minimal bundle、lightweight `coverage_state` までを docs-first に許し、`supported_kind_refs[]` のような actual kind summary は still later に残すことまで source-backed に切れている。
- `specs/examples/262-current-l2-typed-family-coverage-state-ready-supported-kind-summary-threshold.md`、`263-current-l2-supported-kind-summary-ready-actual-checker-payload-family-comparison.md`、`264-current-l2-actual-checker-payload-family-ready-minimal-checker-payload-family-threshold.md` により、current checker-side line は `coverage_state` で止め、`supported kind` summary は checker-cluster matrix に入れず、actual checker payload family を `payload_family_kind + source_refs` minimal bundle まで docs-first bridge として切る方が自然である。actual checker payload row family / supported kind detail / public checker API は still later に残す。

### 将来 deliverable 候補

- async-control vocabulary comparison
- event-tree / authority-serial / witness-aware commit の formal sketch
- decidable core と external verifier 境界の比較表
- room profile 例に対する ordering / fairness / replay obligation walk-through

## language core と external verifier の境界

これは重い workstream の横断論点である。

### current で残す working question

- どこまでを言語 core に入れるか
- どこからを external verifier / theorem prover / out-of-band analyzer に送るか
- machine-check と human-facing explanation の境界をどこまで formalize するか

### current 方針

- いまは language core を過剰に肥大化させない
- current L2 helper stack に proof / analysis obligations を押し込まない
- ただし将来の移送先として external verifier / theorem prover を明示的に計画へ残す
- first parser cut と first checker cut は、heavy workstream の前に切る narrow gate として扱う

## この workstream に最低限必要なもの

本格着手の前に、少なくとも次が必要である。

- stable enough semantic core
- representative examples と fixture による regression baseline
- parser / AST / runtime の boundary inventory
- proof / analysis / complexity を議論するための最小 IR か relation 定義

## まとめ

これらの heavy workstream は、今すぐ実装しない。
しかし current L2 の decisions が将来それらに接続できるよう、**計画には明示的に残す**。
