---
id: plan/01-phases
status: L1-fixed
maturity: draft
depends_on: [plan/00-gates, plan/02-operating-model, plan/05-i3-entry-contract, spec/06-conformance, meta/source-hierarchy, adr/ADR-0013, adr/ADR-0014, adr/ADR-0015, adr/ADR-0017, adr/ADR-0026, adr/ADR-0027, adr/ADR-0028, adr/ADR-0029, adr/ADR-0030, adr/ADR-0031, adr/ADR-0032, adr/ADR-0033, adr/ADR-0034, plan/04-t0-g0-semantic-assertion-profile]
summary: 実装フェーズT0-T2/I1-I6。theory T1、broad I1 residual、official I2 exit、active ADR-0034 programとI3 lifecycle未entryを分離する。
open_items: [OPEN-032]
---

# 01 — フェーズ計画

**唯一の phase 状態の正本。** 他のどのファイルの存在も phase entry/exit を意味しない。
現在位置は二軸で読む。

- theory lifecycle: **T1**（M2 semantic-assertion profile v3 の pass digest、G0-D3、
  G0 exit、T1 entry を ADR-0017 により受理済み）;
- implementation evidence: **M10 accepted; official I2 entry then exit accepted;
  SYS-0--SYS-7 / ADR-0026 program closed; ADR-0034 I3 bounded program active at
  ALIGN-0 with Plan 250**。architecture/04 の full L2 carrier freeze と OPEN-026/027 が
  残るため broad PHASE-I1 exitは未受理。Official I3 lifecycleは未entry、両候補は
  UNSELECTED、OPEN-032はunresolvedである。

## 理論フェーズ

Theory lifecycle T1 自体は production implementation を受理しない。通常routeの例外は
使い捨て spike と ADR-0014 に従う scoped research artifact / bounded validationだけで
ある。ただしADR-0026は、accepted semanticsをper-locus生成・in-process dispatchへ進める
SYS-0--SYS-7に限り、separateなbounded implementation programを明示的に許可する。
program実装 evidenceは単独ではofficial Theory/I1/I2 lifecycle acceptanceの代用では
ない。ADR-0032はfresh SYS-6 evidenceとindependent reviewを明示的に評価したauthorized
acceptance recordとして、bounded official I2 entry後exitだけを適用した。

| Phase | ゴール(exit) | 動くもの / 実用性 |
|---|---|---|
| T0 語彙と決定 | G0 exit。canon 発効、LAB 格下げ、旧語彙注記 | 何も動かない。判断の基準器が立つ |
| T1 計算体系 | G1 exit + G2/G3 の statement 群。SCN 期待の最終化 | 紙と Lean statement。以後の全実装の仕様が確定 |
| T2 骨格証明 | OBL-020/021/002 の証明骨格、G5 statement 群 | Lean 上で核が回る。理論の破綻はここまでに露見する |

## 実装フェーズ

| Phase | ゴール(exit criteria) | 動くもの | 実用性 | 非宣言 |
|---|---|---|---|---|
| I1 参照実装 | mir-parse/check/elab/run が C-static+C-runtime 10/10、carrier 凍結(arch/04) | 単一プロセスで全 SCN | 教育・検証用。言語に触れる | 性能・分散・永続 |
| I2 多 locus | プロセス内 multi-place、生成通信の実 dispatch、devtools 最小 panel | ローカル toy world | 一人で遊べる箱庭 | 実網・耐障害 |
| I3 実 transport | 2 OS プロセス+実 socket、C-distributed(SCN-01/02/03/06) | LAN で双六が二人で遊べる | 最初の「本物が動く」点 | WAN・セキュリティ強度 |
| I4 永続と patch | save/load(local durable)、ライブ patch(SCN-09 を実セッションで) | 落として上げ直せる world | 継続世界の試作 | 分散 durable(R3/R4) |
| I5 射影と View | ブラウザ client への projection、View FFI(pose 契約)、viewer devtools | 人に見せられる仮想空間デモ | デモ可能な α | 最終 ABI・複数エンジン |
| I6 分散永続と連合 | R3/R4、複数サーバ、federation 入口、限定公開 | 招待制の常設小世界 | 限定公開 α | 一般公開・スケール保証 |

## I1+ deterministic reference-profile acceptance

ADR-0025 は source/validation cut
`23f5a8130334bf0c8516d51e9dcea38b92f50db1`、tree
`d8a296fac7a94a37da92563d5feeeeaa96dbc682` の有限 M10 profileを受理した。
fresh remote clone は同一 output SHA-256
`083523518fdae0a111522f49b148c818ca0d5c21b4b7cc4f34dd476f10d172e7`
を二回再現し、26 static + 47 runtime rows、zero mismatch/missing、anchor match、
waiverなしで `ConformanceAccepted` となった。

これは owner-approved M0--M10 program の実行可能 reference-profile close である。
architecture/04 の L2 carrier freeze / OPEN-030 を解決せずに、spec/06 の広い
PHASE-I1 exit、public grammar/API/ABI/wire、C-distributed、I2 activationを主張しない。
I2 entry は ADR-0025 の entry contractとnew owner-directed current roadmapを必要とした。
その後の充足とacceptanceは下のADR-0032 recordが決める。

## Mirrorea I2 Systems Foundation program activation

PROPOSAL-029 / ADR-0026 と `LAB:plan/249` は、ADR-0025の要求したnew owner direction、
OPEN-030のinternal bounded resolution lane、generated per-locus artifact/dispatch、minimal
typed devtools acceptance path、public non-freezeを一つのbounded programへ束縛した。
SYS-0はbaseline/goal alignmentをcloseした。PROPOSAL-030 / ADR-0027 はSYS-1の
crate-private semantic kernelとowner/designated remote-input lifecycleをsource cut
`94e3707c7bc98d4a0764c51f13a12b1dae1968c6`でcloseした。OPEN-030はこのI2-internal
bounded contractに限りresolvedである。PROPOSAL-031 / ADR-0028 はSYS-2のdeterministic
ST、single-owner OW1、M9 successor generation visibility、finite ordering refinementを
source cut `920d3fe050b8b909253f8511d9ad897272323ced`でcloseした。OBL-058は
`model-checked-bounded`、OBL-059は`runtime-monitored`であり、Lean/general theoremでは
ない。PROPOSAL-032 / ADR-0029 はSYS-3のpure checked-Core projector、owned
per-locus fragments、generated communication/effect/observation/persistence plans、
source/Core/artifact correspondenceを選択した。candidate cut
`ded622fef91bab2cadc571ba944e5ee2c69a7b63`はE-CONSUME evaluator→named-consumer
path欠落によりpartial regression evidenceへ戻して一度reopenし、bounded non-final
`designated consume E.result at C` AST/M6/M7 Core edge、exactly-one source-named
consumer、consumer artifact/deliveryをsource/evidence cut
`3013e7fe075a7605a1ffe01e0b14f4a0856eaeb9`で受理した。そのretry fieldはstatic SYS-4
refinement requirementだけを記録し、legacy M8/M10を
idempotent-return evidenceへ再解釈しない。actual carrier-side return/wrapperとendpoint
testsはSYS-4 direct-consumer obligationである。production relation graphはcurrent checked
two-anchor shapeだけを扱い、finite DAG extensionはsource-bound test-only pressure
nonclaimである。OBL-060はstatic finite compiler/projector evidenceだけを
`runtime-monitored`とし、Lean/general theorem、runtime admission/dispatch、public
contractを主張しない。PROPOSAL-033 / ADR-0030 はSYS-4のgenerated-plan-only
independent locus endpoints、actual staged dispatch、ST multi-owner/eligible OW1 selected
correspondence、source/Core/artifact/edge/runtime occurrence attribution、one-M8-consume
same-consumer idempotent return、typed fault/quarantine、observer-safe failure boundaryを
source/evidence cut `22196f93b0112b8fd2987ec078021c8865b71651`で受理した。さらにST
whole-fabric local cut/restoreとquiescent designated-only checked patchを有限受理したが、
OW1 cut/patchは`BackendIneligible`、public CLI/API/ABI/wire/JSON、real transport、durable
persistence、general theoremは非主張である。OBL-061はこのfinite runtime correspondence
だけを`runtime-monitored`とし、Lean/model/general theorem statusを追加しない。ADR-0030
acceptance時点ではSYS-4がcompleted、SYS-5がnextだった。

PROPOSAL-034 / ADR-0031 はprovisional explicit relation-anchor locusをAST/M6/M7/
Core/projectionに保存し、四locus local toyでowner RMW、designated publish/consume、
A-primary/B-fallback relation、ViewerC local projectionをactual generated endpoint上で実行する
SYS-5をsource/evidence cut `53a21e64b5a17e24b522f720db10b6e539c058e0`で受理した。
source-bound ParticipantA leaveはexact M9 membership/capability/witness lineageを退役した後に
ParticipantB ownerがfallbackを公開し、fresh reacquireはそのexact tombstoneにjoinした
distinct epoch/incarnationとfresh lineageを発行する。selected ST lifecycleは
failure-atomic candidate、post-leave cut/restoreはexact retired lineageを保存する。
observer-safe joined reportはsource/Core/artifact/edge/occurrenceからstate/relation/designated/
save/patch/failureまでを一つの因果線で示す。OBL-062はこのfinite
executable/devtools correspondenceだけを`runtime-monitored`とする。public CLI/API/ABI/
wire/JSON、browser/View、real transport、durable persistence、general theoremは非主張である。
SYS-5はcompletedし、SYS-6は後述のfinite I2 conformanceでcompletedした。ADR-0032
acceptance時点ではSYS-7だけがactiveだったが、後述のADR-0033でcompletedした。

Program activation自体はofficial phase entry/exit recordではなかった。Broad
PHASE-I1 exitはarchitecture/04 carrierのfull internal freeze criteriaとOPEN-026/027を
引き続き満たさない。I2はprocess内multi-locus generated dispatchとminimal panelの
actual criteriaを満たし、fresh SYS-6 evidence・independent review・ADR-0032の明示
acceptance recordが揃ったため、I2 entry後exitを受理した。public API/ABI/wire freezeは
どちらの条件にも暗黙追加していない。

## Mirrorea I2 lifecycle acceptance

PROPOSAL-035 / ADR-0032 / spec/15はaccepted implementation/evidence cut
`5429712de89a7e41c46cfd7fb4a39c4a492864c4`のsource-first finite I2 profileを受理した。
producerはactual SYS-2 bounded modelとSYS-3--SYS-5 checked projection、generated
dispatch、local workflowを実行し、verifierはtyped evidence inventoriesだけを読む。
exact 22 rowsはordinary source authority、checked Core/artifacts、generated communication、
actual endpoint dispatch、selected ST/OW1 correspondence、owner data-race freedom、no hidden/
direct remote store/source-free mint、typed failure、relation/fallback/designated、save/patch、
observer-safe devtools、projection determinism、lifecycle non-overclaimを検査する。

各rowはexecuted positive/falsifier evidenceとproperty-specific actual provenanceを必要とし、
missing/unexecuted evidence、wrong diagnostic、missing anchor、row omissionはfail-closedである。
21 rowsは`runtime-monitored`、no-source-free-authority rowは既存OBL-058を参照する
`model-checked-bounded`。aggregate OBL-063は`runtime-monitored`であり、Lean/general
theoremではない。SYS-6 25/25 + CLI 8/8、SYS-2 28/28、SYS-3 28/28、SYS-4 104/104、
SYS-5 62/62、M10 67/67 + CLI 4/4、workspace、format、warnings-denied Clippy、diff、
independent ACCEPTが受理証拠である。

ADR-0025のI2 entry contractはADR-0026のowner-directed roadmap、ADR-0027のnarrow
internal carrier、SYS-3--SYS-5のactual systems capability、SYS-6のfresh assuranceにより
満たされた。よってADR-0032は次をこの順で適用した。

```text
official I2 entry accepted
  -> official I2 exit accepted
```

`conform-i2` outputのlifecycle bitsがfalseで`I2 lifecycle exit`をnon-claimとすることは
矛盾しない。runtime/verifierはphaseをself-activateできず、official stateはこのplanと
ADR-0032だけが決める。Theory T1、broad PHASE-I1、public contract、I3 activationは
動かない。SYS-6はcompletedし、SYS-7は下のinactive entry contract closeでcompleted
した。ADR-0033 close時点ではI3 program/lifecycleともinactiveだった。

## Mirrorea I3 active bounded program / lifecycle not entered

PROPOSAL-036 / ADR-0033はplan/05のinactive I3 entry contractだけを受理した。二つの
reliable-stream候補はともに**UNSELECTED**で、transport/session/certificateはauthority
でない。failure/order、C-distributed gate、public-wire等の正本は
`plan/05-i3-entry-contract`である。

PROPOSAL-037 / ADR-0034はこのcontractをconsumeし、Plan 250をsole roadmap、ALIGN-0を
active goalとする。これはofficial I3 entry/exitではない。official I2 exitを維持し、
I3 lifecycleは未entry、OPEN-032はunresolvedである。selectionはI3-0の両候補同条件
2-process canaryとseparate ADRを必要とする。

## T0/G0 phase-governance profile

`phase-governance/t0-g0` version 3 is the **only current** T0 interpretation.
Its exact semantic contract, deterministic producer, artifact grammar, and
acceptance boundary are `plan/04-t0-g0-semantic-assertion-profile` under
ADR-0017. It is phase governance, not an implementation, an
`arch/03-toolchain` `mir-conform` output, or SCN conformance under
`spec/06-conformance`.

### Historical artifacts are preserved

| Version | Historical location and meaning | Current use |
| --- | --- | --- |
| v1 | `LAB:plan/155`, SHA-256 `0ad49fa84cd766165c5f28bee4dda9a8794f674873e072bf1919eba9027ca943`; nonconforming historical attempt | never regenerate, rename, or interpret as v3 |
| v2 | `LAB:plan/198`; valid historical `fail` from fixed control drift | never re-pin, re-evaluate, or accept as v3 |

The accepted G0-D1 evidence cut, v2 adoption history (G0-D2), and G0-D4 waiver
remain historical facts governed by ADR-0013. M2 does not reaccept or rebase
them. The v3 change is limited to replacing mutable whole-file control pins
with the six revision-bound semantic assertions.

### Version-3 acceptance boundary

The first fresh artifact at `LAB:plan/248` was regenerated from source revision
`644ec1cdfa7d69600af3463ab60a6b7d745913c8`, has root `pass` and canonical
digest `b32bd2c87e1dc77ca2a4f7a7426cda0bff8bcbf80155d19addd7db3a8288aa23`, and
passed its negative controls and exact reproduction validation. The acceptance
amendment in `plan/04` and ADR-0017 applied, in order:

```text
v3 pass digest acceptance → G0-D3 acceptance → G0 exit → T1 entry
```

The resulting `T1` entry does not authorize I1 and does not claim
SCN/C-static/C-runtime/C-distributed conformance, proof/OBL completion, runtime
implementation, or a public contract.

## Universal phase-exit rule

Each phase exits only through its canonically defined profile result and the
authorized acceptance record. T0 uses v3 above. SCN C-static/C-runtime/
C-distributed JSON follows `spec/06-conformance` and the `mir-conform`
contract. **Phase を跨ぐ最適化の先取りは禁止**（BND-006 の意味保存を先に）。
OPEN-032: I3のtransport選定は未決である。I2 exit条件ではなく、recorded plan/05
entry contractをconsumeするcurrent ADR-0034 programのI3-0 delegated decisionで、
候補A/Bを同じexecutable two-process failure/order gatesにより比較して初めてADR対象に
できる。ADR-0033自体はtransportを選ばず、official I3 lifecycleをactivateしない。
