# ドキュメント要約

## リポジトリの目的

このリポジトリは、次の subsystem を中心とした **specification-first research repo** である。

- **Mir** — 意味論コア言語
- **Mirrorea** — 分散 fabric と control plane
- **PrismCascade** — 独立した media-graph kernel
- **Typed-Effect Wiring Platform** — inspectable / routable / contract-aware な effect layer

これらは related だが、repo では **separable subsystem** として扱う。

## 現在の読み

- プロジェクト全体は **architecture / semantics first** の段階にある。
- ただし current repo は docs-only skeleton ではなく、
  - parser-free PoC
  - compile-ready minimal actualization
  - fixed-subset source sample の runnable ladder
  を already 持つ。
- current status は 3 lane で読むのが自然である。
  - execution lane:
    `Macro 4 active on fixed authored/prototype floor`
    （current-l2 authored sixteen と corrected prototype set `p01...p16` は fixed 済みだが、sample corpus 自体は theory-line の adequacy corpus として active に使う）
  - theory-lab lane:
    `Macro 5 final-layer closeout packages active`
    （`specs/examples/458...519` で compare / actual-adoption / helper-local / deeper-theory floor が揃い、`520...604` で final-layer closeout defaults、Lean first slice、IFC / finite-index widening、parser-side tranche、theorem/model-check bridge reconnect、order-handoff source surface / artifact route tightening、authoritative-room first scenario helper summary tightening、authoritative-room reserve strengthening lane tightening、guided problem sample entrypoint / residual bundle matrix / problem bundle actualization / parser-side companion and bridge / parser-side inspector / parser-side representative mapping matrix / explained representative problem sample bundles / representative problem bundle smoke runner / representative problem bundle aggregate smoke summary / representative problem bundle failure-focused smoke diagnostics / representative problem bundle quickstart walkthrough hardening / representative problem quickstart CLI mirror / representative problem quickstart parity checks / representative problem mixed-gate reopen map refresh / representative problem split-package map refresh / Problem 1 typed source principal split helper actualization / Problem 1 theorem public-contract split helper actualization / Problem 1 model-check public-contract split helper actualization / Problem 2 source wording / emitted schema split helper actualization / Problem 2 witness-provider public-shape split helper actualization / representative problem reopen-map split closeout sync / remaining residual lane summary actualization / Problem 1 final-public-seam lane helper actualization / Problem 2 final-public-seam lane helper actualization / syntax-modality final-marker lane helper actualization / typed-checker first executable slice actualization / theorem-first emitted-artifact loop hardening / authoritative-room runnable scenario loop hardening / Problem 1 executable residual reopen sync / Problem 2 executable residual reopen sync が順に actualize 済みである。representative Lean sample set `e5 / p06 / p10 / p11 / p12 / p15 / p16 / p07 / p08 / p09 / p13 / p14` actual Lean execution も reached 済みであり、current remaining active line は Package 132 after-loop closeout sync / later mixed-user-spec residual に移った）
  - reserve integration lane:
    `Macro 6 minimal working subset actual default / Macro 7 mixed`
    （authoritative room minimal working subset と repo-local near-end success criteria は current default に上がったが、installed-binary / packaging / FFI / engine adapter / exhaustive shared-space catalog は still later に残る）

## source-backed で既にあるもの

- current L2 semantics / parser-free validation substrate / compile-ready minimal actualization
- fixed-subset source sample authored sixteen:
  `e1 / e2 / e3 / e4 / e5 / e12 / e14 / e15 / e16 / e13 / e19 / e21 / e22 / e18 / e20 / e23`
- runnable prototype sample set:
  `p01 / p02 / p03 / p04 / p05 / p06 / p07 / p08 / p09 / p10 / p11 / p12 / p13 / p14 / p15 / p16`
  （`samples/prototype/` に置き、current lowerer / runner へ explicit path で流す）
  - `p01...p05 / p07 / p08 / p09 / p13 / p14` は order/handoff family
  - `p06 / p10 / p11 / p12` は typed/theorem/model-check / IFC sample-visible corrected prototype
  - `p15 / p16` は typed capture/lifetime / simple cost sample-visible corrected prototype
- helper-local debug output preview:
  prototype / sample 実行時に `debug_*` または `_debug_` を含み `_output` / `_pipe` で終わる target の record を `debug_outputs` として見せる current cut がある
- helper-local verification preview:
  prototype / sample 実行時に `formal_hook_status`、`subject_kind`、obligation list を `verification_preview` として見せる current cut がある
- helper-local artifact preview:
  prototype / sample 実行時に proof notebook review unit / model-check concrete carrier の derived row preview を `artifact_preview` として見せる current cut がある
- helper-local order-handoff surface preview:
  prototype / sample 実行時に `minimal_companion` / `stage_block_secondary` / `serial_scope_reserve` の surface family を `surface_preview` として見せる current cut がある
- helper-local typed checker-hint preview:
  prototype / sample 実行時に source-side first strong typing sample set `p10 / p11 / p12 / p15 / p16` を `typed_checker_hint_preview` として見せ、`family_refs[] + coverage_state` を sample-local helper preview に留める current cut がある
- helper-local actual checker payload-family threshold:
  prototype / sample 実行時に source-side first strong typing sample set `p10 / p11 / p12 / p15 / p16` を `actual_checker_payload_family_threshold` として見せ、`payload_family_kind + source_refs` を checker-adjacent payload bridge として sample-local helper threshold に留める current cut がある
- helper-local checker payload row-family threshold:
  prototype / sample 実行時に source-side first strong typing sample set `p10 / p11 / p12 / p15 / p16` を `actual_checker_payload_row_family_threshold` として見せ、`payload_family_ref + row_family_kind` を row grouping bridge として sample-local helper threshold に留める current cut がある
- helper-local checker payload row-detail / row-body / supported-kind-summary / public-schema sketch / public-checker-api sketch threshold:
  prototype / sample 実行時に source-side first strong typing sample set `p10 / p11 / p12 / p15 / p16` を `actual_checker_payload_row_detail_threshold`、`actual_checker_payload_row_body_threshold`、`actual_checker_payload_supported_kind_summary_threshold`、`actual_checker_payload_public_schema_sketch_threshold`、`actual_public_checker_api_sketch_threshold`、`actual_public_checker_entry_criteria_threshold`、`actual_public_checker_command_surface_threshold`、`actual_shared_output_contract_threshold` として見せ、`payload_row_family_ref + row_source_ref + row_reason_kind`、`row_body = { selected_option_ref, visibility_target_ref }`、`payload_row_family_ref + supported_kind_scope + supported_kind_refs`、`actual_checker_payload_family_ref + checker_payload_row_family_ref + checker_payload_row_detail_ref + checker_payload_row_body_ref + checker_payload_supported_kind_summary_ref`、`checker_subject + public_checker_payload_schema_ref`、`public_checker_api_ref + entry_criteria_refs + next_comparison_target_ref + deferred_boundary_refs`、`command_surface_kind + family_facade_command_refs + public_checker_api_ref`、`output_contract_kind + checker_cluster_name + checker_status + public_checker_payload_schema_ref` を row-detail / row-body / supported-kind-summary / public-schema / public-checker-api / public-checker-entry-criteria / public-checker-command-surface / shared-output-contract bridge として sample-local helper threshold に留める current cut がある
- helper-local order-handoff negative static stop:
  `p13` missing publication witness と `p14` handoff-before-publish を current-L2 source sample runner の helper-local static gate で止め、late-join visibility line の negative corpus を sample-visible に actualize する current cut がある
- verifier preview alignment pre-floor:
  helper-local preview を final public verifier contract にせず、sample-local preview-aligned typed artifact route を compare floor に置く current cut がある
- model-check projection pre-floor:
  row-local `model_check_concrete_carriers`、small-cluster projection reserve、property/tool seam を public adoption へ上げず、helper-local compare floor に置く current cut がある
- theorem discharge pre-floor:
  row-local `proof_notebook_review_unit`、abstract discharge-entry reserve、transport/public-contract seam を public adoption へ上げず、helper-local compare floor に置く current cut がある
- theorem-first experimental pilot actualization:
  notebook-first theorem lineを public contract に上げず、repo-local emitted artifact / compare floor まで helper-local に actualize する current cut がある
- theorem-prover experimental binding preflight:
  notebook-first theorem lineを concrete prover brand や public theorem contract に上げず、brand-neutral preflight manifest まで helper-local に actualize する current cut がある
- theorem Lean-first non-production stub pilot actualization:
  theorem-first external integration target を public theorem contract や final public verifier contract に上げず、review-unit first / brand-neutral preflight anchor keep / repo-local emitted stub refs first の Lean-first non-production stub pilot まで helper-local に actualize する current cut がある
- theorem review-unit to Lean-stub repo-local artifact-conformance bridge:
  authored current-L2 representative sample `e2 / e5` を使って、formal hook / proof notebook review unit / Lean stub artifact の pair alignment を repo-local helper と regression bundle に actualize する current cut がある
- theorem Lean-stub representative trace-alignment bridge:
  representative runtime/static/prototype corpus `e2 / e5 / p06 / p07 / p08` と guard-only `p05` を使って、review unit / Lean stub pair alignment を helper-local runtime test に actualize する current cut がある
- principal theory spine / proof roadmap:
  multimodal dependent core research-direction、layered typing/proof architecture、compatibility metatheory package、Lean-first proof roadmap を current theory-lab direction として整理する current cut がある
- repo-local Lean sample corpus:
  `samples/lean/foundations/` に actual small proof fragment、`samples/lean/current-l2/` に representative Lean sample set generated stub corpus、`samples/lean/manifest.json` に verification summary を置く current cut がある
  - generated current-L2 theorem stub は `sorry` を含むため、artifact well-formedness / bridge alignment evidence として読む
  - `foundations/` は IFC label model、secret-key valid/invalid concrete example、proof-skeleton first fragment の actual small proof として読む
- auditable-authority-witness strengthening actualization:
  `p07` reached、`p08/p05` guard-only の minimal witness core を final public witness schema に上げず、helper-local strengthening manifest に actualize する current cut がある
- delegated-rng-service practical actualization:
  `p09` reached、`p07/p08` guard-only の provider-placement cut を final public provider receipt schema に上げず、helper-local practical manifest に actualize する current cut がある
- model-check second-line concretization:
  `e5 / p06 / p07 / p08 / p09` reached、`p05` guard-only の row-local property preview / request preflight / public-checker second reserve split を final property language や final public verifier contract に上げず、helper-local second-line manifest に actualize する current cut がある
- theorem discharge actual-format probe:
  `e5 / p06 / p07 / p08` reached、`p05` guard-only の transport preview / public-contract preview / notebook-consumer-first boundary を actual discharge transport や public theorem contract に上げず、helper-local actual-format probe に actualize する current cut がある
- theorem discharge / public-theorem-contract threshold default:
  `e5 / p06 / p07 / p08` reached、`p05` guard-only の review-unit-first / discharge-entry-adjacent / notebook-consumer-first / brand-neutral theorem request default を actual discharge transport や public theorem contract に上げず、helper-local threshold manifest に actualize する current cut がある
- theorem contract shape threshold default:
  `e5 / p06 / p07 / p08` reached、`p05` guard-only の refs-only reserve schema first / review-unit transport anchor / brand-neutral request manifest keep / consumer-shaped payload later default を actual discharge transport や public theorem contract に上げず、helper-local threshold manifest に actualize する current cut がある
- theorem transport/public-contract coupled later gate:
  `e5 / p06 / p07 / p08` reached、`p05` guard-only の transport/public-contract adjacent but distinct / review-unit anchor / refs-only reserve schema first / consumer-shaped payload later default を actual discharge transport adoption や public theorem contract adoption に上げず、helper-local actualization manifest に actualize する current cut がある
- theorem review-unit transport / notebook-contract actual adoption:
  `e5 / p06 / p07 / p08` reached、`p05` guard-only の review-unit transport first / notebook-consumer contract first / brand-neutral binding reserve keep を theorem result public object や final public verifier contract に上げず、helper-local actual adoption manifest に actualize する current cut がある
- theorem result-object preview / proof-object-schema reserve actualization:
  `e5 / p06 / p07 / p08` reached、`p05` guard-only の notebook-consumer object first / consumer-shaped payload preview only / proof-object-schema reserve keep を final public theorem result object や proof object public schema に上げず、helper-local actualization manifest に actualize する current cut がある
- theorem result-object preview CLI helper mirror:
  `e5 / p06 / p07 / p08` reached、`p09` guard-only の theorem result-object preview manifest を `run-source-sample` helper summary に mirror し、result-object route / payload preview / proof-object-schema reserve を final public theorem contract 群に上げず inspectable に保つ current cut がある
- theorem result-object route actual adoption:
  `e5 / p06 / p07 / p08` reached、`p05` guard-only の review-unit transport first / notebook-consumer object first / consumer-shaped payload preview keep / proof-object-schema-prover-brand later を final public theorem result object や consumer-shaped theorem payload public contract や proof object public schema に上げず、helper-local actual adoption manifest に actualize する current cut がある
- theorem final public-contract reopen threshold:
  `e5 / p06 / p07 / p08` reached、`p05` guard-only の result-object-and-payload first / prover-brand-and-proof-schema second / final public verifier contract third を final public theorem result object や consumer-shaped theorem payload public contract や concrete theorem prover brand や proof object public schema や final public verifier contract に上げず、helper-local threshold manifest に actualize する current cut がある
- theorem proof-object schema / prover-brand coupled later gate:
  `e5 / p06 / p07 / p08` reached、`p05` guard-only の result-object preview keep / refs-only public-schema candidate only / brand-neutral preflight anchor keep / concrete brand not adopted を final public theorem result object や proof object public schema や concrete theorem prover brand に上げず、helper-local coupled-later manifest に actualize する current cut がある
- model-check public-checker artifact preview / verifier-handoff reserve actualization:
  `e5 / p06 / p07 / p09` reached、`p05` guard-only の consumer-shaped artifact preview only / verifier-handoff reserve keep / brand-neutral tool-binding reserve keep を final public checker artifact や actual public checker migration に上げず、helper-local actualization manifest に actualize する current cut がある
- model-check public-checker preview CLI helper mirror:
  `e5 / p06 / p07 / p09` reached、`p08` guard-only の public-checker preview manifest を `run-source-sample` helper summary に mirror し、checker artifact preview / verifier-handoff reserve / tool-binding reserve を final public checker artifact 群に上げず inspectable に保つ current cut がある
- model-check tool-brand / verifier-handoff coupled later gate:
  `e5 / p06 / p07 / p09` reached、`p05` guard-only の public-checker artifact preview keep / verifier-handoff candidate only / tool-brand candidate only を concrete model-check tool brand や final public checker artifact や actual emitted verifier handoff artifact に上げず、helper-local coupled-later manifest に actualize する current cut がある
- model-check public-checker artifact / migration coupled later gate:
  `e5 / p06 / p07 / p09` reached、`p05` guard-only の consumer-shaped public checker artifact candidate side / actual public checker migration candidate side を final public checker artifact や actual public checker migration や actual emitted verifier handoff artifact に上げず、helper-local coupled-later manifest に actualize する current cut がある
- model-check checker-artifact route actual adoption:
  `e5 / p06 / p07 / p09` reached、`p05` guard-only の row-local property route first / checker-boundary contract anchor / consumer-shaped checker-artifact candidate only / migration candidate adjacent keep を final public checker artifact や actual public checker migration や actual emitted verifier handoff artifact に上げず、helper-local actual-adoption manifest に actualize する current cut がある
- model-check final public-contract reopen threshold:
  `e5 / p06 / p07 / p09` reached、`p05` guard-only の property-language-and-tool-brand first / public-checker-artifact-and-migration second / verifier-handoff-and-runtime-policy-contract third / final public verifier contract fourth を first settled property language や concrete model-check tool brand や final public checker artifact や actual public checker migration や actual emitted verifier handoff artifact や production checker/runtime-policy contract や final public verifier contract に上げず、helper-local threshold manifest に actualize する current cut がある
- order-handoff source wording / emitted-artifact coupled later gate:
  `p07 / p08` reached、`p05` guard-only の source-wording candidate side / emitted-artifact-schema candidate side を final source-surface handoff wording や final emitted-artifact schema に上げず、helper-local coupled-later manifest に actualize する current cut がある
- theorem result object / payload public-contract coupled later gate:
  `e5 / p06 / p07 / p08` reached、`p05` guard-only の final result-object candidate side / consumer-shaped payload public-contract candidate side を final public theorem result object や consumer-shaped theorem payload public contract に上げず、helper-local coupled-later manifest に actualize する current cut がある
- model-check property/tool-seam probe:
  `e5 / p06 / p07 / p08 / p09` reached、`p05` guard-only の property-language probe / tool-seam probe / checker-boundary probe を first settled property language や actual public checker migration に上げず、helper-local mixed-gate probe に actualize する current cut がある
- model-check property-language / tool-brand threshold default:
  `e5 / p06 / p07 / p08 / p09` reached、`p05` guard-only の row-local-property-preview first / small-cluster-projection second / brand-neutral model-check request / public-checker-contract-later default を first settled property language や concrete tool brand に上げず、helper-local threshold manifest に actualize する current cut がある
- witness/provider/artifact public-shape threshold default:
  `p07 / p08 / p09` reached、`p05` guard-only の claim/payload split first / repo-local emitted artifact refs first / optional attachment refs only / combined public contract later default を final public witness schema や provider receipt や emitted-handoff contract に上げず、helper-local threshold manifest に actualize する current cut がある
- witness/provider public-contract / emitted-handoff coupled later gate:
  `p07 / p08 / p09` reached、`p05` guard-only の claim/payload split first / witness-provider route non-collapse / repo-local emitted artifact refs first / combined public contract later / final emitted-handoff contract later を final public witness schema や provider receipt や emitted-handoff contract に上げず、helper-local coupled-later manifest に actualize する current cut がある
- witness/provider public-schema coupled later gate:
  `p07 / p08 / p09` reached、`p05` guard-only の witness-schema candidate side / provider-receipt candidate side / combined public-contract candidate side を final public witness schema や final public provider receipt schema や combined public contract や final emitted-handoff contract に上げず、helper-local coupled-later manifest に actualize する current cut がある
- authoritative-room vertical-slice actualization:
  `p07/p08` current default room profile を final emitted schema に上げず、helper-local vertical-slice manifest に actualize する current cut がある
- minimal companion experimental surface:
  `p07/p08` current default room profile を final grammar に上げず、helper-local companion lines に actualize する current cut がある
- stage-block secondary surface:
  `p07/p08` current default room profile を final grammar に上げず、`stage` / `after` / `witness` family を strong secondary candidate として helper-local に actualize する current cut がある
- order-handoff negative static-stop pair:
  `p13 / p14` current late-join visibility line の negative pair を final parser grammar や final source wording に上げず、helper-local static-stop evidence に actualize する current cut がある
- exact rough stimulus preservation bucket:
  `samples/not_implemented/`
- underdeclared source omission actualization:
  `e5-underdeclared-lineage` と `e12-underdeclared-target-missing` は `samples/current-l2/` authored corpus に actualize 済みであり、helper-local `verification_preview` / `artifact_preview` でも `fixture_static_cluster` route を reached として見せる

## current first lines

- typed / theorem / model-check:
  checker-adjacent semantic carrier principal、finite decidable index fragment principal、`Ψ ; Γ ; Δ ⊢ e : A @ m ! ε ▷ C` conceptual spine、security label / taint / capture / lifetime / simple cost first-class target、structural marker family first、notebook-first theorem line、row-local model-check carrier first
- order / handoff:
  cut family decomposition、relation decomposition principal、`authority_serial_transition_family` first、`witness_aware_commit_family` second、low-level `memory_order` family retained-later reference
- syntax / modality:
  semantic honesty first、`lambda_circle_box` partial basis、guarded / MDTT / MTT / Fitch-style multimodal retained stronger family、boundary-pressure trigger

## docs-first theory-lab で既にあるもの

- shared-space identity / admission / authority の docs-first boundary
- capability-scoped host-I/O / adapter boundary の docs-first cut
- order / memory-model / authority-handoff / syntax / modality / verifier-boundary comparison package:
  `specs/examples/405...412`
- typed / theorem / model-check / order-handoff follow-up package:
  `specs/examples/413...422`
- reserve integration closeout and follow-up:
  `specs/examples/423...432`
- theory-lab reserve hardening and threshold framing:
  `specs/examples/433...448`
- public operational CLI / mixed-gate boundary:
  `specs/examples/449...450`
- runnable prototype / helper-local preview package:
  `specs/examples/451...457`
- FAQ 006 drift audit / first-line / stop-line / queue reconstruction integration:
  `specs/examples/458`
- verifier-boundary / typed-theorem-model-check current-first-line integration:
  `specs/examples/459`
- order / handoff cut-relation-authority current-first-line integration:
  `specs/examples/460`
- syntax / modality current-first-line integration:
  `specs/examples/461`
- theory-line near-end closeout and mixed-gate-only reading:
  `specs/examples/462`
- verifier preview alignment pre-floor:
  `specs/examples/463`
- model-check projection pre-floor:
  `specs/examples/464`
- theorem discharge pre-floor:
  `specs/examples/465`
- Problem 1 actual adoption package and theorem-first pilot:
  `specs/examples/466`
- Problem 2 actual adoption package and authoritative-room default profile:
  `specs/examples/467`
- syntax / modality convergence and current recommendation:
  `specs/examples/468`
- near-end closeout after actual adoption defaults:
  `specs/examples/469`
- theorem-first experimental pilot actualization:
  `specs/examples/470`
- authoritative-room vertical-slice emitted-artifact ratchet:
  `specs/examples/471`
- minimal companion experimental order-handoff surface:
  `specs/examples/472`
- order-handoff surface narrowing / stage-block secondary candidate:
  `specs/examples/473`
- theorem-prover experimental binding preflight:
  `specs/examples/474`
- principal theory spine / Lean-first proof roadmap:
  `specs/examples/475`
- auditable-authority-witness strengthening actualization:
  `specs/examples/476`
- delegated-rng-service practical actualization:
  `specs/examples/477`
- model-check second-line concretization:
  `specs/examples/478`
- theorem discharge actual-format probe:
  `specs/examples/479`
- model-check property/tool-seam probe:
  `specs/examples/480`
- model-check property/tool-brand threshold default:
  `specs/examples/482`
- witness/provider/artifact public-shape threshold default:
  `specs/examples/483`
- order-handoff surface / artifact threshold default:
  `specs/examples/484`
- theorem discharge / public-theorem-contract threshold default:
  `specs/examples/481`
- theorem contract shape threshold default:
  `specs/examples/485`
- theorem transport/public-contract coupled later gate:
  `specs/examples/486`
- theorem review-unit transport / notebook-contract actual adoption:
  `specs/examples/487`
- theorem result-object preview / proof-object-schema reserve actualization:
  `specs/examples/491`
- theorem result-object route actual adoption:
  `specs/examples/500`
- theorem final public-contract reopen threshold:
  `specs/examples/506`
- theorem Lean-first non-production stub pilot actualization:
  `specs/examples/508`
- theorem review-unit to Lean-stub repo-local artifact-conformance bridge:
  `specs/examples/509`
- theorem Lean-stub representative trace-alignment bridge:
  `specs/examples/510`
- order-handoff serial-scope reserve surface:
  `specs/examples/511`
- witness/provider emitted-contract representative trace-alignment bridge:
  `specs/examples/512`
- theorem actual Lean execution availability probe:
  `specs/examples/513`
- theorem public seam compression after local Lean-unavailable probe:
  `specs/examples/514`
- order-handoff / witness-provider final public seam compression after reserve actualizations:
  `specs/examples/515`
- theorem actual Lean execution toolchain probe and reopen manifest:
  `specs/examples/516`
- model-check public seam compression after threshold and probe:
  `specs/examples/517`
- theorem actual Lean execution narrow probe after global toolchain install:
  `specs/examples/518`
- theorem actual Lean execution representative prototype widening:
  `specs/examples/519`
- model-check checker-artifact route actual adoption:
  `specs/examples/501`
- model-check final public-contract reopen threshold:
  `specs/examples/507`
- witness/provider route actual adoption:
  `specs/examples/502`
- order-handoff source wording route actual adoption:
  `specs/examples/503`
- witness/provider schema route actual adoption:
  `specs/examples/504`
- witness/provider final public-contract reopen threshold:
  `specs/examples/505`
- theorem proof-object schema / prover-brand coupled later gate:
  `specs/examples/494`
- model-check public-checker artifact preview / verifier-handoff reserve actualization:
  `specs/examples/492`
- model-check tool-brand / verifier-handoff coupled later gate:
  `specs/examples/495`
- model-check public-checker artifact / migration coupled later gate:
  `specs/examples/498`
- model-check checker-artifact route actual adoption:
  `specs/examples/501`
- witness/provider route actual adoption:
  `specs/examples/502`
- order-handoff source wording route actual adoption:
  `specs/examples/503`
- witness/provider schema route actual adoption:
  `specs/examples/504`
- witness/provider final public-contract reopen threshold:
  `specs/examples/505`
- theorem final public-contract reopen threshold:
  `specs/examples/506`
- model-check final public-contract reopen threshold:
  `specs/examples/507`
- witness/provider public-schema coupled later gate:
  `specs/examples/499`
- order-handoff source wording / emitted-artifact coupled later gate:
  `specs/examples/496`
- theorem result object / payload public-contract coupled later gate:
  `specs/examples/497`
- witness/provider public-contract / emitted-handoff coupled later gate:
  `specs/examples/493`
- model-check row-local property / checker-boundary actual adoption:
  `specs/examples/488`
- witness/provider/artifact public-shape actual adoption:
  `specs/examples/489`
- order-handoff surface actual adoption:
  `specs/examples/490`

## current self-driven queue

- current live line は、
  - Package 132 repo-local once-through closeout sync after executable loops
  - later mixed gate residual maintenance
  - true user-spec residual
  に移っている。

`specs/examples/466...555` は、
actual adoption package、near-end closeout、helper-local actualization floor を source-backed に揃える current anchor である。
`specs/examples/556...567` は、
phase6 parser second-tranche first-package close、first strong typing layer finite-index spine default、phase6 reserve formal tool binding inventory closeout、phase6 parser-side follow-up package sequencing closeout、phase6 shared single attachment frame first-package closeout、fixed-subset source-sample corpus scope-and-file-layout closeout、request clause suite publicization closeout、perform head structural carrier closeout、perform-head-request-clause-bundle-attachment compare floor、perform-head-request-clause-bundle thin-wrapper helper actualization、finite-index first-layer capture / lifetime / cost actualization、Lean-first formal skeleton hardening after finite-index widening を source-backed に同期する current anchor である。
`specs/examples/568...604` は、
Problem 1 / Problem 2 representative bundle hardening、split helper 群、residual lane summary、final-public-seam lane helper 群、syntax-modality final-marker lane helper、theorem-first emitted-artifact loop hardening、authoritative-room runnable scenario loop hardening、Problem 1 / Problem 2 executable residual reopen sync までを source-backed に同期する current anchor である。

## まだ OPEN のもの

- final parser grammar
- final public parser / checker / runtime API
- stronger typed surface actual adoption
- final public verifier contract
- concrete theorem prover / model-check tool binding
- final source-surface order / handoff wording
- shared-space final operational catalog
- backend / external codegen success criteria
- raw FFI / engine adapter target
- upper-layer application target

## 読み進める入口

1. repo の規範判断:
   `specs/00-document-map.md`、`specs/01`、`specs/02`、`specs/03`、`specs/09`
2. current status の snapshot:
   `progress.md`、`tasks.md`
3. long-lived repository memory:
   `plan/00-index.md`、`plan/01-status-at-a-glance.md`、`plan/10-roadmap-overall.md`、`plan/11-roadmap-near-term.md`
4. theory-lab の detail:
   `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
5. current explanation:
   `faq_005.md`
   `faq_006.md`
   `faq_007.md`
   `faq_008.md`
   `faq_009.md`
   `faq_010.md`
   `sub-agent-pro/codex_final_layer_closeout_handoff_2026-04-19.md`
   `sub-agent-pro/codex_once_completion_handoff_after_faq010_2026-04-20.md`

## 直近で特に重要な文書

- theory-lab operating model と comparison bundle:
  `specs/examples/405...412`
- typed / theorem / model-check / ordering の adjacent package:
  `specs/examples/413...448`
- current tranche / preview package:
  `specs/examples/451...588`
- current near-term order:
  `plan/11-roadmap-near-term.md`
- theory-lab の detail:
  `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- open problems / heavy line:
  `plan/12-open-problems-and-risks.md`、`plan/13-heavy-future-workstreams.md`

## 補足

- `faq_006.md` は current explanation delta であり、規範判断の正本ではない。
- `faq_007.md` は 2026-04-18 時点の current explanation snapshot であり、「どこまで終わっているか」「二大問題は完全解決済みか」「何を答えればどこまで自走できるか」を整理する。
- `faq_008.md` は 2026-04-18 時点の current explanation refresh であり、`faq_007.md` 以後の genuine progress、現状の finished / not finished / mixed gate / true user-spec gate、および「何を答えればどこまで自走できるか」の current limit を整理する。
- `faq_009.md` は 2026-04-19 時点の current explanation refresh であり、representative Lean sample set actual Lean execution floor を含む最新 status を踏まえて、done / not done / overall ladder / remaining gate / self-drive bound を整理する。
- `faq_010.md` は 2026-04-20 時点の current explanation refresh であり、`faq_009.md` 以後の Phase 6 parser-side tranche actualization を含む最新 status を踏まえて、二大問題の closeout 到達点、language implementation の current limit、remaining mixed gate / true user-spec gate、および「ここからどこまで自走できるか」の current bound を整理する。
- `sub-agent-pro/codex_final_layer_closeout_handoff_2026-04-19.md` は 2026-04-19 時点の detailed closeout handoff であり、layered strong typing / IFC、Lean formal skeleton、first completion scope、reopened self-driven queue の current explanation source として使ってよい。
- `sub-agent-pro/codex_once_completion_handoff_after_faq010_2026-04-20.md` は 2026-04-20 時点の repo-local once-through completion handoff であり、Package 91/92 actualization と、その後ろに置く Package 93...98 の staged self-driven sequence を current explanation source として使ってよい。current repo reading では Package 98 close 後の active queue は residual bundle 群に移ったと読む。
- `specs/examples/569` は Package 95 closeout として、edge-row principal / stage-block secondary / repo-local emitted artifact reading / delegated RNG reserve / late-join negative pair を `run-source-sample` helper summary に actualize した current source-backed route tightening note である。
- `specs/examples/570` は Package 96 closeout として、authoritative-room first default profile の representative reached pair `p07 / p08`、delegated RNG reserve `p09`、late-join negative static-stop pair `p13 / p14` を `run-source-sample` helper summary の `authoritative_room_first_scenario_actual_adoption` に actualize した current source-backed tightening note である。
- `specs/examples/571` は Package 97 closeout として、`auditable_authority_witness`、`delegated_rng_service`、model-check second line を `run-source-sample` helper summary の `authoritative_room_reserve_strengthening_lane` に separate status のまま actualize し、first completion line と reserve package 群の boundary を current source-backed に同期した note である。
- `specs/examples/572` は Package 98 closeout として、Problem 1 / Problem 2 の representative sample entrypoint を `samples/` README と `scripts/current_l2_guided_samples.py` に actualize し、guided sample reading を repo-local helper cut に留めた current source-backed note である。
- `specs/examples/573` は Package 99 closeout として、Problem 1 representative bundle `p06 / p10 / p11 / p12 / p15 / p16` の theorem/model-check public-seam residual を `matrix problem1` helper に actualize し、`p06` representative と first strong typing quintet の bridge-floor bundle を drift なく読めるようにした current source-backed note である。
- `specs/examples/574` は Package 100 closeout として、Problem 2 representative bundle `p07 / p08 / p09 / p13 / p14` の public-shape residual を `matrix problem2` helper に actualize し、first-line representative / reserve practical route / negative static-stop pair を drift なく読めるようにした current source-backed note である。
- `specs/examples/575` は Package 101 closeout として、Problem 1 を `bundle problem1` helper と `samples/lean/current-l2/` representative artifact、anchor spec / report まで一本道で辿れる theorem-first pilot bundle に actualize した current source-backed note である。
- `specs/examples/576` は Package 102 closeout として、Problem 2 を `bundle problem2` helper と representative pair / reserve / negative pair の Lean artifact、anchor spec / report まで一本道で辿れる authoritative-room scenario bundle に actualize した current source-backed note である。
- `specs/examples/577` は Package 103 closeout として、`p06 / p07 / p08` representative slice を `samples/prototype/current-l2-parser-companion/` と `mir-ast` parse test で actualize し、thin experimental companion surface を parser-side carrier に戻した current source-backed note である。
- `specs/examples/578` は Package 104 closeout として、`bundle problem1 / problem2` helper に parser companion path を actualize し、original prototype / parser companion / Lean artifact を一本道で辿れる bridge を作った current source-backed note である。
- `specs/examples/579` は Package 105 closeout として、`Stage3RequestHeadClauseBundle` の representative parse result を repo-local JSON / pretty inspector command に actualize し、parser-side carrier を docs-only で終わらせない current source-backed note である。
- `specs/examples/580` は Package 106 closeout として、`p06 / p07 / p08` representative slice の original prototype / parser companion / guided bundle / Lean artifact / anchor spec-report 対応を `mapping` helper と sample README table に actualize し、parser-side tranche の readable mapping cut を固定した current source-backed note である。
- `specs/examples/581` は Package 107 closeout として、二大問題の representative sample を `samples/problem-bundles/` の簡潔な日本語 guide と `bundle problem1|problem2` の `sample_bundle_doc` へ actualize し、`samples/` 側の explained bundle 導線を固定した current source-backed note である。
- `specs/examples/582` は Package 108 closeout として、`smoke problem1|problem2` helper を actualize し、representative sample bundle guide に書いた主要 command 群を repo-local helper から end-to-end で再現できるようにした current source-backed note である。
- `specs/examples/583` は Package 109 closeout として、`smoke-all` helper を actualize し、Problem 1 / Problem 2 の representative smoke 成否と step inventory を 1 コマンドで compact に俯瞰できるようにした current source-backed note である。
- `specs/examples/584` は Package 110 closeout として、`smoke-all` に failure-focused diagnostics を actualize し、failed step / command / return code / output excerpt を compact に surfacing しつつ aggregate failure を non-zero exit に戻した current source-backed note である。
- `specs/examples/585` は Package 111 closeout として、`samples/problem-bundles/problem1|problem2` に `最短 quickstart` と `見るべき結果` を actualize し、representative sample guide を doc 単体でも読める quickstart bundle に harden した current source-backed note である。
- `specs/examples/586` は Package 112 closeout として、`quickstart problem1|problem2` helper を actualize し、bundle doc 側の representative 4-step quickstart を helper-side summary にも mirror した current source-backed note である。
- `specs/examples/587` は Package 113 closeout として、`quickstart-parity` helper を actualize し、sample bundle doc と helper-side quickstart mirror の 4-step parity を focused に確認できるようにした current source-backed note である。
- `specs/examples/588` は Package 114 closeout として、`reopen-map` helper と sample bundle doc 側の `現在の mixed gate 再開点` section を actualize し、representative sample floor の後でどの mixed gate をどの command から reopen するかを helper / docs / snapshot で揃えた current source-backed note である。
- 規範判断の正本は常に `specs/` に残る。
- `plan/` は long-lived repository memory であり、snapshot ではない。
- `progress.md` と `tasks.md` は current queue と remaining gate を mirror する薄い snapshot として保つ。
