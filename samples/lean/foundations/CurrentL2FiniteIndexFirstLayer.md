# CurrentL2FiniteIndexFirstLayer.lean

## 要約

- finite-index first layer の capture/lifetime/cost を固定する小さな proof fragment。

## このファイルを置く理由

- Package 93 の Lean-first hardening として、finite decidable index fragment を IFC だけでなく capture / lifetime / simple cost まで小さな自己完結 proof として置く。ここでは final typed calculus を与えず、first strong typing sample set を支える 最小 preorder / subset / budget fact だけを mechanization-ready に固定する。
- 生成された current-L2 sample stub と違い、このファイルは `sorry` ではなく実際に小さな証明を含む。
- ただし依然として helper-local / non-production cut に留める。目的は first mechanization-ready core を固定することであり、final public type system や verifier contract を凍らせることではない。
