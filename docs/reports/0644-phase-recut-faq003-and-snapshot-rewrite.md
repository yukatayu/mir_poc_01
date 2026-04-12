# Report 0644 — phase recut faq003 and snapshot rewrite

- Date: 2026-04-12T11:43:43.041457Z
- Author / agent: Codex (GPT-5)
- Scope: Recut the repo-wide phase/progress/task structure, answer the user's planning questions in `faq_003.md`, and sync snapshot/policy docs without changing normative semantics.
- Decision levels touched: Read-only on normative semantics. Snapshot / planning / policy wording updated across repo-local docs and relevant `plan/`.

## 1. Objective

- Reframe the repo's big-picture progress so that old `Phase 7 = FutureWork` is no longer the only container for everything later-stage.
- Distinguish:
  - what is already runnable now,
  - what is only docs-first,
  - what remains heavy future research,
  - what requires user decisions.
- Write a careful Japanese answer in `faq_003.md`.
- Rewrite `progress.md` and `tasks.md` from scratch so stale structure does not remain.
- Update `AGENTS.md` and `.docs` so Discord `progress` / `complete` behavior matches the user's clarified expectation.

## 2. Scope and assumptions

- Followed the repo read order: `README.md`, `Documentation.md`, `progress.md`, `specs/00..03`, `specs/09`, then relevant subsystem / roadmap / plan documents.
- Treated `specs/` as the normative source, `plan/` as repository memory, and snapshot docs as secondary mirrors.
- Did not change normative semantics. The work is a planning / explanation / mirror rewrite.
- Used supporting analysis reports `0645` and `0646`, plus doc-consistency review `0647`, as subordinate evidence within this same task.

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md` (pre-rewrite)
- `tasks.md` (pre-rewrite)
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/04-mir-core.md`
- `specs/05-mirrorea-fabric.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/04-core-semantics-current-l2.md`
- `plan/05-fallback-lease-and-chain-semantics.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `faq_002.md`
- `samples/current-l2/README.md`
- `docs/reports/0645-rust-python-implementation-split.md`
- `docs/reports/0646-feature-maturity-matrix-for-progress-faq003.md`
- `docs/reports/0647-uncommitted-doc-consistency-review.md`

## 4. Actions taken

1. Re-read the required baseline docs and current planning/status docs.
2. Split the repo's planning view into:
   - legacy checkpoint labels
   - macro phases
   - feature maturity stages
3. Wrote `faq_003.md` in Japanese, including:
   - current capability snapshot
   - the new macro phase reading
   - per-feature maturity reading
   - current recommendation on LLVM/backend timing
   - current recommendation on `atomic_cut` sample expansion vs higher-level control families
   - Rust/Python split
4. Rewrote `progress.md` from scratch to include:
   - capability snapshot
   - macro phase map
   - feature-family progress matrix
   - layer/subsystem status
   - current self-driven line
   - research-discovery vs user-decision split
5. Rewrote `tasks.md` from scratch to separate:
   - ordered self-driven packages
   - research-discovery items
   - user-decision items
6. Updated `AGENTS.md` and `.docs/continuous-task-policy.md` so:
   - `progress` means a natural checkpoint with more work still possible without user input
   - `complete` is sent only once when stopping, either because the requested scope is done or because further work needs user input
   - average intermediate-report cadence is roughly hourly rather than overly chatty
7. Added `.docs/progress-task-axes.md` as the shared rule for progress/task recuts.
8. Updated relevant mirror docs in `README.md`, `Documentation.md`, `plan/01`, `plan/10`, `plan/11`, and `plan/17`.
9. Ran targeted validation and then a reviewer pass; fixed the findings.

## 5. Files changed

- Added `docs/reports/0644-phase-recut-faq003-and-snapshot-rewrite.md`
- Added `docs/reports/0645-rust-python-implementation-split.md`
- Added `docs/reports/0646-feature-maturity-matrix-for-progress-faq003.md`
- Added `docs/reports/0647-uncommitted-doc-consistency-review.md`
- Added `.docs/progress-task-axes.md`
- Added `faq_003.md`
- Rewrote `progress.md`
- Rewrote `tasks.md`
- Updated `AGENTS.md`
- Updated `.docs/continuous-task-policy.md`
- Updated `README.md`
- Updated `Documentation.md`
- Rewrote `plan/01-status-at-a-glance.md`
- Rewrote `plan/10-roadmap-overall.md`
- Rewrote `plan/11-roadmap-near-term.md`
- Rewrote `plan/17-research-phases-and-autonomy-gates.md`
- Updated `docs/reports/0646-feature-maturity-matrix-for-progress-faq003.md`
- Updated this report after review findings were addressed

## 6. Commands run and exact outputs

Representative commands:

```text
$ df -h .
Filesystem      Size  Used Avail Use% Mounted on
/dev/vda2        99G   76G   19G  81% /

$ free -h
               total        used        free      shared  buff/cache   available
Mem:           960Mi       638Mi        75Mi       1.0Mi       399Mi       321Mi
Swap:           19Gi       1.2Gi        18Gi

$ python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
Task baseline recorded.

$ python3 .agents/skills/discord-report/scripts/discord_notify.py progress --summary "phase再整理の軸とfeature棚卸しを固め、通知運用の差分も反映する段階に入りました。" --next-step "AGENTS.md/.docsへ途中経過と完了の運用差分を明記し、faq_003.md・progress.md・tasks.mdを全面更新します。" --cwd .
Progress notification sent.

$ python3 scripts/new_report.py --slug phase-recut-faq003-and-snapshot-rewrite
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
/home/yukatayu/dev/mir_poc_01/docs/reports/0644-phase-recut-faq003-and-snapshot-rewrite.md

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 645 numbered report(s).

$ python3 scripts/current_l2_source_sample_regression.py inventory
current L2 fixed-subset first-cluster inventory
sample stem | authored status | expected static | expected runtime | formal hook | file state | note
----------------------------------------------------------------------------------------------------
e1-place-atomic-cut | source-authored | valid | explicit_failure | runtime_try_cut_cluster | present | first widened authored row runtime path
e2-try-fallback | source-authored | valid | success | runtime_try_cut_cluster | present | current authored runtime path
e21-try-atomic-cut-frontier | source-authored | valid | success | runtime_try_cut_cluster | present | second widened authored row runtime path
e4-malformed-lineage | source-authored | malformed | not_evaluated | fixture_static_cluster | present | current authored static stop
e23-malformed-try-fallback-missing-fallback-body | source-authored | malformed | not_evaluated | fixture_static_cluster | present | current authored static stop
e3-option-admit-chain | source-target-only | not_yet_authored | not_yet_authored | not_yet_authored | absent | deferred authored row

$ cargo test -p mir-runtime --test current_l2_source_sample_runner
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.25s
     Running tests/current_l2_source_sample_runner.rs (target/debug/deps/current_l2_source_sample_runner-2924f94bfa55528b)

running 7 tests
test current_l2_source_sample_runner_accepts_explicit_e4_path ... ok
test current_l2_source_sample_runner_accepts_named_e1_sample ... ok
test current_l2_source_sample_runner_accepts_named_e21_sample ... ok
test current_l2_source_sample_runner_accepts_named_e23_sample ... ok
test current_l2_source_sample_runner_rejects_unknown_sample_stem ... ok
test current_l2_source_sample_runner_rejects_existing_out_of_scope_explicit_file ... ok
test current_l2_source_sample_runner_resolves_named_e2_sample_and_runs_runtime ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

$ git diff --check
(no output)

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 646 numbered report(s).
```

## 7. Evidence / outputs / test results

### Current planning read

- old `Phase 7` had become too large to function as a planning unit.
- The repo reads more clearly when split into:
  - `Macro 0` repository memory
  - `Macro 1` semantic kernel
  - `Macro 2` parser-free validation substrate
  - `Macro 3` compile-ready minimal actualization
  - `Macro 4` executable fixed-subset sample expansion
  - `Macro 5` theorem/model-check/static reasoning bridge
  - `Macro 6` fabric/shared-space/runtime evolution
  - `Macro 7` toolchain/backend/developer surface
  - `Macro 8` domain/application realization

### Current capability read

- parser-free current L2 PoC is runnable.
- The fixed-subset source-sample corpus is runnable for the current authored quintet:
  - `e1`
  - `e2`
  - `e21`
  - `e4`
  - `e23`
- The ladder is not uniform:
  - `e1` / `e2` / `e21` reach interpreter-backed `runtime_try_cut_cluster`
  - `e4` / `e23` are static-stop examples with `fixture_static_cluster`
- `e3` remains source-target-only / deferred authored row.

### Rust / Python split

- Rust currently carries parser/semantics/runtime/formal-handoff core paths.
- Python currently carries docs/report helpers, detached-loop orchestration, source-sample regression, and narrow compare helpers.
- The strongest current reading is **Rust-heavy core + mixed-tool helper workflow**, not "everything eventually becomes Rust" as an explicit settled rule.

### Policy alignment

- The user's clarified expectation on intermediate vs final reporting required a repo-local policy update.
- `AGENTS.md` and `.docs/continuous-task-policy.md` now explicitly say:
  - use `progress` for continuing checkpoints
  - use `complete` only when stopping
  - avoid overly frequent notifications

### Review follow-up

- reviewer report `0647` found three doc-consistency issues:
  - `0644` was still a template in an earlier draft even though `progress.md` cited it
  - `faq_003.md` overstated the fixed-subset ladder as if all authored rows shared the same interpreter/formal-hook reach
  - `0646` contained a scope note inconsistent with the repo read-order policy
- all three were fixed in the working tree before final validation:
  - `0644` was filled
  - `faq_003.md` now distinguishes `runtime_try_cut_cluster` rows from `fixture_static_cluster` rows
  - `0646` now lists `specs/00` and `specs/01` in consulted inputs and no longer claims to skip them

## 8. What changed in understanding

- The clearest split is no longer "implemented vs not implemented" but:
  - decided semantics
  - docs-first boundary
  - narrow implementation
  - executable validation ratchet
  - sample/docs integration
- The most useful planning unit is now the macro phase, while the old Phase labels remain as historical checkpoint anchors.
- The repo's strongest current differentiator is the combination of:
  - strong semantic boundary discipline
  - runnable fixed subset
  - explicit separation between Mir core, fabric/shared-space, backend/tooling, and upper-layer application tracks

## 9. Open questions

- Which source-sample family should be widened immediately after `e3`?
- What is the minimal compare-ready bridge sketch shape that does not prematurely force concrete tool binding?
- When should backend / public operational surface inventory graduate into an actual pilot?
- Which upper-layer domain target should become the first application realization line?

## 10. Suggested next prompt

Continue from the current self-driven line:

1. compare-ready bridge sketch second reopen
2. deferred `e3` actualization reopen timing
3. actual `e3` authored-row reopen

and then re-check whether the next narrow step should be a proof/model-check concrete tool pilot or second source-sample cluster widening.
