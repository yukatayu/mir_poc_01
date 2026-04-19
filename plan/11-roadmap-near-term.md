# plan/11 — 近接ロードマップ

## 目的

この文書は、今から数 package 先までの near-term execution order を示す。
step 数や task 数は厳密な約束ではなく、**rough estimate** である。

execution lane、theory-lab lane、reserve integration lane は分けて書く。

## current reading

- current execution line は `Macro 4 active on fixed authored/prototype floor` である。
- current theory-lab line は `Macro 5 final-layer closeout packages active` である。
- current reserve integration line は `Macro 6 minimal working subset actual default / Macro 7 mixed` である。
- corrected runnable floor は current mapped corpus で already reached しており、next packages はその floor を作るためではなく closeout / mixed-gate narrowing のために置いている。
- current defaults:
  - theorem-first external integration target
  - repo-local runnable CLI + tests + emitted artifacts + reproducible compare floor = near-end success
  - first application target = authoritative shared-space turn-based room
  - shared-space scope = minimal working subset, not exhaustive final catalog
  - first room default = authority-ack / single room authority / authoritative serial transition / authority_rng / late join visible past / stale reconnect fail-then-refresh / replay invalidation

## recently closed packages

| package band | close evidence | current reading |
|---|---|---|
| `0...10` | `docs/reports/0740...0751` + `specs/examples/466...475` | actual adoption package、syntax/modality convergence、theory spine / Lean-first proof roadmap |
| `11...19` | `docs/reports/0752...0761` + `specs/examples/476...484` | reserve strengthening / practical actualization、model-check second line、mixed-gate threshold defaults |
| `20...35` | `docs/reports/0762...0778` + `specs/examples/485...500` | theorem/model-check/order-handoff/shared-space coupled-later / actual-adoption packages |
| `36...42` | `docs/reports/0779...0785` + `specs/examples/501...507` | checker-artifact / witness-provider / theorem/model-check final-public-contract reopen thresholds |
| `43...50` | `docs/reports/0786...0795` + `specs/examples/508...515` | Lean-stub pilot、artifact-conformance、trace alignment、public-seam compression、serial reserve surface |
| `51...54` | `docs/reports/0796...0799` + `specs/examples/516...519` | toolchain probe / reopen manifest、model-check public-seam compression、representative Lean sample set actual Lean execution |
| `55` | `docs/reports/0801` + `specs/examples/520` | final-layer closeout defaults and reopened self-driven queue |
| `57 first slice` | `docs/reports/0802` + `specs/examples/521` | committed Lean sample corpus、IFC first fragment、proof-skeleton / proof-obligation first foundations |
| `56 concrete slice` | `docs/reports/0803` + `specs/examples/522` | IFC secret valid/invalid concrete example、`samples/lean/` 日本語 explanation sync、Package 56 narrowed queue |
| `56 source pair` | `docs/reports/0804` + `specs/examples/523` | source-side IFC explicit authority pair、representative Lean sample set widening、Package 56 source-side evidence close |
| `56 label-flow close` | `docs/reports/0805` + `specs/examples/524` | source-side IFC label-flow negative、representative Lean sample set widening、Package 56 actual package close |
| `58 first widening slice` | `docs/reports/0806` + `specs/examples/525` | delegated RNG provider placement carry-over、representative Lean sample set widening、Package 58 started |
| `58 helper/CLI surface` | `docs/reports/0807` + `specs/examples/526` | order-handoff helper CLI `surface_preview`、`p07 / p08 / p09` reached/guarded actualization、Package 58 helper hardening first slice |
| `58 negative static stop` | `docs/reports/0808` + `specs/examples/527` | order-handoff late-join visibility negative pair `p13 / p14`、helper-local static stop actualization、Package 58 negative corpus tightening |
| `58 theorem-side negative carry-over` | `docs/reports/0809` + `specs/examples/528` | order-handoff late-join visibility negative pair `p13 / p14` を representative Lean sample set / committed Lean corpus へ carry over、Package 58 broader theorem-side widening |
| `58 IFC checker-hint preview` | `docs/reports/0810` + `specs/examples/529` | source-side IFC trio `p10 / p11 / p12` を `typed_checker_hint_preview` として actualize、`family_refs[] + coverage_state` mirror、Package 58 IFC helper widening |
| `58 theorem/model-check helper preview` | `docs/reports/0811` + `specs/examples/530` | theorem result-object preview と model-check public-checker preview を `run-source-sample` helper summary へ widen し、`p08` / `p09` の reached/guard 非対称を固定、Package 58 diagnostics widening close |
| `59 near-end closeout sync` | `docs/reports/0812` + `specs/examples/531` | Package 58 close 後の queue / mixed gate / user-spec residual を再圧縮し、next self-driven queue を residual mixed-gate packages `60 / 61` へ再構成 |
| `60 theorem/model-check residual mixed-gate compression` | `docs/reports/0813` + `specs/examples/532` | theorem/model-check final public-contract reopen threshold を `run-source-sample` helper summary に mirror し、`p08` / `p09` の非対称を helper-local operational summary に固定、Package 60 close |
| `61 order-handoff/shared-space residual mixed-gate compression` | `docs/reports/0814` + `specs/examples/533` | order-handoff / witness-provider public seam compression を `run-source-sample` helper summary に mirror し、`p07 / p08` reached・`p09` guard の current cut を helper-local operational summary に固定、Package 61 close |
| `62 typed/IFC helper-to-checker ratchet` | `docs/reports/0815` + `specs/examples/534` | `actual_checker_payload_family_threshold` を `run-source-sample` helper summary に mirror し、`p10 / p11 / p12` の current checker-adjacent payload bridge を `payload_family_kind + source_refs` で固定、Package 62 close |
| `63 checker payload row-family ratchet` | `docs/reports/0816` + `specs/examples/535` | `actual_checker_payload_row_family_threshold` を `run-source-sample` helper summary に mirror し、`p10 / p11 / p12` の current row grouping bridge を `payload_family_ref + row_family_kind` で固定、Package 63 close |
| `64 checker payload row-detail ratchet` | `docs/reports/0817` + `specs/examples/536` | `actual_checker_payload_row_detail_threshold` を `run-source-sample` helper summary に mirror し、`p10 / p11 / p12` の current row-detail bridge を `payload_row_family_ref + row_source_ref + row_reason_kind` で固定、Package 64 close |
| `65 checker payload row-body ratchet` | `docs/reports/0818` + `specs/examples/537` | `actual_checker_payload_row_body_threshold` を `run-source-sample` helper summary に mirror し、`p10 / p11 / p12` の current row-body bridge を `row_body = { selected_option_ref, visibility_target_ref }` で固定、Package 65 close |
| `66 checker payload supported-kind-summary ratchet` | `docs/reports/0819` + `specs/examples/538` | `actual_checker_payload_supported_kind_summary_threshold` を `run-source-sample` helper summary に mirror し、`p10 / p11 / p12` の current row-family keyed summary bridge を `payload_row_family_ref + supported_kind_scope + supported_kind_refs` で固定、Package 66 close |
| `67 checker payload public-schema sketch ratchet` | `docs/reports/0820` + `specs/examples/539` | `actual_checker_payload_public_schema_sketch_threshold` を `run-source-sample` helper summary に mirror し、`p10 / p11 / p12` の current checker-side package bundle を 5 ref wrapper で固定、Package 67 close |
| `68 public-checker-api sketch ratchet` | `docs/reports/0821` + `specs/examples/540` | `actual_public_checker_api_sketch_threshold` を `run-source-sample` helper summary に mirror し、`p10 / p11 / p12` の current checker-side API read relation を `checker_subject + public_checker_payload_schema_ref` で固定、Package 68 close |
| `69 public-checker entry-criteria ratchet` | `docs/reports/0822` + `specs/examples/541` | `actual_public_checker_entry_criteria_threshold` を `run-source-sample` helper summary に mirror し、`p10 / p11 / p12` の current public-looking reopen minimum を `public_checker_api_ref + entry_criteria_refs + next_comparison_target_ref + deferred_boundary_refs` で固定、Package 69 close |
| `70 public-checker command-surface ratchet` | `docs/reports/0823` + `specs/examples/542` | `actual_public_checker_command_surface_threshold` を `run-source-sample` helper summary に mirror し、`p10 / p11 / p12` の current command-surface minimum を `command_surface_kind + family_facade_command_refs + public_checker_api_ref` で固定、Package 70 close |
| `71 shared-output-contract ratchet` | `docs/reports/0824` + `specs/examples/543` | `actual_shared_output_contract_threshold` を `run-source-sample` helper summary に mirror し、`p10 / p11 / p12` の current shared-output-contract minimum を `output_contract_kind + checker_cluster_name + checker_status + public_checker_payload_schema_ref` で固定、Package 71 close |
| `72 public-checker-boundary ratchet` | `docs/reports/0825` + `specs/examples/544` | `actual_public_checker_boundary_threshold` を `run-source-sample` helper summary に mirror し、`p10 / p11 / p12` の current public-checker-boundary minimum を `boundary_kind + public_checker_command_surface_ref + shared_output_contract_ref` で固定、Package 72 close |
| `73 verifier-handoff-surface ratchet` | `docs/reports/0826` + `specs/examples/545` | `actual_verifier_handoff_surface_threshold` を `run-source-sample` helper summary に mirror し、`p10 / p11 / p12` の current verifier-handoff-surface minimum を `handoff_surface_kind + public_checker_boundary_ref + proof_obligation_matrix_ref + handoff_artifact_mode` で固定、Package 73 close |
| `74 minimal-parser-subset-freeze ratchet` | `docs/reports/0827` + `specs/examples/546` | `actual_minimal_parser_subset_freeze_threshold` を `run-source-sample` helper summary に mirror し、`p10 / p11 / p12` の current parser first-tranche minimum を `freeze_kind + accepted_cluster_refs + reject_cluster_refs + retention_floor_refs` で固定、Package 74 close |
| `75 parser-to-checker-reconnect-freeze ratchet` | `docs/reports/0828` + `specs/examples/547` | `actual_parser_to_checker_reconnect_freeze_threshold` を `run-source-sample` helper summary に mirror し、`p10 / p11 / p12` の current reconnect minimum を `reconnect_kind + parser_subset_freeze_ref + checker_floor_refs + retained_helper_refs` で固定、Package 75 close |
| `76 phase1-semantics-closeout ratchet` | `docs/reports/0829` + `specs/examples/548` | `actual_phase1_semantics_closeout_threshold` を `run-source-sample` helper summary に mirror し、`p10 / p11 / p12` の current semantics closeout minimum を `closeout_kind + core_semantics_refs + invariant_bridge_refs + notation_status_refs` で固定、Package 76 close |
| `77 phase2-parser-free-poc-closeout ratchet` | `docs/reports/0830` + `specs/examples/549` | `actual_phase2_parser_free_poc_closeout_threshold` を `run-source-sample` helper summary に mirror し、`p10 / p11 / p12` の current parser-free closeout minimum を `closeout_kind + compile_gate_refs + helper_boundary_refs + detached_loop_policy_refs` で固定、Package 77 close |
| `78 phase4-shared-space-self-driven-closeout ratchet` | `docs/reports/0831` + `specs/examples/550` | `actual_phase4_shared_space_self_driven_closeout_threshold` を `run-source-sample` helper summary に mirror し、`p07 / p08 / p09` の current shared-space closeout minimum を `closeout_kind + current_package_refs + user_spec_required_catalog_refs + retained_later_refs` で固定、Package 78 close |
| `79 phase5-proof-protocol-runtime-policy-handoff-closeout ratchet` | `docs/reports/0832` + `specs/examples/551` | `actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold` を `run-source-sample` helper summary に mirror し、`p07 / p08 / p09` の current proof/protocol/runtime-policy handoff closeout minimum を `closeout_kind + verifier_handoff_surface_ref + theorem_retained_bridge_stop_ref + boundary_inventory_ref + retained_later_refs` で固定、Package 79 close |

## active self-driven packages

| 順番 | package | macro | question | rough weight | current exit signal |
|---|---|---|---|---|---|
| 1 | `80` phase6-actual-parser-ast-carrier-first-tranche ratchet | `Macro 6/7` | phase5 handoff closeout の次段として `mir-ast` non-production carrier をどこまで helper-local threshold に近づけるか | `M` | `mir_ast_current_l2_first_tranche = carrier_kind + accepted_surface_refs + code_anchor_refs + retained_later_refs` current cut を narrow に actualize しつつ、stage3 admit/request/predicate cluster、final parser API、span-rich diagnostics、final grammar を still later に残せる |

## package detail

### Package 56 — layered strong typing / IFC first-fragment

- current source:
  `specs/examples/475`
  `specs/examples/520`
  `specs/examples/521`
  `specs/examples/522`
  `specs/examples/523`
  `specs/examples/524`
- current recommendation:
  close 済み。checker-adjacent principal + layered stack を維持し、stronger typed surface early principal promotion はしない。`CurrentL2LabelModel.lean` と `CurrentL2IfcSecretExamples.lean` により Lean-side first fragment と secret valid/invalid concrete example は actualize 済みであり、`p10 / p11 / p12` により source-side authority success / authority-miss negative / label-flow negative trio も corrected prototype として actualize 済みと読む。
- target corpus:
  `p06-typed-proof-owner-handoff`
  `p10-typed-authorized-fingerprint-declassification`
  `p11-typed-unauthorized-fingerprint-release`
  `p12-typed-classified-fingerprint-publication-block`
- stop line:
  final typed source principal
  final IFC syntax
  final public label API

### Package 57 — Lean formal skeleton / proof obligations

- current reading:
  first slice は close 済みである。
- close evidence:
  `specs/examples/521`
  `samples/lean/foundations/CurrentL2ProofSkeleton.lean`
  `samples/lean/current-l2/`
  `crates/mir-runtime/examples/current_l2_emit_theorem_lean_bundle.rs`
  `scripts/current_l2_lean_sample_sync.py`
- kept later:
  concrete production prover binding
  final proof object public contract
  final public verifier contract

### Package 58 — helper / CLI hardening and broader coverage

- current source:
  `specs/examples/516...530`
- current recommendation:
  close 済み。actual Lean execution floor と committed `samples/lean/` corpus を helper / CLI hardening、broader theorem-side / diagnostics widening に使い、`p09` carry-over、`surface_preview`、`p13 / p14` static stop と theorem-side carry-over、`typed_checker_hint_preview`、theorem result-object preview helper mirror、model-check public-checker preview helper mirror まで actualize 済みと読む。
- promotion rule:
  widened corpus は mixed gate を 1 つ閉じるか、formal skeleton package を concretize する場合だけ足す。
- stop line:
  final public theorem/model-check contract
  packaging / host-facing integration

### Package 59 — near-end closeout sync

- current source:
  `specs/examples/469`
  `specs/examples/520`
  `specs/examples/530`
  `specs/examples/531`
- current recommendation:
  close 済み。Package 58 close 後の queue / mixed gate / true user-spec residual を stale wording なしで同期し、next self-driven queue を residual mixed-gate packages へ再構成した。
- required sync targets:
  `progress.md`
  `tasks.md`
  `plan/01`
  `plan/17`
  `plan/18`
  `plan/90`

### Package 60 — theorem/model-check residual mixed-gate compression

- current source:
  `specs/examples/506`
  `specs/examples/507`
  `specs/examples/530`
  `specs/examples/532`
- current recommendation:
  close 済み。theorem/model-check final public-contract reopen threshold を `run-source-sample` helper summary に narrow に mirror し、`p08` theorem reached / model-check guarded と `p09` theorem guarded / model-check reached の非対称を helper-local operational summary に actualize した。
- stop line:
  final public theorem/model-check contract adoption
  final public verifier contract adoption
  concrete production prover / checker binding

### Package 61 — order-handoff/shared-space residual mixed-gate compression

- current source:
  `specs/examples/505`
  `specs/examples/515`
  `specs/examples/526`
  `specs/examples/533`
- current recommendation:
  close 済み。order-handoff source wording residual / emitted-artifact residual / witness-provider public-seam residual を `run-source-sample` helper summary に narrow に mirror し、`p07 / p08` reached・`p09` guard の current cut を helper-local operational summary に actualize した。
- stop line:
  final source wording adoption
  final public witness/provider/artifact contract adoption
  final parser/public API adoption

### Package 62 — typed/IFC helper-to-checker ratchet

- current source:
  `specs/examples/522`
  `specs/examples/523`
  `specs/examples/524`
  `specs/examples/529`
  `specs/examples/534`
- current recommendation:
  close 済み。`typed_checker_hint_preview` を final typed source principal や final public verifier contract に上げず、checker-adjacent payload threshold まで narrow に ratchet し、`actual_checker_payload_family_threshold` を `payload_family_kind + source_refs` current cut で helper-local operational summary に actualize 済みと読む。
- stop line:
  final typed source principal
  final IFC syntax
  final public checker artifact
  final public verifier contract

### Package 63 — checker payload row-family ratchet

- current source:
  `specs/examples/265`
  `specs/examples/266`
  `specs/examples/534`
  `specs/examples/535`
- current recommendation:
  close 済み。`actual_checker_payload_family_threshold` を final public checker artifact や final public verifier contract に上げず、checker payload row family まで narrow に ratchet し、`actual_checker_payload_row_family_threshold` を `payload_family_ref + row_family_kind` current cut で helper-local operational summary に actualize 済みと読む。
- stop line:
  supported kind detail
  actual checker row payload
  final public checker artifact
  final public verifier contract

### Package 64 — checker payload row-detail ratchet

- current source:
  `specs/examples/267`
  `specs/examples/268`
  `specs/examples/535`
  `specs/examples/536`
- current recommendation:
  close 済み。`actual_checker_payload_row_family_threshold` を final public checker artifact や final public verifier contract に上げず、checker payload row detail まで narrow に ratchet し、`actual_checker_payload_row_detail_threshold` を `payload_row_family_ref + row_source_ref + row_reason_kind` current cut で helper-local operational summary に actualize 済みと読む。
- stop line:
  actual checker row body
  supported kind detail
  final public checker artifact
  final public verifier contract

### Package 65 — checker payload row-body ratchet

- current source:
  `specs/examples/269`
  `specs/examples/270`
  `specs/examples/536`
  `specs/examples/537`
- current recommendation:
  close 済み。`actual_checker_payload_row_detail_threshold` を final public checker artifact や final public verifier contract に上げず、checker payload row body まで narrow に ratchet し、`actual_checker_payload_row_body_threshold` を `payload_row_family_ref + row_source_ref + row_reason_kind + row_body` current cut で helper-local operational summary に actualize 済みと読む。
- stop line:
  supported kind detail
  final public checker artifact
  final public verifier contract

### Package 66 — checker payload supported-kind-summary ratchet

- current source:
  `specs/examples/271`
  `specs/examples/272`
  `specs/examples/537`
  `specs/examples/538`
- current recommendation:
  close 済み。`actual_checker_payload_row_body_threshold` を final public checker artifact や final public verifier contract に上げず、checker payload supported kind summary まで narrow に ratchet し、`actual_checker_payload_supported_kind_summary_threshold` を `payload_row_family_ref + supported_kind_scope + supported_kind_refs` current cut で helper-local operational summary に actualize 済みと読む。
- stop line:
  public checker payload schema
  final public checker artifact
  final public verifier contract

### Package 67 — checker payload public-schema sketch ratchet

- current source:
  `specs/examples/273`
  `specs/examples/274`
  `specs/examples/538`
  `specs/examples/539`
- current recommendation:
  close 済み。`actual_checker_payload_supported_kind_summary_threshold` を final public checker artifact や final public verifier contract に上げず、checker payload public schema sketch まで narrow に ratchet し、`actual_checker_payload_public_schema_sketch_threshold` を 5 ref wrapper current cut で helper-local operational summary に actualize 済みと読む。
- stop line:
  public checker API
  final public checker artifact
  final public verifier contract

### Package 68 — checker payload public-checker-api sketch ratchet

- current source:
  `specs/examples/275`
  `specs/examples/276`
  `specs/examples/539`
  `specs/examples/540`
- current recommendation:
  close 済み。`actual_checker_payload_public_schema_sketch_threshold` を actual command surface や final public verifier contract に上げず、public checker API sketch まで narrow に ratchet し、`actual_public_checker_api_sketch_threshold` を `checker_subject + public_checker_payload_schema_ref` current cut で helper-local operational summary に actualize 済みと読む。
- stop line:
  public checker entry criteria
  actual command surface
  shared output contract
  final public verifier contract

### Package 69 — public-checker entry-criteria ratchet

- current source:
  `specs/examples/277`
  `specs/examples/278`
  `specs/examples/540`
  `specs/examples/541`
- current recommendation:
  close 済み。`actual_public_checker_api_sketch_threshold` を actual command surface や parser-front public checker boundary に上げず、public-checker comparison 専用の entry criteria まで narrow に ratchet し、`actual_public_checker_entry_criteria_threshold` を `public_checker_api_ref + entry_criteria_refs + family_facade_support_ref + family_facade_script_refs + smoke_command_refs + next_comparison_target_ref + deferred_boundary_refs` current cut で helper-local operational summary に actualize 済みと読む。
- stop line:
  actual command surface
  shared output contract
  parser-front public checker boundary
  emitted verifier handoff surface
  final public verifier contract

### Package 70 — public-checker command-surface ratchet

- current source:
  `specs/examples/279`
  `specs/examples/280`
  `specs/examples/541`
  `specs/examples/542`
- current recommendation:
  close 済み。`actual_public_checker_entry_criteria_threshold` を shared output contract や parser-front public checker boundary に上げず、public-checker command surface ready sketch まで narrow に ratchet し、`actual_public_checker_command_surface_threshold` を `command_surface_kind + family_facade_command_refs + public_checker_api_ref` current cut で helper-local operational summary に actualize 済みと読む。
- stop line:
  detached-loop `smoke-*` wrapper の public surface 昇格
  generic shared public checker entry
  shared output contract
  parser-front public checker boundary
  emitted verifier handoff surface
  final public verifier contract

### Package 71 — shared-output-contract ratchet

- current source:
  `specs/examples/281`
  `specs/examples/282`
  `specs/examples/542`
  `specs/examples/543`
- current recommendation:
  close 済み。`actual_public_checker_command_surface_threshold` を parser-front public checker boundary や emitted verifier handoff surface に上げず、shared-output-contract ready sketch まで narrow に ratchet し、`actual_shared_output_contract_threshold` を `output_contract_kind + checker_cluster_name + checker_status + public_checker_payload_schema_ref` current cut で helper-local operational summary に actualize 済みと読む。
- stop line:
  generic shared public checker entry
  parser-front public checker boundary
  emitted verifier handoff surface
  final public verifier contract

### Package 72 — public-checker-boundary ratchet

- current source:
  `specs/examples/283`
  `specs/examples/284`
  `specs/examples/543`
  `specs/examples/544`
- current recommendation:
  close 済み。`actual_shared_output_contract_threshold` を final parser grammar や emitted verifier handoff surface に上げず、public checker boundary ready sketch まで narrow に ratchet し、`actual_public_checker_boundary_threshold` を `boundary_kind + public_checker_command_surface_ref + shared_output_contract_ref` current cut で helper-local operational summary に actualize 済みと読む。
- stop line:
  final parser grammar
  generic shared public checker entry
  emitted verifier handoff surface
  final public verifier contract

### Package 73 — verifier-handoff-surface ratchet

- current source:
  `specs/examples/285`
  `specs/examples/286`
  `specs/examples/544`
  `specs/examples/545`
- current recommendation:
  close 済み。`actual_public_checker_boundary_threshold` を actual emitted verifier handoff artifact や theorem / protocol / runtime-policy dedicated split に上げず、verifier handoff surface ready sketch まで narrow に ratchet し、`actual_verifier_handoff_surface_threshold` を `handoff_surface_kind + public_checker_boundary_ref + proof_obligation_matrix_ref + handoff_artifact_mode` current cut で helper-local operational summary に actualize 済みと読む。
- stop line:
  actual emitted verifier handoff artifact
  theorem / protocol / runtime-policy dedicated contract
  final parser grammar
  final public verifier contract

### Package 74 — minimal-parser-subset-freeze ratchet

- current source:
  `specs/examples/287`
  `specs/examples/288`
  `specs/examples/545`
  `specs/examples/546`
- current recommendation:
  close 済み。`actual_verifier_handoff_surface_threshold` を final parser grammar や parser-to-checker reconnect freeze に上げず、minimal parser subset freeze ready sketch まで narrow に ratchet し、`actual_minimal_parser_subset_freeze_threshold` を `freeze_kind + accepted_cluster_refs + reject_cluster_refs + retention_floor_refs` current cut で helper-local operational summary に actualize 済みと読む。
- stop line:
  final parser grammar
  parser-to-checker reconnect freeze
  final public parser/checker API
  final public verifier contract

### Package 75 — parser-to-checker-reconnect-freeze ratchet

- current source:
  `specs/examples/289`
  `specs/examples/290`
  `specs/examples/546`
  `specs/examples/547`
- current recommendation:
  close 済み。`actual_minimal_parser_subset_freeze_threshold` を final parser grammar や final public parser/checker API に上げず、parser-to-checker reconnect freeze ready sketch まで narrow に ratchet し、`actual_parser_to_checker_reconnect_freeze_threshold` を `reconnect_kind + parser_subset_freeze_ref + checker_floor_refs + retained_helper_refs` current cut で helper-local operational summary に actualize 済みと読む。
- stop line:
  final parser grammar
  final public parser/checker API
  final public verifier contract

### Package 76 — phase1-semantics-closeout ratchet

- current source:
  `specs/examples/291`
  `specs/examples/292`
  `specs/examples/547`
- current recommendation:
  close 済み。`actual_parser_to_checker_reconnect_freeze_threshold` を final parser grammar や final type system に上げず、phase1 semantics closeout ready sketch まで narrow に ratchet し、`actual_phase1_semantics_closeout_threshold` を `closeout_kind + core_semantics_refs + invariant_bridge_refs + notation_status_refs` current cut で helper-local operational summary に actualize 済みと読む。
- stop line:
  final parser grammar
  final type system
  actual external schema
  final public verifier contract

### Package 77 — phase2-parser-free-poc-closeout ratchet

- current source:
  `specs/examples/293`
  `specs/examples/294`
  `specs/examples/548`
  `specs/examples/549`
- current recommendation:
  close 済み。`actual_phase1_semantics_closeout_threshold` を reference update / bless workflow や public exporter API に上げず、phase2 parser-free PoC closeout ready sketch まで narrow に ratchet し、`actual_phase2_parser_free_poc_closeout_threshold` を `closeout_kind + compile_gate_refs + helper_boundary_refs + detached_loop_policy_refs` current cut で helper-local operational summary に actualize 済みと読む。
- stop line:
  reference update / bless workflow
  final retention/path policy
  public exporter API
  production host interface

### Package 78 — phase4-shared-space-self-driven-closeout ratchet

- current source:
  `specs/examples/295`
  `specs/examples/296`
  `specs/examples/549`
  `specs/examples/550`
- current recommendation:
  close 済み。`actual_phase2_parser_free_poc_closeout_threshold` を final activation / authority / auth / identity / admission / consistency / fairness catalog に上げず、phase4 shared-space self-driven closeout ready sketch まで narrow に ratchet し、`actual_phase4_shared_space_self_driven_closeout_threshold` を `closeout_kind + current_package_refs + user_spec_required_catalog_refs + retained_later_refs` current cut で helper-local operational summary に actualize 済みと読む。
- stop line:
  exhaustive shared-space final catalog
  stronger fairness / replay profile
  final public witness/provider/artifact contract
  packaging / host-facing realization

### Package 79 — phase5-proof-protocol-runtime-policy-handoff-closeout ratchet

- current source:
  `specs/examples/297`
  `specs/examples/298`
  `specs/examples/550`
  `specs/examples/551`
- current recommendation:
  close 済み。`actual_phase4_shared_space_self_driven_closeout_threshold` を actual subject row materialization や boundary-specific handoff artifact family に上げず、phase5 proof / protocol / runtime-policy handoff closeout ready sketch まで narrow に ratchet し、`actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold` を `closeout_kind + verifier_handoff_surface_ref + theorem_retained_bridge_stop_ref + boundary_inventory_ref + retained_later_refs` current cut で helper-local operational summary に actualize 済みと読む。
- stop line:
  actual subject row materialization
  boundary-specific handoff artifact family
  actual emitted verifier artifact
  concrete theorem/model-check tool binding
  actual public checker migration
  final public verifier contract

### Package 80 — phase6-actual-parser-ast-carrier-first-tranche ratchet

- current source:
  `specs/examples/299`
  `specs/examples/300`
  `specs/examples/551`
- current recommendation:
  next active line。`actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold` を stage3 admit/request/predicate cluster や final parser grammar に上げず、`carrier_kind + accepted_surface_refs + code_anchor_refs + retained_later_refs` current cut の phase6 actual parser / AST carrier first tranche ready sketch まで narrow に ratchet する。
- stop line:
  stage3 admit slot surface
  stage3 request clause suite
  stage3 predicate fragment
  perform head final public parser API
  span-rich diagnostics
  final grammar

## later mixed-gate topics

| 順番 | lane | macro | topic | next gate |
|---|---|---|---|---|
| 5 | theory-lab | `Macro 5/7` | final public theorem result object / consumer-shaped theorem payload public contract / concrete theorem prover brand / proof object public schema | mixed gate |
| 6 | theory-lab | `Macro 5/7` | first settled property language / concrete model-check tool brand / final public checker artifact / actual public checker migration / actual emitted verifier handoff artifact | mixed gate |
| 7 | theory-lab | `Macro 5/6` | final source-surface handoff wording / emitted-artifact schema | mixed gate |
| 8 | theory-lab | `Macro 5/6` | final public witness schema / provider receipt schema / combined public contract / emitted-handoff contract | mixed gate |
| 9 | theory-lab | `Macro 5` | final modal foundation / source marker | mixed gate |
| 10 | reserve integration | `Macro 6/7` | exhaustive shared-space catalog / packaging / FFI targets | mixed + user-spec residual |

## current recommendation

- comparison debt より adoption debt を優先する。
- `specs/examples/458...465` は integrator / compare floor として保持するが、current line の principal anchor は `466...469`、`475`、`520` に移った。
- representative Lean sample set actual Lean execution reached は current queue zero を意味しない。
- current self-driven queue は queue zero ではなく、phase6-actual-parser-ast-carrier-first-tranche ratchet を含む。
- authoritative-room default profile と append-friendly contrast room を shared-space current working subset に置く。
- `auditable_authority_witness` と `delegated_rng_service` は close 済み strengthening / practical actualization に移し、final public provider receipt / witness schema は mixed gate に残す。
- final parser grammar、final public API、final public verifier contract、installed binary、exhaustive catalog は near-term line に入れない。
