# Report 0826 — Package 73 IFC verifier-handoff-surface ratchet

## Objective

`specs/examples/285` と `specs/examples/286` で compare-floor に置いていた verifier-handoff-surface line を、
final parser grammar や actual emitted verifier handoff artifact に上げずに、
`run-source-sample` helper-local summary の current cut として actualize する。

## Scope and assumptions

- 規範判断の正本は `specs/` とし、今回の変更は helper-local mirror / queue sync に留める。
- actualization 対象は source-side IFC trio `p10 / p11 / p12` に限定する。
- `subject_kind` / `subject_ref` / `proof_obligation_rows`、dedicated handoff artifact family、actual emitted verifier handoff artifact、final parser grammar、final public verifier contract は still later に残す。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/285-current-l2-minimal-public-checker-boundary-ready-verifier-handoff-surface-comparison.md`
- `specs/examples/286-current-l2-verifier-handoff-surface-ready-minimal-verifier-handoff-surface-threshold.md`
- `specs/examples/544-current-l2-ifc-public-checker-boundary-threshold-helper-mirror.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `tasks.md`
- `crates/mir-runtime/src/current_l2_cli.rs`
- `crates/mir-runtime/tests/current_l2_operational_cli.rs`

## Actions taken

1. `current_l2_operational_cli` の test に verifier-handoff-surface threshold reached / pretty / guard-only / next-target assertion を追加した。
2. `run-source-sample` helper summary に `actual_verifier_handoff_surface_threshold` を追加し、`p10 / p11 / p12` reached・それ以外 guard-only の narrow cut を実装した。
3. `specs/examples/545` を追加し、current minimal bundle と deferred surface refs / stop line / retained later を文書化した。
4. `Documentation.md`、`progress.md`、`tasks.md`、`plan/`、`specs/`、`plan/90-source-traceability.md` を Package 73 close / Package 74 next queue に同期した。

## Evidence / outputs / test results

### RED -> GREEN

- RED:

```text
cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_json_reports_ifc_authority_success_checker_hint_preview -- --exact
```

失敗理由:

```text
value["actual_verifier_handoff_surface_threshold"]["status"] was Null
```

- GREEN:

```text
cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_json_reports_ifc_authority_success_checker_hint_preview -- --exact
```

結果:

```text
test result: ok. 1 passed; 0 failed
```

- Full suite:

```text
cargo test -p mir-runtime --test current_l2_operational_cli
```

結果:

```text
test result: ok. 16 passed; 0 failed
```

### CLI evidence

- reached JSON (`p10`):

```text
1016:  "actual_verifier_handoff_surface_threshold": {
1019:    "handoff_surface_kind": "docs_only_mixed_row_bundle_verifier_surface",
1020:    "public_checker_boundary_ref": "public_checker_boundary_ready_sketch",
1021:    "proof_obligation_matrix_ref": "current_l2_proof_obligation_matrix",
1022:    "handoff_artifact_mode": "docs_only_mixed_row_bundle",
1023:    "next_comparison_target_ref": "minimal_parser_subset_freeze_comparison",
```

- pretty summary (`p11`):

```text
876:actual_verifier_handoff_surface_threshold:
879:  handoff_surface_kind: docs_only_mixed_row_bundle_verifier_surface
880:  public_checker_boundary_ref: public_checker_boundary_ready_sketch
881:  proof_obligation_matrix_ref: current_l2_proof_obligation_matrix
882:  handoff_artifact_mode: docs_only_mixed_row_bundle
883:  next_comparison_target_ref: minimal_parser_subset_freeze_comparison
```

- guard-only JSON (`p06`):

```text
757:  "actual_verifier_handoff_surface_threshold": {
758:    "status": "guarded_not_reached",
775:      "guard:actual_verifier_handoff_surface_threshold_not_reached"
786:    "guard_reason": "current actual verifier handoff surface threshold only actualizes the IFC trio (`p10` / `p11` / `p12`) after actual public checker boundary threshold reaches the checker-adjacent helper floor for `p06-typed-proof-owner-handoff`"
```

- next target (`p12`):

```text
1021:  "actual_verifier_handoff_surface_threshold": {
1028:    "next_comparison_target_ref": "minimal_parser_subset_freeze_comparison",
1104:      "guard:minimal_parser_subset_freeze_comparison_next",
```

## What changed in understanding

- verifier-handoff-surface line は、actual emitted verifier handoff artifact や dedicated handoff artifact family をまだ保留したままでも、docs-only mixed-row bridge minimum を helper-local operational summary に actualize してよい段階にあることが確認できた。
- current live queue は queue zero ではなく、Package 74 minimal-parser-subset-freeze ratchet へ自然に接続できる状態である。

## Open questions

- minimal parser subset freeze の accepted cluster を stage 1 only / stage 1 + stage 2 structural floor / stage 3 cluster のどこで止めるか。
- parser-to-checker reconnect freeze を helper-local threshold に actualize する前に、reject cluster と retention floor をどこまで current cut に含めるか。
- final parser grammar、final public parser/checker API、actual emitted verifier handoff artifact、final public verifier contract をどの reopen order で later mixed gate に残すか。

## Suggested next prompt

Package 74 として minimal-parser-subset-freeze ratchet を進め、`freeze_kind + accepted_cluster_refs + reject_cluster_refs + retention_floor_refs` current cut を `run-source-sample` helper summary に actualize し、Package 75 parser-to-checker reconnect freeze へ queue を進めてください。
