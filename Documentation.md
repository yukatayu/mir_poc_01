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
    （current-l2 authored sixteen と corrected prototype set `p01...p14` は fixed 済みだが、sample corpus 自体は theory-line の adequacy corpus として active に使う）
  - theory-lab lane:
    `Macro 5 final-layer closeout packages active`
    （`specs/examples/458...465` compare floor、`466...469` actual adoption floor、`470...474` helper-local actualization / narrowing floor、`475...519` deeper-theory / reserve / mixed-gate / actual-execution actualization floor が揃った。corrected runnable floor と representative Lean sample set `e5 / p06 / p10 / p11 / p12 / p07 / p08 / p09 / p13 / p14` actual Lean execution は reached 済みであり、`specs/examples/520`、`521`、`522`、`523`、`524`、`525`、`526`、`527`、`528`、`529`、`530`、`531` により Lean formal skeleton / proof obligations first slice、IFC secret valid/invalid concrete example、source-side authority pair、source-side label-flow negative、delegated RNG provider placement carry-over、order-handoff helper CLI surface preview、order-handoff negative static-stop pair、order-handoff negative pair theorem-side Lean carry-over、IFC typed checker-hint preview narrow actualization、theorem/model-check helper preview widening、near-end closeout sync after Package 58 は `samples/lean/` committed corpus、source-side prototype corpus、helper-local CLI summary、source-sample runner static gate、export/sync helper、snapshot / roadmap / traceability sync に actualize 済みと読む。current remaining active line は residual mixed-gate compression と later true user-spec residual に narrowed した）
  - reserve integration lane:
    `Macro 6 minimal working subset actual default / Macro 7 mixed`
    （authoritative room minimal working subset と repo-local near-end success criteria は current default に上がったが、installed-binary / packaging / FFI / engine adapter / exhaustive shared-space catalog は still later に残る）

## source-backed で既にあるもの

- current L2 semantics / parser-free validation substrate / compile-ready minimal actualization
- fixed-subset source sample authored sixteen:
  `e1 / e2 / e3 / e4 / e5 / e12 / e14 / e15 / e16 / e13 / e19 / e21 / e22 / e18 / e20 / e23`
- runnable prototype sample set:
  `p01 / p02 / p03 / p04 / p05 / p06 / p07 / p08 / p09 / p10 / p11 / p12 / p13 / p14`
  （`samples/prototype/` に置き、current lowerer / runner へ explicit path で流す）
  - `p01...p05 / p07 / p08 / p09 / p13 / p14` は order/handoff family
  - `p06 / p10 / p11 / p12` は typed/theorem/model-check / IFC sample-visible corrected prototype
- helper-local debug output preview:
  prototype / sample 実行時に `debug_*` または `_debug_` を含み `_output` / `_pipe` で終わる target の record を `debug_outputs` として見せる current cut がある
- helper-local verification preview:
  prototype / sample 実行時に `formal_hook_status`、`subject_kind`、obligation list を `verification_preview` として見せる current cut がある
- helper-local artifact preview:
  prototype / sample 実行時に proof notebook review unit / model-check concrete carrier の derived row preview を `artifact_preview` として見せる current cut がある
- helper-local order-handoff surface preview:
  prototype / sample 実行時に `minimal_companion` / `stage_block_secondary` / `serial_scope_reserve` の surface family を `surface_preview` として見せる current cut がある
- helper-local typed checker-hint preview:
  prototype / sample 実行時に source-side IFC trio `p10 / p11 / p12` を `typed_checker_hint_preview` として見せ、`family_refs[] + coverage_state` を sample-local helper preview に留める current cut がある
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
  checker-adjacent semantic carrier principal、structural marker family first、notebook-first theorem line、row-local model-check carrier first
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
  - layered strong typing / IFC first-fragment
  - actual Lean execution の representative Lean sample set `e5 / p06 / p10 / p11 / p12 / p07 / p08 / p09 / p13 / p14` から helper/CLI hardening と broader coverage への widening
  - order-handoff helper-local `surface_preview` actualization
  - order-handoff negative static-stop pair `p13 / p14` actualization
  - order-handoff negative pair `p13 / p14` theorem-side Lean carry-over
  - IFC typed checker-hint preview narrow actualization
  - theorem / model-check helper preview widening
  - near-end closeout sync after Package 58
  - committed `samples/lean/` corpus を基点にした broader theorem-side / diagnostics / order-handoff widening
  - later mixed gate residual maintenance
  - true user-spec residual
  に narrow 化している。

`specs/examples/466...531` は、
actual adoption package、near-end closeout、helper-local actualization floor を source-backed に揃える current anchor である。

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
   `sub-agent-pro/codex_final_layer_closeout_handoff_2026-04-19.md`

## 直近で特に重要な文書

- theory-lab operating model と comparison bundle:
  `specs/examples/405...412`
- typed / theorem / model-check / ordering の adjacent package:
  `specs/examples/413...448`
- current tranche / preview package:
  `specs/examples/451...531`
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
- `sub-agent-pro/codex_final_layer_closeout_handoff_2026-04-19.md` は 2026-04-19 時点の detailed closeout handoff であり、layered strong typing / IFC、Lean formal skeleton、first completion scope、reopened self-driven queue の current explanation source として使ってよい。
- 規範判断の正本は常に `specs/` に残る。
- `plan/` は long-lived repository memory であり、snapshot ではない。
- `progress.md` と `tasks.md` は current queue と remaining gate を mirror する薄い snapshot として保つ。
