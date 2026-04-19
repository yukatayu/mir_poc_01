# 0771 — witness/provider public-contract and emitted-handoff coupled later gate

## Objective

shared-space / order-handoff line の `final public witness/provider/artifact contract` と
`final emitted-handoff contract` mixed gate を compare-only のまま残さず、
helper-local coupled-later gate として source-backed に actualize し、
docs / plan / snapshot を同期する。

## Scope and assumptions

- 規範判断の正本は `specs/` に置く。
- final public witness schema、final public provider receipt schema、delegated provider attestation、combined provider+witness public contract、final emitted-handoff contract、final source-surface handoff wording、exhaustive shared-space catalogはこの package で固定しない。
- current default は claim/payload split first、witness/provider route non-collapse、repo-local emitted artifact refs first、combined public contract later、final emitted-handoff contract later と読む。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/471-current-l2-authoritative-room-vertical-slice-emitted-artifact-ratchet.md`
- `specs/examples/483-current-l2-witness-provider-artifact-public-shape-threshold-default.md`
- `specs/examples/489-current-l2-witness-provider-artifact-public-shape-actual-adoption.md`
- `specs/examples/490-current-l2-order-handoff-surface-actual-adoption.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`

## Actions taken

1. `489` と `490` を再読し、shared-space public-shape actual adoption と order-handoff surface actual adoption の次 line を、final adoption ではなく coupled-later gate として切る方針を確認した。
2. RED として `crates/mir-runtime/tests/current_l2_witness_provider_emitted_contract_coupled_later_gate.rs` を追加し、support helper に未実装 symbol がないことを確認した。
3. support helper に `CurrentL2SourceSampleWitnessProviderEmittedContractCoupledLaterGate` と builder を追加し、witness/provider public-contract side と emitted-handoff contract side を adjacent but distinct later gate として actualize した。
4. `p09` compare-floor expectation に drift が出たため、delegated-provider practical line を principal に保ち、authoritative-room vertical-slice compare floor を carry しない読みへテスト expectation を修正した。
5. `specs/examples/493-current-l2-witness-provider-public-contract-and-emitted-handoff-coupled-later-gate.md` を追加し、current recommendation / retained alternatives / stop line を source-backed に記述した。
6. `Documentation.md`、`progress.md`、`tasks.md`、relevant `specs/` / `plan/` / `docs/research_abstract/` / `plan/90-source-traceability.md` を current reading へ同期した。

## Evidence / outputs / test results

- RED:
  - `cargo test -p mir-runtime --test current_l2_witness_provider_emitted_contract_coupled_later_gate`
  - unresolved import `CurrentL2SourceSampleWitnessProviderEmittedContractCoupledLaterGate` / builder missing で失敗
- intermediate:
  - 同 test で `p09` compare-floor expectation drift を検出
- GREEN:
  - `cargo test -p mir-runtime --test current_l2_witness_provider_emitted_contract_coupled_later_gate`
  - `4 passed; 0 failed`

## What changed in understanding

- shared-space / order-handoff line の次 mixed gate は、final public contract を即採ることではなく、public-contract side と emitted-handoff side を `adjacent but distinct later gate` として helper-local manifest に actualize するところまでは self-driven に進められると整理できた。
- `p09` は delegated-provider practical line が compare floor の principal であり、authoritative-room vertical-slice compare floor を carry しない方が source-backed reading と整合する、と明確になった。

## Open questions

- final public witness schema と final public provider receipt schema を別 gate のまま reopen するか。
- combined provider+witness public contract と final emitted-handoff contract を同時に reopen するか。
- final source-surface handoff wording を shared-space public-contract gate と分けて扱い続けるか。

## Suggested next prompt

`493` を representative validation と traceability まで同期したうえで、theorem final public result/proof-object gate と model-check final public checker/tool-brand gate のどちらを次に reopen するかを narrow に比較し、helper-local actualization を 1 本進めてください。`
