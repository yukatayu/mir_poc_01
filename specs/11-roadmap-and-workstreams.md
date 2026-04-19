# 11 — ロードマップと Workstream

## 原則

このプロジェクトは、大規模な実装へ進む前に意味論を安定化することで前進すべきである。
意味の切り分けが曖昧なまま public surface や backend を凍らせてはならない。

ここでいう roadmap は strict waterfall ではない。
mainline actualization と docs-first research package が並走してよい。

## 推奨 workstream

### Workstream A — Mir 仕様コア

目標:

- Mir-0 の最小形式意味論を確定する
- 現時点で合意されている primitive の正確な syntax と type rule を記述する
- cut、effect、contract、failure、monotone な ownership / lifetime behavior を固定する

### Workstream B — Mir runtime proof of concept

目標:

- single-process interpreter を構築する
- event graph extraction を支援する
- fallback / try / cut を支援する
- trace artifact を生成する

### Workstream C — Mirrorea の最小 fabric

目標:

- 論理名
- route rebinding
- overlay registration
- downstream patch activation
- 基本的な audit

### Workstream D — PrismCascade の最小 kernel

目標:

- Meta / Core / Runtime 分離
- graph normalization
- 最小 scheduler と memory plan
- 最小 trace output

### Workstream E — 共有統合面

目標:

- 共有 identifier
- 適切な範囲での共有 contract schema
- link された tracing strategy
- Mir と Prism の最小 bridge

### Workstream F — 可視化と editor support

目標:

- language server の基礎
- graph view
- cut / route / patch visualization
- report-driven workflow support

### Workstream G — アプリケーション実験

目標:

- 少なくとも 1 つの小さな synchronized shared-space 例
- 1 つの小さな virtual-world または collaborative editing 例
- 1 つの小さな route / overlay insertion 例

## cross-cutting docs-first theory packages

上の workstream に加えて、current repo では **meaning first, implementation later** を守るための
cross-cutting theory package を docs-first に進めてよい。

### A/E/G に跨る理論線

1. **typed / theorem / model-check boundary**
   - first typed attachment candidate、obligation owner matrix、
     semantic-core theorem pilot、
     model-check projection / property-family reserve を
     full type system / concrete tool binding の前に docs-first で整理する。
2. **cut family / order family**
   - `atomic_cut` の local nucleus を維持したまま、
     observation / snapshot、ordering-only barrier、commit-bearing durable cut、
     publication / observation / witness / finalization relation を比較する。
3. **authority-handoff / shared-space order**
   - authority placement、provider placement、witness、fairness source、replay attachment を同軸化せずに比較する。
4. **thread / node parity**
   - source-level causal language の平等性と、
     lowering / transport / evidence / failure / durability policy の非対称性を切り分ける。
5. **syntax / semantics honesty**
   - compactness ではなく semantic honesty、checker legibility、modal adequacy、misreading resistance を主軸に syntax candidate を比較する。
6. **modal foundation / verifier boundary**
   - `lambda-circle-box` を partial basis candidate に留めつつ、
     guarded / modal dependent / multimodal line を stronger candidate として比較する。
   - property-to-boundary matrix は
     `core_static_checker` / `theorem_prover_boundary` /
     `protocol_verifier_boundary` / `runtime_policy_boundary`
     を維持して整理する。

### package discipline

- these theory packages are **comparison / adequacy / operating-model work**, not immediate public API work.
- final parser grammar、final public API、shared-space final catalog、backend success criteria、upper-layer app contract はここで固定しない。
- 必要なら tiny non-production prototype / simulator / compare helper を使ってよいが、mainline runtime semantics へ直結させない。
- current mixed-gate concretization でも、helper-local preview を final public contract に昇格させず、sample-local compare floor を別 helper として切ってよい。

### current compare-floor / actual-adoption / actualization chain

`faq_006.md`、`faq_007.md`、`faq_008.md` と
`specs/examples/458...506` の current reading では、
theory-lab line は次の 10 段で読むのが自然である。

1. compare-floor / integrator floor
   - `specs/examples/458...465`
   - current first line、retained alternatives、stop line、helper-local compare floor を揃える
2. actual-adoption floor
   - `specs/examples/466...469`
   - surviving current first line と user-authorized defaults を near-end actual adoption package に上げる
3. helper-local actualization floor
   - `specs/examples/470...474`
   - actual adoption package を public contract に上げず、repo-local emitted artifact / vertical-slice helper / narrowing helper まで actualize する
4. principal theory spine / proof-roadmap integration floor
   - `specs/examples/475`
   - actual adoption package と helper-local actualization floor を支える deeper theory spine、layered typing/proof architecture、compatibility metatheory package、Lean-first proof roadmap を current recommendation に上げる
5. reserve strengthening actualization floor
   - `specs/examples/476`
   - shared-space witness strengthening を public witness schema に上げず、minimal witness core / claim-payload split / symbolic binding ref cut を helper-local manifest へ actualize する
6. reserve practical actualization floor
   - `specs/examples/477`
   - shared-space provider placement を public provider receipt schema に上げず、narrow prototype / optional attachment cut / authority-kept-commit split を helper-local manifest へ actualize する
7. model-check second-line actualization floor
   - `specs/examples/478`
   - row-local property preview / request preflight / public-checker second reserve split を first settled property language や concrete tool brand に上げず、helper-local second-line manifest へ actualize する
8. theorem discharge actual-format probe floor
   - `specs/examples/479`
   - transport preview / public-contract preview / notebook-consumer-first boundary を actual discharge transport や public theorem contract に上げず、helper-local actual-format probe に actualize する
9. model-check property/tool-seam probe floor
   - `specs/examples/480`
   - property-language probe / tool-seam probe / checker-boundary probe を first settled property language や actual public checker migration に上げず、helper-local mixed-gate probe に actualize する
10. theorem discharge / public-theorem-contract threshold floor
   - `specs/examples/481`
   - review-unit-first / discharge-entry-adjacent / notebook-consumer-first / brand-neutral theorem request default を actual discharge transport や public theorem contract に上げず、helper-local threshold manifest に actualize する
11. theorem contract shape threshold floor
   - `specs/examples/485`
   - refs-only reserve schema first / review-unit transport anchor / brand-neutral request manifest keep / consumer-shaped theorem payload later を actual discharge transport や public theorem contract に上げず、helper-local threshold manifest に actualize する
12. theorem transport/public-contract coupled later-gate floor
   - `specs/examples/486`
   - transport/public-contract adjacent but distinct / review-unit anchor / refs-only reserve schema first / consumer-shaped payload later を actual discharge transport adoption や public theorem contract adoption に上げず、helper-local actualization manifest に actualize する
13. theorem review-unit transport / notebook-contract actual-adoption floor
   - `specs/examples/487`
   - review-unit transport first / notebook-consumer contract first / brand-neutral binding reserve keep を theorem result public object や final public verifier contract に上げず、helper-local actual adoption manifest に actualize する
14. theorem result-object preview / proof-object-schema reserve actualization floor
   - `specs/examples/491`
   - notebook-consumer object first / consumer-shaped payload preview only / proof-object-schema reserve keep を final public theorem result object や proof object public schema に上げず、helper-local actualization manifest に actualize する
15. theorem proof-object schema / prover-brand coupled-later floor
   - `specs/examples/494`
   - result-object preview keep / refs-only public-schema candidate only / brand-neutral preflight anchor keep / concrete brand not adopted を final public theorem result object や proof object public schema や concrete theorem prover brand に上げず、helper-local coupled-later manifest に actualize する
16. model-check property/tool threshold floor
   - `specs/examples/482`
   - row-local-property-preview first / small-cluster-projection second / brand-neutral model-check request keep / public-checker-contract-later default を first settled property language や concrete tool brand に上げず、helper-local threshold manifest に actualize する
17. model-check row-local property / checker-boundary actual-adoption floor
   - `specs/examples/488`
   - row-local property route first / checker-boundary contract first / brand-neutral tool-binding reserve keep を first settled property language や actual public checker migration に上げず、helper-local actual-adoption manifest に actualize する
18. witness/provider/artifact public-shape threshold floor
   - `specs/examples/483`
   - claim/payload split first / repo-local emitted artifact refs first / optional attachment refs only / combined public contract later を final public witness schema や provider receipt や emitted-handoff contract に上げず、helper-local threshold manifest に actualize する
19. witness/provider/artifact public-shape actual-adoption floor
   - `specs/examples/489`
   - witness/provider route non-collapse / repo-local emitted artifact refs first / combined public contract later を final public witness schema や provider receipt や emitted-handoff contract に上げず、helper-local actual-adoption manifest に actualize する
20. order-handoff surface/artifact threshold floor
   - `specs/examples/484`
   - edge-row principal / stage-block secondary / repo-local emitted artifact refs first を final source wording や final emitted-artifact schema に上げず、helper-local threshold manifest に actualize する
21. order-handoff surface actual-adoption floor
   - `specs/examples/490`
   - edge-row principal / stage-block secondary keep / repo-local emitted artifact refs first を final source wording や final emitted-artifact schema に上げず、helper-local actual-adoption manifest に actualize する
22. model-check public-checker artifact preview / verifier-handoff reserve actualization floor
   - `specs/examples/492`
   - consumer-shaped artifact preview only / verifier-handoff reserve keep / brand-neutral tool-binding reserve keep を final public checker artifact や actual public checker migration に上げず、helper-local actualization manifest に actualize する
23. model-check tool-brand / verifier-handoff coupled-later floor
   - `specs/examples/495`
   - public-checker artifact preview keep / verifier-handoff candidate only / tool-brand candidate only を concrete model-check tool brand や final public checker artifact や actual emitted verifier handoff artifact に上げず、helper-local coupled-later manifest に actualize する
24. witness/provider public-contract / emitted-handoff coupled-later floor
   - `specs/examples/493`
   - claim/payload split first / witness-provider route non-collapse / repo-local emitted artifact refs first / combined public contract later / final emitted-handoff contract later を final public witness schema や provider receipt や emitted-handoff contract に上げず、helper-local coupled-later manifest に actualize する
25. order-handoff source wording / emitted-artifact coupled-later floor
   - `specs/examples/496`
   - edge-row principal / stage-block secondary keep / thread-node same causal language keep / repo-local emitted artifact refs first / source-wording and emitted-artifact candidate only を final source wording や final emitted-artifact schema に上げず、helper-local coupled-later manifest に actualize する
26. theorem result-object / payload public-contract coupled-later floor
   - `specs/examples/497`
   - notebook-consumer object first / consumer-shaped payload candidate only / proof-object-schema-prover-brand adjacent keep を final public theorem result object や consumer-shaped theorem payload public contract に上げず、helper-local coupled-later manifest に actualize する
27. model-check public-checker artifact / migration coupled-later floor
   - `specs/examples/498`
   - consumer-shaped public-checker artifact candidate only / actual public checker migration candidate only / tool-brand-verifier-handoff adjacent keep を final public checker artifact や actual public checker migration や actual emitted verifier handoff artifact に上げず、helper-local coupled-later manifest に actualize する
28. witness/provider public-schema coupled-later floor
   - `specs/examples/499`
   - witness-schema candidate only / provider-receipt candidate only / combined public-contract candidate only / repo-local emitted artifact refs first を final public witness schema や final public provider receipt schema や combined public contract や final emitted-handoff contract に上げず、helper-local coupled-later manifest に actualize する
29. theorem result-object route actual-adoption floor
   - `specs/examples/500`
   - review-unit transport first / notebook-consumer object first / consumer-shaped payload preview keep / proof-object-schema-prover-brand later を final public theorem result object や consumer-shaped theorem payload public contract や proof object public schema に上げず、helper-local actual-adoption manifest に actualize する
30. theorem final-public-contract reopen-threshold floor
   - `specs/examples/506`
   - result-object-and-payload first / prover-brand-and-proof-schema second / final public verifier contract third を final public theorem result object や consumer-shaped theorem payload public contract や concrete theorem prover brand や proof object public schema や final public verifier contract に上げず、helper-local threshold manifest に actualize する
31. model-check checker-artifact route actual-adoption floor
   - `specs/examples/501`
   - row-local property route first / checker-boundary contract anchor / consumer-shaped checker-artifact candidate only / migration candidate adjacent keep を final public checker artifact や actual public checker migration や actual emitted verifier handoff artifact に上げず、helper-local actual-adoption manifest に actualize する
32. witness/provider route actual-adoption floor
   - `specs/examples/502`
   - witness/provider route first / public-schema candidate keep / combined public-contract later / final emitted-handoff contract adjacent keep を final public witness schema や final public provider receipt schema や combined provider+witness public contract や final emitted-handoff contract に上げず、helper-local actual-adoption manifest に actualize する
33. order-handoff source-wording route actual-adoption floor
   - `specs/examples/503`
   - principal source wording route first / emitted-artifact schema candidate keep / final source wording later を final source-surface handoff wording や final emitted-artifact schema や final public witness/provider contract に上げず、helper-local actual-adoption manifest に actualize する
34. witness/provider schema-route actual-adoption floor
   - `specs/examples/504`
   - witness-schema candidate keep + witness route first / provider-receipt candidate keep + provider route first / combined public-contract candidate keep / final emitted-handoff contract adjacent keep を final public witness schema や final public provider receipt schema や combined provider+witness public contract や final emitted-handoff contract に上げず、helper-local actual-adoption manifest に actualize する
35. witness/provider final-public-contract reopen-threshold floor
   - `specs/examples/505`
   - public-schema pair first / delegated attestation + combined provider+witness public contract second / final emitted-handoff contract third / exhaustive shared-space catalog later を final public witness schema や final public provider receipt schema や delegated provider attestation や combined provider+witness public contract や final emitted-handoff contract に上げず、helper-local threshold manifest に actualize する

compare-floor 側は、
**何も決まっていない OPEN comparison** ではなく、
actual package へ上げる前の surviving candidate chain と読む。

actual-adoption floor では、
次を current package judgment として読んでよい。

- Problem 1:
  `specs/examples/466`
  - Workstream A / B / E に跨る typed / theorem / model-check actual adoption package
  - checker-adjacent principal
  - structural marker family first
  - theorem-first external integration target
  - notebook-first theorem line
  - row-local model-check carrier first
- Problem 2:
  `specs/examples/467`
  - Workstream A / C / E / G に跨る order / handoff / authoritative-room default profile actual adoption package
  - relation decomposition principal
  - `authority_serial_transition_family` first
  - thread/node parity default wording
  - authoritative room first actual adoption profile
- syntax / modality:
  `specs/examples/468`
  - Workstream A / E に跨る syntax / semantics coupling principle、5 evaluation axes、partial basis keep
- near-end closeout:
  `specs/examples/469`
  - repo-local near-end success criteria default
  - queue nonzero reading
  - mixed gate / user-spec residual separation
- helper-local actualization:
  `specs/examples/470...474`
  - theorem-first pilot actualization
  - theorem-prover experimental binding preflight
  - authoritative-room vertical-slice ratchet
  - minimal companion / experimental order-handoff surface
  - order-handoff surface narrowing / stage-block secondary candidate
- principal theory spine / proof roadmap:
  `specs/examples/475`
  - multimodal dependent core research direction
  - layered theory stack
  - compatibility metatheory package
  - Lean-first proof roadmap
- final-layer closeout defaults / reopened queue:
  `specs/examples/520`
  - layered strong typing / IFC first-fragment
  - Lean formal skeleton / proof obligations
  - first completion scope
  - reopened self-driven closeout package order
- reserve strengthening actualization:
  `specs/examples/476`
  - `auditable_authority_witness` minimal witness core actualization
  - claim / payload split preserve
  - `p07` reached / `p08,p05` guard-only
  - public witness schema / public provider receipt schema は still later
- reserve practical actualization:
  `specs/examples/477`
  - `delegated_rng_service` provider placement actualization
  - `p09` reached / `p07,p08` guard-only
  - optional provider attachment cut
  - public provider receipt schema / delegated attestation は still later
- model-check second-line actualization:
  `specs/examples/478`
- theorem discharge actual-format probe:
  `specs/examples/479`
- model-check property/tool-seam probe:
  `specs/examples/480`
  - row-local property preview
  - brand-neutral request preflight
  - public-checker second reserve split
  - `e5 / p06 / p07 / p08 / p09` reached / `p05` guard-only
- theorem discharge / public-theorem-contract threshold default:
  `specs/examples/481`
  - review-unit first
  - discharge-entry adjacent
  - notebook-consumer first
  - brand-neutral theorem request
  - `e5 / p06 / p07 / p08` reached / `p05` guard-only
- theorem contract shape threshold default:
  `specs/examples/485`
  - refs-only reserve schema first
  - review-unit transport anchor
  - brand-neutral request manifest keep
  - consumer-shaped theorem payload later
  - `e5 / p06 / p07 / p08` reached / `p05` guard-only
- theorem transport/public-contract coupled later gate:
  `specs/examples/486`
  - transport/public-contract adjacent but distinct
  - review-unit anchor
  - refs-only reserve schema first
  - consumer-shaped payload later
  - `e5 / p06 / p07 / p08` reached / `p05` guard-only
- theorem review-unit transport / notebook-contract actual adoption:
  `specs/examples/487`
  - review-unit transport first
  - notebook-consumer contract first
  - brand-neutral binding reserve keep
  - `e5 / p06 / p07 / p08` reached / `p05` guard-only
- theorem result-object preview / proof-object-schema reserve actualization:
  `specs/examples/491`
  - notebook-consumer object first
  - consumer-shaped payload preview only
  - proof-object-schema reserve keep
  - `e5 / p06 / p07 / p08` reached / `p05` guard-only
- theorem proof-object schema / prover-brand coupled later gate:
  `specs/examples/494`
  - result-object preview keep
  - refs-only public-schema candidate only
  - brand-neutral preflight anchor keep
  - concrete brand not adopted
  - `e5 / p06 / p07 / p08` reached / `p05` guard-only
- model-check property/tool-brand threshold default:
  `specs/examples/482`
  - row-local property preview first
  - small-cluster semantic projection second
  - brand-neutral model-check request
  - public checker contract later
  - `e5 / p06 / p07 / p08 / p09` reached / `p05` guard-only
- model-check row-local property / checker-boundary actual adoption:
  `specs/examples/488`
  - row-local property route first
  - checker-boundary contract first
  - brand-neutral tool-binding reserve keep
  - public checker handoff later
  - `e5 / p06 / p07 / p08 / p09` reached / `p05` guard-only
- model-check public-checker artifact preview / verifier-handoff reserve actualization:
  `specs/examples/492`
  - consumer-shaped artifact preview only
  - verifier-handoff reserve keep
  - brand-neutral tool-binding reserve keep
- model-check tool-brand / verifier-handoff coupled later gate:
  `specs/examples/495`
  - public-checker artifact preview keep
  - verifier-handoff candidate only
  - tool-brand candidate only
  - `e5 / p06 / p07 / p09` reached / `p05` guard-only
- order-handoff source wording / emitted-artifact coupled later gate:
  `specs/examples/496`
  - edge-row principal
  - stage-block secondary keep
  - thread/node same causal language keep
  - repo-local emitted artifact refs first
  - `p07 / p08` reached / `p05` guard-only
- theorem result object / payload public-contract coupled later gate:
  `specs/examples/497`
  - notebook-consumer object first
  - consumer-shaped payload candidate only
  - proof-object-schema / prover-brand adjacent keep
  - `e5 / p06 / p07 / p08` reached / `p05` guard-only
- theorem result-object route actual adoption:
  `specs/examples/500`
  - review-unit transport first
  - notebook-consumer object first
  - consumer-shaped payload preview keep
  - proof-object-schema / prover-brand later
  - `e5 / p06 / p07 / p08` reached / `p05` guard-only
- theorem final public-contract reopen threshold:
  `specs/examples/506`
  - result-object and payload first
  - prover-brand and proof-schema second
  - final public verifier contract third
  - `e5 / p06 / p07 / p08` reached / `p05` guard-only
- theorem Lean-first non-production stub pilot actualization:
  `specs/examples/508`
  - review-unit first
  - brand-neutral preflight anchor keep
  - Lean-first non-production stub only
  - `e5 / p06 / p07 / p08` reached / `p05` guard-only
- theorem review-unit to Lean-stub repo-local artifact-conformance bridge:
  `specs/examples/509`
  - representative runtime/static source sample `e2 / e5`
  - formal hook / review unit / Lean stub pair alignment
  - regression-integrated reproducible compare floor
  - actual Lean tool execution / public theorem contract later
- theorem Lean-stub representative trace-alignment bridge:
  `specs/examples/510`
  - representative runtime/static/prototype corpus `e2 / e5 / p06 / p07 / p08`
  - guard-only contrast `p05`
  - helper-local pair-alignment runtime test
  - actual Lean tool execution / public theorem contract later
- order-handoff serial-scope reserve surface:
  `specs/examples/511`
  - `p07 / p08 / p09` reached / `p05` guard-only
  - authoritative-room-specific `serial on ...` reserve surface
  - principal edge-row surface unchanged
- witness/provider emitted-contract representative trace-alignment bridge:
  `specs/examples/512`
  - `p07 / p08 / p09` reached / `p05` guard-only
  - route side / emitted-contract side pair alignment
  - final public witness/provider contract later
- theorem actual Lean execution availability probe:
  `specs/examples/513`
  - local `lean` / `lake` / `elan` availability probe
  - tool unavailable なら Lean-stub floor keep
  - actual Lean execution / public theorem contract later
- theorem public seam compression after local Lean-unavailable probe:
  `specs/examples/514`
  - theorem final public-contract reopen threshold carry-over
  - Lean-stub representative bridge carry-over
  - actual Lean execution environment stop line
  - remaining theorem public seam residual matrix
- order-handoff / witness-provider final public seam compression after reserve actualizations:
  `specs/examples/515`
  - principal source wording route carry-over
  - `serial` reserve surface carry-over
  - witness/provider representative trace alignment carry-over
  - shared-space final public-contract residual matrix
- theorem actual Lean execution toolchain probe and reopen manifest:
  `specs/examples/516`
  - `lean` / `lake` / `elan` ready probe
  - sample-aware reopen manifest
  - actual Lean execution later
- model-check public seam compression after threshold and probe:
  `specs/examples/517`
  - property/tool seam carry-over
  - checker-artifact route carry-over
  - model-check public-seam residual matrix
- theorem actual Lean execution narrow probe after global toolchain install:
  `specs/examples/518`
  - official `elan` stable first
  - representative static sample `e5` actual Lean execution
  - placeholder-fix carry-over
- theorem actual Lean execution representative prototype widening:
  `specs/examples/519`
  - representative prototype trio `p06 / p07 / p08` actual Lean execution
  - runtime support cut first
  - helper / CLI unification later
- final-layer closeout queue reconstruction:
  `specs/examples/520`
  - representative Lean sample set actual Lean execution 後の self-driven queue は empty ではない
  - layered strong typing / IFC first-fragment
  - Lean formal skeleton / proof obligations
  - broader negative corpus / helper hardening / near-end closeout sync
- Lean sample corpus and first foundations:
  `specs/examples/521`
  - `samples/lean/foundations/` actual small proof fragment
  - `samples/lean/current-l2/` representative Lean sample set generated stub corpus
  - `current_l2_emit_theorem_lean_bundle` + `current_l2_lean_sample_sync.py`
  - generated stub = artifact well-formedness / bridge alignment, not final discharge
- IFC secret valid/invalid foundation and Japanese Lean corpus sync:
  `specs/examples/522`
  - `CurrentL2IfcSecretExamples.lean` actualize
  - secret-key valid/invalid + explicit authority declassification concrete example
  - `samples/lean/` explanation wording を日本語で同期
- source-side IFC authority prototype pair and Lean corpus widening:
  `specs/examples/523`
  - `p10 / p11` source-side explicit authority pair
  - representative Lean sample set widening
  - stronger typed source principal は still later
- source-side IFC label-flow negative closeout and Lean corpus widening:
  `specs/examples/524`
  - `p12` source-side label-flow negative
  - verifier preview / model-check projection / theorem Lean actualization representative set widening
  - Package 56 closeout、helper/CLI hardening next
- delegated RNG provider placement representative carry-over:
  `specs/examples/525`
  - `p09` delegated RNG provider placement
  - representative Lean sample set carry-over
  - helper/CLI hardening と broader coverage widening の first slice
- order-handoff helper CLI surface preview actualization:
  `specs/examples/526`
  - `run-source-sample` helper-local `surface_preview`
  - `p07 / p08 / p09` principal/secondary/reserve reached/guarded actualization
  - Package 58 helper/CLI hardening first slice
- order-handoff negative static-stop actualization:
  `specs/examples/527`
  - `p13 / p14` late-join visibility negative pair
  - helper-local source-sample runner static stop
  - Package 58 order-handoff negative corpus tightening first pair
- order-handoff negative pair representative Lean sample-set carry-over:
  `specs/examples/528`
  - `p13 / p14` late-join visibility negative pair
  - representative Lean sample set / committed Lean corpus carry-over
  - Package 58 broader theorem-side widening after static-stop actualization
- IFC typed checker-hint preview actualization:
  `specs/examples/529`
  - `p10 / p11 / p12` source-side IFC trio
  - sample-local `typed_checker_hint_preview`
  - `family_refs[] + coverage_state` narrow mirror
  - Package 58 IFC helper widening after representative Lean sample set carry-over
- theorem/model-check helper preview widening:
  `specs/examples/530`
  - theorem result-object preview helper mirror
  - model-check public-checker preview helper mirror
  - `p08` theorem reached / model-check guarded
  - `p09` theorem guarded / model-check reached
  - Package 58 diagnostics widening close after IFC checker-hint mirror
- near-end closeout sync after Package 58:
  `specs/examples/531`
  - Package 58 close 後の queue reconstruction
  - residual mixed-gate packages `60 / 61`
  - true user-spec residual split keep
  - stale wording suppression after helper preview widening
- theorem/model-check reopen-threshold helper mirror:
  `specs/examples/532`
  - theorem final public-contract reopen threshold helper mirror
  - model-check final public-contract reopen threshold helper mirror
  - `p08` theorem reached / model-check guarded
  - `p09` theorem guarded / model-check reached
  - Package 60 residual mixed-gate compression close
- order-handoff/shared-space public-seam helper mirror:
  `specs/examples/533`
  - order-handoff / witness-provider public seam helper mirror
  - `p07 / p08` reached / `p09` guard-only
  - source wording / emitted artifact / witness-provider residual compressed keep
  - Package 61 residual mixed-gate compression close
- IFC actual-checker-payload-family threshold helper mirror:
  `specs/examples/534`
  - `p10 / p11 / p12` source-side IFC trio
  - `actual_checker_payload_family_threshold`
  - `payload_family_kind + source_refs` checker-adjacent helper mirror
  - Package 62 helper-to-checker ratchet close、Package 63 checker payload row-family ratchet next
- IFC checker-payload-row-family threshold helper mirror:
  `specs/examples/535`
  - `p10 / p11 / p12` source-side IFC trio
  - `actual_checker_payload_row_family_threshold`
  - `payload_family_ref + row_family_kind` row grouping helper mirror
  - Package 63 checker payload row-family ratchet close、Package 64 checker payload row-detail ratchet next
- IFC checker-payload-row-detail threshold helper mirror:
  `specs/examples/536`
  - `p10 / p11 / p12` source-side IFC trio
  - `actual_checker_payload_row_detail_threshold`
  - `payload_row_family_ref + row_source_ref + row_reason_kind` row-detail helper mirror
  - Package 64 checker payload row-detail ratchet close、Package 65 checker payload row-body ratchet next
- IFC checker-payload-row-body threshold helper mirror:
  `specs/examples/537`
  - `p10 / p11 / p12` source-side IFC trio
  - `actual_checker_payload_row_body_threshold`
  - `row_body = { selected_option_ref, visibility_target_ref }` row-body helper mirror
  - Package 65 checker payload row-body ratchet close、Package 66 checker payload supported-kind-summary ratchet next
- IFC checker-payload-supported-kind-summary threshold helper mirror:
  `specs/examples/538`
  - `p10 / p11 / p12` source-side IFC trio
  - `actual_checker_payload_supported_kind_summary_threshold`
  - `payload_row_family_ref + supported_kind_scope + supported_kind_refs` row-family keyed summary mirror
  - Package 66 checker payload supported-kind-summary ratchet close、Package 67 checker payload public-schema sketch ratchet next
- IFC checker-payload-public-schema-sketch threshold helper mirror:
  `specs/examples/539`
  - `p10 / p11 / p12` source-side IFC trio
  - `actual_checker_payload_public_schema_sketch_threshold`
  - `actual_checker_payload_family_ref + checker_payload_row_family_ref + checker_payload_row_detail_ref + checker_payload_row_body_ref + checker_payload_supported_kind_summary_ref` 5 ref wrapper mirror
  - Package 67 checker payload public-schema sketch ratchet close、Package 68 checker payload public-checker-api sketch ratchet next
- IFC public-checker-api sketch threshold helper mirror:
  `specs/examples/540`
  - `p10 / p11 / p12` source-side IFC trio
  - `actual_public_checker_api_sketch_threshold`
  - `checker_subject + public_checker_payload_schema_ref` minimal API read relation mirror
  - Package 68 public-checker-api sketch ratchet close、Package 69 public-checker entry-criteria ratchet close
- IFC public-checker-entry-criteria threshold helper mirror:
  `specs/examples/541`
  - `p10 / p11 / p12` source-side IFC trio
  - `actual_public_checker_entry_criteria_threshold`
  - `public_checker_api_ref + entry_criteria_refs + next_comparison_target_ref + deferred_boundary_refs` entry-criteria minimum mirror
  - Package 69 public-checker entry-criteria ratchet close、Package 70 public-checker command-surface ratchet close
- IFC public-checker-command-surface threshold helper mirror:
  `specs/examples/542`
  - `p10 / p11 / p12` source-side IFC trio
  - `actual_public_checker_command_surface_threshold`
  - `command_surface_kind + family_facade_command_refs + public_checker_api_ref` command-surface minimum mirror
  - Package 70 public-checker command-surface ratchet close、Package 71 shared-output-contract ratchet close
- IFC shared-output-contract threshold helper mirror:
  `specs/examples/543`
  - `p10 / p11 / p12` source-side IFC trio
  - `actual_shared_output_contract_threshold`
  - `output_contract_kind + checker_cluster_name + checker_status + public_checker_payload_schema_ref` shared-output-contract minimum mirror
  - Package 71 shared-output-contract ratchet close、Package 72 public-checker-boundary ratchet close
- IFC public-checker-boundary threshold helper mirror:
  `specs/examples/544`
  - `p10 / p11 / p12` source-side IFC trio
  - `actual_public_checker_boundary_threshold`
  - `boundary_kind + public_checker_command_surface_ref + shared_output_contract_ref` public-checker-boundary minimum mirror
  - Package 72 public-checker-boundary ratchet close、Package 73 verifier-handoff-surface ratchet close
- IFC verifier-handoff-surface threshold helper mirror:
  `specs/examples/545`
  - `p10 / p11 / p12` source-side IFC trio
  - `actual_verifier_handoff_surface_threshold`
  - `handoff_surface_kind + public_checker_boundary_ref + proof_obligation_matrix_ref + handoff_artifact_mode` verifier-handoff-surface minimum mirror
  - Package 73 verifier-handoff-surface ratchet close、Package 74 minimal-parser-subset-freeze ratchet close
- IFC minimal-parser-subset-freeze threshold helper mirror:
  `specs/examples/546`
  - `p10 / p11 / p12` source-side IFC trio
  - `actual_minimal_parser_subset_freeze_threshold`
  - `freeze_kind + accepted_cluster_refs + reject_cluster_refs + retention_floor_refs` parser first-tranche minimum mirror
  - Package 74 minimal-parser-subset-freeze ratchet close、Package 75 parser-to-checker-reconnect-freeze ratchet close
- IFC parser-to-checker-reconnect-freeze threshold helper mirror:
  `specs/examples/547`
  - `p10 / p11 / p12` source-side IFC trio
  - `actual_parser_to_checker_reconnect_freeze_threshold`
  - `reconnect_kind + parser_subset_freeze_ref + checker_floor_refs + retained_helper_refs` reconnect minimum mirror
  - Package 75 parser-to-checker-reconnect-freeze ratchet close、Package 76 phase1-semantics-closeout ratchet close
- IFC phase1-semantics-closeout threshold helper mirror:
  `specs/examples/548`
  - `p10 / p11 / p12` source-side IFC trio
  - `actual_phase1_semantics_closeout_threshold`
  - `closeout_kind + core_semantics_refs + invariant_bridge_refs + notation_status_refs` semantics closeout minimum mirror
  - Package 76 phase1-semantics-closeout ratchet close、Package 77 phase2-parser-free-poc-closeout ratchet close
- IFC phase2-parser-free-poc-closeout threshold helper mirror:
  `specs/examples/549`
  - `p10 / p11 / p12` source-side IFC trio
  - `actual_phase2_parser_free_poc_closeout_threshold`
  - `closeout_kind + compile_gate_refs + helper_boundary_refs + detached_loop_policy_refs` parser-free closeout minimum mirror
  - Package 77 phase2-parser-free-poc-closeout ratchet close、Package 78 phase4-shared-space-self-driven-closeout ratchet close
- shared-space self-driven closeout threshold helper mirror:
  `specs/examples/550`
  - `p07 / p08 / p09` source-side shared-space trio
  - `actual_phase4_shared_space_self_driven_closeout_threshold`
  - `closeout_kind + current_package_refs + user_spec_required_catalog_refs + retained_later_refs` shared-space practical boundary checkpoint mirror
  - Package 78 phase4-shared-space-self-driven-closeout ratchet close、Package 79 phase5-proof-protocol-runtime-policy-handoff-closeout ratchet close
- phase5 proof/protocol/runtime-policy handoff closeout threshold helper mirror:
  `specs/examples/551`
  - `p07 / p08 / p09` source-side shared-space trio
  - `actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold`
  - `closeout_kind + verifier_handoff_surface_ref + theorem_retained_bridge_stop_ref + boundary_inventory_ref + retained_later_refs` handoff stop-line bundle mirror
  - Package 79 phase5-proof-protocol-runtime-policy-handoff-closeout ratchet close、Package 80 phase6-actual-parser-ast-carrier-first-tranche ratchet close
- phase6 parser / AST carrier first tranche threshold helper mirror:
  `specs/examples/552`
  - `p07 / p08 / p09` source-side shared-space trio
  - `actual_phase6_actual_parser_ast_carrier_first_tranche_threshold`
  - `carrier_kind + accepted_surface_refs + code_anchor_refs + retained_later_refs` parser first-tranche minimum mirror
  - Package 80 phase6-actual-parser-ast-carrier-first-tranche ratchet close、Package 81 phase6-actual-checker-runtime-skeleton-first-tranche ratchet next
- phase6 checker/runtime first tranche threshold helper mirror:
  `specs/examples/553`
  - `p07 / p08 / p09` source-side shared-space trio
  - `actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold`
  - `skeleton_kind + semantic_entry_refs + runtime_bridge_refs + parser_bridge_contract_refs + retained_later_refs` checker/runtime first-tranche minimum mirror
  - Package 81 phase6-actual-checker-runtime-skeleton-first-tranche ratchet close、Package 82 phase6-compile-ready-verification-and-formal-hook ratchet close
- phase6 compile-ready verification / formal hook threshold helper mirror:
  `specs/examples/554`
  - `p07 / p08 / p09` source-side shared-space trio
  - `actual_phase6_compile_ready_verification_and_formal_hook_threshold`
  - `verification_gate_refs + smoke_gate_refs + formal_hook_shape + source_artifact_refs + validation_refs + retained_later_refs` compile-ready checkpoint minimum mirror
  - Package 82 phase6-compile-ready-verification-and-formal-hook ratchet close、Package 83 phase6-next-reopen-sequencing ratchet close
- phase6 next-reopen sequencing threshold helper mirror:
  `specs/examples/555`
  - `p07 / p08 / p09` source-side shared-space trio
  - `actual_phase6_next_reopen_sequencing_threshold`
  - `sequencing_kind_ref + fixed_entry_criteria_refs + selected_first_reopen_ref + deferred_reopen_refs + minimum_guard_refs + kept_later_refs` sequencing minimum mirror
  - Package 83 phase6-next-reopen-sequencing ratchet close、Package 84 phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package ratchet next
- phase6 parser second-tranche attached-slot / predicate fragment first-package threshold helper mirror:
  `specs/examples/556`
  - `CurrentL2SecondTrancheManifest`
  - `current_l2_second_tranche_manifest()`
  - `carrier_kind + accepted_surface_refs + code_anchor_refs + retained_later_refs` parser second-tranche minimum mirror
  - Package 84 phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package ratchet close、Package 85 phase6-reserve-formal-tool-binding-inventory ratchet next
- first strong typing layer finite-index spine default:
  `specs/examples/557`
  - `Ψ ; Γ ; Δ ⊢ e : A @ m ! ε ▷ C`
  - finite decidable index fragment / IFC / capture / lifetime / simple cost principal target
  - local inference aggressive / public-module boundary annotation required / declassification・handoff・external effect annotation required
  - finite index soundness / limited completeness / explicit-flow noninterference / selected resource model cost soundness
  - stronger typed surface promotion は still mixed gate
- phase6 reserve formal tool binding inventory threshold helper mirror:
  `specs/examples/558`
  - `CurrentL2Phase6ReserveFormalToolBindingInventoryManifest`
  - `current_l2_phase6_reserve_formal_tool_binding_inventory_manifest()`
  - `inventory_kind + fixed_entry_criteria_refs + first_reserve_ref + second_reserve_ref + minimum_guard_refs + next_comparison_target_ref + kept_later_refs` reserve inventory minimum mirror
  - Package 85 phase6-reserve-formal-tool-binding-inventory ratchet close、Package 86 phase6-parser-side-follow-up-package-sequencing ratchet next
- phase6 parser-side follow-up package sequencing threshold helper mirror:
  `specs/examples/559`
  - `CurrentL2Phase6ParserSideFollowupPackageSequencingManifest`
  - `current_l2_phase6_parser_side_followup_package_sequencing_manifest()`
  - `sequencing_kind + fixed_entry_criteria_refs + selected_next_package_ref + deferred_reopen_refs + minimum_guard_refs + next_comparison_target_ref + kept_later_refs` sequencing minimum mirror
  - Package 86 phase6-parser-side-follow-up-package-sequencing ratchet close、Package 87 phase6-parser-second-tranche-shared-single-attachment-frame-first-package ratchet next
- phase6 parser second-tranche shared single attachment frame first-package threshold helper mirror:
  `specs/examples/560`
  - `CurrentL2SharedSingleAttachmentFrameManifest`
  - `current_l2_shared_single_attachment_frame_manifest()`
  - `carrier_kind + accepted_surface_refs + code_anchor_refs + retained_later_refs` shared single attachment frame minimum mirror
  - Package 87 phase6-parser-second-tranche-shared-single-attachment-frame-first-package ratchet close、Package 88 fixed-subset-source-sample-corpus-scope-and-file-layout ratchet next
- fixed-subset source-sample corpus scope-and-file-layout threshold helper mirror:
  `specs/examples/561`
  - `CurrentL2FixedSubsetSourceSampleCorpusScopeAndFileLayoutManifest`
  - `current_l2_fixed_subset_source_sample_corpus_scope_and_file_layout_manifest()`
  - `scope_kind + source_cluster_refs + directory_ref + file_layout_ref + file_extension_policy + sample_id_policy + non_goal_refs` source-corpus minimum mirror
  - Package 88 fixed-subset-source-sample-corpus-scope-and-file-layout ratchet close、Package 89 phase6 request-clause-suite publicization line を受ける前段 anchor
- phase6 request-clause-suite publicization threshold helper mirror:
  `specs/examples/562`
  - `CurrentL2RequestClauseSuiteManifest`
  - `current_l2_request_clause_suite_manifest()`
  - `Stage3RequestClauseSuite`
  - `parse_stage3_request_clause_suite_text()`
  - Package 89 phase6 request-clause-suite publicization actualization close、Package 90 phase6 perform-head-final-public-parser-API comparison next
- phase6 perform-head structural carrier threshold helper mirror:
  `specs/examples/563`
  - `CurrentL2PerformHeadManifest`
  - `current_l2_perform_head_manifest()`
  - `Stage3PerformTargetRef`
  - `Stage3PerformHead`
  - `parse_stage3_perform_head_text()`
  - Package 90 phase6 perform-head structural carrier actualization close、Package 91 phase6 perform-head-request-clause-bundle-attachment comparison next
- model-check public checker artifact / migration coupled later gate:
  `specs/examples/498`
  - consumer-shaped artifact candidate only
  - actual public checker migration candidate only
  - tool-brand / verifier-handoff adjacent keep
  - `e5 / p06 / p07 / p09` reached / `p05` guard-only
- model-check checker-artifact route actual adoption:
  `specs/examples/501`
  - row-local property route first
  - checker-boundary contract anchor
  - consumer-shaped checker-artifact candidate only
  - migration candidate adjacent keep
  - `e5 / p06 / p07 / p09` reached / `p05` guard-only
- witness/provider route actual adoption:
  `specs/examples/502`
  - witness/provider route first
  - public-schema candidate keep
  - combined public-contract later
  - final emitted-handoff contract adjacent keep
  - `p07 / p08 / p09` reached / `p05` guard-only
- order-handoff source wording route actual adoption:
  `specs/examples/503`
  - principal source wording route first
  - emitted-artifact schema candidate keep
  - final source wording later
  - `p07 / p08` reached / `p05` guard-only
- witness/provider schema route actual adoption:
  `specs/examples/504`
  - witness-schema candidate keep + witness route first
  - provider-receipt candidate keep + provider route first
  - combined public-contract candidate keep
  - final emitted-handoff contract adjacent keep
  - `p07 / p08 / p09` reached / `p05` guard-only
- witness/provider final public-contract reopen threshold:
  `specs/examples/505`
  - public-schema pair first
  - delegated attestation + combined public-contract second
  - final emitted-handoff contract third
  - exhaustive shared-space catalog later
  - `p07 / p08 / p09` reached / `p05` guard-only
  - `p07 / p08 / p09` reached / `p05` guard-only
- witness/provider public-contract / emitted-handoff coupled later gate:
  `specs/examples/493`
  - claim/payload split first
  - witness/provider route non-collapse
  - repo-local emitted artifact refs first
  - combined public contract later
  - final emitted-handoff contract later
  - `p07 / p08 / p09` reached / `p05` guard-only
- witness/provider public-schema coupled later gate:
  `specs/examples/499`
  - witness-schema candidate only
  - provider-receipt candidate only
  - combined public-contract candidate only
  - repo-local emitted artifact refs first
  - `p07 / p08 / p09` reached / `p05` guard-only
- witness/provider/artifact public-shape threshold default:
  `specs/examples/483`
  - claim/payload split first
  - repo-local emitted artifact refs first
  - optional attachment refs only
  - combined public contract later
  - `p07 / p08 / p09` reached / `p05` guard-only
- witness/provider/artifact public-shape actual adoption:
  `specs/examples/489`
  - witness/provider route non-collapse
  - repo-local emitted artifact refs first
  - optional attachment refs only
  - combined public contract later
  - `p07 / p08 / p09` reached / `p05` guard-only
- order-handoff surface / artifact threshold default:
  `specs/examples/484`
  - edge-row vertical continuation principal
  - readable relation vocabulary
  - stage-block secondary candidate
  - repo-local emitted artifact refs first
  - `p07 / p08` reached / `p05` guard-only
- order-handoff surface actual adoption:
  `specs/examples/490`
  - edge-row vertical continuation principal
  - readable relation vocabulary
  - stage-block secondary keep
  - repo-local emitted artifact refs first
  - `p07 / p08` reached / `p05` guard-only

その後の next self-driven queue は representative trace-alignment / reserve surface package に narrowed する。

helper-local compare floor としては、
`specs/examples/463`、`464`、`465` を retained validation hardening に保つ。
これらは public contract adoption ではなく、
actual adoption package の直後に mixed gate を narrow に保つための compare floor である。

したがって、

- stronger typed surface actual adoption
- theorem discharge result / public-contract concretization
- settled property language / concrete tool seam / public checker artifact
- final source-surface handoff wording / final emitted-artifact schema
- shared-space stronger fairness / replay profile
- final public witness schema / final public provider receipt schema / combined public contract
- final adopted calculus / exact compatibility manifest / public proof artifact contract

は引き続き mixed gate に残す。

## 推奨される phase 順序

1. Workstream A
2. Workstream B
3. Workstream C
4. Workstream D
5. Workstream E
6. Workstream F
7. Workstream G

この順序は mainline の大勢であり、cross-cutting theory package を否定しない。
current repo では、Workstream A/E/G に跨る docs-first theory package を、
mainline actualization と separable に ratchet 方式で進めてよい。

## 仮の実装推奨（アーキテクチャ上の法則ではない）

- Core runtime、graph processing、tooling backend には Rust。
- Mir / Mirrorea / Prism component ごとに分離した native crate。
- Engine integration は adapter の背後に置く。
- 暗黙結合よりも、明示的 schema と version づけられた interface を優先する。
