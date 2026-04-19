# Report 0828 — Package 75 IFC parser-to-checker-reconnect-freeze ratchet

## Objective

`specs/examples/289` と `specs/examples/290` で compare-floor に置いていた parser-to-checker reconnect freeze line を、
public parser/checker API や runtime/proof boundary actualization に上げずに、
`run-source-sample` helper-local summary の current cut として actualize する。

## Scope and assumptions

- 規範判断の正本は `specs/` とし、今回の変更は helper-local mirror / queue sync に留める。
- actualization 対象は source-side IFC trio `p10 / p11 / p12` に限定する。
- stage 3 reconnect / `e19` redesign / runtime contrast `E21 / E22` / final parser grammar / final public parser-checker API / actual external verifier schema / final public verifier contract は still later に残す。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/289-current-l2-minimal-parser-subset-freeze-ready-parser-to-checker-reconnect-freeze-comparison.md`
- `specs/examples/290-current-l2-parser-to-checker-reconnect-freeze-ready-minimal-parser-to-checker-reconnect-freeze-threshold.md`
- `specs/examples/546-current-l2-ifc-minimal-parser-subset-freeze-threshold-helper-mirror.md`
- `docs/reports/0606-phase3-parser-to-checker-reconnect-freeze-package.md`
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

1. `current_l2_operational_cli` の test に parser-to-checker reconnect freeze threshold reached / pretty / guard-only / next-target assertion を追加した。
2. `run-source-sample` helper summary に `actual_parser_to_checker_reconnect_freeze_threshold` を追加し、`p10 / p11 / p12` reached・それ以外 guard-only の narrow cut を実装した。
3. `specs/examples/547` を追加し、current reconnect minimum と stop line / retained later を文書化した。
4. `Documentation.md`、`progress.md`、`tasks.md`、`plan/`、`specs/`、`plan/90-source-traceability.md` を Package 75 close / Package 76 next queue に同期した。

## Evidence / outputs / test results

### RED -> GREEN

- RED:

```text
cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_json_reports_ifc_authority_success_checker_hint_preview -- --exact
```

失敗理由:

```text
value["actual_parser_to_checker_reconnect_freeze_threshold"]["status"] was Null
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
1217:  "actual_parser_to_checker_reconnect_freeze_threshold": {
1220:    "reconnect_kind": "stage1_stage2_checker_floor_bridge",
1221:    "parser_subset_freeze_ref": "minimal_parser_subset_freeze_ready_sketch",
1222:    "checker_floor_refs": [
1226:    "retained_helper_refs": [
1231:    "next_comparison_target_ref": "phase1_semantics_closeout_comparison",
```

- pretty summary (`p11`):

```text
1063:actual_parser_to_checker_reconnect_freeze_threshold:
1066:  reconnect_kind: stage1_stage2_checker_floor_bridge
1067:  parser_subset_freeze_ref: minimal_parser_subset_freeze_ready_sketch
1068:  checker_floor_refs:
1071:  retained_helper_refs:
1075:  next_comparison_target_ref: phase1_semantics_closeout_comparison
```

- guard-only JSON (`p06`):

```text
819:  "actual_parser_to_checker_reconnect_freeze_threshold": {
820:    "status": "guarded_not_reached",
836:      "guard:actual_parser_to_checker_reconnect_freeze_threshold_not_reached"
847:    "guard_reason": "current actual parser-to-checker reconnect freeze threshold only actualizes the IFC trio (`p10` / `p11` / `p12`) after actual minimal parser subset freeze threshold reaches the stage1+stage2 parser floor for `p06-typed-proof-owner-handoff`"
```

- next target (`p12`):

```text
1222:  "actual_parser_to_checker_reconnect_freeze_threshold": {
1236:    "next_comparison_target_ref": "phase1_semantics_closeout_comparison",
1310:      "guard:phase1_semantics_closeout_comparison_next",
```

## What changed in understanding

- parser-to-checker reconnect line は、stage 3 reconnect や runtime/proof boundary actualization をまだ保留したままでも、stage 1 summary + stage 2 structural contract minimum を helper-local operational summary に actualize してよい段階にあることが確認できた。
- current live queue は queue zero ではなく、Package 76 phase1-semantics-closeout ratchet へ自然に接続できる状態である。

## Open questions

- Phase 1 semantics closeout の current minimum を semantics / invariant bridge / notation status のどこまでに留めるか。
- notation drift correction と proof-obligation wording bridge をどこまで current cut にまとめるか。
- final parser grammar、final type system、actual external verifier schema、final public verifier contract をどの reopen order で later mixed gate に残すか。

## Suggested next prompt

Package 76 として phase1-semantics-closeout ratchet を進め、semantics / invariant bridge / notation status の narrow closeout bundle を `run-source-sample` helper summary に actualize してください。
