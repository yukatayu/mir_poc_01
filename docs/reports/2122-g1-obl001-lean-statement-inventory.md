# Report 2122 — G1 OBL-001 Lean statement inventory

- Date: 2026-07-03 20:26 JST
- Author / agent: Codex
- Scope: LAB-only G1 / THM-001 / OBL-001 statement-inventory planning
- Decision levels touched: LAB repository memory only; no canon status, ADR, SCN expectation, Gate/Phase exit, or OBL status changed

## Objective

Add a LAB repository-memory inventory for the minimum Lean-facing vocabulary,
predicate split, theorem-shape pressure, SCN row coverage, and overfit guards
needed before writing an actual repo-local OBL-001 / THM-001 Lean statement.

## Scope and assumptions

Scope is inventory-only:

- add `plan/73-g1-obl001-lean-statement-inventory.md`;
- synchronize source-traceability and snapshot docs;
- register the new plan file in structural validators;
- validate existing Lean and Surface elaboration evidence without creating a
  new Lean statement file.

Assumptions:

- `mirrorea_canon/` remains the normative source;
- `plan/73` is LAB evidence / repository memory only;
- OBL-001 remains open in `mirrorea_canon/theory/11-metatheory-ledger.md`;
- OBL-020 and OBL-021 remain separate from OBL-001;
- current Lean foundations are proof-engineering examples, not MirCore
  assignment semantics.

## Start state / dirty state

Package start was clean after commit `ce0cc5a9 Add G1 SCN static consequence drilldown`.

`git status --short --branch` at package start:

```text
## main...origin/main
```

Discord baseline had already been recorded for this package with:

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
```

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `mirrorea_canon/README.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/NORTH-STAR.md`
- `mirrorea_canon/GLOSSARY.md`
- `mirrorea_canon/meta/agent-instructions.md`
- `mirrorea_canon/plan/00-gates.md`
- `mirrorea_canon/plan/01-phases.md`
- `mirrorea_canon/theory/README.md`
- `mirrorea_canon/theory/00-overview.md`
- `mirrorea_canon/theory/01-mircore-v0.md`
- `mirrorea_canon/theory/02-types-effects-failures.md`
- `mirrorea_canon/theory/03-elaboration.md`
- `mirrorea_canon/theory/05-authority.md`
- `mirrorea_canon/theory/07-observation.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`
- `mirrorea_canon/spec/04-core-ir.md`
- `mirrorea_canon/spec/06-conformance.md`
- `mirrorea_canon/spec/07-diagnostics-format.md`
- `mirrorea_canon/architecture/02-boundary-contracts.md`
- `mirrorea_canon/scenarios/README.md`
- `mirrorea_canon/scenarios/SCN-01-sugoroku-roll.md`
- `mirrorea_canon/scenarios/SCN-02-attack.md`
- `plan/00-index.md`
- `plan/71-g1-ordinary-assignment-target.md`
- `plan/72-g1-scn01-scn02-static-consequence-drilldown.md`
- `plan/90-source-traceability.md`
- `samples/lean/README.md`
- `samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean`
- `samples/lean/foundations/CurrentL2LabelModel.lean`
- `samples/lean/foundations/CurrentL2IfcSecretExamples.lean`
- `samples/lean/foundations/CurrentL2ProofSkeleton.lean`
- `samples/lean/manifest.json`
- `/home/codex/.codex/docs/oracle-chatgpt-pro.md`
- `.docs/oracle-chatgpt-pro-operations.md`
- sub-agent review result for G1 OBL-001 Lean statement inventory
- Oracle follow-up result for G1 OBL-001 Lean statement inventory

`mirrorea_canon/Documentation.md` was attempted but does not exist; canon entry
docs are `README.md`, `MAP.md`, and the directory-specific files above.

## Actions taken

1. Added `plan/73-g1-obl001-lean-statement-inventory.md` as a LAB-only,
   inventory-only plan file.
2. Recorded minimum Lean-facing ingredients with explicit ingredient IDs:
   locus / identity carriers, simple assignment scope, core term/consequence
   vocabulary, generated result carriers, row containment, authority obligation
   carriers, ownership, dependency, span, determinism separator, and WF
   separator.
3. Converted the theorem-shape sketch to fenced `text`, not fenced `lean`, so
   it cannot be mistaken for an implemented theorem or stub.
4. Added a `plan/72` gap-intake section for RHS read materialization,
   simple-vs-compound split, failure containment, nested-locus non-authority,
   publish/observe, diagnostic-id mismatch, and conformance boundary.
5. Added an explicit close condition and non-claims to prevent `lean-stated`,
   OBL completion, G1 exit, conformance, runtime, or API overclaim.
6. Updated `plan/00-index.md` and `plan/90-source-traceability.md`.
7. Updated `progress.md` and `tasks.md` to make `plan/73` the current
   inventory state and shift the next package toward an actual OBL-001 Lean
   statement draft or a narrow SCN dependency-gap package.
8. Registered `plan/73` in `scripts/check_source_hierarchy.py`,
   `scripts/validate_docs.py`, `scripts/tests/test_validate_docs.py`, and
   `scripts/README.md`.
9. Ran focused docs, canon-index, Lean, Surface elaboration, and diff
   validations.

## Files changed

- `plan/73-g1-obl001-lean-statement-inventory.md`
- `plan/00-index.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `scripts/README.md`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `scripts/tests/test_validate_docs.py`
- `docs/reports/2122-g1-obl001-lean-statement-inventory.md`

No files under `mirrorea_canon/`, `samples/lean/`, `crates/`, or
`samples/full-system-v1-surface/` were edited.

## Commands run

```bash
git status --short --branch
date '+%Y-%m-%d %H:%M %Z'
find docs/reports -maxdepth 1 -type f -name '[0-9][0-9][0-9][0-9]-*.md' | sort | tail -n 8
sed -n '1,260p' docs/reports/TEMPLATE.md
sed -n '1,240p' /home/codex/.codex/docs/oracle-chatgpt-pro.md
sed -n '1,240p' .docs/oracle-chatgpt-pro-operations.md
find mirrorea_canon -maxdepth 3 -type f | sort | sed -n '1,160p'
sed -n '1,220p' mirrorea_canon/README.md
sed -n '1,220p' mirrorea_canon/Documentation.md
sed -n '1,220p' mirrorea_canon/plan/00-gates.md
sed -n '1,240p' mirrorea_canon/MAP.md
sed -n '1,220p' mirrorea_canon/NORTH-STAR.md
sed -n '1,260p' mirrorea_canon/GLOSSARY.md
sed -n '1,220p' mirrorea_canon/meta/agent-instructions.md
sed -n '1,220p' mirrorea_canon/theory/README.md
sed -n '1,260p' mirrorea_canon/theory/00-overview.md
sed -n '1,320p' mirrorea_canon/theory/01-mircore-v0.md
sed -n '1,320p' mirrorea_canon/theory/03-elaboration.md
sed -n '1,320p' mirrorea_canon/theory/02-types-effects-failures.md
sed -n '1,280p' mirrorea_canon/theory/05-authority.md
sed -n '1,260p' mirrorea_canon/theory/07-observation.md
sed -n '1,340p' mirrorea_canon/theory/11-metatheory-ledger.md
sed -n '1,240p' mirrorea_canon/scenarios/README.md
sed -n '1,300p' mirrorea_canon/scenarios/SCN-01-sugoroku-roll.md
sed -n '1,320p' mirrorea_canon/scenarios/SCN-02-attack.md
sed -n '1,260p' mirrorea_canon/spec/04-core-ir.md
sed -n '1,260p' mirrorea_canon/spec/06-conformance.md
sed -n '1,260p' mirrorea_canon/spec/07-diagnostics-format.md
sed -n '1,220p' mirrorea_canon/architecture/02-boundary-contracts.md
sed -n '1,240p' mirrorea_canon/plan/01-phases.md
sed -n '1,240p' plan/00-index.md
sed -n '1,280p' plan/71-g1-ordinary-assignment-target.md
sed -n '1,320p' plan/72-g1-scn01-scn02-static-consequence-drilldown.md
sed -n '1,280p' plan/90-source-traceability.md
sed -n '1,320p' tasks.md
sed -n '1,300p' samples_progress.md
sed -n '1,220p' samples/lean/README.md
sed -n '1,180p' samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean
sed -n '1,180p' samples/lean/foundations/CurrentL2ProofSkeleton.lean
sed -n '1,240p' samples/lean/foundations/CurrentL2LabelModel.lean
sed -n '1,260p' samples/lean/foundations/CurrentL2IfcSecretExamples.lean
jq '{lean_version, foundations_count:(.foundations|length), clean_near_end_count:(.clean_near_end|length), first_clean:(.clean_near_end[0]|{sample_id, theorem_names})}' samples/lean/manifest.json
sed -n '1,240p' scripts/current_l2_lean_sample_sync.py
lean --version
lake --version
rg -n 'plan/7[0-9]|plan/72|plan/71|source hierarchy|required' scripts/check_source_hierarchy.py scripts/validate_docs.py scripts/tests/test_validate_docs.py scripts/README.md Documentation.md progress.md tasks.md samples_progress.md
sed -n '250,330p' scripts/tests/test_validate_docs.py
sed -n '430,455p' scripts/validate_docs.py
sed -n '112,128p' scripts/check_source_hierarchy.py
sed -n '1,30p' scripts/README.md
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
python3 mirrorea_canon/meta/build-index.py --check
python3 meta/build-index.py --check
for f in samples/lean/foundations/*.lean; do lean "$f" || exit 1; done
python3 scripts/current_l2_lean_sample_sync.py
python3 scripts/surface_mir_samples.py check-all --format json
python3 scripts/surface_mir_samples.py check-all --format json | jq '{sample_count, passed_count:(.passed|length), failed_count:(.failed|length), validation_errors:(.validation_errors|length), workflow_ready}'
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
git diff --check
```

Oracle command already running at package start:

```bash
ask-chatgpt-pro-followup follow-up-for-the-mirrorea -p "<G1 OBL-001 Lean statement inventory prompt>"
```

## Evidence / outputs / test results

- `python3 -m unittest scripts.tests.test_validate_docs`
  - pass: 20 tests
- `python3 scripts/check_source_hierarchy.py`
  - pass: required 563, present 563, missing 0
- `python3 scripts/validate_docs.py`
  - pass before this report: documentation scaffold complete, 1273 reports
  - pass after this report: documentation scaffold complete, 1274 reports
- `python3 mirrorea_canon/meta/build-index.py --check`
  - failed because the command was run from `mirrorea_canon/` while still
    including the `mirrorea_canon/` path prefix
- `python3 meta/build-index.py --check`
  - pass: `ok: 69 files indexed`
- `for f in samples/lean/foundations/*.lean; do lean "$f" || exit 1; done`
  - pass: all four foundation Lean files compiled with no output
- `python3 scripts/current_l2_lean_sample_sync.py`
  - pass: printed `/home/codex/dev/mir_poc_01/samples/lean/manifest.json`
  - follow-up `git status --short --branch` showed no changes under
    `samples/lean/`
- `python3 scripts/surface_mir_samples.py check-all --format json`
  - pass: output was large; summary showed 46 passed, 0 failed
- `python3 scripts/surface_mir_samples.py check-all --format json | jq ...`
  - pass:
    `sample_count = 46`, `passed_count = 46`, `failed_count = 0`,
    `validation_errors = 0`, `workflow_ready = false`
- `cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture`
  - pass: 14 passed, 0 failed
- `git diff --check`
  - pass: no whitespace errors
- Oracle follow-up completed in about 5m47s. It recommended exactly this
  inventory-only package shape, `plan/73-g1-obl001-lean-statement-inventory.md`,
  fenced `text` rather than fenced `lean`, no `.lean` file, no `theory/11`
  edit, explicit OBL-020/021 separation, and Plan/72 gap intake.
  The browser wrapper reported `requested=Pro; resolved=(unavailable);
  verified=no`, so the answer is treated as advisory review input only.
- Sub-agent review agreed that current Lean files are reusable only as proof
  engineering patterns, not MirCore semantics, and highlighted RHS dependency
  under-modeling as the main semantic risk.

## What changed in understanding

The safest next step is not to write Lean yet. Before the first OBL-001 Lean
file, the repo needs a stable LAB inventory that separates:

- assignment-local elaboration soundness from whole-program no-hidden-comm;
- authority-obligation representation from authority validity;
- RHS dependency coverage from OPEN-014 materialization policy;
- success-side failure containment from negative diagnostic ids;
- OBL-001 from OBL-020/021 and OBL-002.

Existing Lean foundations are valuable for finite inductive style,
predicate-as-set containment, and proof-stub workflow discipline, but they are
not a source of MirCore assignment semantics.

## Open questions

- What exact namespace and path should hold the first repo-local OBL-001 Lean
  statement draft without implying canon `lean-stated` status?
- Should the first Lean statement be a single conjunction theorem or a named
  lemma family with a wrapper statement?
- Should `C` and `O` be separate Lean fields immediately, or one obligation
  carrier refined later?
- Should an SCN dependency-gap package add exact LAB evidence for SCN-01
  same-field RHS and SCN-02 two-read RHS before the Lean statement?
- How much WF context belongs as an OBL-001 premise versus a separate OBL-020
  input theorem?

## Suggested next prompt

次は `plan/73` に従い、OBL-001 の actual repo-local Lean statement draft を
statement-only で作成してください。`theory/11` の status は動かさず、Lean file
を追加する場合は namespace / path / non-claim を report に明記してください。

## Plan update status

更新済み:

- `plan/73-g1-obl001-lean-statement-inventory.md` を追加。
- `plan/00-index.md` に `plan/73` を追加。
- `plan/90-source-traceability.md` に `plan/73` の source traceability row を追加。

## Documentation.md update status

`Documentation.md` 更新不要:

- This package changes a detailed LAB planning inventory and current snapshot,
  not the concise reader-facing repo summary.

## progress.md update status

更新済み:

- `plan/73` の OBL-001 inventory note を追加。
- next gap を actual OBL-001 Lean statement draft または SCN dependency-gap
  package に更新。
- recent log に 2026-07-03 20:21 JST の作業行を追加。

## tasks.md update status

更新済み:

- holding state に `plan/73` を追加。
- candidate next strategy を OBL-001 Lean statement draft /
  SCN exact LAB gap drilldown / OBL-020/021 dependency inventory に更新。

## samples_progress.md update status

`samples_progress.md` 更新不要:

- No runnable sample root, validation command, sample status, debug surface, or
  blocker changed. Existing Lean and Surface helpers were rerun only as
  validation evidence.

## Reviewer findings and follow-up

Sub-agent findings integrated:

- Scope should remain statement-only / inventory-only, not proof or runtime.
- Main risk is under-modeling RHS dependencies for SCN-01 same-field read and
  SCN-02 two-read RHS.
- Current Lean foundations must not be reused as MirCore semantics.
- Reusable patterns are finite inductive style, set-containment predicates, and
  proof-stub identity discipline.
- Must not claim G0/G1/T1 exit, OBL-020/021 completion, OBL-004,
  C-static/C-runtime/C-distributed pass, authority theorem, observation theorem,
  final API/grammar, exact read materialization, or runtime dispatch.

Oracle findings integrated:

- Use `plan/73-g1-obl001-lean-statement-inventory.md`.
- Keep this as a LAB-only Lean statement inventory, not a Lean statement.
- Do not add or edit `.lean` files in this package.
- Use fenced `text`, not fenced `lean`, for statement-shape sketches.
- Add Plan/72 gap intake and explicit close condition.
- Keep OBL-002/020/021/003/004 boundaries separate.

Local follow-up:

- `plan/73` was adjusted after Oracle returned so it no longer contains a
  fenced `lean` block or theorem declaration.
- `plan/73` explicitly says no Lean theorem file, no theorem statement, no OBL
  status movement, no G1 exit, and no conformance/runtime/API/canon change.

## Skipped validations and reasons

- Full workspace `cargo test --workspace --all-targets --no-fail-fast` skipped:
  docs / plan / validator-only package; focused Surface elaboration regression
  and docs validators were run instead.
- Product Alpha / Full System V1 release checks skipped: no product, runtime,
  projection, provider, or release-check files changed.
- No new Lean statement file was checked because no `.lean` file was added by
  design. Existing foundation Lean files and the Lean sync workflow were
  checked instead.
- No canon `INDEX.json` regeneration was committed because canon files were not
  changed and `meta/build-index.py --check` passed.

## Commit / push status

Pending at report write.

## Sub-agent session close status

Sub-agent `019f27b1-2adf-7f10-9afc-b16c793e7466` (`Ampere`) completed a
read-only review and was closed after its findings were integrated.

Oracle follow-up session `follow-up-for-mirrorea-package` completed and was
used as advisory review input. No Oracle session remains needed for this
package.
