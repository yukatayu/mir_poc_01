# Report 2576 — goal-first semantic integration and I1 entry

- Date: 2026-08-01
- Author / agent: Codex
- Scope: Record the user-approved goal as an active goal, create a Canon-first
  LAB authority-aware semantic-cut packet, and synchronize the reader-facing
  current status. This package creates no Canon decision, formal model,
  prototype, or implementation contract.
- Decision levels touched: none. D0/D3/D4 are LAB candidate boundaries and
  C1-A/C2-A are amendment hypotheses used only for plan/report comparison, not
  L0/L1/L2/L3 Canon records, a prototype, an OBL, Gate/Phase change,
  C-static/C-runtime result, or implementation authorization.

## Objective

Make the project move toward its actual product goal without treating the
official fixed-control governance drift as a program-semantics blocker. The
near-term objective is an authority-aware, falsifiable comparison that can
identify the exact Core/SCN amendment needed before a common semantic model or
I1 single-process reference implementation is attempted. Mir, Mirrorea,
Typed-Effect Wiring, PrismCascade, and outer adapter/view layers remain
separate.

## Scope and assumptions

The goal assumes the Canon direction in P012, P013, P015, P016, and P017, but
does not extend any of them. `World` and `Game` remain user-defined Mir
concepts. The package preserves the official lifecycle fact that the repository
is at T0 and its v2 governance artifact is a valid `fail`; it only establishes
an autonomous plan/report comparison lane. A retained formal model, prototype,
or implementation that requires a Core, Config, SaveObject, failure, SCN,
profile, or external-contract change must wait for an ordinary proposal and
owner/Canon selection.

## Start state / dirty state

At `25b53a8be7c3ece8c1d1c3d5b4b1470472473334`, `HEAD` equalled
`origin/main`, the worktree was clean, and the latest commit was
`docs: add WRK-0046 frontier checkpoint`. Disk availability was approximately
12 GiB on the repository filesystem, so this documentation/theory package does
not generate new heavyweight build artifacts. The previous current snapshot
correctly described T0 and the bounded, non-promoted WRK-0045/0046 evidence,
but still made semantic composition appear serially blocked behind governance.

## Documents consulted

- Operations: `AGENTS.md`, `CANON.md`, `.docs/progress-task-axes.md`, the
  repository-local Oracle operation notes, and the Discord-report skill.
- Canon entry and direction: `mirrorea_canon/README.md`, `MAP.md`,
  `NORTH-STAR.md`, theory 01--08 and 11, ADR-0013, ADR-0014, P012, P013,
  P015, P016, P017, the Gate/Phase/conformance documents, and SCN-01--10.
- LAB/current state: `README.md`, `Documentation.md`, `docs/project-status.md`,
  `progress.md`, `tasks.md`, `samples_progress.md`, Plans 180, 196, 197, 199,
  and 227, the Full System V1 runner, and Report 2575.
- Advisory review: a temporary Oracle consultation on the goal-first T1/I1
  consolidation question, a planner review, and a code-evidence mapping review.
  External consultation is advisory only; its useful conclusions are restated
  here and in Plan 246 rather than retained as normative state.

## Actions taken

1. Registered the user-approved, detailed project goal after the user cleared
   the prior goal. The active goal explicitly separates official lifecycle
   acceptance from autonomous semantic integration and requires package-level
   evidence, rollback triggers, commit, push, and status synchronization.
2. Re-read the Canon-first theory and lifecycle sources against the actual
   end-to-end target. Confirmed that a World/Game primitive, transport-as-
   authority shortcut, untyped observation leak, or a fake E2E wrapper would
   contradict the stated architecture.
3. Used independent planning, code mapping, and Oracle consultation to check
   whether fixed-control drift is a semantic blocker. The convergent conclusion
   is that it blocks official Gate/Phase movement, not the construction and
   falsification of a common semantic model.
4. Added Plan 246. It records the dual lane, the I1 boundary, D0/D3/D4 LAB
   candidates, D1/C1-A and D2/C2-A amendment hypotheses, exact
   non-guarantees, decisive falsifiers, SCN consumers, reuse boundary for Full
   System V1, and the authority-aware S1--S6 execution sequence.
5. Refined D1 after adversarial review: a same-target-owner read-dependent
   assignment is a bounded owner transition that performs owned reads, pure
   calculation, and validated mutation in one SW1 occurrence. The hypothesis
   applies only to target-owner reads; cross-owner operands are deliberately
   deferred to a separate proposal choice. This is not a distributed
   transaction and is explicitly falsified by a stale blind-write/lost-update
   trace.
6. Updated stable lookup, project status, progress snapshot, task map, and the
   Plan index. No runnable sample or dashboard row was changed.

## Files changed

- `plan/246-goal-first-semantic-integration-and-i1-entry.md`
- `plan/00-index.md`
- `plan/196-t0-t2-implementation-entry-roadmap.md`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `docs/reports/2576-goal-first-semantic-integration.md`

## Commands run

- Goal-state inspection and active-goal registration.
- Git baseline, origin parity, dirty-state, disk/memory, and diff checks.
- Canon/LAB source reads and targeted cross-reference searches.
- Discord task baseline: `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`.
- Temporary Oracle consultation and status/result retrieval.
- Independent planner and code-evidence mapping reviews.
- Initial `make docs` exposed the required registration of the new numbered
  Plan; added it to both documentation scaffolds.
- Final commands: `make docs`, three focused numbered-Plan unit tests,
  `make cargo-check`, `cargo test -q -p mir-semantics --test typed_ir_interpreter`,
  and `python3 scripts/full_system_v1_samples.py check-all --format json`.

## Evidence / outputs / test results

The source audit found no foundational contradiction among the existing
direction documents. The material gap is integration: total elaboration;
same-owner read-dependent update; request/validation/service/result/receipt/
one-shot-use relation state; scalar terminal; and cut/save/load closure must
be placed in one non-opaque model before a reference implementation can claim
to represent Mir semantics. The final reviewer established that D1/D2 change
current Core/SCN wording, so this package stops before constructing that model.

The code-evidence mapper independently reproduced existing bounded LAB
evidence: the typed IR interpreter test had 20 passes, the Full System V1
runtime session test had 17 passes, the CLI test had 10 passes, and
`full_system_v1_samples.py check-all --format json` reported 50 passing rows.
Those results show reusable parser/checker/runtime/harness material; they do
not establish Canon conformance.

The main-agent final checks passed: Canon index `134` files, source hierarchy
`796/796`, documentation scaffold with `1730` numbered reports, all three
focused Plan-registration tests, Cargo workspace check, the 20-test typed-IR
suite, and all 50 Full System V1 `check-all` rows. `git diff --check` passed.

## What changed in understanding

T0 had become an overloaded label: it correctly describes official lifecycle
state, but it should not make every plan/report comparison wait for a
fixed-control disposition that has no program-meaning effect. The correct
control is two explicit lanes that rejoin before official acceptance, not a
phase shortcut. The autonomous lane is still bounded: a Core/SCN hypothesis is
not a license to create the corresponding model, Lean artifact, or runtime.

The key SCN-02 correction is also now concrete. Owner seriality of already
computed writes is insufficient against stale read/write races. C1-A is a
precise amendment hypothesis for a target-owner transition, not current-Core
semantics. It preserves ordinary source ergonomics without inventing a
distributed transaction, while explicitly deferring cross-owner operand and
nested-request semantics.

## Open questions

- The exact accepted `Surface/Core/Config/Step/WellFormed/Elab/SaveLoad`
  carrier and statement domain remain OPEN; D0--D4 are a test target, not an
  accepted model.
- The precise X relation fields, result/receipt mapping, generated-edge form,
  scalar representation, source fragment, and failure classification remain
  OPEN until the common model exposes their amendment surface.
- The fixed-control disposition, official T1/T2 profiles, C-static timing,
  ledger mapping, and I1 authorization remain independent owner/Canon matters.

## Suggested next prompt

Continue S2-A: in `plan/` and a new task report only, compare C1-A with the
determined-value alternative and C2-A with the no-receipt-state status quo.
Record adverse traces, exact current-Core/SCN deltas, non-effects, and rollback
conditions, then prepare the ordinary owner/Canon amendment packet. Do not add
Lean/model/runtime artifacts before a selection.

## Plan update status

`plan/` 更新済み: Plan 246 is the detailed repository-memory packet for the
new goal-first lane; Plan 196 now states that official lifecycle governance
does not serialize semantic research, and `plan/00-index.md` registers Plan
246. No historical plan was rewritten as Canon.

## Documentation.md update status

`Documentation.md` 更新済み: added the concise lookup entry and the
Canon-versus-LAB working-candidate boundary for Plan 246.

## docs/project-status.md update status

更新済み: added the dual-lane current status, D0--D4 scope, immediate
falsifiers, and the explicit ordinary-proposal rejoin point.

## progress.md update status

`progress.md` 更新済み: the three-axis current snapshot now records S2 as the
autonomous semantic mainline while retaining official T0 and all open OBL/Gate
facts. The recent log records that no Canon or implementation authorization
changed.

## tasks.md update status

`tasks.md` 更新済み: S1 is marked as the completed planning packet; S2 and S3
are listed as the current self-driven route, and CP-3 is clarified as official
acceptance rather than a prerequisite to S2 research.

## samples_progress.md update status

`samples_progress.md` 更新不要: this package changes no runnable sample,
runner, validation command, debug surface, or sample workflow readiness.

## Reviewer findings and follow-up

The completed planner found that adding further fixture-only WRKs would delay
the I1 route; it recommended one integrated semantic-cut packet and a separate
official governance lane. The completed code mapper found that Full System V1
has useful parser/checker/runtime/report/harness components but cannot be
relabeled as Canon semantics. The temporary Oracle review recommended the
bounded target-owner D1 transition rather than stale remote read followed by
blind write.

The independent reviewer initially found seven blocking accuracy and authority
problems: D1 was presented too much like current Core, S2 lacked an ADR-0014
entry guard, cross-owner operands were underspecified, D2 mixed static `G_e`
with dynamic receipt facts, the non-transactional contrast lacked a trace,
S3/S4 were too early, and status views overstated closure. The packet was
revised to make D1/D2 amendment hypotheses, defer cross-owner operands, keep
receipt out of existing `G_e`, add the contrast trace and S2-A guard, and
sequence S2-B/S3 after ordinary selection. Narrow re-review found those seven
issues resolved and required only this report synchronization, which is now
applied. It also passed `git diff --check`.

## Skipped validations and reasons

No new Lean model, parser, runtime, transport, or sample source exists in this
documentation/planning package, so an I1 claim, C-static/C-runtime claim, or
new proof claim is not tested or made. Full System V1 is rerun only as bounded
LAB regression evidence. Visual/browser review is not relevant because no HTML
or frontend presentation changed.

The combined command
`python3 -m unittest scripts.tests.test_validate_docs scripts.tests.test_full_system_v1_samples`
was stopped after more than one hour. It had completed only eight tests while
performing over 55 million read system calls and approximately 23 GiB of reads,
then repeatedly blocked in process creation; no assertion failure was emitted.
It is explicitly **not** counted as passing. The direct validator and the
three tests that cover the newly changed Plan-registration contract passed in
separate focused runs.

## Commit / push status

Pending final reviewer result, local validation, scoped commit with
`--no-gpg-sign`, push to `origin/main`, and post-push parity verification.

## Sub-agent session close status

Planner `Hegel` completed and has been closed after returning its roadmap
review. Code mapper `Ramanujan` completed and was closed after returning its
test and reuse-boundary evidence. Reviewer `Jason` completed the initial and
narrow follow-up reviews, confirmed the final correction, and was closed. The
temporary Oracle consultation completed; it is an external advisory input, not
retained project state.
