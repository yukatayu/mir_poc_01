---
id: root/changelog
status: L1-fixed
maturity: draft
depends_on: [meta/style-guide]
summary: canon 自体の版履歴と改定手続きの要約。
open_items: []
---

# Changelog

## 改定手続き(要約)

1. 提案(人間または AI)を `meta/proposals/PROPOSAL-###.md` として起票。
2. 人間(プロジェクトオーナー)が採否を決定。
3. 採択なら該当ファイルを改定し、L0/L1 に触れる場合は ADR を追記・改訂し、本ファイルに 1 行記録。
4. `python3 meta/build-index.py` で `INDEX.json` を再生成し、参照整合を検証。

## 履歴

- **2026-08-04** PROPOSAL-023 と ADR-0020 により、M1--M4 の有限
  `SurfaceFragment`/`Core`/`Config`/`Step`/`WellFormed`/trace/diagnostic/
  relation/cut-save carrier を一つの concrete M5 shared model に採用した。
  result frontier/version、relation frontier/epoch、presentation context、cut は
  distinct type のままとし、owner bind → `publish-relation` による exact
  relation/binding carrier の保持 → consumer-local projection と consumer
  materialization/J-mutation reject を有限 evidence にした。
  これは M6 grammar、general theorem、save/load algorithm、patch/runtime/transport、
  public contract を採用・主張しない。
- **2026-08-04** PROPOSAL-022 と ADR-0019 により、M4 の owner-held
  maintained relation と consumer-local late projection を採用した。relation は
  `publish-relation` で投影され、absolute value / adapter stream には具体化しない。
  semantic fallback は同一 lineage 内で単調、fresh reacquire は新 lineage、presentation
  gap は semantic state 非変異である。有限 Lean evidence は exact M4 rows のみを対象とし、
  general fallback / two-layer-time proof、grammar、wire/API、save/load 実装、runtime
  conformance は主張しない。
- **2026-08-04** PROPOSAL-021 と ADR-0018 により、M3 の有限 `EvalPlan` を採用した。
  owner RMW は caller authority と owner evaluation を分離して serial service し、
  other-owner operand は explicit receipt または Diagnostic とする。designated evaluator
  は frontier/versioned value を決定する。これは M6 grammar、wire/API、transaction、
  save/load/patch、I1/conformance/deployment を固定・主張しない。
- **2026-08-04** ADR-0017 / Plan 04 の M2 acceptance record に、revision-bound
  semantic-assertion v3 の fresh `pass` artifact (`LAB:plan/248`、digest
  `b32bd2c87e1dc77ca2a4f7a7426cda0bff8bcbf80155d19addd7db3a8288aa23`) を受理した。
  順に G0-D3、G0 exit、T1 entry を適用した。これは phase-governance のみであり、SCN、
  proof/OBL、runtime、I1、public contract/deployment は動かしていない。
- **2026-08-04** PROPOSAL-019 と ADR-0016 により、`root/design-constitution`
  を Mir Theory v0 / deterministic I1+ の横断判断として採用した。SCN-02 は
  requester authority origin と S-side owner RMW を区別するよう是正し、semantic
  fallback と consumer-local presentation fallback を分離した。pre-M6 Surface
  profile は historical compatibility candidate として保持し、final grammar を
  先取りしない。T0/G0/T1、OBL/proof、conformance、runtime、public contract は
  動かしていない。
- **2026-08-03** PROPOSAL-018 の owner disposition と ADR-0015 により、Mir
  Theory v0 / I1+ Milestones 0--10 に限る evidence-gated bounded autonomy、
  一つの semantic frontier、原則一 milestone 一 report、独立 review、実証済み
  proof-ledger 更新を採用した。ADR-0014 はこの program 外の default research
  route として残り、North Star・保証の弱化・final public contract・production
  deployment は owner-reserved のままである。
- **2026-07-29** PROPOSAL-017 の owner disposition として、V1/R1 の
  cross-locus read に限る `X1 relation-state envelope` を記録した。これは
  ADR-0014 適格な最小 L3 設計・反例 package を開くのみであり、Core、Config、
  SaveObject、failure、theory/11、SCN、Gate、Phase、runtime、source grammar、
  wire/API、public contract は変更しない。
- **2026-07-28** PROPOSAL-004/008/012/013/015/016 に owner disposition を
  記録した。Participant-only Surface v0、outcome totality の別 obligation、
  V1/R1/SW1/conditional A2、M1 request-local validation context、explicit
  scalar terminal fallback / `return` exclusion、narrow T2 と separate I1
  readiness/bootstrap が後続設計 package の方向となった。Core rule、grammar、
  ledger、SCN、Gate、Phase、runtime、public contract はこの記録だけでは変更しない。
- **2026-07-28** PROPOSAL-014 と ADR-0013 amendment により
  `phase-governance/t0-g0` version 2 を採用した。v1 artifact は
  nonconforming historical evidence として byte-preserved し、既存の fixed
  evidence/control predicates を rebase せず one-off fresh v2 artifact だけを
  許可する。G0-D3、G0 exit、T1 entry、I1 authorization は引き続き defer /
  non-effect である。
- **2026-07-24** Added PROPOSAL-013, an owner decision request for the semantic
  provenance of post-admission request validation context. It selects no Core,
  queue, wire, runtime, OBL, Gate, Phase, or public behavior.
- **2026-07-24** `PROPOSAL-009` の owner disposition を記録。将来の
  OBL-001 proof-facing package は THM-001 の既存 every-write Core `c` 条件を
  直接表明できる。この記録は再triage を許すだけで、新規 WRK、Core
  representation / traversal、OBL status、proof、Gate / Phase を選ばない。
- **2026-07-21** PROPOSAL-007 により、ADR-0014 の既存 L3 retained-evidence
  condition を WRK の append-only `Evidence commits:` と reachable-DAG audit
  で精密化した。これは L2 activation、reserved boundary、L0/L1 theory を変更せず、
  既存 LAB lane の証拠帰属を機械検査可能にする運用改定である。
- **2026-07-21** ADR-0014 を PROPOSAL-006 により改訂。owner-maintained exact
  editable-target table を standing bounded autonomy に置換し、agent-maintained
  canon surface を `working/` に限定した。L3 pre-registration、existing-lane
  evidence、L2 の author/reviewer distinct signed frozen-material review、forward rollback を要する。L0/L1、contracts、
  SCN/Gate/Phase、`theory/11`、final proof、public claim は引き続き留保する。
- **2026-07-21 (superseded operating detail)** ADR-0014 により、existing LAB lane における可逆な L2/L3
  working-theory research を委任した。canon update は owner-maintained exact
  editable-target row、rebased frozen evidence/diff、independent review、reviewed
  rollback を要する。L0/L1、external contract、SCN/Gate/Phase、全 `theory/11`、
  implementation/public status は委任していない。初期 editable-target row は空。
- **2026-07-15** ADR-0013 により `phase-governance/t0-g0` version 1 と、
  pinned evidence cut に対する G0-D1 acceptance / G0-D4 waiver を採択した。
  one-off LAB-derived JSON は許可したが、G0-D3 は defer のため G0 exit / T1
  entry / SCN conformance / implementation state は変更していない。
- **2026-07-14** `PROPOSAL-001` の owner disposition を記録。abstract
  OBL-020 Lean statement shape は、full OBL-020 completion を伴わない
  G1-supporting proposal-preparation scope としてのみ受理された。OBL status、
  proof、artifact identity / wrapper、Gate / Phase は変更していない。
- **v0.1.0** (2026-07-02) 初回生成。ADR-0001〜0012 制定。MirCore v0 初稿、Surface 文法 v0、SCN-01〜10 凍結、Gate/Phase 計画制定。全証明は OBL 台帳(未 discharge)。既存 repo mir_poc_01 を LAB に格下げ(手続きは meta/source-hierarchy.md)。
