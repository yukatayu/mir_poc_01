# Report 2120 - G1 ordinary assignment target draft

- Date: 2026-07-03 20:00 JST
- Author / agent: Codex
- Scope: LAB-only G1 ordinary simple-assignment target/proof-boundary draft
- Decision levels touched: no canon decision changed; LAB process memory only

## Objective

Draft the next safe G1 ordinary assignment target boundary in repository memory,
using `mirrorea_canon/` as the normative source and old Surface Mir material as
LAB evidence only.

## Scope and assumptions

Scope was limited to LAB documentation, validator scaffold, current snapshots,
and advisory-review integration. The task did not edit canon semantics, ADRs,
scenarios, theorem/proof status, runtime code, sample source, or expected sample
outputs.

Working assumption: the first G1 target should be simple assignment, not
compound read-modify-write assignment, because compound assignment adds a read
side that should be stated as a separate lemma or extension.

## Start state / dirty state

`git status --short --branch` at package start showed `## main...origin/main`
and no dirty files. A Discord task baseline was recorded before editing.

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `.docs/oracle-chatgpt-pro-operations.md`
- `/home/codex/.codex/docs/oracle-chatgpt-pro.md`
- `mirrorea_canon/README.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/meta/source-hierarchy.md`
- `mirrorea_canon/adr/ADR-0001.md`
- `mirrorea_canon/adr/ADR-0002.md`
- `mirrorea_canon/adr/ADR-0012.md`
- `mirrorea_canon/GLOSSARY.md`
- `mirrorea_canon/architecture/02-boundary-contracts.md`
- `mirrorea_canon/plan/00-gates.md`
- `mirrorea_canon/plan/01-phases.md`
- `mirrorea_canon/plan/02-operating-model.md`
- `mirrorea_canon/theory/00-overview.md`
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
- `LAB:specs/39-surface-mir-placement-elaboration.md`
- `LAB:plan/64-surface-mir-placement-roadmap.md`
- `LAB:plan/69-consultation-synthesis-and-management-roadmap.md`
- `LAB:plan/70-lab-to-canon-reconciliation-ledger.md`
- `LAB:samples/full-system-v1-surface/elaboration/README.md`
- `LAB:crates/mir-semantics/src/surface_to_core_elaboration.rs`
- `LAB:crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- read-only sub-agent review findings for this package
- ChatGPT 5.5 Pro Extended Oracle advisory review for this package

## Actions taken

- Added `plan/71-g1-ordinary-assignment-target.md` as LAB-only repository
  memory.
- Framed the G1 target as ordinary simple-assignment elaboration, not G1 exit
  or proof discharge.
- Split the target into canon anchors, target wording, case split,
  proof-obligation split, SCN-01/SCN-02 mapping, LAB evidence, non-claims, and
  next safe packages.
- Updated `plan/70` with a narrow cross-link from the existing assignment row
  to `plan/71`.
- Updated `plan/00-index.md`, `plan/90-source-traceability.md`,
  `progress.md`, and `tasks.md` so the new memory is discoverable and the next
  packages move to SCN-01/SCN-02 static trace drilldown or OBL-001 statement
  inventory.
- Updated `.docs/oracle-chatgpt-pro-operations.md` and `Documentation.md` to
  remove touched stale `specs/`-as-normative wording and make Oracle guidance
  canon-first.
- Extended documentation validators' required path lists and tests to include
  `plan/71`.
- Closed the read-only sub-agent session after recording its findings.

## Files changed

- `.docs/oracle-chatgpt-pro-operations.md`
- `Documentation.md`
- `plan/00-index.md`
- `plan/70-lab-to-canon-reconciliation-ledger.md`
- `plan/71-g1-ordinary-assignment-target.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `scripts/README.md`
- `scripts/check_source_hierarchy.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/validate_docs.py`
- `tasks.md`
- `docs/reports/2120-g1-ordinary-assignment-target.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git status --short --branch
sed -n '1,260p' plan/00-index.md
sed -n '1,260p' plan/90-source-traceability.md
sed -n '220,270p' Documentation.md
sed -n '1,240p' .docs/oracle-chatgpt-pro-operations.md
sed -n '1,260p' scripts/validate_docs.py
sed -n '1,260p' scripts/check_source_hierarchy.py
sed -n '1,260p' scripts/tests/test_validate_docs.py
sed -n '1,240p' scripts/README.md
sed -n '1,260p' progress.md
sed -n '1,260p' tasks.md
sed -n '1,240p' docs/reports/TEMPLATE.md
date '+%Y-%m-%d %H:%M %Z'
rg -n 'plan/69|plan/70|plan/39\.\.|Oracle consult|normative source|specs/' scripts/validate_docs.py scripts/check_source_hierarchy.py scripts/tests/test_validate_docs.py scripts/README.md Documentation.md .docs/oracle-chatgpt-pro-operations.md
rg -n 'Recent|recent|作業ログ|log|最終更新|ordinary|G1|plan/70|Next gap|current milestone position' progress.md tasks.md
sed -n '260,520p' progress.md
sed -n '260,520p' tasks.md
ls docs/reports | tail -n 20
rg -n 'assignment|ordinary|source-language' plan/70-lab-to-canon-reconciliation-ledger.md
ask-chatgpt-pro ... --session-name review-the-next-package-plan --file <21 repo files>
git diff --stat
git diff -- plan/71-g1-ordinary-assignment-target.md .docs/oracle-chatgpt-pro-operations.md Documentation.md progress.md tasks.md
python3 -m unittest scripts.tests.test_validate_docs
git status --short --branch
sed -n '1,220p' plan/71-g1-ordinary-assignment-target.md
rg -n 'specs/` is normative source|specs/` as normative source|normative `specs/|G1 exit|G0 exit|theorem discharge|runtime MessageEnvelope dispatch|final public grammar|Core IR JSON field freeze' plan/71-g1-ordinary-assignment-target.md .docs/oracle-chatgpt-pro-operations.md Documentation.md progress.md tasks.md plan/00-index.md plan/90-source-traceability.md scripts/README.md
python3 scripts/check_source_hierarchy.py
python3 scripts/surface_mir_samples.py check-all --format json
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
python3 meta/build-index.py --check
python3 scripts/validate_docs.py
git diff --check
git diff -- plan/70-lab-to-canon-reconciliation-ledger.md plan/71-g1-ordinary-assignment-target.md progress.md tasks.md
rg -n 'MessageEnvelope|compound|simple-assignment|G1 exit|theorem discharge|runtime dispatch|canon vocabulary' plan/71-g1-ordinary-assignment-target.md plan/70-lab-to-canon-reconciliation-ledger.md progress.md tasks.md
```

One early manual `rg` scan used a double-quoted shell pattern containing
backticks and produced shell noise. It was not used as evidence; the safe
single-quoted scan listed above was rerun.

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

- `python3 -m unittest scripts.tests.test_validate_docs`: 20 tests passed.
- `python3 scripts/check_source_hierarchy.py`: required 561, present 561,
  missing 0.
- `python3 scripts/validate_docs.py`: Documentation scaffold looks complete;
  found 1271 numbered reports before adding this report.
- `python3 meta/build-index.py --check` in `mirrorea_canon/`: ok, 69 files
  indexed.
- `python3 scripts/surface_mir_samples.py check-all --format json`: 46 samples
  passed, failed 0, `workflow_ready = false`, and non-claims preserved.
- `cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture`:
  14 tests passed.
- `git diff --check`: passed with no output.
- Safe stale-wording scan found only intentional non-claim references for G0/G1
  exit, theorem discharge, runtime dispatch, final grammar, and LAB
  MessageEnvelope evidence.

The Cargo run briefly waited on the package cache lock, then completed
successfully.

Post-report final checks also passed:

- `python3 -m unittest scripts.tests.test_validate_docs`: 20 tests passed.
- `python3 scripts/check_source_hierarchy.py`: required 561, present 561,
  missing 0.
- `python3 scripts/validate_docs.py`: Documentation scaffold looks complete;
  found 1272 numbered reports.
- `git diff --check`: passed with no output.
- `git status --short --branch`: only the expected package files were dirty
  before commit.

## What changed in understanding

The next safe theory object is not a broad Surface program theorem. It is a
small target/proof-boundary document for simple assignment elaboration: every
write must become owner-local or owner-directed, generated request/publish/
observe/dependency consequences must be explicit, generated failures must be
row-contained, authority obligations must remain explicit, source spans must be
preserved, and determinism must remain a separate obligation.

Compound assignment should not be silently included in the first statement,
because it is a read-plus-write case. LAB `MessageEnvelope` rows are useful
evidence, but the target wording should use canon request/publish/observe
vocabulary.

## Open questions

- What exact Lean statement should OBL-001 use without overfitting LAB helper
  field names?
- Should SCN-01/SCN-02 static trace drilldown come before or alongside the
  OBL-001 statement inventory?
- Does canon need a short human-approved mental-model clarification for
  ordinary assignment before any canon theorem/proof work?
- How should OPEN-014 cross-locus read materialization be phrased in the simple
  assignment statement without freezing optimization policy?

## Suggested next prompt

Proceed with the G1 SCN-01/SCN-02 static trace drilldown: map each expected
static consequence to canon anchors, LAB evidence rows, and open proof
obligations without claiming G1 exit or runtime conformance.

## Plan update status

Updated:

- Added `plan/71-g1-ordinary-assignment-target.md`.
- Updated `plan/00-index.md`, `plan/70`, and `plan/90`.

## Documentation.md update status

Updated: the Oracle operations section now says Oracle is advisory and does not
replace `mirrorea_canon/`, legacy LAB memory, snapshots, or reports.

## progress.md update status

Updated: recorded `plan/71`, kept it as LAB memory, and moved the next safe
self-driven packages to SCN-01/SCN-02 static trace drilldown or OBL-001 Lean
statement inventory.

## tasks.md update status

Updated: removed the completed ordinary-assignment target draft candidate and
added the next G1 candidate packages.

## samples_progress.md update status

`samples_progress.md` update unnecessary. No runnable sample status, validation
command, debug surface, blocker, or workflow readiness row changed.

## Reviewer findings and follow-up

Sub-agent review found that the safe package is a target/proof-boundary document
only. It warned against treating LAB P-SURF evidence as canon completion,
claiming G1/G0/T1 exit, theorem discharge, Lean proof completion, runtime
MessageEnvelope dispatch, C-runtime/C-distributed conformance, final transport,
final ABI, final public grammar, final devtools/telemetry ABI, or final
hot-plug semantics. It also emphasized THM-001, BND-001, OBL-001/020/021,
SCN-01, and SCN-02 as the right anchors.

Follow-up applied: `plan/71` is LAB-only, uses canon anchors, names non-claims
explicitly, keeps `World`/domain vocabulary out of core, and avoids G1 exit
claims.

Oracle review agreed that the package is safe only as a non-normative target
draft. It added two refinements:

- separate simple assignment from compound read-modify-write assignment;
- use canon `request` / `publish` / `observe` vocabulary in the target and keep
  LAB `MessageEnvelope` as evidence only.

Follow-up applied: `plan/71` now scopes the draft to simple assignment, calls
compound assignment a later lemma/extension, and labels LAB `MessageEnvelope`
rows as helper/sample evidence rather than canon vocabulary.

## Skipped validations and reasons

- Full workspace Cargo tests were not rerun because this package changed
  documentation and validator required-file lists only. A focused
  `mir-semantics` elaboration test was run because `plan/71` cites that LAB
  evidence.
- Product Alpha release checks and operational product helpers were not run
  because no product/sample/runtime status changed.
- Storage/disk audit was not run because this package did not create heavy
  build artifacts, generated sample artifacts, LLVM/Lean artifacts, or external
  workdir outputs.
- No canon index update was needed because canon files were not edited; the
  canon index check was still run.

## Commit / push status

At report creation: not yet committed. The task close process will run final
validation, then commit with `git commit --no-gpg-sign` and push.

## Sub-agent session close status

The read-only sub-agent completed, returned findings, made no file edits, and
was closed after its findings were incorporated.
