# 0790 Runnable Status Audit And Snapshot Rewrite

## Objective

Re-audit how far implementation and execution-backed comparison has progressed, determine whether current mapped examples are already in a runnable corrected state, and rewrite `progress.md` / `tasks.md` plus relevant `plan/` / docs snapshots so they no longer overstate final-public completion or understate the remaining reserve and mixed-gate work.

## Scope and assumptions

- Scope is limited to snapshot and repository-memory alignment for current status, remaining work, and self-driven viability.
- The task does not introduce new normative decisions beyond restating already source-backed current first lines and stop lines.
- "Runnable" in this report means repo-local runner / CLI / regression / test-backed execution for the current mapped corpus, not final parser-backed public language completion.

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `.docs/progress-task-axes.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `faq_008.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/06-surface-notation-status.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`

## Actions taken

1. Re-read the status / roadmap / autonomy / traceability chain from `README.md` through the required specs, snapshots, and relevant `plan/` files.
2. Audited drift between:
   - current runnable evidence
   - `progress.md` / `tasks.md`
   - `Documentation.md`
   - `plan/01`, `plan/11`, `plan/17`, `plan/18`
   - condensed research abstracts
3. Rewrote `progress.md` from scratch so it distinguishes:
   - current mapped runnable floor
   - reserve packages
   - mixed gates
   - true user-spec residuals
4. Rewrote `tasks.md` from scratch so the queue is explicit, nonzero, and no longer conflates runnable-floor completion with final-public completion.
5. Updated `Documentation.md`, relevant `plan/` files, and condensed docs to remove stale "queue none" or equivalent readings.
6. Prepared traceability follow-up in `plan/90-source-traceability.md`.

## Evidence / outputs / test results

- Docs integrity:
  - `python3 scripts/validate_docs.py`
  - Result: passed (`Documentation scaffold looks complete.`)
- Working tree integrity:
  - `git diff --check`
  - Result: passed
- Fixed-subset authored source inventory:
  - `python3 scripts/current_l2_source_sample_regression.py inventory`
  - Result: authored sixteen present with expected static/runtime/formal-hook staging
- Representative execution / comparison evidence:
  - `CARGO_BUILD_JOBS=1 cargo test -p mir-runtime --test current_l2_source_sample_runner --test current_l2_operational_cli --test current_l2_theorem_prover_binding_preflight --test current_l2_theorem_lean_stub_trace_alignment_bridge --test current_l2_authoritative_room_vertical_slice_actualization --test current_l2_order_handoff_surface_actual_adoption`
  - Result: all selected suites passed
    - source sample runner: 22 passed
    - operational CLI: 12 passed
    - theorem prover binding preflight: 4 passed
    - theorem Lean-stub representative trace alignment: 6 passed
    - authoritative-room vertical slice: 3 passed
    - order-handoff surface actual adoption: 3 passed

## What changed in understanding

- The current mapped corpus is already beyond the "can corrected examples run?" milestone.
- Remaining work is not primarily runtime enablement; it is reserve narrowing, trace-alignment strengthening, optional external-tool probing, and mixed/user-spec gates for final public surfaces.
- Several snapshot and condensed-memory docs still implied a false-closeout reading even though the source-backed queue remained nonzero.
- `progress.md` and `tasks.md` needed a full rewrite, not incremental edits, to accurately separate:
  - runnable corrected floor
  - reserve packages
  - mixed gates
  - true user-spec residuals

## Open questions

- Whether Package 46 should keep `serial on ...` as a pure reserve sugar or retire it after helper-local comparison.
- How far Package 47 should go in emitted-contract trace alignment before it starts pressuring final public witness/provider/artifact contracts.
- Whether local Lean availability justifies Package 48 in this environment, or whether that package should remain dormant until the tool is explicitly present.

## Suggested next prompt

Continue with Package 46 or Package 47 from the rewritten `tasks.md`, keeping the runnable floor fixed and treating all remaining work as reserve or mixed-gate narrowing rather than final public-language completion.

## Maintenance notes

- `plan/` updated in this task: yes
- `progress.md` updated in this task: yes
- `tasks.md` updated in this task: yes
