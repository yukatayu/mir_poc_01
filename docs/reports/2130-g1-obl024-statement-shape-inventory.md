# Report 2130 - G1 OBL-024 Statement-Shape Inventory

- Date: 2026-07-03 22:05 JST
- Author / agent: Codex
- Scope: LAB-only OBL-024 explanation soundness statement-shape inventory
- Decision levels touched: L1/L2 canon consulted, no canon edit; LAB repository memory updated

## Objective

Inventory the minimum statement-shape prerequisites for OBL-024 explanation
soundness without creating a Lean file, proving OBL-024, changing diagnostic
code, freezing a final Diagnostic ABI, claiming conformance, claiming G1 exit,
or editing canon.

## Scope and assumptions

The normative source is `mirrorea_canon/`. This task edits only LAB repository
memory, snapshot docs, validator path lists, and this report.

Working assumption: OBL-024 should first be organized as a relation between an
emitted Diagnostic and an actual failed judgment premise under reported
bindings. E-ROW is the immediate G1 pressure case, but the inventory must not
turn E-ROW helper evidence into the whole theorem.

## Start state / dirty state

The task started from clean `main...origin/main` at commit `030dc018 Add G1
diagnostic carrier inventory`. A new Discord baseline was recorded with
`discord_notify.py begin --cwd .`.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `AGENTS.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/theory/03-elaboration.md`
- `mirrorea_canon/theory/06-existence-fallback.md`
- `mirrorea_canon/theory/10-diagnostics.md`
- `mirrorea_canon/spec/07-diagnostics-format.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`
- `plan/79-g1-erow-diagnostic-alignment.md`
- `plan/80-g1-diagnostic-carrier-inventory.md`
- `crates/mir-ast/src/textual_alpha.rs`
- `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- `scripts/surface_mir_samples.py`
- `samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/elab-10-visibility-failure-row-negative/expected/elaboration.json`
- `/home/codex/.codex/docs/oracle-chatgpt-pro.md`
- `.docs/oracle-chatgpt-pro-operations.md`

## Actions taken

- Added `plan/81-g1-obl024-statement-shape-inventory.md`.
- Recorded the OBL-024 target reading as emitted Diagnostic plus reported
  rule instance / failed premise / bindings replaying to failure exactly there.
- Listed minimum abstract vocabulary needed before a Lean statement:
  `JudgmentInput`, `Diagnostic`, `RuleInstance`, `Premise`, `Bindings`,
  `Replay`, `FailsExactlyAt`, `CarrierReports`, and `SpanBlame`.
- Added an E-ROW instantiation target around BND-001 row containment without
  treating current helper output as final canon ABI.
- Recorded dependencies and OPEN items before any OBL-024 Lean statement.
- Integrated read-only sub-agent review, especially the E-ROW-only concrete
  scope, ELAB-07/10/04 evidence classification, OBL-021-as-context boundary,
  and expected-JSON projection overclaim guard.
- Integrated Oracle advisory review, especially the distinction between
  carrier inventory and relation inventory, diagnostic-to-judgment association,
  trace-local "exactly there", and doing this inventory before any E-ROW
  additive carrier prototype.
- Updated plan index, source traceability, snapshot docs, and validator path
  lists.
- Started and integrated one Oracle advisory review in parallel for theory
  critique.
- Started and integrated one read-only sub-agent semantic review in parallel.

## Files changed

- `Documentation.md`
- `plan/00-index.md`
- `plan/81-g1-obl024-statement-shape-inventory.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `scripts/README.md`
- `scripts/check_source_hierarchy.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/validate_docs.py`
- `tasks.md`
- `docs/reports/2130-g1-obl024-statement-shape-inventory.md`

No code, sample, Lean, manifest, `samples_progress.md`, or `mirrorea_canon/`
file was edited.

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
sed -n '1,260p' /home/codex/.codex/docs/oracle-chatgpt-pro.md
sed -n '1,260p' .docs/oracle-chatgpt-pro-operations.md
sed -n '1,260p' mirrorea_canon/theory/10-diagnostics.md
sed -n '1,220p' mirrorea_canon/spec/07-diagnostics-format.md
sed -n '1,260p' mirrorea_canon/theory/11-metatheory-ledger.md
sed -n '1,300p' mirrorea_canon/theory/03-elaboration.md
sed -n '1,180p' mirrorea_canon/theory/06-existence-fallback.md
sed -n '1,180p' mirrorea_canon/MAP.md
rg -n "TextualMirDiagnostic|SourceSpan|diagnostic|failure_row_complete|required_failures|declared_failures|generated_from|source_span|VisibilityDenied|generated_failure_not_declared" crates/mir-ast/src/textual_alpha.rs crates/mir-semantics/src/surface_to_core_elaboration.rs crates/mir-semantics/examples/surface_to_core_elaborate.rs crates/mir-semantics/tests/surface_to_core_elaboration.rs scripts/surface_mir_samples.py
jq '{accepted, diagnostic_codes, remote_request_summaries, source_span_entity_kinds, obligation_codes, final_public_api_frozen}' samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/expected/elaboration.json
jq '{accepted, diagnostic_codes, remote_request_summaries, source_span_entity_kinds, obligation_codes, final_public_api_frozen}' samples/full-system-v1-surface/elaboration/elab-10-visibility-failure-row-negative/expected/elaboration.json
rg -n "OBL-024|Explanation soundness|explanation soundness|Diag.Sound|failed_premise|rule_instance|suggested_repair|Diagnostic" mirrorea_canon plan specs docs samples scripts crates
date '+%Y-%m-%d %H:%M %Z'
ask-chatgpt-pro -p "<OBL-024 statement-shape review prompt>" --file ...
```

Post-report validation commands are listed in the evidence section.

## Evidence / outputs / test results

The package is docs-only. No Rust, Python helper, Lean, sample matrix, expected
JSON, or manifest behavior changed.

Post-report validation:

```bash
python3 scripts/check_source_hierarchy.py
# required: 577
# present: 577
# missing: 0

python3 scripts/validate_docs.py
# Documentation scaffold looks complete.
# Found 1282 numbered report(s).

python3 -m unittest scripts.tests.test_validate_docs
# Ran 20 tests in 0.347s
# OK

git diff --check
# exit 0
```

## What changed in understanding

OBL-024 is not just "the diagnostic code is the right code" and not just
"the helper projection contains a diagnostic code". Its statement shape needs a
replay relation connecting the emitted Diagnostic's reported rule instance,
failed premise, and bindings to an actual failed judgment premise. Current
E-ROW evidence already has useful ingredients
(`required_failures`, `declared_failures`, and `failure_row_complete: false`),
but those ingredients are not yet emitted as a Diagnostic carrier and do not
prove replay.

Oracle review sharpened the distinction: `plan/80` already records carrier
gaps, while `plan/81` must record relation prerequisites such as
diagnostic-to-judgment association, actual rule-instance membership,
premise-of-rule membership, binding reconstructability, and trace-local replay
failure.

## Open questions

- Should the first OBL-024 Lean statement quantify over all diagnostics, or
  over an E-ROW fragment with an explicit later-generalization boundary?
- What are the final names for `RuleInstance`, `Premise`, and `Bindings`?
- Should replay be whole-judgment replay or a rule-local replay witness?
- How should declaration-site and use-site spans participate in `SpanBlame`?
- How should OBL-024 relate to OBL-021 diagnostic equivalence without freezing
  diagnostic ordering or ABI?
- How should trace-local "exactly there" be strengthened, if at all, without
  claiming global root-cause uniqueness or ranking?

## Suggested next prompt

自走で E-ROW additive diagnostic carrier prototype か、OBL-025
statement-shape inventory のどちらが先に必要かを比較し、低リスクな方から
進めてください。OBL-024/025 proof、final diagnostic ABI、conformance、G1 exit
は主張しないでください。

## Plan update status

更新済み:

- `plan/00-index.md`
- `plan/81-g1-obl024-statement-shape-inventory.md`
- `plan/90-source-traceability.md`

## Documentation.md update status

更新済み: Surface Mir line now mentions OBL-024 statement-shape inventory as
LAB repository memory, not diagnostic ABI freeze, OBL-024/025 discharge, proof
discharge, or G1 exit.

## progress.md update status

更新済み: Added the current OBL-024 statement-shape inventory note and recent
log entry.

## tasks.md update status

更新済み: Added OBL-024 statement-shape inventory to current holding state and
updated next candidates to additive E-ROW carrier prototype / OBL-025
statement-shape inventory / later OBL-024 Lean statement draft.

## samples_progress.md update status

`samples_progress.md` 更新不要: no runnable sample, validation command, debug
surface, Lean artifact, manifest entry, expected JSON, or sample workflow
status changed in this package.

## Reviewer findings and follow-up

Read-only sub-agent review found:

- the concrete package scope should start with E-ROW-shaped Surface elaboration
  diagnostics only, while keeping the broader OBL-024 theorem general;
- current LAB carrier gap is still `code/message/span`, with failure-row
  evidence living separately in remote request summaries;
- OBL-024 requires replay of the same judgment input and bindings to the
  reported named premise, not merely the same helper code;
- `ELAB-07` is clean E-ROW-001-shaped evidence, `ELAB-10` is clean E-ROW-002
  pressure evidence, and `ELAB-04` is mixed E-ROW-shaped evidence;
- underdeclared cases must eventually blame the missing declaration site, with
  use-site span as related evidence when available;
- OBL-021 diagnostic equality / determinism is prerequisite context, not
  discharged by this inventory;
- expected JSON projections must not be treated as raw or final Diagnostic
  carriers.

These findings were integrated into `plan/81`.

Oracle advisory review found:

- `plan/81` is safe only if it is relation/prerequisite inventory, not another
  carrier inventory and not a statement draft;
- it should record diagnostic-to-judgment association, actual rule-instance
  membership, premise membership, binding reconstruction, replay, trace-local
  "exactly there", ID compatibility, span/blame, and missing-evidence
  relations;
- E-ROW is safer as the concrete pressure case, but non-E-ROW families must
  remain future work;
- "exactly there" should not be read as global root-cause uniqueness when
  multiple premises may be false;
- OBL-025 repair validity/completeness should remain separate;
- doing this inventory before an E-ROW additive carrier prototype is safer
  because it constrains the later prototype without turning helper JSON into
  de facto proof evidence.

These findings were integrated into `plan/81`.

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

Read-only reviewer sub-agent `019f2813-dc0c-7cc3-9ffa-65fb6f366b42` was closed
after its findings were integrated.
