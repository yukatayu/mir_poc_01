# Report 0909 — Sugoroku world runtime attachment vertical slice

- Date: 2026-04-23T05:31:56.939245Z
- Author / agent: Codex
- Scope: repo-local Mir / Mirrorea vertical-slice implementation, sample corpus, runner, beginner hands-on docs, and validation evidence.
- Decision levels touched: implementation and documentation mirror for current L2/L3 work; no new L0/L1 normative decision was introduced.

## 1. Objective

Implement the handoff design from `sub-agent-pro/codex_sugoroku_runtime_attachment_handoff_2026-04-23.md` as a repo-local vertical slice:

- boot an empty logical world server;
- attach `SugorokuGame#1` at runtime;
- model Alice, Bob, Carol, and Dave as participants/principals distinct from places;
- run admin-only start/reset, owner-only roll, roll -> publish -> witness -> handoff, late join, leave, owner leave, reset interleaving, detach TODO;
- expose deterministic sample commands and debug outputs;
- document the slice in beginner-oriented Japanese hands-on material;
- record validation results and current limitations.

## 2. Scope and assumptions

The implemented slice is intentionally a single OS process logical multi-place emulator. It models places as execution loci with local state and queues, not as real network processes. The slice is active under `samples/clean-near-end/sugoroku-world/` and is wired through `scripts/sugoroku_world_samples.py`.

Temporary working assumptions:

- dice output is deterministic (`draw = 4`) so examples and tests are stable;
- Dave's late join can observe published history but is not inserted into the current turn order in this slice;
- detach is represented as a lifecycle TODO boundary, with domain actions after detach rejected or returned as `todo_deferred`;
- the `.mir` files remain current companion syntax samples, not the final public parser grammar.

## 3. Documents consulted

- `sub-agent-pro/codex_sugoroku_runtime_attachment_handoff_2026-04-23.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `.docs/progress-task-axes.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/04-mir-core.md`
- `specs/05-mirrorea-fabric.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/01-status-at-a-glance.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- Existing clean-sample runner and test patterns under `scripts/`.

## 4. Actions taken

- Added ten active Sugoroku world samples under `samples/clean-near-end/sugoroku-world/`.
- Added `scripts/sugoroku_world_samples.py`, a deterministic repo-local runner for list/run/check-all/model-check/closeout commands.
- Added `scripts/tests/test_sugoroku_world_samples.py` covering sample discovery, runtime state, negative guards, JSON output, model-check properties, and limitations.
- Added beginner-oriented Japanese hands-on docs under `docs/research_abstract/`.
- Updated top-level docs, spec map/open-question/roadmap mirrors, plan memory, progress snapshot, tasks snapshot, and clean-sample README.
- Recorded resource status before heavy validation: filesystem was 91% used with about 9.2G available; memory was about 960Mi total with swap available.
- Kept the handoff file as an untracked input file. It was read but not added to the commit because it was already outside the tracked worktree scope at task start.

## 5. Implemented files

Samples:

- `samples/clean-near-end/sugoroku-world/00_world_bootstrap.mir`
- `samples/clean-near-end/sugoroku-world/01_runtime_attach_game.mir`
- `samples/clean-near-end/sugoroku-world/02_admin_start_reset.mir`
- `samples/clean-near-end/sugoroku-world/03_roll_publish_handoff.mir`
- `samples/clean-near-end/sugoroku-world/04_non_owner_roll_rejected.mir`
- `samples/clean-near-end/sugoroku-world/05_late_join_history_visible.mir`
- `samples/clean-near-end/sugoroku-world/06_leave_non_owner.mir`
- `samples/clean-near-end/sugoroku-world/07_owner_leave_reassign.mir`
- `samples/clean-near-end/sugoroku-world/08_reset_interleaving_model_check.mir`
- `samples/clean-near-end/sugoroku-world/09_detach_todo.mir`
- `samples/clean-near-end/sugoroku-world/README.md`

Runner and tests:

- `scripts/sugoroku_world_samples.py`
- `scripts/tests/test_sugoroku_world_samples.py`

Hands-on docs:

- `docs/research_abstract/hands_on_sugoroku_00_overview.md`
- `docs/research_abstract/hands_on_sugoroku_01_world_bootstrap.md`
- `docs/research_abstract/hands_on_sugoroku_02_runtime_attach.md`
- `docs/research_abstract/hands_on_sugoroku_03_admin_start_reset.md`
- `docs/research_abstract/hands_on_sugoroku_04_roll_publish_handoff.md`
- `docs/research_abstract/hands_on_sugoroku_05_join_leave.md`
- `docs/research_abstract/hands_on_sugoroku_06_model_check.md`
- `docs/research_abstract/hands_on_sugoroku_detail.md`

Documentation and planning mirrors:

- `README.md`
- `Documentation.md`
- `docs/research_abstract/README.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/01-status-at-a-glance.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`

## 6. Sample list

`python3 scripts/sugoroku_world_samples.py list` reports:

```text
00_world_bootstrap: Bootstrap an empty world server with Alice, Bob, and Carol as active participants.
01_runtime_attach_game: Attach SugorokuGame#1 at runtime and appoint Alice as game admin.
02_admin_start_reset: Accept admin start and reject non-admin reset.
03_roll_publish_handoff: Dice owner Alice rolls, publishes, witnesses, and hands off to Bob.
04_non_owner_roll_rejected: Reject Carol's roll because Bob is the current dice owner.
05_late_join_history_visible: Dave late-joins, sees published history, and remains pending for turn order.
06_leave_non_owner: Carol leaves, membership epoch advances, and stale actions are invalidated.
07_owner_leave_reassign: Bob leaves while owning dice and ownership is reassigned to Alice.
08_reset_interleaving_model_check: Model-check that reset invalidates old-epoch handoff.
09_detach_todo: Represent detach as a Mirrorea lifecycle TODO boundary.
```

## 7. Runtime model

Implemented runtime components:

- `PlaceRuntime`
- `MessageQueue`
- `WorldServerPlace`
- `ParticipantPlace`
- `SugorokuGamePlace`
- `MembershipRegistry`
- `SugorokuState`
- `RollRecord`
- `ActionRecord`

The runner keeps a deterministic runtime state in Python dataclasses. It is not a final Mir runtime, but it gives the repository an executable current-layer target for the handoff scenario.

## 8. Place model

The slice keeps participant/principal and place separate.

Principals:

- `Server`
- `Alice`
- `Bob`
- `Carol`
- `Dave`

Places:

- `WorldServerPlace`
- `ParticipantPlace[Alice]`
- `ParticipantPlace[Bob]`
- `ParticipantPlace[Carol]`
- `ParticipantPlace[Dave]`
- `SugorokuGamePlace#1`

`Place` means a logical execution locus with local state, queue, capability, and observation frontier. In this slice all places are emulated inside one OS process.

## 9. Membership model

- `membership_epoch` increments on join and leave.
- Each member has an `incarnation`, which increments on leave so stale actions can be rejected.
- Late join sees published history, but current turn-order insertion is deferred.
- Owner leave either reassigns to the next active participant or would pause if no active participant remains.
- Reset increments `game_epoch`, clears pending actions, and rejects old-epoch commits.

## 10. Static checks and runtime guards

Static-check rows reported by `check-all`:

- `admin-only start/reset`
- `owner-only roll`
- `handoff target must be active`
- `stale member incarnation cannot commit action`
- `old game_epoch action cannot commit after reset`

Runtime guards reported by `check-all`:

- `handoff requires publish witness`
- `handoff target must be active`
- `stale member incarnation cannot commit action`
- `old game_epoch action cannot commit after reset`
- `detached game rejects domain action or returns todo_deferred`

Observed rejection examples:

- `04_non_owner_roll_rejected` rejects Carol's roll because Bob is the current owner.
- `06_leave_non_owner` rejects Carol's stale old-incarnation action after leave.
- `08_reset_interleaving_model_check` rejects an old-epoch handoff after reset.
- `09_detach_todo` rejects domain action after detach.

## 11. Model-check properties

The runner reports the required properties:

- `no_double_dice_owner`
- `owner_only_rolls`
- `roll_is_published_before_handoff`
- `handoff_target_is_active`
- `late_join_sees_published_history`
- `owner_leave_reassigns_or_pauses`
- `stale_action_after_leave_is_rejected`
- `reset_invalidates_pending_actions`
- `detach_rejects_domain_actions`
- `admin_reset_does_not_interleave_with_roll_commit_badly`

`python3 scripts/sugoroku_world_samples.py model-check --format json` reports a passing reset interleaving scenario and also includes a broken reference variant where an old epoch handoff commits after reset as a counterexample.

## 12. Debug output

Implemented debug modes:

- `--debug summary`
- `--debug turn-trace`
- `--debug membership`
- `--debug verification`
- `--format json`

Example:

```text
$ python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug summary
WORLD EmptyWorld
  membership_epoch: 0
  active: Alice, Bob, Carol
  inactive: none
  attached:
    SugorokuGame#1 phase=Running epoch=0 admin=Alice dice_owner=Bob
```

Example:

```text
$ python3 scripts/sugoroku_world_samples.py run 05_late_join_history_visible --debug membership
MEMBERSHIP
  epoch: 1
  Alice active incarnation=0 place=ParticipantPlace[Alice]
  Bob active incarnation=0 place=ParticipantPlace[Bob]
  Carol active incarnation=0 place=ParticipantPlace[Carol]
  Dave active incarnation=0 place=ParticipantPlace[Dave]
```

## 13. Validation command results

All commands below completed successfully.

```text
$ python3 -m unittest scripts.tests.test_sugoroku_world_samples -v
Ran 10 tests in 0.005s
OK
```

```text
$ python3 -m unittest discover scripts/tests -v
Ran 110 tests in 0.258s
OK
```

```text
$ python3 scripts/sugoroku_world_samples.py list
10 samples listed: 00_world_bootstrap through 09_detach_todo.
```

```text
$ python3 scripts/sugoroku_world_samples.py check-all
sample_count: 10
failed: []
static checks, runtime guards, and model-check properties were reported.
```

```text
$ python3 scripts/sugoroku_world_samples.py run 00_world_bootstrap
WORLD EmptyWorld
  server place: WorldServerPlace
  membership epoch: 0
  active members: Alice, Bob, Carol
  attached components: none
```

```text
$ python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game
ATTACH SugorokuGame#1
  world: EmptyWorld
  package checked: yes
  game place: SugorokuGamePlace#1
  admin: Alice
  dice owner: Alice
```

```text
$ python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug summary
WORLD EmptyWorld
  membership_epoch: 0
  active: Alice, Bob, Carol
  attached:
    SugorokuGame#1 phase=Running epoch=0 admin=Alice dice_owner=Bob
```

```text
$ python3 scripts/sugoroku_world_samples.py run 05_late_join_history_visible --debug membership
MEMBERSHIP
  epoch: 1
  Alice active incarnation=0 place=ParticipantPlace[Alice]
  Bob active incarnation=0 place=ParticipantPlace[Bob]
  Carol active incarnation=0 place=ParticipantPlace[Carol]
  Dave active incarnation=0 place=ParticipantPlace[Dave]
```

```text
$ python3 scripts/sugoroku_world_samples.py model-check --format json
model_check_result: pass
checked_scenarios: reset_invalidates_pending_actions, admin_reset_does_not_interleave_with_roll_commit_badly
broken_variant.model_check_result: counterexample
```

```text
$ python3 scripts/sugoroku_world_samples.py closeout --format json
sample_count: 10
runtime_components: PlaceRuntime, MessageQueue, WorldServerPlace, ParticipantPlace, SugorokuGamePlace, MembershipRegistry, SugorokuState, RollRecord, ActionRecord
limitations: no real network yet; no multi-server consensus; no durable distributed commit; detach is TODO lifecycle boundary; final parser grammar remains deferred; final public API remains deferred
```

```text
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 907 numbered report(s).
```

```text
$ git diff --check
No whitespace errors reported.
```

## 14. Hands-on docs

The added Japanese hands-on docs explain the slice for readers who have only light programming experience. They describe purpose, whole picture, complete sample code, line-by-line explanations, keyword explanations, builtin/current companion syntax/user-defined boundaries, commands, pretty output, JSON output, rejection cases, and future-layer limits.

Key points made explicit:

- `Participant` / `Principal` and `Place` are not the same concept.
- `Place` is modeled as a logical execution locus inside one process for this slice.
- `publish`, `witness`, `handoff`, `membership_epoch`, and `game_epoch` are current companion syntax / current sample runtime concepts, not final public syntax commitments.
- The user-defined domain is Sugoroku-specific: admin, dice owner, turn order, roll record, and board history.
- Network transport, consensus, durable commit, detach lifecycle, final parser grammar, and final public API are future layers.

## 15. Limitations

- no real network yet
- no multi-server consensus
- no durable distributed commit
- detach is TODO lifecycle boundary
- final parser grammar remains deferred
- final public API remains deferred

Additional limitation: this is a deterministic repo-local executable sample runner, not a production runtime or a complete Mir verifier.

## 16. What changed in understanding

This task promotes the virtual-space example from documentation-only discussion to an executable repo-local vertical slice. The implementation confirms that the project can keep the following boundaries separate while still running an end-to-end sample:

- participant/principal vs place;
- static guard vs runtime guard vs model-check property;
- local single-process emulator vs future distributed runtime;
- current companion syntax vs final public parser syntax.

No normative change was made to collapse Mir, Mirrorea, PrismCascade, and the Typed-Effect Wiring Platform into one implementation.

## 17. Open questions

- How should late-join turn-order insertion be specified for the final user-facing game semantics?
- What is the final detach lifecycle: graceful stop, archival detach, tombstone, transferable game state, or another form?
- Which event-log format should become the long-term bridge to model checking and theorem artifacts?
- Where should real network transport and multi-server consensus enter the roadmap once the single-process logical place model is stable?

## 18. Suggested next prompt

Promote the Sugoroku world slice from deterministic Python runner to a parser-free Mir runtime fixture artifact pipeline: emit machine-readable traces, static-guard artifacts, and model-check traces in the same style as the clean near-end sample suite, while keeping final parser grammar and final public API deferred.
