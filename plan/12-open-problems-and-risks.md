# plan/12 — 未解決問題とリスク

## 目的

この文書は、current repo の compact risk register である。
detail-side の研究手順は `plan/18`、重い将来線は `plan/13` に分ける。

## risk register

| 項目 | 種別 | current 状態 | リスク | current 対応 |
|---|---|---|---|---|
| fallback intuition drift | semantics / notation | 継続中 | outer wrapper 直感が chain semantics を上書きする | explicit edge-row form と fixture explanation を維持 |
| final parser grammar の premature freeze | syntax | OPEN | syntax を早く決めすぎると semantics を拘束する | companion notation に留める |
| public surface hidden promotion | helper / public API | 継続中 | thin facade、shell、support-only、repo-local helper が混線する | `plan/09` の label と `stable/support/excluded` split を維持 |
| fixed-subset widening outruns docs | sample / validation | 継続中 | sample widening が progress/tasks/plan を追い越す | runner / ladder / regression / docs mirror を同 task で閉じる |
| typed work の premature formalization | type / logic | OPEN | full type system を急ぐと runtime / parser / proof を同時に凍らせる | first attachment candidate inventory を先に切る |
| theorem tool brand premature fixation | proof workflow | OPEN | tool choice が carrier / shell / public surfaceへ逆流する | semantic-core theorem pilot planningを先に切る |
| model-check property family ambiguity | model-check | OPEN | carrier はあっても何を検査するかが曖昧なままになる | projection grain と property-family inventory を先に切る |
| ordering / `memory_order` overreach | semantics / runtime policy | OPEN | low-level vocabulary を先に入れると mainline 全体が重くなる | `atomic_cut` nucleus と higher-level ordering boundary を分ける |
| shared-space confusion / replay drift | shared-space | OPEN | identity / admission / authority / replay / rejoin が混線する | room-profile と confusion/replay compact table を reserve で切る |
| premature backend / codegen binding | implementation | OPEN | syntax / lowering / runtime boundary を早期固定する | backend は heavy future workstream に残す |
| external target ambiguity | integration | OPEN | visualizer / FFI / engine adapter の first target が揺れる | host-facing boundary までは docs-first、target choice は later |
| docs drift from long package chains | process | 継続中 | snapshot 文書に fixed chain が蓄積して stale になる | progress/tasks を薄く、detail を `plan/` へ移す |

## current open problems

### 1. typed first attachment candidate

- 概要
  - typed work を semantic carrier / checker boundary / source-visible surface のどこから始めるか。
- current reading
  - **まだ implementation-ready ではないが、boundary inventory は十分進められる段階** である。

### 2. theorem pilot order

- 概要
  - theorem line をどの invariant family から pilot 化するか。
- current reading
  - **semantic-core theorem pilot の計画化は進められる**。
  - concrete prover binding は still later である。

### 3. model-check projection and property family

- 概要
  - current machine-facing carrier の次に、何を first property family に置くか。
- current reading
  - **projection / property-family inventory までは進められる**。
  - concrete tool binding と production contract は still later である。

### 4. ordering / `memory_order` reinterpretation

- 概要
  - `atomic_cut` の local nucleus と higher-level ordering / fairness / handoff family をどう分けるか。
- current reading
  - **theory-first inventory は進められる**。
  - implementation と final syntax へ上げるには早い。

## current recommendation

- type / proof / model-check line は「重い future work だから完全に後回し」ではなく、detail-side planning を今進める。
- ordering line も「まだ触れない」ではなく、boundary inventory と handoff split を今進める。
- ただし、どちらも current executable mainline を置き換えない。
