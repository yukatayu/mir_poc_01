# Report 2129 - G1 Diagnostic Carrier Inventory

- Date: 2026-07-03 21:59 JST
- Author / agent: Codex
- Scope: LAB-only diagnostic carrier inventory for OBL-024/025 prerequisites
- Decision levels touched: L1/L2 canon consulted, no canon edit; LAB repository memory updated

## Objective

Inventory the gap between the canon Diagnostic carrier and current LAB
diagnostic evidence, especially for E-ROW-shaped Surface elaboration
diagnostics, without implementing a final diagnostic ABI, stating/proving
OBL-024/025, claiming explanation soundness/completeness, claiming
conformance, or editing canon.

## Scope and assumptions

The normative source is `mirrorea_canon/`. This task edits only LAB repository
memory, snapshot docs, validator path lists, and this report.

Working assumption: a docs-only inventory is the right next step before any
additive carrier prototype or helper diagnostic ID split. Code/test changes
would be a separate package because they would start defining executable API
shape.

## Start state / dirty state

The task started from clean `main...origin/main` at commit `c4b36820 Add G1
EROW diagnostic alignment`. A new Discord baseline was recorded with
`discord_notify.py begin --cwd .`.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `AGENTS.md`
- `mirrorea_canon/theory/10-diagnostics.md`
- `mirrorea_canon/spec/07-diagnostics-format.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`
- `plan/79-g1-erow-diagnostic-alignment.md`
- `crates/mir-ast/src/textual_alpha.rs`
- `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- `crates/mir-semantics/examples/surface_to_core_elaborate.rs`
- `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- `scripts/surface_mir_samples.py`
- `samples/full-system-v1-surface/elaboration/elab-04-undeclared-generated-failure-negative/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/elab-10-visibility-failure-row-negative/expected/elaboration.json`

## Actions taken

- Added `plan/80-g1-diagnostic-carrier-inventory.md`.
- Listed canon Diagnostic fields required by `theory/10` and `spec/07`.
- Inventoried current LAB `TextualMirDiagnostic` as `code/message/span`.
- Inventoried Surface elaboration evidence around diagnostics, remote request
  failure rows, source-span sidecars, helper raw JSON, helper projection, and
  expected JSON rows.
- Recorded the E-ROW-specific gap between current helper-local
  `generated_failure_not_declared` and future canon-shaped diagnostic carrier.
- Recorded a candidate additive carrier shape for a later implementation
  package, explicitly non-final.
- Updated plan index, source traceability, snapshot docs, and validator path
  lists.
- Integrated read-only sub-agent review, especially the distinction between
  raw helper JSON, projection summaries, expected JSON, and source-span sidecar
  evidence.

## Files changed

- `Documentation.md`
- `plan/00-index.md`
- `plan/80-g1-diagnostic-carrier-inventory.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `scripts/README.md`
- `scripts/check_source_hierarchy.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/validate_docs.py`
- `tasks.md`
- `docs/reports/2129-g1-diagnostic-carrier-inventory.md`

No code, sample, Lean, manifest, `samples_progress.md`, or `mirrorea_canon/`
file was edited.

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
sed -n '1,180p' mirrorea_canon/theory/10-diagnostics.md
sed -n '1,120p' mirrorea_canon/spec/07-diagnostics-format.md
sed -n '1,80p' crates/mir-ast/src/textual_alpha.rs
rg -n "struct .*Diagnostic|Diagnostic \\{|diagnostic\\(|diagnostic_codes|span|message|code|suggested|failed_premise|rule_instance|missing_evidence|refs" crates/mir-ast/src crates/mir-semantics/src scripts/surface_mir_samples.py samples/full-system-v1-surface/elaboration/elab-04-undeclared-generated-failure-negative/expected/elaboration.json samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/expected/elaboration.json samples/full-system-v1-surface/elaboration/elab-10-visibility-failure-row-negative/expected/elaboration.json
jq '{diagnostic_codes, remote_request_summaries, source_span_entity_kinds, obligation_codes}' samples/full-system-v1-surface/elaboration/elab-04-undeclared-generated-failure-negative/expected/elaboration.json
jq '{diagnostic_codes, remote_request_summaries, source_span_entity_kinds, obligation_codes}' samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/expected/elaboration.json
jq '{diagnostic_codes, remote_request_summaries, source_span_entity_kinds, obligation_codes}' samples/full-system-v1-surface/elaboration/elab-10-visibility-failure-row-negative/expected/elaboration.json
date '+%Y-%m-%d %H:%M %Z'
```

Post-report validation commands are listed in the evidence section.

## Evidence / outputs / test results

The package is docs-only. No Rust, Python helper, Lean, sample matrix, expected
JSON, or manifest behavior changed.

Post-report validation:

```bash
python3 scripts/check_source_hierarchy.py
# required: 576
# present: 576
# missing: 0

python3 scripts/validate_docs.py
# Documentation scaffold looks complete.
# Found 1281 numbered report(s).

python3 -m unittest scripts.tests.test_validate_docs
# Ran 20 tests in 0.349s
# OK

git diff --check
# exit 0
```

## What changed in understanding

The canon carrier requires `id`, `severity`, file/range/line-column span,
`rule_instance`, `failed_premise`, `missing_evidence[]`,
`suggested_repair[]`, and `refs[]`. Current LAB diagnostic evidence has only a
smaller core directly attached to diagnostics, while some future carrier inputs
exist separately in remote request rows and source-span sidecars.

The important gap is not only "rename diagnostic code". It is a carrier and
replay-evidence gap: rule instance, failed premise with bindings, missing
evidence, repair, refs, and multi-span declaration/use split are not present in
the current helper diagnostic object.

## Open questions

- Should future helper JSON expose both `legacy_code` and `canon_id` during
  migration?
- Which rule-instance vocabulary should be used for Surface elaboration
  failures before final theorem statements exist?
- How should declaration-site and use-site spans be represented without
  freezing final JSON shape too early?
- Should E-ROW repair rows be added before broader diagnostic families are
  inventoried?

## Suggested next prompt

自走で E-ROW additive diagnostic carrier prototype を実装するか、先に
OBL-024 statement-shape inventory を docs-only で進めるかを比較し、安全な方から
進めてください。final diagnostic ABI、proof discharge、conformance、G1 exit は
主張しないでください。

## Plan update status

更新済み:

- `plan/00-index.md`
- `plan/80-g1-diagnostic-carrier-inventory.md`
- `plan/90-source-traceability.md`

## Documentation.md update status

更新済み: Surface Mir line now mentions diagnostic carrier inventory as LAB
repository memory, not diagnostic ABI freeze, OBL-024/025 discharge, proof
discharge, or G1 exit.

## progress.md update status

更新済み: Added the current diagnostic carrier inventory note and recent log
entry.

## tasks.md update status

更新済み: Added diagnostic carrier inventory to current holding state and
updated next candidates to additive E-ROW carrier prototype / OBL-024
statement-shape inventory.

## samples_progress.md update status

`samples_progress.md` 更新不要: no runnable sample, validation command, debug
surface, Lean artifact, manifest entry, expected JSON, or sample workflow
status changed in this package.

## Reviewer findings and follow-up

Read-only sub-agent review found:

- canon carrier fields to inventory are exactly `id`, `severity`, `span`,
  `rule_instance`, `failed_premise`, `missing_evidence[]`,
  `suggested_repair[]`, and `refs[]`;
- current Rust `TextualMirDiagnostic` has only `code`, `message`, and `span`;
- current `SourceSpan` has `start`, `end`, `line`, and `column`, but not the
  final file-bearing / byte-range / line-column shape;
- Surface remote request rows carry useful future carrier inputs such as
  `required_failures`, `declared_failures`, `failure_row_complete`,
  `generated_from`, and `source_span`;
- helper raw JSON, helper projection, and expected JSON must be distinguished;
- this package should remain docs-only, with code/test changes deferred to a
  separate implementation package.

These findings were integrated into `plan/80`.

## Skipped validations and reasons

- Cargo tests / build / clippy: skipped because no Rust code changed.
- Surface helper execution: skipped because no sample matrix, helper behavior,
  or expected output changed.
- Lean validation: skipped because no Lean files or Lean manifest changed.
- Runtime / release checks: skipped because this package makes no runtime,
  product, conformance, or release claim.
- Canon validators: skipped because `mirrorea_canon/` was not edited.

## Commit / push status

Pending at report update. This package should be committed with
`git commit --no-gpg-sign` and pushed after final diff review.

## Sub-agent session close status

Read-only reviewer sub-agent `019f2809-617f-7633-aac2-bd1876945278` was closed
after its findings were integrated.
