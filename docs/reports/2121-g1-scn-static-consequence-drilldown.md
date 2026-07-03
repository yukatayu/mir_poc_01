# Report 2121 - G1 SCN static consequence drilldown

- Date: 2026-07-03 20:12 JST
- Author / agent: Codex
- Scope: LAB-only SCN-01/SCN-02 C-static consequence mapping for G1
- Decision levels touched: no canon decision changed; LAB process memory only

## Objective

Map SCN-01 and SCN-02 C-static expectations to canon anchors, the `plan/71`
simple-assignment target, current LAB Surface elaboration evidence, and explicit
open gaps without claiming conformance, G1 exit, or theorem discharge.

## Scope and assumptions

Scope was limited to LAB documentation, validator scaffold, current snapshots,
and advisory-review integration. The task did not edit canon semantics, ADRs,
scenarios, runtime code, sample source, expected sample outputs, or theorem
status.

Working assumption: this package should produce a static consequence map only.
Here "static consequence" means parse/check/elaborate-time generated request /
publish / observe / dependency / obligation / diagnostic inventory, not runtime
occurrence trace.

## Start state / dirty state

`git status --short --branch` at package start showed `## main...origin/main`
and no dirty files after commit `4921abe6`. A Discord task baseline was
recorded before editing this package.

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `mirrorea_canon/README.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/meta/source-hierarchy.md`
- `mirrorea_canon/plan/00-gates.md`
- `mirrorea_canon/plan/01-phases.md`
- `mirrorea_canon/theory/01-mircore-v0.md`
- `mirrorea_canon/theory/02-types-effects-failures.md`
- `mirrorea_canon/theory/03-elaboration.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`
- `mirrorea_canon/spec/02-surface-grammar.md`
- `mirrorea_canon/spec/03-static-semantics.md`
- `mirrorea_canon/spec/04-core-ir.md`
- `mirrorea_canon/spec/06-conformance.md`
- `mirrorea_canon/scenarios/SCN-01-sugoroku-roll.md`
- `mirrorea_canon/scenarios/SCN-02-attack.md`
- `LAB:plan/70-lab-to-canon-reconciliation-ledger.md`
- `LAB:plan/71-g1-ordinary-assignment-target.md`
- `LAB:samples/full-system-v1-surface/elaboration/README.md`
- `LAB:samples/full-system-v1-surface/elaboration/matrix.json`
- `LAB:samples/full-system-v1-surface/elaboration/*/expected/elaboration.json`
- `LAB:samples/full-system-v1-surface/elaboration/*/main/src/*.mir`
- `LAB:crates/mir-semantics/src/surface_to_core_elaboration.rs`
- `LAB:crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- read-only sub-agent review findings for this package
- ChatGPT 5.5 Pro Extended Oracle follow-up review for this package

## Actions taken

- Added `plan/72-g1-scn01-scn02-static-consequence-drilldown.md` as LAB-only
  repository memory.
- Mapped SCN-01 static consequences for owner-directed write request, RHS read
  dependency, visible publish/observe, failure containment, authority
  obligation, source spans, and C-runtime boundary.
- Mapped SCN-02 static consequences for owner-directed write request, two RHS
  read dependencies, failure containment, nested-locus non-authority, direct
  local-write rejection, and C-runtime boundaries.
- Recorded the main LAB gaps before OBL-001: RHS dependency rows for SCN-01 and
  SCN-02 are not directly exposed by current LAB expected JSON, canon diagnostic
  ids are not identical to LAB helper diagnostics, and OPEN-014 remains live.
- Updated `plan/71` so the next safe package now points to OBL-001 statement
  inventory and exact LAB gap drilldown if needed.
- Updated `plan/00-index.md`, `plan/90-source-traceability.md`, `progress.md`,
  and `tasks.md` to make `plan/72` discoverable and move the next candidate to
  OBL-001 statement inventory.
- Extended the documentation validators' required path lists and tests to
  include `plan/72`.
- Closed the read-only sub-agent session after incorporating its findings.

## Files changed

- `docs/reports/2121-g1-scn-static-consequence-drilldown.md`
- `plan/00-index.md`
- `plan/71-g1-ordinary-assignment-target.md`
- `plan/72-g1-scn01-scn02-static-consequence-drilldown.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `scripts/README.md`
- `scripts/check_source_hierarchy.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/validate_docs.py`
- `tasks.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git status --short --branch
oracle status --hours 2 --limit 10
oracle session review-the-next-package-plan
ask-chatgpt-pro-followup review-the-next-package-plan -p "<SCN package review prompt>"
sed -n '1,220p' mirrorea_canon/scenarios/SCN-01-sugoroku-roll.md
sed -n '1,240p' mirrorea_canon/scenarios/SCN-02-attack.md
sed -n '1,220p' mirrorea_canon/spec/06-conformance.md
sed -n '1,220p' samples/full-system-v1-surface/elaboration/README.md
sed -n '1,140p' samples/full-system-v1-surface/elaboration/matrix.json
rg -n 'ELAB-0|remote|publish|observe|failure|span|MessageEnvelope|VisibilityDenied' samples/full-system-v1-surface/elaboration -g '*.json' -g '*.md'
find samples/full-system-v1-surface/elaboration -path '*/expected/elaboration.json' -maxdepth 4 -print | sort
for f in samples/full-system-v1-surface/elaboration/elab-01-cross-place-read-positive/expected/elaboration.json ...; do jq '{accepted, diagnostic_codes, generated_edge_kinds, source_span_entity_kinds, remote_request_summaries, publication_summaries, observation_summaries, proof_hooks}' "$f"; done
for f in samples/full-system-v1-surface/elaboration/elab-01-cross-place-read-positive/main/src/cross-place-read-positive.mir ...; do sed -n '1,140p' "$f"; done
rg -n 'READ-|WRITE-|LOCUS-BLOCK|HANDLER|publish|observe|request|failure|span|BND-001|THM-001|OBL-001|OBL-020|OBL-021|OPEN-014' mirrorea_canon/theory/01-mircore-v0.md mirrorea_canon/theory/02-types-effects-failures.md mirrorea_canon/theory/03-elaboration.md mirrorea_canon/theory/11-metatheory-ledger.md
python3 scripts/surface_mir_samples.py run ELAB-01 --format json
python3 scripts/surface_mir_samples.py run ELAB-02 --format json
python3 scripts/surface_mir_samples.py run ELAB-05 --format json
python3 scripts/surface_mir_samples.py run ELAB-07 --format json
python3 scripts/surface_mir_samples.py run ELAB-08 --format json
python3 scripts/surface_mir_samples.py run ELAB-09 --format json
python3 scripts/surface_mir_samples.py run ELAB-10 --format json
python3 scripts/surface_mir_samples.py check-all --format json
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
rg -n 'static trace|trace drilldown|SCN trace|plan/72-g1-scn-static' plan progress.md tasks.md scripts Documentation.md .docs/oracle-chatgpt-pro-operations.md
date '+%Y-%m-%d %H:%M %Z'
```

One exploratory `jq` / shell-glob command built a nonexistent `elab-010-*`
path and exited with an error. It was not used as evidence; the target expected
JSON files were then listed explicitly and read successfully.

Post-report final commands to run before commit:

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
git status --short
```

## Evidence / outputs / test results

Pre-report checks passed:

- `python3 scripts/surface_mir_samples.py run ELAB-01 --format json`: expected
  and actual matched for cross-locus read/observe request evidence.
- `python3 scripts/surface_mir_samples.py run ELAB-02 --format json`: expected
  and actual matched for nested foreign place write request evidence.
- `python3 scripts/surface_mir_samples.py run ELAB-05 --format json`: expected
  and actual matched for source-span evidence.
- `python3 scripts/surface_mir_samples.py run ELAB-07 --format json`: expected
  and actual matched for underdeclared write failure-row rejection.
- `python3 scripts/surface_mir_samples.py run ELAB-08 --format json`: expected
  and actual matched for nested place read request evidence.
- `python3 scripts/surface_mir_samples.py run ELAB-09 --format json`: expected
  and actual matched for visible write publish/observe evidence.
- `python3 scripts/surface_mir_samples.py run ELAB-10 --format json`: expected
  and actual matched for visibility failure-row underdeclaration evidence.
- `python3 scripts/surface_mir_samples.py check-all --format json`: 46 samples
  passed, failed 0, `workflow_ready = false`, and non-claims preserved.
- `cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture`:
  14 tests passed.
- `python3 -m unittest scripts.tests.test_validate_docs`: 20 tests passed.
- `python3 scripts/check_source_hierarchy.py`: required 562, present 562,
  missing 0.
- `python3 scripts/validate_docs.py`: Documentation scaffold looks complete;
  found 1272 numbered reports before adding this report.
- `git diff --check`: passed with no output.
- stale wording scan for old `static trace` / `trace drilldown` / old plan/72
  path references: no matches after rename.

Post-report final checks also passed:

- `python3 -m unittest scripts.tests.test_validate_docs`: 20 tests passed.
- `python3 scripts/check_source_hierarchy.py`: required 562, present 562,
  missing 0.
- `python3 scripts/validate_docs.py`: Documentation scaffold looks complete;
  found 1273 numbered reports.
- `git diff --check`: passed with no output.
- `git status --short --branch`: only the expected package files were dirty
  before commit.

## What changed in understanding

The useful next object is not "SCN-01/02 passes" but a C-static consequence map.
Current LAB ELAB rows support the request/publish/observe/source-span/failure
containment shape, but they do not directly expose the SCN-01 same-field RHS
dependency row or the SCN-02 two-read RHS dependency set. Those are now explicit
gaps before a confident OBL-001 statement.

Canon diagnostic ids E-ROW-001 / E-ROW-002 also remain distinct from the LAB
helper diagnostic `generated_failure_not_declared`. The plan records that as an
alias/gap rather than pretending the LAB helper names are canon.

## Open questions

- How should OBL-001 state read dependency rows so SCN-01 same-field RHS read
  and SCN-02 two-read RHS are covered without overfitting LAB JSON?
- Under OPEN-014, should cross-locus RHS reads be represented as dependency
  rows only, read-request/observe rows, or both in the initial Lean statement?
- Are exact LAB rows needed before OBL-001 for SCN-01 missing
  `VisibilityDenied` on visible write publish and SCN-02 `target.hp` /
  `self.atk` RHS reads?
- Should canon receive a later clarification proposal for ordinary assignment
  if OBL-001 statement inventory exposes ambiguity?

## Suggested next prompt

Proceed with the G1 OBL-001 Lean statement inventory: use `plan/71` and
`plan/72` to list the minimum statement ingredients and gaps for THM-001,
without writing a proof or claiming any obligation complete.

## Plan update status

Updated:

- Added `plan/72-g1-scn01-scn02-static-consequence-drilldown.md`.
- Updated `plan/00-index.md`, `plan/71`, and `plan/90`.

## Documentation.md update status

`Documentation.md` update unnecessary. The repository-level reader summary did
not need a new top-level explanation for this narrow LAB theory-memory package.

## progress.md update status

Updated: recorded `plan/72`, kept it as LAB memory, and moved the next safe
self-driven theory package to OBL-001 Lean statement inventory.

## tasks.md update status

Updated: removed the completed SCN-01/SCN-02 drilldown candidate and added the
remaining OBL-001 statement inventory plus exact LAB gap drilldown reserve.

## samples_progress.md update status

`samples_progress.md` update unnecessary. No runnable sample status,
validation command, debug surface, blocker, or workflow readiness row changed.
The ELAB rows were rerun only as evidence checks for a docs/theory package.

## Reviewer findings and follow-up

Sub-agent review found three important issues:

- current LAB ELAB rows do not support SCN-01/SCN-02 RHS dependency-row
  requirements for assignment expressions;
- canon diagnostic ids E-ROW-001 / E-ROW-002 are not the same as LAB
  `generated_failure_not_declared`;
- OPEN-014 remains live and the plan must not freeze the read materialization
  policy.

Follow-up applied: `plan/72` now treats RHS dependency rows as main LAB gaps,
labels `generated_failure_not_declared` as LAB alias/gap, and uses
"dependency/read consequence is explicit" rather than a frozen transport/cache
shape.

Oracle follow-up agreed that the package is safe only as a LAB C-static
consequence drilldown. It warned against "trace" being misread as runtime
trace, recommended row IDs, and emphasized static/runtime split, simple vs
compound assignment split, and LAB `MessageEnvelope` evidence-only wording.

Follow-up applied: the plan file was renamed to
`plan/72-g1-scn01-scn02-static-consequence-drilldown.md`, the tables now use
row IDs, and the file defines static consequence as generated-edge /
obligation / diagnostic inventory rather than runtime occurrence trace.

## Skipped validations and reasons

- Full workspace Cargo tests were not rerun because this package changed
  documentation and validator required-file lists only. Focused
  `mir-semantics` elaboration tests and Surface helper rows were run because
  `plan/72` cites that LAB evidence.
- Product Alpha release checks and operational product helpers were not run
  because no product/sample/runtime status changed.
- Storage/disk audit was not run because this package did not create heavy
  build artifacts, generated sample artifacts, LLVM/Lean artifacts, or external
  workdir outputs.
- No canon index update was needed because canon files were not edited.

## Commit / push status

At report creation: not yet committed. The task close process will run final
validation, then commit with `git commit --no-gpg-sign` and push.

## Sub-agent session close status

The read-only sub-agent completed, returned findings, made no file edits, and
was closed after its findings were incorporated.
