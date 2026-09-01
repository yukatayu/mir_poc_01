---
id: arch/02-boundary-contracts
status: L1-fixed
maturity: draft
depends_on: [arch/01-strata, theory/03-elaboration]
summary: 層間契約 BND-001..016。各境界の不変条件と、越えてよい情報・越えてはならない情報。
open_items: []
---

# 02 — 層間契約 BND

各契約は「保証すること / 越えてはならないこと」。違反は設計バグであり、plan/03 のリスク台帳に載る。

- **BND-001 Surface→Core(elaboration)**: theory/03 の 6 条項(隠れ辺なし・span 保存・row 包含・権限義務・決定性・権限非創出)。越えてはならない: 生成物からの逆流を正本化すること(ADR-0009)。
- **BND-002 Core→Checker**: checker は unified judgment の decidable 断片のみを主張し、残余は ResidualObligation として明示。越えてはならない: 未 discharge 義務の hidden success。
- **BND-003 Checker→Prover/ModelChecker**: 義務は carrier(ModelObligation / ProofObligation)で外出しでき、結果は台帳(theory/11)へ還流。越えてはならない: model-check 結果を一般証明と言い換えること。
- **BND-004 Core→Runtime**: runtime は verdict 済み Core IR のみ実行。fail-closed。越えてはならない: eval(ADR-0006)、検査回避の実行路。
- **BND-005 Runtime→Transport**: transport は Envelope の配送のみを担う。認証・認可・membership・capability・witness を transport metadata に潰さない(ADR-0005/0011)。
- **BND-006 Runtime→Projection**: 射影は意味保存が先、最適化は後。保存必須: 所有、read/write 依存、effect/failure row、capability/witness 要求、可視性・redaction・retention、fallback 系譜、cut/save-load 義務、provider 非所有、span。通信境界は検査後の導出物。
- **BND-007 Runtime/Projection→View**: View/providerはauthoritative domain semantics、mutation authority、persistent stateの正本を所有しない。observer-safe projectionを受け、座標変換、補間、IK、camera、culling、frame-local relation評価等のpresentation-local computationを行ってよい。redaction弱化、意味の再決定、direct storeは禁止。入力はBND-013でのみ戻す。
- **BND-008 Runtime→Devtools**: 観測は typed effect(theory/07)。redaction 単調・retention 明示・行は必ず H 由来。越えてはならない: helper-local な生ログ漏れ。
- **BND-009 Canon→LAB**: 規範は canon のみ。LAB は evidence(`LAB:` 引用)。越えてはならない: LAB 文書の暗黙規範化。
- **BND-010 Package→Browser Admission**: package/content/provenance、checked source/Core、requested capability/effect/resourceからT0 admission verdictを作る。admission、signature、origin、package identityはgrant/activationでない。
- **BND-011 Browser Runtime→Mirrorea Fabric**: admitted package instanceからlocus allocation、storage namespace、membership/capabilityを別々に要求し、fabric authority ownerが再検証する。process/session/reconnect/deploymentはauthorityでない。
- **BND-012 View→Renderer/Engine**: redacted presentation representationだけを渡し、renderer-local compute/cacheを許す。pose、frame、renderer cache、deviceはsemantic state又はauthorityでない。
- **BND-013 Input→Mir Command**: device/input occurrenceはprincipal/capability/handler付きtyped commandへ変換し、Mir ownerが再検証する。raw eventからのdirect storeは禁止。
- **BND-014 Typed Effect→Provider**: declared effect/failure、policy/capability/resource、provenanceをtrusted adapterで検証し、typed result/failureをMirへ戻す。providerはgrantをmintせず、stateを直接変更せず、retry/failureを隠さない。
- **BND-015 Privileged Native Plugin / Raw FFI**: T1 packageにはraw FFIを与えない。T3は別のprivileged admission、least privilege、provenance、data/resource access、crash boundary、revocationを持ち、process isolationを優先する。
- **BND-016 Resource / Sandbox Envelope**: CPU/time、memory、storage namespace/quota、effect/network rate、device/data access、loop/allocation/observation abuse、terminationをenforceできる場合だけactivateする。enforcement不能はfail-closed。

BND-007とBND-010--016の完全なinput/verdict、validation owner、authority、typed
failure、revocation/termination、observation/redaction、non-freeze契約は
`arch/07-browser-host-trust-boundaries`を正本とする。
