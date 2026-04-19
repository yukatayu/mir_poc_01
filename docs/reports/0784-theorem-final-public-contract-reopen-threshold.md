# 0784 — theorem final public-contract reopen threshold

## Objective

theorem line の remaining mixed gate をさらに 1 本 narrow にし、
`theorem result-object route actual adoption` と
`theorem proof-object schema / prover-brand coupled-later gate`
を prior floor にしたまま、
remaining final public-contract gate の reopen 順を helper-local threshold として actualize する。

## Scope and assumptions

- `specs/` を規範正本として扱う。
- final public theorem result object / consumer-shaped theorem payload public contract / concrete theorem prover brand / proof object public schema / final public verifier contract は今回 fixed しない。
- representative corpus は `e5 / p06 / p07 / p08` reached、`p05` guard-only に置く。
- theorem-first external integration target と notebook-first theorem line は維持する。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/494-current-l2-theorem-proof-object-schema-and-prover-brand-coupled-later-gate.md`
- `specs/examples/497-current-l2-theorem-result-object-and-payload-public-contract-coupled-later-gate.md`
- `specs/examples/500-current-l2-theorem-result-object-route-actual-adoption.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`

## Actions taken

1. theorem final public-contract line の current docs を再読し、unordered mixed gate のまま残っていた reopen 順を `result-object and payload first` / `prover-brand and proof-schema second` / `final public verifier contract third` に切り分けた。
2. RED 先行で `crates/mir-runtime/tests/current_l2_theorem_final_public_contract_reopen_threshold.rs` を使い、未実装 struct/builder import による compile failure を確認した。
3. `crates/mir-runtime/tests/support/current_l2_source_sample_emitted_artifact_support.rs` に
   - `CurrentL2SourceSampleTheoremFinalPublicContractReopenThreshold`
   - `build_current_l2_source_sample_theorem_final_public_contract_reopen_threshold`
   - reopen-threshold helper 群
   を追加した。
4. `specs/examples/506-current-l2-theorem-final-public-contract-reopen-threshold.md` を新設し、current recommendation / retained alternatives / stop line / non-goal を threshold package として固定した。
5. `Documentation.md`、`tasks.md`、`progress.md`、relevant `plan/` / `specs/` / `docs/research_abstract/` を同期し、traceability addendum を追加した。

## Evidence / outputs / test results

- RED:
  - `cargo test -p mir-runtime --test current_l2_theorem_final_public_contract_reopen_threshold`
  - unresolved imports
    - `CurrentL2SourceSampleTheoremFinalPublicContractReopenThreshold`
    - `build_current_l2_source_sample_theorem_final_public_contract_reopen_threshold`
- focused GREEN:
  - `cargo test -p mir-runtime --test current_l2_theorem_final_public_contract_reopen_threshold`
  - `5 passed`
- repo-wide validation:
  - `cargo test -p mir-runtime`
- regression:
  - `python3 scripts/current_l2_source_sample_regression.py regression`
- docs validation:
  - `python3 scripts/validate_docs.py`
  - rerun after adding this report
- diff hygiene:
  - `git diff --check`

## What changed in understanding

- theorem final public-contract line の next ratchet は final public theorem contract adoption ではなく、reopen order の threshold を helper-local に固定することだった。
- `e5 / p06 / p07 / p08` reached と `p05` guard-only の corpus で、result-object-and-payload first / prover-brand-and-proof-schema second / final public verifier contract third を current recommendation として machine-check できる。
- final public verifier contract を theorem result object / payload / proof-schema / prover-brand と collapsed single gate にしない方が current theorem boundary と整合する。

## Open questions

- final public theorem result object と consumer-shaped theorem payload public contract を truly paired adoption にするか、helper-local threshold の先で分けるか。
- concrete theorem prover brand と proof object public schema を同一 reopen で閉じるか。
- final public verifier contract を theorem line だけで閉じるか、public checker/runtime-policy line と連動させるか。
- theorem result object / payload / proof object / verifier を unified public contract として束ねる必要が本当にあるか。

## Suggested next prompt

`theorem final public-contract reopen threshold の次として、result-object/payload pair・prover-brand/proof-schema pair・final public verifier contract のどこから helper-local actualization を始めるかを narrow に比較し、必要なら次の package を actualize してください。`
