# Report 1135 — P-A0-28 Stage-A Imported-Baseline Reconciliation

- Date: 2026-05-03
- Author / agent: Codex
- Scope: rerun the imported Stage A validation floor for the current alpha line, then synchronize `specs/17`, `plan/43`, `plan/01`, `Documentation.md`, `progress.md`, `tasks.md`, and `samples_progress.md` so the large-stage-first reading is sequential from Stage A through Stage F
- Decision levels touched: `L1` none, `L2` wording and stage-boundary reconciliation only

## Objective

Close `P-A0-28` as a narrow docs/spec reconciliation package so the repo can honestly say “alpha-0.5 first” without leaving Stage A at `90%` while Stage B..F are already `100%`.

## 日本語要約

Stage B..F は既に current scope で閉じていましたが、Stage A だけが snapshot 上 `90%` のまま残っており、large stage を順に閉じていく読みとずれていました。今回の package では、新しい runtime 実装は増やさず、current-L2 / clean-near-end / Lean / Sugoroku / avatar / typed external / network canary / projection / viewer / hot-plug narrow floor の既存 evidence を rerun して、Stage A を imported baseline として `100%` に数える境界を `specs/17`、`plan/43`、snapshot docs に明記しました。

これは imported baseline reconciliation であり、`samples/alpha/` runnable-root promotion、parser/runtime front door、final public product、public ABI freeze は新たに主張しません。large-stage-first alpha line は current scope で `Stage A..F` の順に読めるようになりましたが、next reopen point は引き続き later-family blocker lane です。

## Scope and assumptions

- The task is documentation/spec synchronization only. No new runtime semantics, parser bridge, or public API claim is introduced.
- `Stage A` is treated as an imported baseline for the alpha line, not as a newly implemented `samples/alpha/` family.
- Existing Stage B..F closeouts remain authoritative and unchanged in scope.
- I used focused local review only in this turn and did not spawn sub-agents.

## Start state / dirty state

- Worktree started clean.
- Current repo state already had `P-A0-23` through `P-A0-27` closed and pushed.
- Snapshot docs still showed Stage A as `90%` while Stage B..F were `100%`.
- Resource probe before the longer validation pass:
  - `df -h .` showed `/dev/vda2` with `99G` total, `65G` used, `30G` available.
  - `free -h` showed `960Mi` RAM total, `344Mi` available, with swap present.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `plan/00-index.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/15-cut-save-load-checkpoint.md`
- `specs/17-mirrorea-spaces-alpha-scope.md`
- `plan/01-status-at-a-glance.md`
- `plan/41-save-load-checkpoint-roadmap.md`
- `plan/43-alpha-e2e-roadmap.md`
- `samples/README.md`
- `samples/alpha/README.md`
- `docs/reports/TEMPLATE.md`

## Actions taken

- Re-read the Stage A / Stage B boundary docs and confirmed the inconsistency: Stage B..F were closed, but Stage A remained phrased as “mostly reached”.
- Reran the imported baseline evidence floor:
  - current-L2 / clean-near-end closeout
  - Lean sync
  - Sugoroku closeout
  - avatar representative closeout
  - typed external preview closeout
  - network canary `check-all`
  - projection/codegen bridge `check-all`
  - viewer prototype closeout
  - hot-plug narrow runtime cargo test
- Added an explicit `Stage A imported-baseline boundary` section to `specs/17`.
- Updated `plan/43` so Stage A is recorded as an imported baseline closeout with named validation anchors.
- Updated `plan/01` repository memory to include `P-A0-28` and to note that the alpha large-stage reading now counts Stage A as fixed imported evidence.
- Updated `Documentation.md`, `progress.md`, `tasks.md`, and `samples_progress.md` so the alpha large-stage reading is `Stage A..F` sequentially for current scope.
- Prepared this report with the required scaffold and a Japanese summary section.

## Files changed

- `specs/17-mirrorea-spaces-alpha-scope.md`
- `plan/43-alpha-e2e-roadmap.md`
- `plan/01-status-at-a-glance.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/1135-p-a0-28-stage-a-imported-baseline-reconciliation.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git status --short
date '+%Y-%m-%d %H:%M %Z'
df -h .
free -h
python3 scripts/current_l2_guided_samples.py closeout --format json
python3 scripts/current_l2_lean_sample_sync.py
python3 scripts/clean_near_end_samples.py closeout
python3 scripts/sugoroku_world_samples.py closeout --format json
python3 scripts/avatar_follow_samples.py closeout --format json
python3 scripts/typed_external_boundary_samples.py closeout --format json
python3 scripts/network_transport_samples.py check-all --format json
python3 scripts/projection_codegen_samples.py check-all --format json
python3 scripts/visual_debugger_viewer_samples.py closeout --format json
cargo test -p mir-runtime --test hotplug_runtime_skeleton
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

## Evidence / outputs / test results

- `python3 scripts/current_l2_guided_samples.py closeout --format json` passed and emitted the current-L2 canonical inventory.
- `python3 scripts/current_l2_lean_sample_sync.py` passed and refreshed `samples/lean/manifest.json`.
- `python3 scripts/clean_near_end_samples.py closeout` passed.
- `python3 scripts/sugoroku_world_samples.py closeout --format json` passed with the 10-sample Sugoroku closeout inventory.
- `python3 scripts/avatar_follow_samples.py closeout --format json` passed with the active representative `FAIRY-01/02/03/04/06` slice.
- `python3 scripts/typed_external_boundary_samples.py closeout --format json` passed with the helper-local preview subset only.
- `python3 scripts/network_transport_samples.py check-all --format json` passed for `NET-02..05`.
- `python3 scripts/projection_codegen_samples.py check-all --format json` passed for `P15-GEN-01..04`.
- `python3 scripts/visual_debugger_viewer_samples.py closeout --format json` passed for the typed viewer inventory.
- `cargo test -p mir-runtime --test hotplug_runtime_skeleton` passed all 8 tests.
- `python3 -m unittest scripts.tests.test_validate_docs` passed 11 tests.
- `python3 scripts/check_source_hierarchy.py` passed `60/60`.
- `python3 scripts/validate_docs.py` reported `Documentation scaffold looks complete.` and `Found 1137 numbered report(s).`
- `cargo fmt --check` passed.
- `git diff --check` passed.

## What changed in understanding

- The repo did not need more implementation work to satisfy the user’s “alpha-0.5 first” request. The actual inconsistency was the snapshot reading, not the Stage B implementation floor.
- Treating Stage A as an imported baseline is already consistent with the existing repo hierarchy, as long as the admissible evidence is named explicitly and the non-claims remain explicit.
- The cleanest next step was therefore a narrow reconciliation package, not a new runtime or carrier package.

## Open questions

- No new normative open question was introduced by this package.
- Later-family reopen choice is still unresolved:
  - `CUT-10/12/16`
  - `LIF-15`
  - `VAR-14`
  - transport / lifecycle widening
  - public-boundary `U1`

## Suggested next prompt

`P-A0-28` is closed. Review the later-family blockers and choose whether the next narrow promoted line should be `CUT-10/12/16`, `LIF-15`, `VAR-14`, or a separate public-boundary `U1` preparation package.

## Plan update status

`plan/` 更新済み: `plan/43-alpha-e2e-roadmap.md` and `plan/01-status-at-a-glance.md` now record Stage A as an imported baseline closeout for the current alpha line and add `P-A0-28` to repository memory.

## Documentation.md update status

`Documentation.md` 更新済み: the Mirrorea Spaces alpha summary now states that Stage A is counted by imported baseline revalidation and that this does not add runnable-root or public claims.

## progress.md update status

`progress.md` 更新済み: the large stage map now reads `Stage A..F` sequentially with Stage A at `100%`, `P-A0-28` is the last closed package, and the imported baseline rerun is reflected in current status.

## tasks.md update status

`tasks.md` 更新済み: current alpha status now points to `P-A0-28`, records the imported baseline rerun, and keeps the next reopen on later-family blockers or `U1`.

## samples_progress.md update status

`samples_progress.md` 更新済み: the alpha large-stage line now includes Stage A at `100%`, `PH0` references `1135`, and the last closed package is `P-A0-28`.

## Reviewer findings and follow-up

- Focused local review only.
- No sub-agent review was requested in this turn because the package is a narrow docs/spec reconciliation task and the current session constraint for this continuation was to avoid sub-agents.
- Follow-up applied locally:
  - kept Stage A wording explicitly imported and non-public
  - kept `samples/alpha/` non-promotion wording intact
  - kept Stage B..F scope boundaries unchanged

## Skipped validations and reasons

- I did not rerun the full Stage B..F runtime closeout suites in this package because no runtime code or alpha sidecar semantics changed; the package only revalidated the imported Stage A baseline and synchronized docs/spec wording around it.
- I did not run broad crate tests beyond `cargo test -p mir-runtime --test hotplug_runtime_skeleton` because Stage A imported baseline depends on the hot-plug narrow floor only, not the later Stage B..F runtime implementation families.

## Commit / push status

Pending at report write.

## Sub-agent session close status

No sub-agents were spawned in this package.
