# 333 — current L2 proof-notebook-bridge-sketch-reopen-ordering-ready first-widened-authored-row-e1 actualization comparison

## 目的

`specs/examples/331-current-l2-deferred-authored-row-widen-sequencing-ready-proof-notebook-bridge-sketch-reopen-ordering-comparison.md`
と
`specs/examples/332-current-l2-proof-notebook-bridge-sketch-reopen-ordering-ready-minimal-proof-notebook-bridge-sketch-reopen-ordering-threshold.md`
で theorem-side reopen ordering を fixed した次段として、

- first widened authored row `e1-place-atomic-cut` をどの shape で actual source row に上げるか
- current lowerer / runner / regression helper / formal-hook family をどこまで保ったまま widen するか
- representative prose mirror pressure と multiline clause suite pressure をどこまで後段へ残すか

を比較する。

ここで固定するのは
**current L2 proof-notebook-bridge-sketch-reopen-ordering-ready first-widened-authored-row-e1 actualization comparison**
であり、

- `e21` / `e3` widening
- theorem-side plain bridge sketch actualization
- compare-bless metadata
- concrete theorem / model-check tool binding

はまだ固定しない。

## scope

- entry criteria は `specs/examples/315...332` で fixed 済みの source corpus scope / mapping / lowering / runner / ladder / source-sample policy / theorem-first pilot / authored-row sequencing / bridge-sketch orderingに置く。
- current widen order は `e1 -> e21 -> e3` に固定済みと読む。
- actual widening は `e1-place-atomic-cut` だけを扱い、`e21` / `e3` は still later に残す。

## current 前提

current repo では次が成立している。

1. current lowerer first cut は single-line `require` / `ensure` clause と inline `admit` fragment を helper-local に受ける。
2. representative prose `E1` には multiline `ensure:` block の読みがあるが、current lowerer は multiline clause suite を fail-closed に止める。
3. `e1` 自体の runtime/fomal-hook family は `runtime_try_cut_cluster` の current top に収まる。
4. current theorem-first review-unit pilot は row-local consumer であり、`e1` widening のために bridge sketch reopen を先に要求しない。

したがって current 問いは、
**multiline widening を呼ばずに `e1` を narrow authored row として actualize する最小 shapeは何か**
である。

## 比較観点

1. current lowerer / runner / regression helper / README ladder を同じ task で同期できるか
2. current tool-neutral formal-hook top `runtime_try_cut_cluster` を保てるか
3. representative prose mirror pressure や multiline clause suite widening を premature に混ぜないか
4. next line `e21` widen と later line `e3` guard comparisonへ narrow に handoff できるか

## 比較対象

### 案 1. helper-compatible single-line `ensure` source row として `e1` を actualize する

#### shape

```text
samples/current-l2/e1-place-atomic-cut.txt = {
  place root {
    place session {
      place authority_cell {
        perform update_authority on profile_authority
          require write
          ensure owner_is(session_user)

        atomic_cut

        perform append_audit on authority_log
          require append
      }
    }
  }
}
```

#### 利点

- current lowerer accepted floor を広げずに `e1` を authored row に昇格できる。
- runner accepted set、regression helper current authored inventory、README ladder row をそのまま narrow widen できる。
- `runtime_try_cut_cluster` reached row を current formal-hook family の内側で増やせる。

#### 欠点

- representative prose multiline `ensure:` block を exact mirror した source row にはならない。

### 案 2. representative prose mirror を優先し、multiline `ensure:` block ごと `e1` を actualize する

#### 利点

- representative prose と source text の見た目は近くなる。

#### 欠点

- current lowerer accepted floor を広げる pressure を呼ぶ。
- `e1` widening package ではなく multiline clause suite widening package に近づく。

### 案 3. multiline clause suite widening まで `e1` actualization 自体を defer する

#### 利点

- representative prose mirror とのズレは出ない。

#### 欠点

- current widen order `e1 -> e21 -> e3` の first slot を閉じられない。
- runtime try/cut compatible widen を先に厚くする current staged line に反する。

## current judgment

current L2 で最も自然なのは、
**案 1. helper-compatible single-line `ensure` source row として `e1` を actualize する**
である。

理由は次の通り。

1. `e1` widening の主眼は representative prose の exact mirror ではなく、current source-backed sample ladder を narrow に前進させることにある。
2. `e1` は current `runtime_try_cut_cluster` family の内側で reached row に上げられる最小 runtime row である。
3. multiline clause suite widening をここで混ぜると、`e1` actualization と parser-side widening が不必要に結び付く。

## current first choice details

- `e1-place-atomic-cut.txt` を helper-compatible single-line `ensure owner_is(session_user)` source row として actualize する。
- `mir_runtime::current_l2` runner accepted sample set に `e1-place-atomic-cut.txt` を加える。
- `python3 scripts/current_l2_source_sample_regression.py` inventory / regression current authored inventory に `e1` を昇格する。
- `samples/current-l2/README.md` ladder row では `e1` を `static gate -> interpreter(explicit_failure) -> runtime_try_cut_cluster formal hook` reached row として扱う。
- theorem-side bridge sketch reopen ordering は still fixed entry criteria に留め、`e1` widening の中には混ぜない。

## open questions

- `e1` source sample と representative prose `E1` の multiline `ensure:` 差分を、どの later parser-side widening task で詰めるか
- `e21` actualization を `e1` 直後に置くか、narrow maintenance を 1 枚挟むか
- `e3` widening 前の theorem-side / formal-hook guard comparison を docs-only first choice に留めるか
