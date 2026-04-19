# Phase 5 要約 — small decidable core と proof boundary

## この phase の役割

Phase 5 は、**何を local / structural / decidable core に残し、何を theorem / protocol / runtime-policy 側へ送るか** を切る phase である。

## 固まった current reading

- current 4-way split:
  - `core_static_checker`
  - `theorem_prover_boundary`
  - `protocol_verifier_boundary`
  - `runtime_policy_boundary`
- theorem-first / model-check-second / runtime-policy-later の順を current practical reading に置く。
- typed-core は full calculus first ではなく checker-adjacent carrier first に置く。
- theorem-side は semantic-core invariant family first に置く。
- model-check は row-local carrier first に置く。
- `atomic_cut` は local finalizing cut の nucleus に留め、higher-level order / witness / replay / fairness line は外に残す。
- `program_order / dependency_order / publication_order / observation_order / witness_order / finalization_order / scoped_happens_before` は current working relation family として整理済みである。

## source-backed evidence

- current checker-side package chain
- theorem notebook / review-unit pilot
- row-local model-check carrier
- order / cut / modality comparison bundle

## まだここで決めていないこと

- stronger typed surface adoption
- theorem discharge transport / public theorem contract
- model-check first settled property language
- concrete theorem prover / model-check tool binding
- final ordering / handoff source surface
- final modal foundation adoption

## 次へ渡したもの

Phase 6 は、この boundary を actual code path と sample-visible evidence に narrow に接続する。
一方で、残りの twin peaks は `plan/18` の theory-lab program として
- current first-line / stop-line integration までは self-driven に進め、
- actual adoption / concrete binding / final public contract は mixed gate に残す。
