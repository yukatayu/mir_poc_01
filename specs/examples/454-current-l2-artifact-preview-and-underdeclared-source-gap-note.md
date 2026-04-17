# 454 — current L2 artifact preview helper cut

## 目的

current runnable sample / prototype を使った研究で、
theorem / model-check bridge の current floor をもう一段 sample-visible にする。

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
## current non-goals

- final public verifier JSON contract を決めること
- proof notebook review unit / model-check carrier の final shell schema を決めること
- current parser convenience cut を final grammar adoption と読むこと
