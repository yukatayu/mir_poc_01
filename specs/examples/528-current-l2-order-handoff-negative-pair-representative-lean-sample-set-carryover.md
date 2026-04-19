# 528 — current L2 order-handoff negative pair representative Lean sample-set carry-over

## 目的

Package 58 の broader theorem-side widening として、

- `p13 / p14` late-join visibility negative pair を
- helper-local static stop だけで止めず
- representative Lean sample set / committed Lean corpus に carry over できるか

を整理する。

ここで actualize するのは、
**`p13-dice-late-join-missing-publication-witness` と `p14-dice-late-join-handoff-before-publication` を representative Lean sample set / committed `samples/lean/current-l2/` corpus に carry over し、static rejection cluster の Lean bridge artifact を inspectable に保つ current cut**
である。

したがって、ここでは次を fixed しない。

- final public theorem contract
- final proof-object public schema
- final parser grammar
- final public parser / checker / runtime API
- final source-surface handoff wording

## current question

`specs/examples/527` で helper-local static stop に actualize した

- missing publication witness
- handoff before publish

を、runtime 未到達の negative pair のまま theorem-side representative sample set に carry over してよいか。

## current first line

current recommendation は次である。

1. `p13 / p14` は static rejection cluster の representative sample として representative Lean sample set に入れてよい
2. carry-over は `samples/lean/current-l2/` committed corpus と `manifest.json` まで actualize してよい
3. current guarantee は Lean 受理 + artifact well-formedness / bridge alignment に留める
4. final public theorem contract / proof object schema / final parser/public API には上げない

## actualized carry-over

| sample | static verdict | Lean carry-over reading | current role |
|---|---|---|---|
| `p13-dice-late-join-missing-publication-witness` | `Underdeclared` | `canonical_normalization_law` / `no_re_promotion` を持つ static rejection cluster として committed Lean corpus に保存する | late-join visibility line の witness 欠如 negative pair |
| `p14-dice-late-join-handoff-before-publication` | `Malformed` | `canonical_normalization_law` / `no_re_promotion` を持つ static rejection cluster として committed Lean corpus に保存する | late-join visibility line の order violation negative pair |

この carry-over により、representative Lean sample set は

`e5 / p06 / p10 / p11 / p12 / p07 / p08 / p09 / p13 / p14`

で読む current floor に上がる。

## なぜこの carry-over を許してよいか

- `specs/examples/527` ですでに helper-local static stop として source-backed になっている。
- `verification_preview.subject_kind = fixture_static_cluster` と `artifact_preview` により、theorem-side bridge floor は runtime 未到達 sample でも inspectable である。
- `crates/mir-runtime/tests/current_l2_theorem_actual_lean_execution_prototype_widening.rs` で `p13 / p14` の actual Lean execution が通るため、committed corpus に昇格しても無理がない。
- final theorem contract や final proof object schema を reopen せずに、broader theorem-side widening を 1 本閉じられる。

## evidence

- prior helper-local static stop
  - `specs/examples/527-current-l2-order-handoff-negative-static-stop-actualization.md`
- Lean export / sync route
  - `scripts/current_l2_lean_sample_sync.py`
  - `scripts/tests/test_current_l2_lean_sample_sync.py`
- theorem-side actual Lean execution regression
  - `crates/mir-runtime/tests/current_l2_theorem_actual_lean_execution_prototype_widening.rs`
- committed Lean corpus
  - `samples/lean/current-l2/p13-dice-late-join-missing-publication-witness/`
  - `samples/lean/current-l2/p14-dice-late-join-handoff-before-publication/`

## retained later

- final public theorem contract
- proof-object public schema
- final public verifier contract
- final parser/public API
- final source wording / final emitted-artifact contract
- broader theorem-side / IFC helper widening beyond this carry-over

## stop line

この package の stop line は、

- `p13 / p14` が representative Lean sample set carry-over sample として regenerate / verify できる
- committed `samples/lean/current-l2/` corpus と `manifest.json` で追える
- docs / plan / snapshot が Package 58 broader theorem-side widening を `p13 / p14` carry-over として追える

の 3 点である。
