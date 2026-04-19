# 0783 — witness/provider final public-contract reopen threshold

## Objective

shared-space line の remaining mixed gate をさらに 1 本 narrow にし、
`witness/provider public-contract / emitted-handoff coupled-later gate` と
`witness/provider schema route actual adoption`
を prior floor にしたまま、
remaining final public-contract gate の reopen 順を helper-local threshold として actualize する。

## Scope and assumptions

- `specs/` を規範正本として扱う。
- final public witness schema / final public provider receipt schema / delegated provider attestation / combined provider+witness public contract / final emitted-handoff contract は今回 fixed しない。
- representative corpus は `p07 / p08 / p09` reached、`p05` guard-only に置く。
- order-handoff final source wording は shared-space final public-contract reopen threshold に collapse しない。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/493-current-l2-witness-provider-public-contract-and-emitted-handoff-coupled-later-gate.md`
- `specs/examples/499-current-l2-witness-provider-public-schema-coupled-later-gate.md`
- `specs/examples/504-current-l2-witness-provider-schema-route-actual-adoption.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`

## Actions taken

1. shared-space final public-contract line の current docs を再読し、unordered mixed gate のまま残っていた reopen 順を `public-schema pair first` / `delegated attestation + combined public-contract second` / `final emitted-handoff contract third` に切り分けた。
2. RED 先行で `crates/mir-runtime/tests/current_l2_witness_provider_final_public_contract_reopen_threshold.rs` を追加し、未実装 struct/builder import による compile failure を確認した。
3. `crates/mir-runtime/tests/support/current_l2_source_sample_emitted_artifact_support.rs` に
   - `CurrentL2SourceSampleWitnessProviderFinalPublicContractReopenThreshold`
   - `build_current_l2_source_sample_witness_provider_final_public_contract_reopen_threshold`
   - reopen-threshold helper 群
   を追加した。
4. `specs/examples/505-current-l2-witness-provider-final-public-contract-reopen-threshold.md` を新設し、current recommendation / retained alternatives / stop line / non-goal を threshold package として固定した。
5. `Documentation.md`、`tasks.md`、`progress.md`、relevant `plan/` / `specs/` / `docs/research_abstract/` を同期し、traceability addendum を追加した。

## Evidence / outputs / test results

- RED:
  - `cargo test -p mir-runtime --test current_l2_witness_provider_final_public_contract_reopen_threshold`
  - unresolved imports
    - `CurrentL2SourceSampleWitnessProviderFinalPublicContractReopenThreshold`
    - `build_current_l2_source_sample_witness_provider_final_public_contract_reopen_threshold`
- focused GREEN:
  - `cargo test -p mir-runtime --test current_l2_witness_provider_final_public_contract_reopen_threshold`
  - `4 passed`
- repo-wide validation:
  - `cargo test -p mir-runtime`
  - rerun after docs sync
- regression:
  - `python3 scripts/current_l2_source_sample_regression.py regression`
- docs validation:
  - `python3 scripts/validate_docs.py`
  - rerun after adding this report
- diff hygiene:
  - `git diff --check`

## What changed in understanding

- shared-space final public-contract line の next ratchet は final public contract adoption ではなく、reopen order の threshold を helper-local に固定することだった。
- `p07 / p08 / p09` reached と `p05` guard-only の corpus で、public-schema pair first / delegated-attestation-and-combined-contract second / final emitted-handoff contract third を current recommendation として machine-check できる。
- order-handoff final source wording を shared-space final public-contract reopen threshold と同じ package に collapse しない方が current boundary と整合する。

## Open questions

- final public witness schema と final public provider receipt schema を truly paired adoption にするか、helper-local threshold の先で分けるか。
- delegated provider attestation と combined provider+witness public contract を同一 reopen で閉じるか。
- final emitted-handoff contract を shared-space line だけで閉じるか、order-handoff emitted-artifact schema mixed gate と連動させるか。
- exhaustive shared-space catalog を final public contract adoption とどこまで切り分けるか。

## Suggested next prompt

`witness/provider final public-contract reopen threshold の次として、public-schema pair / delegated attestation + combined public-contract / final emitted-handoff contract のどこから helper-local actualization を始めるかを narrow に比較し、必要なら次の package を actualize してください。`
