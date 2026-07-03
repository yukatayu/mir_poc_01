# Report 2144 - OBL-025 repair-shape statement refinement

- Date: 2026-07-04 01:54 JST
- Author / agent: Codex
- Scope: LAB OBL-025 Lean statement-shape refinement
- Decision levels touched: L3 LAB statement draft only

## Objective

Refine the LAB-only OBL-025 Lean statement draft so the boundary introduced by
`plan/96` is explicit in the compile-checked statement shape:

- single-source-edit repair witnesses can be covered;
- set insertion can enter only if it also satisfies the current single-edit
  coverage relation;
- conjunctive bundles and partial guidance are not current single-edit
  coverage;
- one child repair or one missing-failure atom cannot be mistaken for whole
  rejected-gap coverage;
- OBL-025 remains compile-check-only with no proof or canon ledger movement.

## Scope and assumptions

Scope included:

- `samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.lean`.
- The matching explanation docs under `samples/lean/lab-statements/obl025/`.
- `plan/87` and source-traceability / snapshot docs.
- Lean compile-check and manifest sync validation.

Assumptions:

- `RepairCompletenessStatementDraft.lean` remains a LAB `Prop` shape, not a
  theorem or proof.
- `ELAB-10` and `ELAB-13..16` remain the current singleton repair-bearing
  executable evidence.
- `ELAB-04/07` remain no-repair.
- Adding predicates to abstract carriers is acceptable only if the docs keep
  final ABI, ranking, and multi-edit support unclaimed.

## Start state / dirty state

Start state for this package:

- Branch: `main`
- Upstream: `origin/main`
- Starting HEAD: `a70791d77b988c57e396fba8def6e6b8aca63901`
- Start dirty state: clean at package start.
- Discord baseline: `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `/home/codex/.codex/docs/oracle-chatgpt-pro.md`
- `mirrorea_canon/README.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/theory/02-types-effects-failures.md`
- `mirrorea_canon/theory/03-elaboration.md`
- `mirrorea_canon/theory/10-diagnostics.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`
- `mirrorea_canon/spec/07-diagnostics-format.md`
- `plan/82-g1-obl025-statement-shape-inventory.md`
- `plan/87-g1-obl025-lean-statement-draft.md`
- `plan/90-source-traceability.md`
- `plan/96-g1-erow-set-insertion-bundle-payload-inventory.md`
- `samples/lean/lab-statements/README.md`
- `samples/lean/lab-statements/obl025/README.md`
- `samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.lean`
- `samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.md`
- `samples/lean/manifest.json`
- `scripts/current_l2_lean_sample_sync.py`
- `scripts/tests/test_current_l2_lean_sample_sync.py`

Oracle advisory session:

- `we-are-working-in-the-3` completed and advised avoiding a concrete repair
  shape enum / type, adding whole rejected-gap guards, and naming set
  insertion, grouped multi-edit, and partial guidance with abstract predicates
  / helper relations only.

## Actions taken

- Added abstract predicates for:
  - set-insertion repair witnesses;
  - grouped multi-edit repair witnesses;
  - repair witnesses that cover the whole rejected gap;
  - suggested repairs that cover the whole rejected gap;
  - complete local repair suggestions;
  - partial guidance suggestions.
- Updated `EligibleSingleEditRepair` so the witness must be a single-edit
  witness, must not be grouped multi-edit, and must cover the whole rejected
  gap.
- Updated `SuggestionCoversWitness` so the suggestion must be a complete local
  repair, must not be partial guidance, and must cover the whole rejected gap.
- Added helper relations for eligible set insertion, complete grouped
  multi-edit repair, and partial-guidance non-coverage without widening
  `RepairCompletenessForRejection`.
- Updated OBL-025 explanation docs and `plan/87`.
- Updated `Documentation.md`, `progress.md`, `tasks.md`,
  `samples_progress.md`, and `plan/90-source-traceability.md`.

## Files changed

- `Documentation.md`
- `docs/reports/2144-obl025-repair-shape-statement-refinement.md`
- `plan/87-g1-obl025-lean-statement-draft.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `samples/lean/lab-statements/obl025/README.md`
- `samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.lean`
- `samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.md`
- `samples_progress.md`
- `tasks.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git status --short --branch && git rev-parse HEAD origin/main
sed -n '1,260p' plan/87-g1-obl025-lean-statement-draft.md
sed -n '1,260p' samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.lean
sed -n '1,240p' samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.md
sed -n '1,260p' /home/codex/.codex/docs/oracle-chatgpt-pro.md
ask-chatgpt-pro ... --file plan/82-g1-obl025-statement-shape-inventory.md --file plan/87-g1-obl025-lean-statement-draft.md --file plan/96-g1-erow-set-insertion-bundle-payload-inventory.md --file samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.lean --file samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.md
sed -n '1,240p' samples/lean/manifest.json
rg -n 'obl025|RepairCompleteness|statement_drafts' scripts/current_l2_lean_sample_sync.py scripts/tests/test_current_l2_lean_sample_sync.py samples/lean -g '*.py' -g '*.json' -g '*.md'
sed -n '1,240p' samples/lean/lab-statements/README.md
sed -n '1,120p' samples/lean/lab-statements/obl025/README.md
lean samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.lean
python3 scripts/current_l2_lean_sample_sync.py
git status --short --branch
git diff -- samples/lean/manifest.json
oracle status --hours 2 --limit 5
oracle session we-are-working-in-the-3
rg -n 'RepairShape|RepairWitnessShape|SuggestedRepairShape|repair-shape, single-source-edit|complete-local-premise|partial-guidance-only|repair shape / single-source' Documentation.md progress.md tasks.md samples_progress.md plan/87-g1-obl025-lean-statement-draft.md samples/lean/lab-statements/obl025 docs/reports/2144-obl025-repair-shape-statement-refinement.md
date '+%Y-%m-%d %H:%M %Z'
lean samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.lean
python3 scripts/current_l2_lean_sample_sync.py
git diff -- samples/lean/manifest.json
python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync
python3 scripts/check_source_hierarchy.py
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/validate_docs.py
git diff --check
cargo fmt --check
secret scan over diff with repo-standard Discord notification URL/token patterns
git status --short --branch
```

Reviewer sub-agent also ran an independent read-only review and reported no
Lean/spec-shape blocker.

## Evidence / outputs / test results

- `lean samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.lean`
  passed.
- `python3 scripts/current_l2_lean_sample_sync.py` passed and rewrote
  `samples/lean/manifest.json` with no git diff.
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`: 9 tests
  passed.
- `python3 scripts/check_source_hierarchy.py`: required 602, present 602,
  missing 0.
- `python3 -m unittest scripts.tests.test_validate_docs`: 20 tests passed.
- `python3 scripts/validate_docs.py`: documentation scaffold complete and 1296
  numbered reports found.
- `git diff --check` passed.
- `cargo fmt --check` passed.
- Secret scan over the diff found no Discord notification URL / token patterns.
- Oracle session `we-are-working-in-the-3` completed. The accepted follow-up was
  to remove the concrete repair-shape type approach, add whole-gap guards, and
  name set insertion / grouped multi-edit / partial guidance without claiming
  coverage widening.
- Reviewer sub-agent `019f28e9-a836-7d63-ab1b-87b330bc765d` found two
  bookkeeping issues: stale report pending text and stale `samples_progress.md`
  timestamp. Both were fixed. The reviewer found no Lean/spec-shape blocker.

## What changed in understanding

The minimal safe refinement is not to add bundle completeness to OBL-025.
Instead, the single-edit coverage relation should positively require:

- `SingleEditRepairWitness`;
- not `GroupedMultiEditRepairWitness`;
- whole rejected-gap coverage for the repair witness;
- complete local repair and whole rejected-gap coverage for the suggestion;
- an explicit exclusion for partial guidance.

Set insertion is a helper relation over `EligibleSingleEditRepair`, not a new
covered class by itself. Partial guidance can be represented, but it is
explicitly excluded from `SuggestionCoversWitness`. Conjunctive bundles remain
outside current OBL-025 coverage unless a later obligation admits grouped
multi-edit witnesses.

## Open questions

- Should a future obligation cover grouped multi-edit repair completeness, or
  should OBL-025 remain single-edit only?
- If set insertion is treated as one edit, what source edit script / span
  vocabulary is required?
- Should partial guidance be emitted inside `suggested_repair[]` with an
  explicit non-coverage marker, or kept in a separate guidance field?
- How should visibility-specific repairs be represented when mixed with base
  E-ROW-001 failures?

## Suggested next prompt

Continue autonomously with `ELAB-07` set-insertion gate review or OBL-024
statement-shape work only after Oracle / reviewer feedback on this OBL-025
refinement is incorporated.

## Plan update status

`plan/` 更新済み:

- Updated `plan/87-g1-obl025-lean-statement-draft.md`.
- Updated `plan/90-source-traceability.md` with `plan/96` and the completed
  Oracle advisory session.

## Documentation.md update status

`Documentation.md` 更新済み:

- The Surface Mir LAB summary now states that the OBL-025 draft has abstract
  whole rejected-gap / set-insertion / grouped multi-edit / complete local
  repair / partial-guidance non-coverage predicates and remains
  compile-check-only.

## progress.md update status

`progress.md` 更新済み:

- Current OBL-025 note, LAB Lean statement row, and recent log now reflect the
  refinement.

## tasks.md update status

`tasks.md` 更新済み:

- OBL-025 statement refinement is recorded as current memory and removed from
  the candidate list.

## samples_progress.md update status

`samples_progress.md` 更新済み:

- Top timestamp, Lean mechanization row, and recent validation log now mention
  the OBL-025 refinement.

## Reviewer findings and follow-up

- Oracle `we-are-working-in-the-3` advised avoiding concrete repair-shape enum /
  type vocabulary, adding whole rejected-gap guards, and keeping set insertion /
  grouped multi-edit / partial guidance abstract and non-widening. Follow-up:
  implemented.
- Reviewer sub-agent `019f28e9-a836-7d63-ab1b-87b330bc765d` found stale report
  pending text and stale `samples_progress.md` timestamp. Follow-up: fixed.
- Reviewer also confirmed no Lean/spec-shape blocker: the draft remains
  statement-shape-only, `EligibleSingleEditRepair` excludes grouped multi-edit
  and requires whole-gap coverage, and `SuggestionCoversWitness` excludes
  partial guidance and requires complete local / whole-gap coverage.

## Skipped validations and reasons

No intended validation skips for this docs / Lean statement-shape package.
No broad Cargo workspace test was run because this package changed only LAB
Lean/docs/report files and no Rust source.

## Commit / push status

Pending before first commit. This report will be updated with pushed commit
identifiers in the follow-up bookkeeping commit.

## Sub-agent session close status

Reviewer sub-agent `019f28e9-a836-7d63-ab1b-87b330bc765d` completed and was
closed.
