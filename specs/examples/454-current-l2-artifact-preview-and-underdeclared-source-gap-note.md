# 454 — current L2 artifact preview and underdeclared source gap note

## 目的

current runnable sample / prototype を使った研究で、
theorem / model-check bridge の current floor をもう一段 sample-visible にしつつ、
underdeclared family の source-form gap を current convenience cut の範囲で明確化する。

## current judgment

1. current operational CLI は、`verification_preview` に加えて
   `artifact_preview` を helper-local に出してよい。
2. `artifact_preview` では少なくとも次を表示してよい。
   - proof notebook review unit preview
     - `obligation_kind`
     - `goal_text`
     - symbolic `evidence_refs`
   - model-check concrete carrier preview
     - `obligation_kind`
     - symbolic `evidence_refs`
3. current `artifact_preview` は、
   current helper route から読める derived preview であり、
   final public theorem/model-check contract や final emitted artifact schema ではない。
4. current preview は少なくとも次の 3 状態を区別してよい。
   - runtime try/cut cluster reached
   - fixture static cluster reached
   - guarded not reached
   - guarded case では `artifact_preview` block 自体は出してよいが、row は empty list に留める。
5. underdeclared family については、fixture-level source of truth は already あるが、
   current source parser / lowerer convenience cut は
   - lineage assertion omission
   - declared target omission
   を source text としてまだ受けない。
6. したがって current source-form stimulus は
   `samples/not_implemented/current-l2-underdeclared/`
   に preservation してよい。
   これは parser widening adoption queue の即時昇格を意味しない。

## current non-goals

- final public verifier JSON contract を決めること
- proof notebook review unit / model-check carrier の final shell schema を決めること
- underdeclared source-form widening を current authored `samples/current-l2/` へ hidden promotion すること
- current parser convenience cut を final grammar adoption と読むこと
