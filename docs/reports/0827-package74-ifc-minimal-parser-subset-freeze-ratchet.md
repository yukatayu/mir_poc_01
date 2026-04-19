# Report 0827 — Package 74 IFC minimal-parser-subset-freeze ratchet

## Objective

`specs/examples/287` と `specs/examples/288` で compare-floor に置いていた minimal parser subset freeze line を、
final parser grammar や parser-to-checker reconnect freeze に上げずに、
`run-source-sample` helper-local summary の current cut として actualize する。

## Scope and assumptions

- 規範判断の正本は `specs/` とし、今回の変更は helper-local mirror / queue sync に留める。
- actualization 対象は source-side IFC trio `p10 / p11 / p12` に限定する。
- stage 3 widening / public parser API / final parser grammar / parser-to-checker reconnect freeze / final public parser-checker API / final public verifier contract は still later に残す。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/287-current-l2-minimal-verifier-handoff-surface-ready-minimal-parser-subset-freeze-comparison.md`
- `specs/examples/288-current-l2-minimal-parser-subset-freeze-ready-minimal-parser-subset-freeze-threshold.md`
- `specs/examples/545-current-l2-ifc-verifier-handoff-surface-threshold-helper-mirror.md`
- `docs/reports/0605-phase3-minimal-parser-subset-freeze-package.md`
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

1. `current_l2_operational_cli` の test に minimal-parser-subset-freeze threshold reached / pretty / guard-only / next-target assertion を追加した。
2. `run-source-sample` helper summary に `actual_minimal_parser_subset_freeze_threshold` を追加し、`p10 / p11 / p12` reached・それ以外 guard-only の narrow cut を実装した。
3. `specs/examples/546` を追加し、current minimum bundle と stop line / retained later を文書化した。
4. `Documentation.md`、`progress.md`、`tasks.md`、`plan/`、`specs/`、`plan/90-source-traceability.md` を Package 74 close / Package 75 next queue に同期した。

## Evidence / outputs / test results

### RED -> GREEN

- RED:

```text
cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_json_reports_ifc_authority_success_checker_hint_preview -- --exact
```

失敗理由:

```text
value["actual_minimal_parser_subset_freeze_threshold"]["status"] was Null
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
1114:  "actual_minimal_parser_subset_freeze_threshold": {
1117:    "freeze_kind": "stage1_stage2_structural_parser_floor",
1118:    "accepted_cluster_refs": [
1122:    "reject_cluster_refs": [
1127:    "retention_floor_refs": [
1132:    "next_comparison_target_ref": "parser_to_checker_reconnect_freeze_comparison",
```

- pretty summary (`p11`):

```text
968:actual_minimal_parser_subset_freeze_threshold:
971:  freeze_kind: stage1_stage2_structural_parser_floor
972:  accepted_cluster_refs:
975:  reject_cluster_refs:
979:  retention_floor_refs:
983:  next_comparison_target_ref: parser_to_checker_reconnect_freeze_comparison
```

- guard-only JSON (`p06`):

```text
788:  "actual_minimal_parser_subset_freeze_threshold": {
789:    "status": "guarded_not_reached",
805:      "guard:actual_minimal_parser_subset_freeze_threshold_not_reached"
817:    "guard_reason": "current actual minimal parser subset freeze threshold only actualizes the IFC trio (`p10` / `p11` / `p12`) after actual verifier handoff surface threshold reaches the helper-local docs-only bridge floor for `p06-typed-proof-owner-handoff`"
```

- next target (`p12`):

```text
1119:  "actual_minimal_parser_subset_freeze_threshold": {
1137:    "next_comparison_target_ref": "parser_to_checker_reconnect_freeze_comparison",
1206:      "guard:parser_to_checker_reconnect_freeze_comparison_next",
```

## What changed in understanding

- minimal parser subset freeze line は、stage 3 widening / public parser API / final parser grammar をまだ保留したままでも、stage 1 + stage 2 structural parser floor minimum を helper-local operational summary に actualize してよい段階にあることが確認できた。
- current live queue は queue zero ではなく、Package 75 parser-to-checker reconnect freeze ratchet へ自然に接続できる状態である。

## Open questions

- parser-to-checker reconnect freeze の current minimum を stage 1 summary / stage 2 structural contract / retained helper refs のどこまでに留めるか。
- stage 3 reopen を declaration-side `admit` / request clause suite / predicate fragment のどれから再開するか。
- final parser grammar、final public parser/checker API、final public verifier contract をどの reopen order で later mixed gate に残すか。

## Suggested next prompt

Package 75 として parser-to-checker reconnect freeze ratchet を進め、stage 1 summary と stage 2 structural contract を checker floor へ reconnect する current cut を `run-source-sample` helper summary に actualize してください。
