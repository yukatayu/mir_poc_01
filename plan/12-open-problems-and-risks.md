# plan/12 — 未解決問題とリスク

## 目的

この文書は、current repo の compact risk register である。
detail-side の研究手順は `plan/18`、重い将来線は `plan/13` に分ける。

## risk register

| 項目 | 種別 | current 状態 | リスク | current 対応 |
|---|---|---|---|---|
| fallback intuition drift | semantics / notation | 継続中 | outer-wrapper 直感が chain semantics を上書きする | explicit edge-row form と fixture explanation を維持 |
| final parser grammar の premature freeze | syntax | OPEN | syntax を早く決めすぎると semantics を拘束する | companion notation に留める |
| public surface hidden promotion | helper / public API | 継続中 | thin facade、shell、support-only、repo-local helper が混線する | `plan/09` の stable / support / excluded split を維持 |
| fixed-subset widening outruns docs | sample / validation | 継続中 | widening が progress/tasks/plan を追い越す | runner / ladder / regression / docs mirror を同 task で閉じる |
| cut-family conflation | semantics / theory | OPEN | `atomic_cut`、snapshot、barrier、durable cut を 1 primitive として誤読する | local finalization / ordering / observation / commit を別 family に保つ |
| low-level memory-order premature import | semantics / runtime policy | OPEN | C++ / ISA vocabulary が source surface を不必要に重くする | `memory_order` family は retained-later reference family に留める |
| syntax overcompression | syntax / UX | OPEN | packed row や hidden precedence が semantic honesty を壊す | syntax 比較軸を explicit 化する |
| thread/node false equality | distributed semantics | OPEN | lowering / transport / evidence / failure の差が消える | same causal language と lower-level asymmetry を分けて書く |
| modality underfitting | type / logic | OPEN | `lambda-circle-box` だけで place / scope / authority / witness を押し切ろうとする | partial basis と stronger candidate を分ける |
| typed / theorem / model-check overclaim | proof workflow | OPEN | full system を持たないのに solved に見せてしまう | boundary / pilot / stop line を package 単位で固定する |
| theory-lab no-falsification loop | process / research | OPEN | literature / prototype / counterexample が 1 本の長文に埋もれる | theory-lab operating model を維持する |
| shared-space confusion / replay drift | shared-space | OPEN | identity / admission / authority / replay / rejoin が混線する | room-profile と confusion/replay compact note を reserve で切る |
| host target premature fixation | integration | OPEN | visualizer / FFI / engine adapter の first target が早く固定される | docs-first boundary までは target-neutral に保つ |
| docs drift from long package chains | process | 継続中 | snapshot 文書に stale current-line が残る | `plan/` を detail memory に、`progress.md` / `tasks.md` を薄い snapshot に保つ |

## current open problems

### 1. typed first attachment candidate

- 何を source-visible first candidate へ昇格させるか。
- current reading:
  **checker-adjacent attachment inventory は fixed 済みであり、次は source-visible comparison である。**

### 2. theorem pilot order

- semantic core の first lemma wording と admissible evidence floor をどう整えるか。
- current reading:
  **pilot order 自体は fixed 済みであり、次は wording / evidence hardening である。**

### 3. model-check projection and property family

- current machine-facing carrier と small-cluster reserve の橋をどう切るか。
- current reading:
  **projection / property-family inventory は fixed 済みであり、次は bridge note である。**

### 4. cut / order / witness decomposition

- `po / dep / pub / obs / wit / final / hb(scope)` と cut family をどこまで working vocabulary にするか。
- current reading:
  **theory-first inventory と falsifier coverage は進められるが、source surface adoption はまだ早い。**
- 補足:
  現在不足しているのは vocabulary そのものよりも、negative corpus coverage と kept-later family の reduction note である。

### 5. thread / node parity

- same causal language と lower-level asymmetry をどう切り分けるか。
- current reading:
  **docs-first wording hardening は進められる。**

### 6. syntax honesty / modal adequacy

- syntax comparison を semantic honesty / checker legibility / modal adequacy / misreading resistance でどう回すか。
- current reading:
  **comparison と kill criteria は進められるが、final grammar / final calculus はまだ先である。**

### 7. shared-space confusion / replay and host bridge

- room-profile、confusion / replay、bridge-only host binding note をどこまで reserve line に寄せるか。
- current reading:
  **docs-first boundary note までは進められる。**

## current recommendation

- type / proof / model-check line は「重い future work だから完全に後回し」ではなく、detail-side planning を今進める。
- order / memory / authority-handoff / syntax / modality line も、「まだ触れない」ではなく、comparison / adequacy / verifier-boundary matrix を今進める。
- ただし、どちらも current executable lane を置き換えない。
