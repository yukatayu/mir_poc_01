# Project status

最終更新: 2026-09-01 21:08 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project
direction, theory, ADRs, conformance, and process. Everything outside
`mirrorea_canon/` is LAB: evidence, history, implementation, and operational
notes. If LAB text conflicts with canon, canon wins.

## この文書の役割

これは人間向けの短い **派生ビュー** である。規範判断は
`mirrorea_canon/`、closed execution historyはPlan 247 / 249、詳細証跡はmilestone
reportsにある。この文書はGate/Phase、OBL、proof、compatibility、goalを決めない。

## 全体の進行チェックリスト

```text
closed M0--M10 finite reference baseline
-> [x] SYS-0 baseline / goal alignment
-> [x] SYS-1 runtime kernel / internal carrier
-> [x] SYS-2 ST / OW1 concurrency refinement
-> [x] SYS-3 checked Core -> per-locus artifacts and generated plans
-> [x] SYS-4 in-process generated dispatch
-> [x] SYS-5 four-locus toy + typed devtools
-> [x] SYS-6 finite I2 assurance / lifecycle closeout
-> [x] SYS-7 inactive I3 entry contract only / program closed
-> [ ] ALIGN-0 bounded-program activation / meta-drift alignment (active/closing)
-> [ ] ALIGN-1 project/product layer constitution (next, not active)
```

Plan 247とPlan 249はclosed recordsである。PROPOSAL-037 / ADR-0034により
Mirrorea I3 Distributed Foundation bounded programがactiveで、Plan 250がsole
current roadmap、ALIGN-0がactive/closing milestoneである。

## 現在地

| 観点 | 状態 | 根拠 |
|---|---|---|
| theory | **T1** | `mirrorea_canon/plan/01-phases.md` |
| broad PHASE-I1 | **unaccepted**; OPEN-026/027とfull carrier freezeが残る | `mirrorea_canon/architecture/04-runtime-carriers.md` |
| bounded I2 lifecycle | **official entry accepted, then official exit accepted** | `mirrorea_canon/adr/ADR-0032.md` |
| ADR-0026 program | **SYS-0--SYS-7 closed** | `mirrorea_canon/adr/ADR-0033.md` |
| active roadmap / goal | **Plan 250 / ALIGN-0 active/closing** | `plan/250-mirrorea-i3-distributed-foundation-current-roadmap.md` |
| I3 / OPEN-032 | **bounded program active; lifecycle entry not official / unresolved** | `mirrorea_canon/adr/ADR-0034.md` |
| public/product | final grammar/CLI/API/ABI/wireもproductionも未受理 | `mirrorea_canon/adr/ADR-0033.md` |

Accepted SYS-6 implementation/evidence cutは
`5429712de89a7e41c46cfd7fb4a39c4a492864c4`、Canon/status integration cutは
`bcb0f767edbb3e9e581c3b4c7f2a49e077f44067`である。provisional
`mir conform-i2`はexact 22 finite rowsを検査するがlifecycleをself-authorizeしない。

SYS-7は候補A TLS-over-TCP framed reliable-stream adapterと候補B QUIC reliable-
stream adapterをともに**UNSELECTED**で保持した。QUIC datagramは未admit・未評価。
version、codec、wire、library、certificate representation、port、retry、deploymentも
未選定である。transport/session/certificate/route metadataはauthorityではない。

## 現在の停止線

ALIGN-0はdocs/status alignment and activation boundaryを閉じつつある。次はALIGN-1
であり、まだactiveではない。固定順序はALIGN-0..2 → I3-0..6 → NEXT-0である。
ALIGN-0 acceptanceはI3 lifecycle entry、transport選定、production/public freezeを
含まない。これらは各後段gate又はowner-reserved boundaryへ残る。
Current authority and milestone gates are
`mirrorea_canon/adr/ADR-0034.md` and
`plan/250-mirrorea-i3-distributed-foundation-current-roadmap.md`.

The active bounded I3 programはinternal carrierとpublic wireを分離し、route/handshake/framing/
disconnect/reconnect/ambiguous delivery/duplicate/reorder/stale authority/backpressure/
timeout/provider/redaction/patch/cut failureをtypedに扱い、network occurrencesをMir
orderingへrefineしなければならない。hidden retry、exactly-once、hidden transactionは不可。

I3-4/I3-6のC-distributed evidenceはordinary-source SCN-01/02/03/06のpositive/falsifier、
source/Core/artifact/carrier/network/runtime correspondence、observer-safe diagnostics、
evidence classification、independent reviewを必要とする。I2 evidenceだけでは満たさない。

Reopen accepted I2 evidenceはmissing/manual edge、owner movement、direct remote store、
source-free mint、selected ST/OW divergence、stale cut/patch mutation、relation/designated
drift、observer leak、lower-layer conformance dependency、M10 regressionの場合だけ。

## オーナーの確認・判断待ち

OPEN-032はI3-0の同条件実行証拠とADRにより、本指令の委任範囲で自律選択する。
次のbounded sequence外の変更だけがowner decisionを必要とする。

- public API/ABI/wire/grammar/CLI compatibility freeze;
- production deployment、external publication、paid resource;
- North Star、authority/privacy/redaction/no-stale guaranteeの変更;
- World/Avatar等のCore primitive化、hidden multi-owner transaction; および
- Constitutionでも解けないirreversible semantic tie。

Authority boundaryは`mirrorea_canon/meta/agent-instructions.md`と
`mirrorea_canon/adr/ADR-0034.md`を参照する。これらは未完了SYS-7 taskではない。

## 根拠と詳細

| 読みたい内容 | 一次の確認先 |
|---|---|
| Canon entry | `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md` |
| lifecycle | `mirrorea_canon/plan/01-phases.md` |
| SYS-6 acceptance | `mirrorea_canon/adr/ADR-0032.md`, `mirrorea_canon/spec/15-sys6-i2-conformance.md` |
| inactive I3 contract | `mirrorea_canon/adr/ADR-0033.md`, `mirrorea_canon/plan/05-i3-entry-contract.md` |
| active bounded I3 program | `mirrorea_canon/adr/ADR-0034.md`, `plan/250-mirrorea-i3-distributed-foundation-current-roadmap.md` |
| proof/evidence class | `mirrorea_canon/theory/11-metatheory-ledger.md` |
| closed I2 roadmap | `plan/249-mirrorea-i2-systems-foundation-current-roadmap.md` |
| SYS-6 close evidence | `docs/reports/2598-mirrorea-i2-systems-foundation-sys6-i2-conformance-closeout.md` |
| SYS-7 close evidence | `docs/reports/2599-mirrorea-i2-systems-foundation-sys7-i3-entry-contract-closeout.md` |
| runnable commands | `samples_progress.md` |

Inherited validation floor: SYS-6 25+8、SYS-2/3/4/5 28/28/104/62、M10 67+4、
workspace、format、warnings-denied Clippy、diff、final independent ACCEPT。SYS-7は
docs/Canon/hierarchy/HTML/diffだけをfresh validationし、runtime evidenceを再分類しない。

## 更新規約

active program/roadmap、official lifecycle、major blocker、accepted cut、evidence class、
またはuser-visible commandが変わるtaskで同期する。authorityは常にCanonへ戻し、
詳細履歴はone milestone reportへ置く。未実行validationをpassと書かず、helper/reportを
general proofやpublic product completionとして数えない。
