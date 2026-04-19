# 0785 — model-check final public-contract reopen threshold

## Objective

model-check line の remaining mixed gate をさらに 1 本 narrow にし、
`model-check checker-artifact route actual adoption` と
`model-check tool-brand / verifier-handoff coupled-later gate`
を prior floor にしたまま、
remaining final public-contract gate の reopen 順を helper-local threshold として actualize する。

## Scope and assumptions

- `specs/` を規範正本として扱う。
- first settled property language / concrete model-check tool brand / final public checker artifact / actual public checker migration / actual emitted verifier handoff artifact / production checker-runtime-policy contract / final public verifier contract は今回 fixed しない。
- representative corpus は `e5 / p06 / p07 / p09` reached、`p05` guard-only に置く。
- theorem-first external integration target を維持しつつ、model-check line は second line として narrow に actualize する。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/495-current-l2-model-check-tool-brand-and-verifier-handoff-coupled-later-gate.md`
- `specs/examples/498-current-l2-model-check-public-checker-artifact-and-migration-coupled-later-gate.md`
- `specs/examples/501-current-l2-model-check-checker-artifact-route-actual-adoption.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`

## Actions taken

1. model-check final public-contract line の current docs を再読し、`specs/examples/495` の next-line ordering を current reopen threshold として再構成した。
2. RED 先行で `crates/mir-runtime/tests/current_l2_model_check_final_public_contract_reopen_threshold.rs` を追加し、未実装 struct/builder import による compile failure を確認した。
3. `crates/mir-runtime/tests/support/current_l2_source_sample_emitted_artifact_support.rs` に
   - `CurrentL2SourceSampleModelCheckFinalPublicContractReopenThreshold`
   - `build_current_l2_source_sample_model_check_final_public_contract_reopen_threshold`
   - reopen-threshold helper 群
   を追加した。
4. `specs/examples/507-current-l2-model-check-final-public-contract-reopen-threshold.md` を新設し、current recommendation / retained alternatives / stop line / non-goal を threshold package として固定した。
5. `Documentation.md`、`tasks.md`、`progress.md`、relevant `plan/` / `specs/` / `docs/research_abstract/` を同期し、traceability addendum を追加した。

## Evidence / outputs / test results

- RED:
  - `cargo test -p mir-runtime --test current_l2_model_check_final_public_contract_reopen_threshold`
  - unresolved imports
    - `CurrentL2SourceSampleModelCheckFinalPublicContractReopenThreshold`
    - `build_current_l2_source_sample_model_check_final_public_contract_reopen_threshold`
- focused GREEN:
  - `cargo test -p mir-runtime --test current_l2_model_check_final_public_contract_reopen_threshold`
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

- model-check line の next ratchet は final public checker / verifier contract adoption ではなく、reopen order の threshold を helper-local に固定することだった。
- `e5 / p06 / p07 / p09` reached と `p05` guard-only の corpus で、property-language-and-tool-brand first / public-checker-artifact-and-migration second / verifier-handoff-and-runtime-policy-contract third / final public verifier contract fourth を current recommendation として machine-check できる。
- model-check final public-contract line でも、public checker / handoff / runtime-policy / verifier contract を collapsed single gate にしない方が current boundary と整合する。

## Open questions

- first settled property language と concrete model-check tool brand を truly paired adoption にするか、helper-local threshold の先で分けるか。
- final public checker artifact と actual public checker migration を同一 reopen で閉じるか。
- actual emitted verifier handoff artifact と production checker-runtime-policy contract を truly paired adoption にするか。
- final public verifier contract を model-check line だけで閉じるか、public checker/runtime-policy line と連動させるか。

## Suggested next prompt

`model-check final public-contract reopen threshold の次として、property-language/tool-brand pair・public-checker/migration pair・verifier-handoff/runtime-policy pair・final public verifier contract のどこから helper-local actualization を始めるかを narrow に比較し、必要なら次の package を actualize してください。`
