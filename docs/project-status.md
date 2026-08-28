# Project status

最終更新: 2026-08-28 18:58 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project
direction, theory, ADRs, conformance, and process. Everything outside
`mirrorea_canon/` is LAB: evidence, history, implementation, and operational
notes. If LAB text conflicts with canon, canon wins.

## この文書の役割

これは人間向けの短い **派生ビュー** である。規範判断は
`mirrorea_canon/`、current execution controlはPlan 249、詳細証跡はmilestone
reportsにある。この文書はGate/Phase、OBL、適合性、proof、public compatibilityを
単独で決めない。

## 全体の進行チェックリスト

```text
closed M0--M10 finite reference baseline
→ [x] SYS-0 baseline / goal alignment
→ [x] SYS-1 runtime kernel / internal carrier
→ [x] SYS-2 ST / OW1 concurrency refinement
→ [x] SYS-3 checked Core -> per-locus artifacts and generated plans
→ [x] SYS-4 in-process generated dispatch
→ [x] SYS-5 four-locus toy + typed devtools
→ [x] SYS-6 finite I2 assurance / lifecycle closeout
→ [~] SYS-7 inactive I3 entry contract only (sole active goal)
```

ADR-0026がSYS-0--SYS-7を許可する。これはbounded programの進行表であり、
official lifecycleはCanon plan/01だけが決める。

## 現在地

| 観点 | 状態 | 根拠 |
|---|---|---|
| theory | **T1** | `mirrorea_canon/plan/01-phases.md` |
| broad PHASE-I1 | **unaccepted**; OPEN-026/027とfull carrier freezeが残る | `mirrorea_canon/architecture/04-runtime-carriers.md` |
| bounded I2 lifecycle | **official entry accepted, then official exit accepted** | `mirrorea_canon/adr/ADR-0032.md` |
| active milestone | **SYS-7 only**; inactive I3 goal/entry contract | `plan/249-mirrorea-i2-systems-foundation-current-roadmap.md` |
| I3 | **inactive**; transport未選定・未実装 | `mirrorea_canon/adr/ADR-0032.md`, OPEN-032 |
| public/product | final grammar/CLI/API/ABI/wireもproductionも未受理 | `mirrorea_canon/spec/15-sys6-i2-conformance.md` |

SYS-6のexact implementation/evidence cutは
`5429712de89a7e41c46cfd7fb4a39c4a492864c4`。provisional command
`mir conform-i2`はcanonical inputsでobserver-safe deterministic reportを生成し、
exact 22 rowsをすべてpassする。21 rowsは`runtime-monitored`、authority no-mint
rowはOBL-058を用いる`model-checked-bounded`、aggregate OBL-063は
`runtime-monitored`である。

primary four-locus whole workflowはSTである。OW1 correspondenceはexactly one
combined owner/source-owner worker locusを持つ別ordinary sourceに限る。four-locus
OW1のtyped `BackendIneligible` residualは非mutationのまま保持する。

## 現在の停止線

SYS-7が行えるのは次だけである。

- accepted I2 artifactsを2つ以上のOS processへ写すfuture goalをinactiveで記録する;
- candidate transportを最大二つ記録するが選定しない;
- transportをauthorityにせず、internal carrierとfuture public wireを分離する;
- disconnect/reconnect、duplicate/reorder、wrong target、stale membership、
  revocation、ordering refinement、C-distributed scenario要件を記録する; そして
- real transport実装、public freeze、I3 activation、productionを開始しない。

`conform-i2`のlifecycle bitsがfalseであることは現在のCanon stateと矛盾しない。
runtime producer/verifierはself-authorizeできず、ADR-0032だけがevidenceを評価して
official I2 entry/exitを適用した。

Reopen SYS-6はmissing/extra edge、owner movement、manual interface、direct remote
store、source-free mint、unbound evidence、wrong diagnostic、selected ST/OW
divergence、stale cut/patch mutation、observer leak、lower-layerからconformanceへの
逆依存、M10 regression、またはI2 criteriaへのcounterexampleが出た場合だけ。

この停止線のauthorityは`mirrorea_canon/adr/ADR-0032.md`と
`plan/249-mirrorea-i2-systems-foundation-current-roadmap.md`である。

## オーナーの確認・判断待ち

SYS-7完了に追加owner decisionは不要である。次はfuture owner-reservedであり、
current blockerではない。

- OPEN-032のreal transport選定とI3 program activation;
- public API/ABI/wire/grammar/CLI compatibility freeze;
- production deployment、external publication、paid resource;
- North Star、authority/privacy/redaction/no-stale guaranteeの変更;
- World/Avatar等のCore primitive化、hidden multi-owner transaction; および
- Constitutionでも解けないirreversible semantic tie。

Owner-reserved条件の正確なprogram boundaryは
`mirrorea_canon/adr/ADR-0026.md`を参照する。

## 根拠と詳細

| 読みたい内容 | 一次の確認先 |
|---|---|
| lifecycle / OPEN-032 | `mirrorea_canon/plan/01-phases.md` |
| SYS-6 decision | `mirrorea_canon/meta/proposals/PROPOSAL-035-sys6-i2-conformance.md`, `mirrorea_canon/adr/ADR-0032.md` |
| fixed conformance profile | `mirrorea_canon/spec/15-sys6-i2-conformance.md` |
| proof/evidence class | `mirrorea_canon/theory/11-metatheory-ledger.md` |
| sole active roadmap | `plan/249-mirrorea-i2-systems-foundation-current-roadmap.md` |
| exact close evidence | `docs/reports/2598-mirrorea-i2-systems-foundation-sys6-i2-conformance-closeout.md` |
| runnable commands | `samples_progress.md`, `samples/clean-near-end/mirrorea-i2-conformance/README.md` |

Validation floor: SYS-6 25+8、SYS-2/3/4/5 28/28/104/62、M10 67+4、full
workspace、format、warnings-denied Clippy、diff、およびfinal independent ACCEPT。
M10 cut `23f5a813...`はimmutable regression baselineで、I2 identityに流用しない。

## 更新規約

active milestone、official lifecycle、major blocker、accepted cut、evidence class、
またはuser-visible commandが変わるtaskで同期する。変更のauthorityは常にCanonへ
戻し、詳細履歴はone milestone reportへ置く。実行していないvalidationをpassと書かず、
helper/reportをgeneral proofやpublic product completionとして数えない。
