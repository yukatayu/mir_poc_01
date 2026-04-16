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
| cut-family conflation | semantics / theory | OPEN | `atomic_cut`、snapshot、barrier、durable cut を 1 primitive として誤読する | local finalization / ordering / observation / commit を別 family として整理する |
| low-level memory-order premature import | semantics / runtime policy | OPEN | C++ / ISA vocabulary を source surface に早く入れると core が重くなる | `memory_order` family は retained-later reference family に留める |
| syntax overcompression | syntax / UX | OPEN | packed row や hidden precedence が semantic honesty を壊す | syntax 比較軸を semantic honesty / checker legibility / modal adequacy / misreading resistance に置く |
| thread/node false equality | distributed semantics | OPEN | lowering / transport / evidence / failure の差まで消してしまう | source-level causal language と lower-level asymmetry を分けて書く |
| modality underfitting | type / logic | OPEN | `lambda-circle-box` だけで place / scope / authority / witness を押し切ろうとする | partial basis と stronger candidate を分ける |
| typed work の premature formalization | type / logic | OPEN | full type system を急ぐと runtime / parser / proof を同時に凍らせる | first attachment candidate inventory を先に切る |
| theorem tool brand premature fixation | proof workflow | OPEN | tool choice が carrier / shell / public surfaceへ逆流する | semantic-core theorem pilot planning を先に切る |
| model-check property family ambiguity | model-check | OPEN | carrier はあっても何を検査するかが曖昧なままになる | projection grain と property-family inventory を先に切る |
| theory-lab absence | process / research | OPEN | literature / prototype / falsification が 1 本の長文に埋もれ、token を浪費する | theory-lab operating model を分離し、accepted candidate だけを昇格する |
| shared-space confusion / replay drift | shared-space | OPEN | identity / admission / authority / replay / rejoin が混線する | room-profile と confusion/replay compact table を reserve で切る |
| premature backend / codegen binding | implementation | OPEN | syntax / lowering / runtime boundary を早期固定する | backend は heavy future workstream に残す |
| external target ambiguity | integration | OPEN | visualizer / FFI / engine adapter の first target が揺れる | host-facing boundary までは docs-first、target choice は later |
| docs drift from long package chains | process | 継続中 | snapshot 文書に fixed chain が蓄積して stale になる | progress/tasks を薄く、detail を `plan/` へ移す |

## current open problems

### 1. cut family decomposition

- 概要
  - local finalization、ordering-only barrier、global observation / snapshot、commit / evidence-bearing cut をどう比較し、どこまで vocabulary に上げるか。
- current reading
  - `atomic_cut` の local nucleus、later `barrier`、later `durable_cut` は source-backed である。
  - global snapshot / consistent-cut family は comparison candidate に留める。

### 2. order / visibility / witness relation decomposition

- 概要
  - `po / dep / pub / obs / wit / final / hb(scope)` のような relation family をどこまで working vocabulary にするか。
- current reading
  - **theory-first inventory は進められる**。
  - source surface / backend lowering / verifier handoff を 1 語彙に潰す段階ではない。

### 3. thread / node parity

- 概要
  - same causal language の line と lower-level asymmetry をどう切り分けるか。
- current reading
  - **docs-first wording hardening は進められる**。
  - final deployment / transport / failure surface を固定する段階ではない。

### 4. syntax-semantics coupling principle

- 概要
  - semantic honesty と readability を syntax comparison の明示的規準にするか。
- current reading
  - **companion notation / theory comparison の判断軸としては進められる**。
  - final parser grammar や reserved keyword を固定する段階ではない。

### 5. modal foundation comparison

- 概要
  - `lambda-circle-box` を partial basis に留め、guarded / MDTT / MTT / Fitch-style stronger candidate をどう比較するか。
- current reading
  - **比較と inventory は進められる**。
  - final foundation adoption は still later である。

### 6. typed first attachment candidate

- 概要
  - typed work を semantic carrier / checker boundary / source-visible surface のどこから始めるか。
- current reading
  - **まだ implementation-ready ではないが、boundary inventory は十分進められる段階** である。

### 7. theorem pilot order

- 概要
  - theorem line をどの invariant family から pilot 化するか。
- current reading
  - **semantic-core theorem pilot の計画化は進められる**。
  - concrete prover binding は still later である。

### 8. model-check projection and property family

- 概要
  - current machine-facing carrier の次に、何を first property family に置くか。
- current reading
  - **projection / property-family inventory までは進められる**。
  - concrete tool binding と production contract は still later である。

### 9. order / handoff verifier boundary

- 概要
  - same-owner structural floor、stage sequencing、witness presence、handoff-before-publication、replay invalidation、fairness claim 等を 4-way split へどう配賦するか。
- current reading
  - **matrix 化と adequacy-corpus 化は進められる**。
  - full theorem / protocol / runtime contract の fixed public schema は still later である。

### 10. theory-lab operating discipline

- 概要
  - literature scout / formalizer / prototyper / falsifier / integrator をどう分け、promotion criteria をどこに置くか。
- current reading
  - **repo process に自然に乗せる docs-first line は進められる**。
  - monolithic long-context 1 本へ戻すべきではない。

## current recommendation

- type / proof / model-check line は「重い future work だから完全に後回し」ではなく、detail-side planning を今進める。
- order / memory / authority-handoff / syntax / modality line も、「まだ触れない」ではなく、comparison / adequacy / verifier-boundary matrix を今進める。
- ただし、どちらも current executable mainline を置き換えない。
