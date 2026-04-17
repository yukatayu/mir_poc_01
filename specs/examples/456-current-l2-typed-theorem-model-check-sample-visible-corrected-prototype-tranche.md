# 456 — current L2 typed/theorem/model-check sample-visible corrected prototype tranche

## 目的

typed marker (`admit` / `require` / `ensure`) を持つ corrected prototype を
`samples/prototype/` に 1 本追加し、
current theorem/model-check bridge floor が

- runtime reached
- helper-local `verification_preview`
- helper-local `artifact_preview`

でどう見えるかを sample-visible にする。

## current judgment

1. typed/theorem/model-check sample-visible corrected prototype tranche の current first cut として、
   `samples/prototype/current-l2-typed-proof-model-check/p06-typed-proof-owner-handoff` を置いてよい。
2. `p06` では、次の marker family を 1 本の runnable prototype に同居させてよい。
   - option-side `admit`
   - request-side `require`
   - request-side `ensure`
   - `atomic_cut`
   - helper-local debug output
3. `p06` の principal reading は、
   **typed marker 自体を final typed syntax として採用することではなく、
   current bridge floor が runtime reached / verifier preview / artifact preview にどう現れるかを比較すること**
   に置く。
4. current package では、
   - public typed contract
   - public theorem discharge contract
   - public model-check property language
   を増やさない。
5. current package close 後の queue は、
   `order/handoff corrected prototype third tranche`
   を next promoted self-driven package に下げてよい。

## current non-goals

- stronger typed surface promotion を行うこと
- theorem discharge transport / public theorem contract を concretize すること
- model-check settled property language / concrete tool seam を concretize すること
- final typed syntax / final proof syntax / final model-check syntax を決めること
