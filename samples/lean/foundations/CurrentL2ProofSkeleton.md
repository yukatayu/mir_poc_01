# CurrentL2ProofSkeleton.lean

## 要約

- review-unit と Lean-stub の整合を固定する mechanization-ready proof-obligation skeleton。

## このファイルを置く理由

- Package 57 の最初の actual Lean fragment である。repo-local review-unit to Lean-stub bridge の構造的 fact を証明し、domain obligation が解けたとは主張せず mechanization-ready carrier の shape を固定する。
- valid pattern がなぜ通るか、invalid pattern がなぜ witness を持てないかを、sample-facing に追いやすい小さな補題と example で固定する。
- 生成された current-L2 sample stub と違い、このファイルは `sorry` ではなく実際に小さな証明を含む。
- ただし依然として helper-local / non-production cut に留める。目的は first mechanization-ready core を固定することであり、final public type system や verifier contract を凍らせることではない。
