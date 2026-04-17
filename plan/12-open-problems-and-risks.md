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

### 1. stronger typed-surface promotion threshold

- request / predicate / `try` cluster typed-surface reserve note と typed-surface family split note は fixed 済みである。
- current reading:
  **次の open problem は、shared attachment shape と stronger typed surface promotion をどの later threshold で reopen するかである。**

### 2. theorem discharge transport / public-contract later gate

- theorem pilot order、lemma wording floor、proof artifact / bridge stop line、admissible evidence contraction、notebook-consumer threshold は fixed 済みである。
- current reading:
  **次の open problem は、abstract discharge-entry reserve の先で concrete discharge transport / public theorem contract をどの later gate に残すかである。**

### 3. model-check property-language / tool-binding later gate

- projection / property-family inventory、bridge grain、sample-facing summary wording、tool-binding stop line、small-cluster projection keep/drop は fixed 済みである。
- current reading:
  **次の open problem は、first settled property language と concrete tool seam をどの later gate に残すかである。**

### 4. modality internalization trigger

- `lambda-circle-box` partial basis、guarded / MDTT / MTT stronger family、promotion threshold note、reduction timing note は fixed 済みである。
- current reading:
  **次の open problem は、place / authority / witness を stronger modality line へ internalize する trigger をどこで認めるかである。**

### 5. shared-space final fairness / replay operational profile later gate

- room-profile / host binding bridge-only note と fairness / replay strengthening reserve note は fixed 済みである。
- current reading:
  **次の open problem は、final operational catalog と fairness / replay operational profile をどこで mixed gate から user-spec-required gate へ送るかである。**

### 6. public operational CLI installed packaging / success-criteria later gate

- current-L2 scoped Rust shell over thin facade と packaging reserve note は fixed 済みである。
- current reading:
  **次の open problem は、installed-binary promotion と backend / tooling success criteria をどこで later mixed gate として扱うかである。**

## current recommendation

- type / proof / model-check line は detail-side follow-up を今進める。
- order / memory / authority-handoff / syntax / modality line も comparison / adequacy / verifier-boundary の次段 bridge を今進める。
- reserve integration lane の packaging / fairness / replay reserve は theory-lab line と並走してよい。
- ただし、どれも current executable lane を置き換えない。
