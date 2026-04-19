# CurrentL2LabelModel.lean

## 要約

- 明示的 authority-sensitive declassification lemma を持つ two-point IFC label model。

## このファイルを置く理由

- Package 56 の最初の actual Lean fragment である。final source syntax は出さず、checker-adjacent IFC line が依拠する 最小 label semantics と authority-sensitive fact を固定する。
- 生成された current-L2 sample stub と違い、このファイルは `sorry` ではなく実際に小さな証明を含む。
- ただし依然として helper-local / non-production cut に留める。目的は first mechanization-ready core を固定することであり、final public type system や verifier contract を凍らせることではない。
