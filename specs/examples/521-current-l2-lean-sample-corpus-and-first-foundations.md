# 521 — current L2 Lean sample corpus and first foundations

## 目的

`representative Lean sample set actual Lean execution` 到達後の next closeout debt として、

- `samples/lean/` に何を置くか
- その Lean evidence をどう読むか
- Package 56 / 57 の first actual fragment をどこで止めるか

を source-backed に固定する。

ここでの主眼は final public theorem contract や final proof object schema ではない。
repo-local actual evidence を inspectable に保ち、helper-local mechanization floor を current package として閉じることである。

## current recommendation

### Lean sample corpus split

current repo は `samples/lean/` を次の 2 bucket に分けて保持してよい。

1. `samples/lean/foundations/`
   - actual small proofs を持つ mechanization-ready core
   - current package では
     - `CurrentL2LabelModel.lean`
     - `CurrentL2IfcSecretExamples.lean`
     - `CurrentL2ProofSkeleton.lean`
     を置く
2. `samples/lean/clean-near-end/`
   - active clean-near-end suite に対する generated Lean theorem stub
   - current package では
     - typing family `01..05`
     - order-handoff family `01..06`
     - model-check family `01..03`
     - modal family `01..02`
     を置く

### proof-strength wording

- `foundations/` は actual small proofs を含む。
  - これは IFC / label-model first fragment、secret-key valid/invalid concrete example、proof-skeleton / obligation-shape first fragment を current mechanization-ready core として読むための floor である。
- `clean-near-end/` は generated Lean stub を含む。
  - Lean 実行は success するが、theorem body は `sorry` を含む。
  - したがって current guarantee は
    - artifact well-formedness
    - review-unit to Lean-stub bridge alignment
    - representative sample attached theorem text の inspectability
    までである。
  - これは final theorem discharge、final public theorem contract、final proof object schema を意味しない。

### helper-local actualization route

current helper-local route として、次を repo-local helper に actualize してよい。

- `lean-toolchain`
  - repo-local toolchain version pin
- `crates/mir-runtime/examples/current_l2_emit_theorem_lean_bundle.rs`
  - representative sample / prototype から Lean bundle を出す helper
- `scripts/current_l2_lean_sample_sync.py`
  - `samples/lean/` の committed corpus を regenerate し、Lean verification を通す helper

この route は helper-local / non-production / examples-support cut に留める。
final public theorem tool API や final public verifier contract に昇格させない。

## Package 56 / 57 への反映

### Package 56 — layered strong typing / IFC first-fragment

current package では、IFC を docs-only sketch に留めず、
`CurrentL2LabelModel.lean` と `CurrentL2IfcSecretExamples.lean` により

- two-point label lattice
- `flowsTo`
- `join`
- explicit authority-sensitive declassification reading
- secret-key valid/invalid concrete example
- authorized fingerprint release と unauthorized release impossibility

を actual Lean fragment として置いてよい。

ただし、これは

- final typed source principal
- final IFC syntax
- covert-channel theorem
- distributed trust finalization
- final label API

を意味しない。

### Package 57 — Lean formal skeleton / proof obligations

current package では、`CurrentL2ProofSkeleton.lean` により

- review unit
- emitted Lean stub
- emission preserves subject / obligation identity
- representative `e5` count / first-entry preservation

を actual Lean fragment として置いてよい。

さらに `samples/lean/clean-near-end/` により、
active clean-near-end suite の current generated theorem text を repo 内に保存してよい。

この package により fixed されるのは

- mechanization-ready core の first slice
- Rust/Lean alignment floor の inspectable sample corpus

までである。

## retained later line

- secret-key / IFC valid-invalid source sample corpus
- declassification / endorsement richer authority carriers
- theorem discharge actual proof scripts
- final public theorem result object
- consumer-shaped theorem payload public contract
- concrete theorem prover brand
- proof object public schema
- final public verifier contract

## stop line

この package は、

- actual small proofs を含む first foundation fragment
- representative Lean sample set generated Lean corpus
- `sorry` を含む generated stub と actual proof fragment の読み分け

までを close する。

それ以上の theorem discharge completion や public contract adoption は still later に残す。
