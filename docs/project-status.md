# Project status

最終更新: 2026-07-17 19:59 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project
direction, theory, ADRs, conformance, and process. Everything outside
`mirrorea_canon/` is LAB: evidence, history, implementation, and operational
notes. If LAB text conflicts with canon, canon wins.

## この文書の役割

これは人間が短時間で全体像を読むための **LAB派生ビュー** です。規範判断、
Gate / Phase 移行、OBL 状態、適合性、実装完了を作りません。

- 規範正本: `mirrorea_canon/`
- 詳細な計画・研究の記憶: `plan/`
- 実行証拠: `docs/reports/`
- runnable LAB の一覧: `samples_progress.md`

## 全体の進行チェックリスト

Gate: [ ] G0 軸と語彙 -> [ ] G1 普通の代入 -> [ ] G2 存在と fallback ->
[ ] G3 権限 -> [ ] G4 効果と観測 -> [ ] G5 cut と保存 -> [ ] G6 射影 ->
[ ] G7 hot-plug

Phase: [ ] T0 語彙と決定 -> [ ] T1 計算体系 -> [ ] T2 骨格証明 ->
[ ] I1 参照実装 -> [ ] I2 multi-locus -> [ ] I3 実 transport ->
[ ] I4 永続と patch -> [ ] I5 射影と View -> [ ] I6 分散永続と連合

チェックは引用可能な canon record が exit を成立させたときだけ埋めます。
基準は `mirrorea_canon/plan/00-gates.md` と
`mirrorea_canon/plan/01-phases.md` を読むものとし、ここでは再定義しません。

## 現在地

| 面 | 現在の読み | 根拠 |
| --- | --- | --- |
| Canon lifecycle | `T0/G0 rebaseline`。G0 exit と T1 entry は未成立。 | `mirrorea_canon/plan/01-phases.md` |
| T0-T2 research | owner 指示により、既存根拠へ接続した非休眠の LAB research work unit を `research-complete` / `decision-ready` まで自走できる。これは canon package close ではない。 | `plan/156-t0-t2-research-autonomy-envelope.md` |
| Runnable LAB | alpha / Surface / operational sample は再現可能な限定証拠。ただし canon 実装状態・適合性・proof ではない。 | `samples_progress.md` |
| いまの研究 | OBL-001/020/021 の反例監査、三つの限定 OBL-020 kernel、13 transition x 5 WF clause の source-adequacy 監査、OBL-021 の三 postcondition source audit、OBL-005 の構造的 flattening kernel、OBL-006 relation boundary、THM-002/OBL-007 trace-formalization boundary、THM-004/OBL-015 mutation-origin boundary、THM-005/OBL-017 observer-safe export boundary、OBL-018 explicit-flow kernel、THM-003/OBL-009 successful-load restoration boundary、OBL-014 Z-cycle equivalence boundary、remaining-ledger research closure map、OBL-024 diagnostic soundness boundary、OBL-025 diagnostic completeness boundary、OBL-026 transparent-overlay composition boundary、OBL-028 revocation-monotonicity boundary、OBL-022 stream read-side boundary、OBL-027 atomic-cut rollback boundary、OBL-023 temporal-coherence boundary、OBL-010 consistent-cut checker kernel、OBL-004 no-undeclared-communication kernel、OBL-003 Line-1 decidability kernel、T0-T2 formalization decision map を `research-complete` とした。これらは正本の証明・定義・status ではない。 | `plan/156-t0-t2-research-autonomy-envelope.md` |
| OBL-020 next decision | formalization organization の A/B/C proposal を起票済み。A は共通の五見出し review checklist、B は必須共通 checklist を置かない package ごとの組織化、C は defer。採択前は canon の concrete transition/WF premise を定義しない。 | `mirrorea_canon/meta/proposals/PROPOSAL-003-obl020-formalization-boundary-review.md` |
| OBL-021 source audit | BND-001 は三つの determinism 結論の目標を明示するが、abstract `Pred` への完全な導出は `0 direct / 0 delegated / 3 missing`。projection coherence、diagnostic equivalence、branch exclusion は将来の proof package が明示する境界。 | `plan/156-t0-t2-research-autonomy-envelope.md` |
| OBL-005 structural audit | leaf/singleton と fallback/left-to-right append のみから、raw な実験用形状の一回の reassociation が構造的出力を変えないことを確認した。hole context は source-level empty fallback ではない。confluence、validity、評価、source-level unit、OBL status は未解決のままである。 | `plan/156-t0-t2-research-autonomy-envelope.md` |
| OBL-006 source audit | `0 direct / 0 delegated / 1 missing`。同順序の出力と各 step の出力保存だけでは confluence は決まらず、正本は term domain・guarded validity・同値/denotation・relation を未指定である。proof-facing statement の前に owner/canon の formalization boundary が要る。 | `plan/156-t0-t2-research-autonomy-envelope.md` |
| THM-002 / OBL-007 source audit | 正本は同一 lineage の非減少と explicit reacquire による fresh な再選択方針を直接固定する。一方、完全な Lean statement は `0 direct / 0 delegated / 1 missing` の trace-formalization boundary であり、trace/selection/lineage-origin/reacquire/freshness/transition-frame を選ぶ前に owner/canon の formalization act が要る。 | `plan/156-t0-t2-research-autonomy-envelope.md` |
| THM-004 / OBL-015 source audit | 正本は delegated capability の grant-lineage 方針と owner-local mutation の別枝を直接固定する。一方、完全な Lean statement は `0 direct / 0 delegated / 1 missing` の coupled mutation-origin/authorization boundary であり、trace/owner-local/declared-transition/mutation-association を選ぶ前に owner/canon の formalization act が要る。 | `plan/156-t0-t2-research-autonomy-envelope.md` |
| THM-005 / OBL-017 source audit | 正本は observer-safe noninterference と観測・redaction・retention の方針を直接固定する。一方、完全な Lean statement は `0 direct / 0 delegated / 1 missing` の coupled low-equivalence/export boundary であり、configuration relation、label/declassification、observer-safe export ABI、出力 equality/renaming/order/multiplicity を選ぶ前に owner/canon の formalization act が要る。 | `plan/156-t0-t2-research-autonomy-envelope.md` |
| THM-003 / OBL-009 source audit | 正本は SaveObject schema と八つの successful-load 必要条件を直接固定し、theory/01 は Config / WellFormed 語彙を与える。一方、完全な Lean statement は `0 direct / 0 delegated / 1 missing` の coupled successful-load restoration boundary であり、Load result、restored Config/prefix、liveness/resurrection bridge、必要条件から結果側安全性への関係を選ぶ前に owner/canon の formalization act が要る。 | `plan/156-t0-t2-research-autonomy-envelope.md` |
| OBL-026 source audit | 正本は transparent overlay の十方向を直接固定するが、完全な Lean statement は `0 direct / 0 delegated / 1 missing` の coupled composition boundary である。contract-field の順序、layer-stack composition、equality/extensionality を選ぶ前に owner/canon の formalization act が要る。実験的な preorder は正本の variance や ABI を選ばない。 | `plan/156-t0-t2-research-autonomy-envelope.md` |
| OBL-028 source audit | 正本は lifecycle と revocation monotonicity を直接固定するが、完全な Lean statement は `0 direct / 0 delegated / 1 missing` の coupled boundary である。revocation、new epoch/evidence、state identity、trace/transition を選ぶ前に owner/canon の formalization act が要る。実験的 action は正本の authority/persistence ABI を選ばない。 | `plan/156-t0-t2-research-autonomy-envelope.md` |
| OBL-022 source audit | 正本は sample を `H` 外の read-side とし typed adapter を直接固定するが、完全な Lean statement は `0 direct / 0 delegated / 1 missing` の coupled boundary である。sample/discrete-state carrier、effect declaration/application、transition、frame/equality を選ぶ前に owner/canon の formalization act が要る。実験的 action は stream/adapter ABI を選ばない。 | `plan/156-t0-t2-research-autonomy-envelope.md` |
| OBL-027 source audit | 正本は locus-local atomic-cut policy を直接固定するが、完全な Lean statement は `0 direct / 0 delegated / 1 missing` の coupled boundary である。occurrence、causality、locus、cut projection、rollback result を選ぶ前に owner/canon の formalization act が要る。実験的 frontier は rollback/persistence ABI を選ばない。 | `plan/156-t0-t2-research-autonomy-envelope.md` |
| OBL-023 source audit | 正本は consumer-frontier admissibility と no-split-frame policy を直接固定するが、完全な Lean statement は `0 direct / 0 delegated / 1 missing` の coupled boundary である。consumer、atomic group、frontier、interpretation、coherence、clock/latency を選ぶ前に owner/canon の formalization act が要る。実験的 shared frame は provider/transport ABI を選ばない。 | `plan/156-t0-t2-research-autonomy-envelope.md` |
| OBL-010 source audit | 正本は direct generating edge の推移閉包と prefix closure を直接固定する。全 direct predecessor closure から `Consistent` を導く一般 kernel は確認したが、有限 checker の carrier、全 edge coverage、decider、result/diagnostic は未選定である。実験的 two-edge checker は実装 ABI を選ばない。 | `plan/156-t0-t2-research-autonomy-envelope.md` |
| OBL-004 source audit | 正本は item ごとの hidden-edge 禁止を直接固定する。itemwise generated-edge containment の合成 kernel は確認したが、program/elaboration、`G_e`、declaration mapping、runtime communication は未選定である。実験的 two-edge checker は transport ABI を選ばない。 | `plan/156-t0-t2-research-autonomy-envelope.md` |
| OBL-003 source audit | 正本は declared finite fragment と row containment の Line-1 方向を直接固定する。有限 failure-row checker kernel は確認したが、complete rule set、AST/parser、declaration/name resolution、carrier/equality、residual split、result/diagnostic は未選定である。実験的 two-bit checker は language ABI を選ばない。 | `plan/156-t0-t2-research-autonomy-envelope.md` |
| OBL-018 source audit | 正本は observer-safe が高ラベル状態と raw witness/auth を出さないことを直接固定する。低位 position だけへの有限 explicit-flow 射影は、低位一致から出力一致を導く。final lattice/declassification、configuration と observer/export ABI、occurrence provenance、collection equality/order/multiplicity は未選定であり、完全な THM-005/OBL-017/018 にはならない。 | `plan/156-t0-t2-research-autonomy-envelope.md` |
| OBL-014 source audit | 正本は Z-cycle checkpoint を recoverable global cut に対して inadmissible とする方針を直接固定する。一方、checker reject と Netzer-Xu useless-checkpoint characterization の同値には、checkpoint graph・zigzag・recoverability・recognizer の定義的接続がない。`CUT-11` は planned-skeleton の synthetic reason-code evidence であり、証明ではない。 | `plan/156-t0-t2-research-autonomy-envelope.md` |
| 残存 ledger map | OBL-002/008/016 は親 statement 待ち、OBL-011..013 は OBL-009 の Load/restored-state/live-after-load 関係待ち、OBL-019 は既知の E-PATCH frame gap と重複する。理論/10 の OBL-024/025 source audit も完了し、現行 ledger と既存 LAB lane における独立 source cut は残っていない。 | `plan/156-t0-t2-research-autonomy-envelope.md` |
| OBL-024 source audit | 正本は Diagnostic の field と actual rule/premise/replay の方向を固定する。一方、emission と rejection の association、rule/premise/bindings、replay と exactly-there、span/equality/ordering は未定義である。既存 E-ROW projection は report-local LAB evidence であり、proof-facing relation ではない。 | `plan/156-t0-t2-research-autonomy-envelope.md` |
| OBL-025 source audit | 正本は Line-1 の修復完全性方向と初期 taxonomy を固定する。一方、rejection / declared fragment / single-edit repair / diagnostic association / suggestion realization は未定義である。E-ROW payload と Lean draft は限定 LAB evidence であり、全称定理・最終 ABI・修復 ranking ではない。 | `plan/156-t0-t2-research-autonomy-envelope.md` |
| T0-T2 formalization map | source audit の次は新しい定理探索ではない。G0-D3 と OBL-020 の既存 organization decision を先に確認し、その後 Gate ごとの scoped package が必要な carrier / relation だけを明示して選ぶ。G1 の step-preservation と elaboration/diagnostics を一つの表現へ併合する判断はしていない。 | `plan/156-t0-t2-research-autonomy-envelope.md` |
| 最新 preflight | literal-RHS foreign-locus write の source 正負 pair は再現できたが、authority/capability/witness carrier を OBL-001 の抽象 predicate へ結ぶ既存 lane はない。`T-RESEARCH-004` は未選定。 | `plan/156-t0-t2-research-autonomy-envelope.md` |

## 現在の停止線

- phase-governance/t0-g0 の `pass` は T0 profile evidence だけであり、G0
  exit、T1 entry、SCN conformance、OBL completion、proof、runtime/product
  readiness を作らない。 | `mirrorea_canon/plan/01-phases.md` |
- canon package の close に owner acceptance、ADR/canon/SCN の変更、
  theory/11 の status 更新、Gate / Phase acceptance、L0/L1 choice が必要なら、
  agent は `decision-ready` で止まる。 | `mirrorea_canon/plan/02-operating-model.md` |
- 新しい helper / evidence lane / schema / CI / Make target / main merge /
  conformance claim は T1 exit 前の研究範囲外。 | `plan/156-t0-t2-research-autonomy-envelope.md` |
- OBL-001 の concrete-evidence bridge には bridge 固有の owner disposition が
  必要である。これは scoped design 比較だけを許し得るもので、committed bridge や
  moratorium 例外を自動的に許可しない。 | `plan/156-t0-t2-research-autonomy-envelope.md` |
- 一般的な継続指示は bridge の defer / authorization を選んだ記録ではない。
  ただし今回の直接的な理論自走指示は、bridge を未選択のまま他の既存 lane を
  選ぶことだけを許す。bridge の設計・defer 記録・artifact は許可しない。 |
  `plan/156-t0-t2-research-autonomy-envelope.md` |

## オーナーの確認・判断待ち

| ID | 状態 | いま必要なこと |
| --- | --- | --- |
| G0-D1 / D2 / D4 | recorded | ADR-0013 に記録済み。再判断待ちではない。 |
| G0-D3 | DEFERRED (dormant) | owner が明示的に reopen するまで研究単位の選定対象にしない。 |
| T0-T2 research autonomy | recorded LAB operating authorization | `plan/156-t0-t2-research-autonomy-envelope.md` の選定規則と停止条件の範囲で agent が調査を継続する。 |
| OBL-001 concrete-evidence bridge | owner record pending | proof-facing need まで defer するか、既存 route と許容 persistence を明記した artifact-free design 比較を許可するか。current LAB recommendation は defer。 |
| OBL-020 formalization organization | owner record pending | PROPOSAL-003 の A/B/C。A は LAB bundle の advisory recommendation に限られ、現時点では proposal 起票のみで採択・ADR・ledger movement はない。 |

`G0-D3` の defer は canon lifecycle を変えず、LAB 上の選定ガードとしてだけ
扱います。 | `plan/155-t0-g0-governance-profile-proposal.md`

## 根拠と詳細

| 知りたいこと | 正本または証拠 |
| --- | --- |
| 目的・体系の地図 | `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, `docs/diagrams/layer-stack.mmd` |
| Gate / Phase と実用化の順序 | `mirrorea_canon/plan/00-gates.md`, `mirrorea_canon/plan/01-phases.md`, `docs/diagrams/workflow.mmd` |
| エージェントと owner の境界 | `mirrorea_canon/plan/02-operating-model.md`, `plan/156-t0-t2-research-autonomy-envelope.md` |
| proof の唯一の状態台帳 | `mirrorea_canon/theory/11-metatheory-ledger.md` |
| runnable LAB の範囲 | `samples_progress.md` |
| 詳細な現況と次の作業 | `progress.md`, `tasks.md`, `plan/156-t0-t2-research-autonomy-envelope.md` |

## 更新規約

まず canon または bounded LAB evidence を更新し、次に `progress.md`、
`tasks.md`、`samples_progress.md` を必要な範囲で同期し、この派生ビューを最後に
更新します。根拠が未解決なら `STALE - source reconciliation required` と明記し、
推測で current state を置き換えません。詳細な履歴は `plan/` と
`docs/reports/` に残します。
