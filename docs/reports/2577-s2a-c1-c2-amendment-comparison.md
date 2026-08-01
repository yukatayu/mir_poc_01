# Report 2577 — S2-A C1/C2 amendment comparison

- Date: 2026-08-01T02:30:30.096370Z (start), 2026-08-01 12:43 JST (closeout)
- Author / agent: Codex, with independent planner, temporary Oracle advisory,
  and two independent reviewers
- Scope: LAB `plan/` / `docs/` comparison only
- Decision levels touched: no L0/L1/L2 decision changed; C1/C2 remain ordinary
  Core/SCN amendment choices for owner/Canon selection

## Objective

Complete the bounded S2-A comparison required by Plan 246 without treating an
unselected Core/SCN amendment as current semantics. The intended consumer is a
later I1 shared kernel model: identify the minimum owner decisions for the
SCN-02 read-dependent write and the P017 X1 request/result/receipt relation,
record adverse traces and non-effects, and stop before model/prototype work.

## Scope and assumptions

- `mirrorea_canon/` remains the normative source. This report neither edits nor
  resolves Canon.
- C1 concerns SCN-02 only when all dynamic RHS state dependencies are owned by
  the write target owner. It does not introduce a multi-owner transaction,
  transport protocol, cache, retry, or final source syntax.
- C2 concerns relation-state residence for cross-locus V1/R1 result and receipt;
  it does not select a carrier, field names, public identifier, message format,
  global exactly-once guarantee, or persistence implementation.
- Existing runnable LAB code and prior finite Lean evidence remain evidence, not
  semantic truth, Gate input, or implementation authorization.

## Start state / dirty state

The starting commit was `abe19b292e20e0e38ee4678bdf8af661343ffac8` on `main`,
equal to `origin/main`, with a clean worktree. S1 had separated the official
T0 governance lane from the goal-first semantic lane and had corrected C1/C2
into amendment hypotheses. This package began with only this task's Plan 246
edit and this new report untracked. Resource check at start: `/` had 7.5 GiB
available (96% used); 9 GiB RAM was available. No heavy build or generated
artifact was created.

## Documents consulted

- Canon hierarchy: `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`,
  `CANON.md`.
- Canon semantics and direction: `theory/01-mircore-v0.md`,
  `theory/03-elaboration.md`, `theory/04-history-and-cuts.md`,
  `theory/05-authority-and-contracts.md`, `theory/06-failure-and-fallback.md`,
  `meta/proposals/PROPOSAL-012-mircore-value-flow-and-occurrence-identity.md`,
  `PROPOSAL-013-post-admission-request-validation-context.md`, and
  `PROPOSAL-017-c2b-c3-relation-state-envelope.md`.
- Canon scenarios: `SCN-01` through `SCN-10`, with direct recheck of
  `scenarios/SCN-02-attack.md`.
- Canon process: `adr/ADR-0013.md`, `adr/ADR-0014.md`.
- LAB: `plan/246-goal-first-semantic-integration-and-i1-entry.md`, Report 2576,
  `Documentation.md`, `docs/project-status.md`, `progress.md`, `tasks.md`,
  `.docs/progress-task-axes.md`, and `samples_progress.md`.
- Advisory inputs: an independent planner sub-agent and one temporary
  browser-backed Oracle consultation. Both are advisory only; their useful
  conclusions are recorded here and in Plan 246 rather than treated as
  normative state.

## Actions taken

1. Re-read the Core, elaboration, P012/P013/P017, and all-ten-scenario
   boundaries against the S1 hypotheses.
2. Compared three C1 outcomes: `C1-A-r` owner-sampled same-owner RMW,
   current-shape `C1-B` requester-sampled determined value, and honest defer.
3. Compared `C2-A-r`, a candidate-specific extension of the P017 X1
   relation-state direction, against no semantic receipt-state defer. Kept C1
   and C2 separable: C1-A-r does not need a requester receipt for its in-scope
   RHS reads; C1-B does.
4. Ran adverse traces for lost update, authority-less private operand,
   other-owner dependency, request collision, failure, duplicate receipt, and
   save/restore consumption reset.
5. Identified a baseline reconciliation that LAB cannot silently repair:
   SCN-02 requires dependency rows for both `player[target].hp` and
   `player[self].atk`, whereas the theory/03 worked shape displays only `atk`.
   Both are called cross-locus in SCN-02, while `[READ-CROSS]` requires
   visibility/observe authority and generated-failure containment. SCN-02 has
   a `fails` row, but no visibility declaration and no `VisibilityDenied`
   containment. This is an owner/Canon question, not proof that the project
   goal is inconsistent.
6. Updated Plan 246 and reader-facing status snapshots to make the completed
   comparison, recommendation, stop line, and no-Canon-effect explicit.
7. Recorded the owner's new autonomous stop condition: once I1 is actually
   startable, close out the theory/readiness entry and stop before the first I1
   implementation package rather than silently beginning it.

## Files changed

- `plan/246-goal-first-semantic-integration-and-i1-entry.md`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `docs/reports/2577-s2a-c1-c2-amendment-comparison.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `git status --short --branch`; `git log -3 --oneline`; `date -Is`; `df -h .`;
  `free -h`
- Canon/LAB targeted `sed` and `rg` inspections, including direct SCN-02,
  Core rule, and elaboration worked-shape reads
- `python3 scripts/validate_docs.py` after report-status declaration correction
- `make docs` (final full documentation validation)
- `git diff --check`

One initial path probe used stale filenames for Canon theory/proposal documents
and returned file-not-found errors. It made no change and was immediately
replaced with `rg --files` followed by reads of the actual paths named above.

## Evidence / outputs / test results

- Direct source evidence: SCN-02 says both `player[target].hp` and
  `player[self].atk` generate cross-locus dependency rows. The theory/03
  worked shape shows only the latter. Core `[READ-CROSS]` requires declared
  visibility/observe authority and generated failure containment. This proves
  an elaboration/authority alignment question, not a settled repair.
- C1-A-r preserves normal write authorization but does not derive private read
  authority from it. For same-owner `hp=100, atk=10` attacks, serial owner
  services yield 80; C1-B can yield 90 through two earlier reads. Frozen SCN-02
  gives only the one-attack 100-to-90 expectation, so this is a choice trace,
  not a current-Canon counterexample.
- C2-A-r proposes an `X` entry anchored injectively to request occurrence `q`.
  Owner result/failure, requester receipt, and one-shot use are dynamic facts,
  not existing static `G_e` rows. Its proposed occurrence/consumption
  presentation, static response path, history/cut, SaveObject, restore, and
  optional redacted-observation deltas are amendment surfaces; P017 X1 alone
  does not select them.
- `make docs` passed: Canon index check reported 134 files; source hierarchy
  reported 796 required and present, zero missing; documentation validation
  completed successfully and reported 1,731 numbered reports.
- The first final `make docs` exposed a report-only validation error: the
  `docs/project-status.md` update declaration was enclosed in backticks, while
  the validator requires exactly one line beginning `更新済み:` or `更新不要:`.
  The validator rule and successful historical reports were read, the marker
  was corrected without changing its claim, the targeted validator passed, and
  the final full `make docs` pass above confirmed the correction.
- No source implementation or sample changed, so no runtime, Cargo, Lean, or
  sample validation was represented as evidence for this documentation-only
  comparison.

## What changed in understanding

The principal result is a smaller and more honest decision boundary. C1-A-r is
the conditionally recommended way to obtain local read-modify-write semantics
without silently creating a distributed transaction, but it needs an explicit
SCN-02 authority/dependency reconciliation. C1-B preserves the current
already-determined-value rule shape but requires C2-style result/receipt state
and has no RMW guarantee. C2-A-r remains the recommended candidate-specific
extension of P017's X1 relation envelope; defer remains valid only if
cross-locus result-dependent computation is deferred too.

Thus S2-A is complete as comparison evidence. It does not make the theory
complete, authorize I1, or establish that an implementation may select a model
on its own. The next value-creating work is an ordinary amendment decision,
not another independent countermodel or a thick prototype.

## Open questions

1. For SCN-02, should both RHS reads be explicit cross-locus dependencies under
   the normal `[READ-CROSS]` visibility/failure discipline? If not, which exact
   read is owner-local and why?
2. Does the product promise cumulative concurrent same-owner attacks? If yes,
   choose C1-A-r; if no, C1-B may retain last-writer behavior but must carry
   its receipt/failure semantics.
3. Should a future operation/declassification authority ever supplement normal
   read/visibility authority? The recommendation is to preserve normal read
   authority for now and keep that possible authority as a separate proposal.
4. Should C2-A-r be selected now for cross-locus result use, including a
   separately selected static response path, history/cut, SaveObject, restore,
   and optional observation amendment surfaces, or should that capability be
   deferred?

## Suggested next prompt

Review the three-section owner packet in Plan 246 and decide: (1) the SCN-02
dependency/read-authority reconciliation, (2) C1-A-r, C1-B, or defer plus the
read-authority policy, and (3) C2-A-r or defer. After an ordinary Canon
proposal freezes the selected surface, proceed with S2-B's shared model and
small permitted Lean/prototype checks.

## Plan update status

`plan/` 更新済み: Plan 246 now contains the complete C1/C2 option matrix,
adverse traces, SCN-01..10 impact audit, owner packet, stop line, and updated
recommendation. It also fixes the I1-entry stop condition. It remains LAB
repository memory, not Canon.

## Documentation.md update status

`Documentation.md` 更新済み: current position now distinguishes completed S2-A
from S2-B's owner/Canon selection dependency and states the two decisions in
reader-facing Japanese.

## docs/project-status.md update status

更新済み: S2-A completion, SCN-02 reconciliation, and S2-B's start condition
changed the current status and route diagram. No normative status changed.

## progress.md update status

`progress.md` 更新済み: logical-specification, blocker, decision-map, and macro
phase rows now identify S2-A as complete and the ordinary C1/C2 choice as the
semantic checkpoint. A dated recent-log entry is added at task close.

## tasks.md update status

`tasks.md` 更新済み: S2-A is marked complete; CP-3 is expanded into its concrete
SCN-02/C1/C2 owner decision and S2-B is correctly blocked on that choice.

## samples_progress.md update status

`samples_progress.md` 更新不要: no runnable sample, validation command, debug
surface, or sample blocker changed.

## Reviewer findings and follow-up

The independent planner found that C1 was sufficient for a bounded comparison
but not for a settled recommendation without an owner statement on concurrent
attack behavior; it recommended C2-A-r over defer while retaining its
receipt/causality/failure/restore details as open. The temporary Oracle review
independently required the C1-A-r narrowing, preservation of read/visibility
authority, and the SCN-02 baseline reconciliation; it also confirmed that C1
and C2 must not be bundled.

The first final reviewer used a pre-report fork. Its report-template finding
therefore did not apply to the current worktree, but its substantive checks did:
C2-A-r had been abbreviated too closely to P017 X1, four SCN impact rows
overstated C2 consequences, and one status table was stale. The source was
rechecked against P017 and theory/01; the Plan, status views, and this report
now identify C2-A-r as a candidate-specific X1 extension, make the static path
an explicit choice, scope projection to export, and retain retry/fairness as a
separate concern.

The fresh final reviewer then found three remaining documentation defects:
an earlier coverage table still bundled C2 with C1-A-r, this report misstated
SCN-02 as lacking a `fails` row, and closeout timestamps/metadata were stale.
The table now distinguishes C1-A-r from C1-B plus selected R1 residence; the
report now records the exact issue as absent visibility authority and absent
`VisibilityDenied` containment in an otherwise present `fails` row; all current
status timestamps were synchronized. The reviewer cleared the C1 distinction,
SCN-02 reconciliation, C2-A-r boundary, later SCN audit, status logic, and I1
stop condition. No Canon file was edited.

## Skipped validations and reasons

- No runtime/Cargo/Lean/sample suite was run because this package changes only
  documentation and makes no executable claim. Prior runnable LAB results are
  not reused as validation of a new semantic model.
- The broader combined Python suite previously showed pathological filesystem
  activity and was not rerun; it is not needed for the changed documentation
  layer and is not counted as passing evidence.

## Commit / push status

The package content was committed as
`26f146563a7705f97ffbd52b835884652b0462a3`
(`docs: record C1 C2 amendment comparison`) and pushed to `origin/main`.
At this report update, local `HEAD` equals `origin/main`. This closeout update
is committed and pushed separately immediately after its final documentation
validation.

## Sub-agent session close status

Planner and both reviewer sub-agents completed and are closed. No sub-agent
edited repository files.
