# 335 — current L2 first-widened-authored-row-e1-actualization-ready second-widened-authored-row-e21 actualization comparison

## 目的

`specs/examples/333-current-l2-proof-notebook-bridge-sketch-reopen-ordering-ready-first-widened-authored-row-e1-actualization-comparison.md`
と
`specs/examples/334-current-l2-first-widened-authored-row-e1-actualization-ready-minimal-first-widened-authored-row-e1-threshold.md`
で first widened authored row `e1` actualization を fixed した次段として、

- second widened authored row `e21-try-atomic-cut-frontier` をどの shape で actual source row に上げるか
- current lowerer / runner / regression helper / formal-hook family をどこまで保ったまま widen するか
- `E21` / `E22` contrast pressure をどこまで後段へ残すか

を比較する。

ここで固定するのは
**current L2 first-widened-authored-row-e1-actualization-ready second-widened-authored-row-e21 actualization comparison**
であり、

- `E21` / `E22` contrast comparison
- `e3` theorem-side / formal-hook guard comparison
- theorem-side plain bridge sketch actualization
- concrete theorem / model-check tool binding

はまだ固定しない。

## scope

- entry criteria は `specs/examples/315...334` で fixed 済みの source corpus scope / mapping / lowering / runner / ladder / source-sample policy / theorem-first pilot / authored-row sequencing / bridge-sketch ordering / `e1` actualizationに置く。
- current widen order は `e1 -> e21 -> e3` に固定済みと読む。
- actual widening は `e21-try-atomic-cut-frontier` だけを扱い、`e3` guard line は still later に残す。

## current 前提

current repo では次が成立している。

1. current lowerer first cut は `try { ... } fallback { ... }` と single-line `require` clause を helper-local に受ける。
2. `e21` fixture 自体は current `runtime_try_cut_cluster` family の内側に収まる。
3. `e21` widening の主眼は `atomic_cut` frontier update の source-backed runtime ratchet であり、`E21` / `E22` contrast 自体の public compare ではない。
4. theorem-side bridge sketch reopen ordering は still fixed entry criteria に留まり、`e21` widening のために先に actualize する必要はない。

したがって current 問いは、
**contrast pressure を混ぜずに `e21` を second widened authored row として narrow actualize する最小 shape は何か**
である。

## 比較観点

1. current lowerer / runner / regression helper / README ladder を同じ task で同期できるか
2. current tool-neutral formal-hook top `runtime_try_cut_cluster` を保てるか
3. `E21` / `E22` contrast pressure を premature に public line へ混ぜないか
4. next line `e3` guard comparisonへ narrow に handoff できるか

## 比較対象

### 案 1. helper-compatible source row として `e21` を actualize し、contrast は later に残す

#### shape

```text
samples/current-l2/e21-try-atomic-cut-frontier.txt = {
  place root {
    place session {
      place draft_profile {
        try {
          perform stage_profile_patch on profile_draft
            require write

          atomic_cut

          perform annotate_profile_patch on profile_draft
            require write

          perform validate_profile_patch on profile_draft
            require stable_after_annotation(profile_draft)
        } fallback {
          perform load_last_snapshot on profile_snapshot
            require read
        }
      }
    }
  }
}
```

#### 利点

- current lowerer accepted floor を広げずに `e21` を authored row に昇格できる。
- runner accepted set、regression helper current authored inventory、README ladder row を narrow widen できる。
- `runtime_try_cut_cluster` reached row を current formal-hook family の内側で増やせる。

#### 欠点

- `E21` / `E22` contrast は別 package に残る。

### 案 2. `e21` actualization と同時に `E21` / `E22` contrast を public line に混ぜる

#### 利点

- frontier update gate の contrast は早く見える。

#### 欠点

- `e21` authored-row widen と contrast package が混線する。
- current source-backed runtime ratchet より先に parser/checker-side public comparison pressure を呼び込む。

### 案 3. `e3` guard comparisonより後ろへ `e21` actualization 自体を送る

#### 利点

- theorem-side line だけを先に整理できる。

#### 欠点

- current widen order `e1 -> e21 -> e3` に反する。
- runtime try/cut compatible widen を先に厚くする current staged line を崩す。

## current judgment

current L2 で最も自然なのは、
**案 1. helper-compatible source row として `e21` を actualize し、contrast は later に残す**
である。

理由は次の通り。

1. `e21` widening の主眼は source-backed sample ladder を narrow に前進させることであり、`E21` / `E22` contrast 比較ではない。
2. `e21` は current `runtime_try_cut_cluster` family の内側で reached row に上げられる第二の runtime row である。
3. contrast pressure をここで混ぜると、narrow authored-row widen と later public comparison line が不必要に結び付く。

## current first choice details

- `e21-try-atomic-cut-frontier.txt` を helper-compatible source row として actualize する。
- `mir_runtime::current_l2` runner accepted sample set に `e21-try-atomic-cut-frontier.txt` を加える。
- `python3 scripts/current_l2_source_sample_regression.py` inventory / regression current authored inventory に `e21` を昇格する。
- `samples/current-l2/README.md` ladder row では `e21` を `static gate -> interpreter(success) -> runtime_try_cut_cluster formal hook` reached row として扱う。
- `E21` / `E22` contrast と `e3` theorem-side guard は still later に残す。

## open questions

- `E21` / `E22` contrast をどの later package で public compare に上げるか
- `e3` guard comparisonを docs-only first choice に留めるか
- theorem-side plain bridge sketch actualization を `e3` guard comparison の直後に置くか
