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

### 1. checker attachment から handoff row への migration

- source-visible typed-surface comparison は fixed 済みであり、checker-adjacent principal source を維持した current line は固まった。
- current reading:
  **次の open problem は、checker attachment と handoff-row reserve の migration cut をどこで切るかである。**

### 2. theorem proof artifact / bridge stop line

- theorem pilot order と lemma wording floor は fixed 済みである。
- current reading:
  **次の open problem は、review artifact / bridge sketch / theorem discharge の stop line をどこまで tighten するかである。**

### 3. model-check sample-facing property wording

- projection / property-family inventory と bridge grain は fixed 済みである。
- current reading:
  **次の open problem は、sample-facing property summary wording をどこまで揃えるかである。**

### 4. order / handoff property-language bridge

- cut / order / witness decomposition、negative falsifier coverage、candidate reduction は fixed 済みである。
- current reading:
  **次の open problem は、candidate reduction と boundary matrix を property-language bridge へどう接続するかである。**

### 5. thread / node parity

- same causal language と lower-level asymmetry を分ける wording は docs-first に固まっている。
- current reading:
  **次の open problem は、その wording を property-language / host binding line とどう接続するかである。**

### 6. modal promotion threshold

- `lambda-circle-box` partial basis と guarded / MDTT / MTT stronger-candidate family の reduction までは固まった。
- current reading:
  **次の open problem は、どの threshold で stronger candidate へ昇格させるかである。**

### 7. shared-space confusion / replay and host bridge

- room-profile、confusion / replay、bridge-only host binding note をどこまで reserve line に寄せるか。
- current reading:
  **docs-first boundary note までは進められる。**

## current recommendation

- type / proof / model-check line は detail-side follow-up を今進める。
- order / memory / authority-handoff / syntax / modality line も comparison / adequacy / verifier-boundary の次段 bridge を今進める。
- reserve integration lane の CLI / host / shared-space boundary note は theory-lab line と並走してよい。
- ただし、どれも current executable lane を置き換えない。
