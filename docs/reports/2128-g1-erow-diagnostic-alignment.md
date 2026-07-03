# Report 2128 - G1 E-ROW Diagnostic Alignment

- Date: 2026-07-03 21:49 JST
- Author / agent: Codex
- Scope: LAB-only diagnostic vocabulary alignment for canon E-ROW-001 / E-ROW-002
- Decision levels touched: L1/L2 canon consulted, no canon edit; LAB repository memory updated

## Objective

Record how current LAB Surface elaboration diagnostics around
`generated_failure_not_declared` align with canon E-ROW-001 / E-ROW-002, while
avoiding diagnostic ABI freeze, conformance pass, OBL-024/025 discharge, G1
exit, runtime behavior claims, or canon edits.

## Scope and assumptions

The normative source is `mirrorea_canon/`. This task edits only LAB repository
memory, snapshot docs, validator path lists, and this report.

Working assumption: current helper diagnostics are useful evidence for
E-ROW-shaped rejection families, but helper diagnostic strings and JSON shape
are not final canon diagnostic IDs or ABI.

## Start state / dirty state

The task started from clean `main...origin/main` at commit `f011d917 Add G1
OBL020 Lean statement draft`. A new Discord baseline was recorded with
`discord_notify.py begin --cwd .`.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `AGENTS.md`
- `mirrorea_canon/theory/03-elaboration.md`
- `mirrorea_canon/theory/10-diagnostics.md`
- `mirrorea_canon/spec/06-conformance.md`
- `mirrorea_canon/spec/07-diagnostics-format.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`
- `plan/75-g1-scn-rhs-dependency-gap-evidence.md`
- `plan/76-g1-obl020-021-dependency-inventory.md`
- `plan/78-g1-obl020-lean-statement-draft.md`
- `samples/full-system-v1-surface/elaboration/README.md`
- `samples/full-system-v1-surface/elaboration/elab-04-undeclared-generated-failure-negative/README.md`
- `samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/README.md`
- `samples/full-system-v1-surface/elaboration/elab-10-visibility-failure-row-negative/README.md`
- `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- `scripts/tests/test_surface_mir_samples.py`

## Actions taken

- Added `plan/79-g1-erow-diagnostic-alignment.md`.
- Mapped canon E-ROW-001 to general generated-failure containment failure.
- Mapped canon E-ROW-002 to undeclared `VisibilityDenied`.
- Recorded current LAB `generated_failure_not_declared` as a helper-local
  diagnostic family, not a canon diagnostic ID.
- Classified `ELAB-07` as clean E-ROW-001-shaped evidence.
- Classified `ELAB-10` as clean E-ROW-002-shaped pressure evidence still
  carried by the helper-local diagnostic.
- Classified `ELAB-04` as useful mixed E-ROW-shaped evidence, not the clean
  E-ROW-002 row.
- Updated plan index, source traceability, snapshot docs, and validator path
  lists.
- Integrated read-only sub-agent review into plan wording.

## Files changed

- `Documentation.md`
- `plan/00-index.md`
- `plan/79-g1-erow-diagnostic-alignment.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `scripts/README.md`
- `scripts/check_source_hierarchy.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/validate_docs.py`
- `tasks.md`
- `docs/reports/2128-g1-erow-diagnostic-alignment.md`

No code, sample, Lean, manifest, `samples_progress.md`, or `mirrorea_canon/`
file was edited.

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
rg -n "E-ROW|failure row|generated_failure|Failure|Diagnostic|diagnostic|VisibilityDenied|TypeMismatch|generated failures|fails" mirrorea_canon/theory/03-elaboration.md mirrorea_canon/theory/10-diagnostics.md mirrorea_canon/spec/07-diagnostics-format.md mirrorea_canon/spec/06-conformance.md mirrorea_canon/theory/11-metatheory-ledger.md
rg -n "generated_failure_not_declared|VisibilityDenied|failure row|failure_row|E-ROW|declared_failures|generated_failures|TypeMismatch" crates/mir-semantics/src/surface_to_core_elaboration.rs crates/mir-semantics/tests/surface_to_core_elaboration.rs samples/full-system-v1-surface/elaboration scripts/surface_mir_samples.py scripts/tests/test_surface_mir_samples.py
date '+%Y-%m-%d %H:%M %Z'
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
python3 -m unittest scripts.tests.test_validate_docs
git diff --check
```

Post-report validation commands are listed in the evidence section.

## Evidence / outputs / test results

The package is docs-only. No Rust, Python helper, Lean, sample matrix, or
manifest behavior changed.

- `python3 scripts/check_source_hierarchy.py` reported 575 required paths,
  575 present, 0 missing.
- `python3 scripts/validate_docs.py` reported the documentation scaffold
  complete, with 1280 numbered reports.
- `python3 -m unittest scripts.tests.test_validate_docs` passed with 20 tests.
- `git diff --check` passed.

Post-report validation passed.

## What changed in understanding

Canon E-ROW-001 and E-ROW-002 should be kept distinct even though the current
LAB helper emits a single diagnostic code. The safe mapping is:

- `ELAB-07` aligns structurally with E-ROW-001;
- `ELAB-10` aligns structurally with E-ROW-002;
- `ELAB-04` remains useful mixed evidence for underdeclared row containment,
  but is not the clean `VisibilityDenied` row;
- LAB `generated_failure_not_declared` is current helper evidence, not a canon
  diagnostic ID.

The current LAB carrier does not satisfy canon diagnostic carrier expectations
for rule instance, failed premise, missing evidence, suggested repair, and
refs. That gap belongs to future diagnostic carrier / OBL-024/025 work.

## Open questions

- Should LAB helper output keep legacy diagnostic strings while adding canon
  ID fields alongside them?
- Should `VisibilityDenied` get a distinct helper diagnostic code before final
  diagnostic ABI work?
- What minimal carrier fields are needed before OBL-024 explanation soundness
  can be stated?
- Should suggested repair rows be generated in LAB expected JSON, or deferred
  until diagnostic ABI work begins?

## Suggested next prompt

自走で diagnostic carrier inventory を進め、OBL-024 / OBL-025 prerequisites
と current LAB diagnostic carrier gap を整理してください。proof discharge、
diagnostic ABI freeze、conformance、G1 exit は主張しないでください。

## Plan update status

更新済み:

- `plan/00-index.md`
- `plan/79-g1-erow-diagnostic-alignment.md`
- `plan/90-source-traceability.md`

## Documentation.md update status

更新済み: Surface Mir line now mentions E-ROW diagnostic alignment as LAB
repository memory, not diagnostic ABI freeze, proof discharge, or G1 exit.

## progress.md update status

更新済み: Added the current E-ROW alignment note and recent log entry.

## tasks.md update status

更新済み: Added E-ROW alignment to current holding state and replaced the
alignment candidate with diagnostic carrier inventory / optional helper ID
split candidates.

## samples_progress.md update status

`samples_progress.md` 更新不要: no runnable sample, validation command,
debug surface, Lean artifact, manifest entry, or sample workflow status changed
in this package.

## Reviewer findings and follow-up

Read-only sub-agent review found:

- E-ROW-001 is general generated-failure row containment.
- E-ROW-002 is the specific undeclared `VisibilityDenied` case.
- `generated_failure_not_declared` should be described as a helper-local
  alias/gap covering E-ROW-shaped rejections, not the canon diagnostic ID.
- `ELAB-07` is the clean E-ROW-001-shaped row.
- `ELAB-10` is the clean E-ROW-002-shaped row.
- `ELAB-04` is useful mixed evidence but should not be used as the clean
  E-ROW-002 row.
- A docs-only plan/report package is sufficient now; no tests/code should be
  changed for this alignment package.

These findings were integrated into `plan/79`.

## Skipped validations and reasons

- Cargo tests / build / clippy: skipped because no Rust code changed.
- Surface helper execution: skipped because no sample matrix, helper behavior,
  or expected output changed.
- Lean validation: skipped because no Lean files or Lean manifest changed.
- Runtime / release checks: skipped because this package makes no runtime,
  product, conformance, or release claim.
- Canon validators: skipped because `mirrorea_canon/` was not edited.

## Commit / push status

Pending at report write. This package should be committed with
`git commit --no-gpg-sign` and pushed after post-report validation.

## Sub-agent session close status

Read-only reviewer sub-agent `019f2803-ad11-7c93-adee-b312da39f26f` was closed
after its findings were integrated.
