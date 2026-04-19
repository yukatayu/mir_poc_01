# 0777 — witness/provider public schema coupled later gate

## Objective

shared-space line の `final public witness schema / final public provider receipt schema / combined public contract`
mixed gate を compare-only のまま残さず、
helper-local coupled-later gate として source-backed に actualize し、
docs / plan / snapshot を同期する。

## Scope and assumptions

- 規範判断の正本は `specs/` に置く。
- final public witness schema、final public provider receipt schema、delegated provider attestation、combined provider+witness public contract、final emitted-handoff contract、exhaustive shared-space catalog はこの package で固定しない。
- current default は final public witness/provider schema candidate only、combined public-contract candidate only、final emitted-handoff contract adjacent keep と読む。

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
- `specs/examples/489-current-l2-witness-provider-artifact-public-shape-actual-adoption.md`
- `specs/examples/493-current-l2-witness-provider-public-contract-and-emitted-handoff-coupled-later-gate.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`

## Actions taken

1. `489` と `493` を再読し、shared-space line の next mixed gate を `witness/provider public schema coupled later gate` として切る方針を確認した。
2. RED として `crates/mir-runtime/tests/current_l2_witness_provider_public_schema_coupled_later_gate.rs` を追加し、support helper に未実装 symbol がないことを確認した。
3. support helper に `CurrentL2SourceSampleWitnessProviderPublicSchemaCoupledLaterGate` と builder を追加し、witness schema side / provider receipt side / combined public-contract side を coupled-later gate として actualize した。
4. `p09` は delegated-rng practical chain を carry-over するのが current compare floor と整合するため、expectation をその current chain に合わせて修正した。
5. `specs/examples/499-current-l2-witness-provider-public-schema-coupled-later-gate.md` を追加し、current recommendation / retained alternatives / stop line を source-backed に記述した。
6. `Documentation.md`、`progress.md`、`tasks.md`、relevant `specs/` / `plan/` / `plan/90-source-traceability.md` を current reading へ同期した。

## Evidence / outputs / test results

- RED:
  - `cargo test -p mir-runtime --test current_l2_witness_provider_public_schema_coupled_later_gate`
  - unresolved import `CurrentL2SourceSampleWitnessProviderPublicSchemaCoupledLaterGate` / builder missing で失敗
- GREEN:
  - `cargo test -p mir-runtime --test current_l2_witness_provider_public_schema_coupled_later_gate`
  - `4 passed; 0 failed`

## What changed in understanding

- shared-space line の next mixed gate は、final public witness schema や provider receipt schema を即採ることではなく、witness/provider schema side と combined public-contract side を coupled-later gate として helper-local manifest に actualize するところまでは self-driven に進められると整理できた。
- `p09` の compare floor は authoritative-room baseline ではなく delegated-rng practical chain を carry-over する方が current repo reading と整合する、と明確になった。

## Open questions

- final public witness schema と final public provider receipt schema を same gate のまま reopen するか。
- combined provider+witness public contract を schema adoption より先に reopen するか。
- final emitted-handoff contract を shared-space public-schema adoption と同じ gate に残すか。

## Suggested next prompt

`499` を representative validation と traceability まで同期したうえで、final source-surface handoff wording / final modal foundation / final public parser-checker-runtime API のどれを next reopen candidate として narrow に actualize するかを比較し、helper-local actualization を 1 本進めてください。
