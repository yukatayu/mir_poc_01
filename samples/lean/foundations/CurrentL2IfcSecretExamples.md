# CurrentL2IfcSecretExamples.lean

## 要約

- secret-key valid/invalid と explicit authority declassification を固定する IFC concrete example 集。

## このファイルを置く理由

- Package 56 の first-fragment を label model の定義だけで止めず、secret-key valid/invalid と explicit authority declassification を mechanization-ready な concrete example として置く。
- 生成された current-L2 sample stub と違い、このファイルは `sorry` ではなく実際に小さな証明を含む。
- ただし依然として helper-local / non-production cut に留める。目的は first mechanization-ready core を固定することであり、final public type system や verifier contract を凍らせることではない。
