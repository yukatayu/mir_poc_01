# Report 2142 - G1 E-ROW mixed / multi repair decomposition inventory

- Date: 2026-07-04 01:14 JST
- Author / agent: Codex
- Scope: G1 LAB Surface-to-Core E-ROW mixed / multi repair policy
- Decision levels touched: L3 LAB repository memory only

## Objective

Create a docs-only inventory for `ELAB-04` and `ELAB-07` that explains why
mixed / multi-missing E-ROW rows remain no-repair after the singleton repair
prototype.

The package must not widen executable `suggested_repair[]`. It must not claim
final diagnostic/repair ABI, OBL-025 proof or completion, explanation
completeness, repair ranking, multi-edit support, conformance, or G1 exit.

## Scope and assumptions

Scope included:

- `ELAB-07` as non-visibility multi-missing `E-ROW-001`.
- `ELAB-04` as mixed visibility / non-visibility multi-missing E-ROW
  evidence.
- The distinction between complete local repair witnesses, conjunctive repair
  bundles, partial repair guidance, and multi-edit deferral.
- Updates to repository memory and snapshot docs only.
- Validator registration for the new `plan/95` document.

Assumptions:

- The current executable singleton gate from `plan/93` / `plan/94` remains
  correct for `missing_failures.len() == 1`.
- A repair item should not be emitted if applying that item alone leaves the
  reported row-containment premise false, unless the payload explicitly says it
  is partial guidance or part of a conjunctive bundle.
- `VisibilityDenied` must not be collapsed into base capability / witness /
  route / membership failures.

## Start state / dirty state

Start state for this package:

- Branch: `main`
- Upstream: `origin/main`
- Starting HEAD: `3ecb9b13c8b143d1cfcd4c3a2f3468e40e10246e`
- Start dirty state: clean at package start.
- Discord baseline: `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`

An initial Oracle command failed before starting a browser session because one
attached `ELAB-04` path used an obsolete root name. The corrected Oracle run
completed as session `we-are-working-in-a`.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `.docs/progress-task-axes.md`
- `tasks.md`
- `samples_progress.md`
- `/home/codex/.codex/docs/oracle-chatgpt-pro.md`
- ChatGPT Pro Oracle consult `we-are-working-in-a`
- `plan/00-index.md`
- `plan/82-g1-obl025-statement-shape-inventory.md`
- `plan/83-g1-erow-repair-payload-inventory.md`
- `plan/88-g1-erow-repair-shape-inventory.md`
- `plan/90-source-traceability.md`
- `plan/93-g1-erow001-singleton-repair-assumption.md`
- `plan/94-g1-erow001-singleton-repair-prototype.md`
- `samples/full-system-v1-surface/elaboration/elab-04-undeclared-generated-failure-negative/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/expected/elaboration.json`
- `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- `docs/reports/TEMPLATE.md`

## Actions taken

- Added `plan/95-g1-erow-mixed-multi-repair-decomposition-inventory.md`.
- Recorded `ELAB-07` as no-repair until set insertion, conjunctive bundle,
  partial-repair guidance, or multi-edit deferral is explicitly chosen.
- Recorded `ELAB-04` as no-repair until the same axes plus visibility-family
  decomposition and ordering / ranking are explicitly chosen.
- Documented the hidden failure mode where one repair item per missing failure
  would be a partial edit that does not discharge the local row-containment
  premise.
- Updated `plan/00-index.md` and `plan/90-source-traceability.md`.
- Updated `plan/88` and `plan/94` to point at the new mixed / multi boundary
  inventory.
- Updated `Documentation.md`, `progress.md`, `tasks.md`, and
  `samples_progress.md`.
- Updated validator required-path lists for `plan/95`.
- Started and incorporated a ChatGPT Pro Oracle advisory review for the mixed
  / multi policy.

## Files changed

- `Documentation.md`
- `plan/00-index.md`
- `plan/88-g1-erow-repair-shape-inventory.md`
- `plan/90-source-traceability.md`
- `plan/94-g1-erow001-singleton-repair-prototype.md`
- `plan/95-g1-erow-mixed-multi-repair-decomposition-inventory.md`
- `progress.md`
- `samples_progress.md`
- `scripts/check_source_hierarchy.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/validate_docs.py`
- `tasks.md`
- `docs/reports/2142-g1-erow-mixed-multi-repair-decomposition-inventory.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
sed -n '1,260p' /home/codex/.codex/docs/oracle-chatgpt-pro.md
git status --short --branch
git rev-parse HEAD origin/main
ask-chatgpt-pro ... --file samples/full-system-v1-surface/elaboration/elab-04-cross-locus-write-negative/expected/elaboration.json ...
rg -n '"sample_id": "ELAB-0[47]"|ELAB-04|ELAB-07' samples/full-system-v1-surface/elaboration/matrix.json samples/full-system-v1-surface/elaboration -g 'README.md'
rg --files samples/full-system-v1-surface/elaboration | rg 'elab-0(4|7).*expected/elaboration\.json'
ask-chatgpt-pro ... --file samples/full-system-v1-surface/elaboration/elab-04-undeclared-generated-failure-negative/expected/elaboration.json --file samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/expected/elaboration.json ...
oracle status --hours 2 --limit 5
oracle session we-are-working-in-a
```

Validation commands run:

```bash
python3 scripts/surface_mir_samples.py run ELAB-04 --format json | python3 -c '...'
python3 scripts/surface_mir_samples.py run ELAB-07 --format json | python3 -c '...'
python3 scripts/surface_mir_samples.py check-all --format json | python3 -c '...'
python3 scripts/check_source_hierarchy.py
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/validate_docs.py
git diff --check
git diff | rg '<webhook-secret-patterns>' || true
```

## Evidence / outputs / test results

Evidence observed before validation:

- `ELAB-04` expected JSON has `missing_failures`:
  `MissingWitness`, `RouteUnavailable`, `StaleMembership`, `VisibilityDenied`;
  it has no `suggested_repair`.
- `ELAB-07` expected JSON has `missing_failures`:
  `MissingWitness`, `RouteUnavailable`, `StaleMembership`; it has no
  `suggested_repair`.
- Current Rust repair emission path returns `None` unless
  `missing_failures.len() == 1`.
- `oracle status --hours 2 --limit 5` first showed corrected Oracle session
  `we-are-working-in-a` as running, then later as completed.
- Oracle session `we-are-working-in-a` recommended the same no-repair policy
  for `ELAB-04/07`, emphasized omission rather than empty
  `suggested_repair: []`, and called out the partial-repair risk when several
  singleton suggestions are emitted for one multi-missing premise.

Validation results:

- `ELAB-04` helper run: accepted, no mismatches, missing failures
  `MissingWitness`, `RouteUnavailable`, `StaleMembership`,
  `VisibilityDenied`, and no `suggested_repair`.
- `ELAB-07` helper run: accepted, no mismatches, missing failures
  `MissingWitness`, `RouteUnavailable`, `StaleMembership`, and no
  `suggested_repair`.
- `python3 scripts/surface_mir_samples.py check-all --format json`: summarized
  as `sample_count = 52`, `failed = []`, `workflow_ready = False`.
- `python3 scripts/check_source_hierarchy.py`: required 601, present 601,
  missing 0.
- `python3 -m unittest scripts.tests.test_validate_docs`: 20 passed.
- `python3 scripts/validate_docs.py`: documentation scaffold complete; 1294
  numbered reports found.
- `git diff --check`: passed.
- Discord webhook secret scan over unstaged diff: no matches.

## What changed in understanding

The main hidden issue is that "one item per missing failure" is not equivalent
to a complete local repair for a multi-missing row. Adding only one missing
failure to `ELAB-07` would still leave the row-containment premise false.

Therefore mixed / multi rows need a different vocabulary before widening:

- set insertion as one edit;
- conjunctive bundle semantics;
- partial repair guidance that does not claim local-premise discharge;
- or explicit multi-edit deferral.

`ELAB-04` additionally requires a visibility-family split because
`VisibilityDenied` has `E-ROW-002`-specific meaning in singleton form.

## Open questions

- Is adding a set of missing failures to one `fails` row one source edit?
- If multiple edits are required together, what payload marks them as
  conjunctive rather than alternatives?
- Should partial repair guidance ever live in `suggested_repair[]`, or should
  it use a different field?
- How should mixed rows expose the `E-ROW-001` / `E-ROW-002` split without
  duplicate or conflicting diagnostics?
- Should the next proof-side step refine OBL-025 around the single-edit /
  multi-edit boundary before any payload vocabulary is widened?

## Suggested next prompt

Continue autonomously with a docs-only `E-ROW set-insertion / bundle payload
inventory`, unless Oracle feedback recommends refining OBL-025 first.

## Plan update status

`plan/` 更新済み:

- Added `plan/95-g1-erow-mixed-multi-repair-decomposition-inventory.md`.
- Updated `plan/00-index.md`, `plan/88`, `plan/90`, and `plan/94`.

## Documentation.md update status

`Documentation.md` 更新済み:

- The Surface Mir LAB summary now mentions the mixed / multi decomposition
  inventory and keeps `ELAB-04/07` no-repair.

## progress.md update status

`progress.md` 更新済み:

- Current E-ROW notes, next gap, feature row, and recent log now include
  `plan/95`.

## tasks.md update status

`tasks.md` 更新済み:

- The mixed / multi decomposition inventory is recorded as current memory.
- The next candidate is set-insertion / bundle payload inventory.

## samples_progress.md update status

`samples_progress.md` 更新済み:

- Dashboard rows and recent validation log now mention `plan/95`. Sample row
  count is unchanged.

## Reviewer findings and follow-up

Oracle advisory session `we-are-working-in-a` completed and found the current
local policy direction consistent:

- keep `ELAB-04/07` no-repair;
- do not emit empty `suggested_repair` arrays;
- document atomicity, decomposition, per-item validity, bundle semantics,
  ordering/ranking, request association, target policy, visibility policy,
  empty-list semantics, and coincident diagnostics before any future widening.

Follow-up:

- Added empty-list semantics and coincident-diagnostic axes to `plan/95`.
- Updated `plan/90` from pending to completed advisory.

Reviewer sub-agent `019f28c7-277c-7a31-9726-de1ec50bdfb5` reported one medium
finding:

- `samples_progress.md` already recorded this package as `docs-only pass`,
  while this report still described validation as pending.

Follow-up:

- Updated `Commands run`, `Evidence / outputs / test results`, and skipped
  validation wording with the completed validation evidence.

The reviewer found no other issues: `plan/95` keeps `ELAB-04/07` no-repair,
distinguishes `ELAB-07` from `ELAB-04`, avoids final ABI / OBL-025 / ranking /
multi-edit / conformance / G1-exit claims, and validator lists include
`plan/95`.

## Skipped validations and reasons

No intended validation skips.

## Commit / push status

Implementation commit:

- `4b05aad4fead4a4212ea5e4cde8649d0052a7d84`
  (`Add G1 E-ROW mixed multi repair decomposition inventory`)

Push status:

- Pushed to `origin/main`.
- Verified immediately after push that local `HEAD` and `origin/main` both
  pointed at `4b05aad4fead4a4212ea5e4cde8649d0052a7d84`.

## Sub-agent session close status

Reviewer sub-agent `019f28c7-277c-7a31-9726-de1ec50bdfb5` closed after its
finding was addressed. Oracle browser session `we-are-working-in-a` completed.
