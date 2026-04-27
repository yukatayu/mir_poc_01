# 0942 projection placement executable widening

## Objective

phase 12 `Projection / placement` の docs-first plan を、
repo-local current layer を壊さずに helper-local / report-local preview floor まで widen し、
system-wide source から place split / observer view refs / provider placement inventory を
evidence-oriented に読める current package closeout を作る。

## Scope and assumptions

- final public projection API、final projection IR、generated place-specific program emitter、optimizer、equivalence checker はこの package の scope 外。
- current line で actualize するのは helper-local preview と report-local inventory のみ。
- active clean near-end suite と Sugoroku world vertical slice を壊さないことを優先する。
- projection / placement と transport / auth / witness / visualization を collapse しない。

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/05-mirrorea-fabric.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/20-projection-and-placement-roadmap.md`
- `plan/90-source-traceability.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/projection_placement_plan_01.md`
- `samples/clean-near-end/sugoroku-world/README.md`
- `sub-agent-pro/mirrorea_future_plan_full_handoff_2026-04-24.md`
- `sub-agent-pro/mirrorea_phase_samples_progress_storage_handoff_2026-04-24.md`

## Actions taken

1. Sugoroku helper `scripts/sugoroku_world_samples.py` に `projection_view` と `--debug projection` を追加し、
   `SugorokuWorldSource#1`、server places、authority place、participant places、observer view refs、
   membership frontier、telemetry refs を helper-local preview として可視化した。
2. `scripts/tests/test_sugoroku_world_samples.py` に projection view assertion と `--debug projection` の test を追加した。
3. `crates/mir-runtime/src/clean_near_end.rs` の `05_delegated_rng_service` に
   report-local `cross_place_projection` view を追加し、
   provider boundary lane で authority placement / provider placement / witness / envelope refs / redaction rules を
   collapse せずに読める inventory を actualize した。
4. reviewer 指摘に従い、runtime-side `cross_place_projection` の `focus_subjects` に
   `authority_place:ParticipantPlace[Alice]` を追加し、
   authority placement が prose だけでなく構造化 subject として残るようにした。
5. `crates/mir-runtime/tests/clean_near_end_samples.rs` に authority placement subject の assertion を追加した。
6. `samples_progress.md`、`progress.md`、`tasks.md`、`plan/20-projection-and-placement-roadmap.md`、
   `plan/01-status-at-a-glance.md`、`plan/11-roadmap-near-term.md`、
   `plan/17-research-phases-and-autonomy-gates.md`、`plan/90-source-traceability.md`、
   `docs/hands_on/current_phase_closeout_01.md`、`docs/hands_on/projection_placement_views_01.md`、
   `docs/research_abstract/projection_placement_plan_01.md`、`docs/research_abstract/mirrorea_future_axis_01.md`、
   `samples/clean-near-end/sugoroku-world/README.md`、`specs/10-open-questions.md`、
   `specs/11-roadmap-and-workstreams.md` を同期した。
7. reviewer 指摘に従い、`samples_progress.md` の projection row は
   compositional E2E completion と誤読されないように `E2E-PROJECTION-TARGET` へ戻し、
   actualized current floor は `PRJ-01` / `PRJ-02` preview rows に寄せた。

## Evidence / outputs / test results

### Validation run

```bash
python3 -m unittest scripts.tests.test_sugoroku_world_samples
cargo test -p mir-runtime --test clean_near_end_samples
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug projection --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json
cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
```

### Validation results

- `python3 -m unittest scripts.tests.test_sugoroku_world_samples`
  - pass, `Ran 35 tests`
- `cargo test -p mir-runtime --test clean_near_end_samples`
  - pass, `25 passed`
- `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug projection --format json`
  - pass, `projection_view` が `SugorokuWorldSource#1`、`WorldServerPlace`、`SugorokuGamePlace#1`、
    `ParticipantPlace[*]`、`observer_views`、`membership_frontier` を返す
- `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json`
  - pass, projection preview 追加後も existing visualization view が壊れていない
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
  - pass, `cross_place_projection` が `authority_place:ParticipantPlace[Alice]`、
    `place:ProviderPlace[AuthorityRng]`、`projection_summary_only` を返す
- `python3 scripts/check_source_hierarchy.py`
  - pass, required `23`, present `23`, missing `0`
- `python3 scripts/validate_docs.py`
  - pass, `Documentation scaffold looks complete.`, `Found 940 numbered report(s).`
- `git diff --check`
  - pass

### Reviewer results

- explorer `019dcf1e-4d2d-7e72-bef3-abc06a304d26`
  - next package の typed external residual wording drift と low-risk file shortlist を返した
- reviewer `019dcf30-ed27-7550-99ac-5d6ce6cf9517`
  - 初回 findings:
    - `0942` report 不在
    - runtime-side authority placement が prose-only
    - `samples_progress.md` の projection row が E2E completion を言い過ぎ
  - 対応:
    - 本 report を追加
    - `authority_place:ParticipantPlace[Alice]` subject と test を追加
    - `E2E-PROJECTION` を future target row に戻した

## What changed in understanding

- phase 12 の current executable widening は、projection を full E2E にすることではなく、
  helper-local / report-local preview floor を壊れにくい carrier と test で固定することが正しい。
- provider placement と authority placement の分離は、notes だけでは drift しやすい。
  `focus_subjects` の structured field と test を持たせる方が current line に合う。
- `samples_progress.md` では、preview floor と future E2E target を同じ row で持つと overclaim になりやすい。
  preview は active matrix、compositional E2E は target row に分ける方が安全である。

## Open questions

1. final projection IR を public object にするか、internal emitter artifact に留めるか。
2. emitted place program と deployment planner の境界をどこで切るか。
3. typed external residual family を reopen した後、projection preview と adapter host schema pressure をどこで結ぶか。
4. cross-place equivalence checker を current repo-local helper の外へどう出すか。

## Suggested next prompt

`Typed external boundary residual planned family review` を進めてください。`EXT-01` / `EXT-02` / `EXT-05` を projection / visualization / host-schema pressure と照らして再監査し、synthetic preview helper subset `EXT-03` / `EXT-04` と active sample / planned sample / final public host contract を混同しない reopen criterion を docs / plan / samples_progress / progress / tasks / report に同期してください。
