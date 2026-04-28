# Report 0948 — P3 Projection Emitted-Program Gate Closeout

- Date: 2026-04-28 09:51 JST
- Author / agent: Codex
- Scope: `P3` Projection / placement residual emitted-program gate closeout
- Decision levels touched: `L1` roadmap / package sequencing clarification, `L2` docs-first boundary / reserve policy / snapshot synchronization

## 1. Objective

helper-local / report-local projection preview floor と、
later emitted place-specific program family の境界を docs-first に固定する。
具体的には、projection validity report minimum、generated artifact reserve policy、
`P15` handoff line、queue promotion (`P4` next / `P5` reopen next) を
repo current line に同期する。

## 2. Scope and assumptions

- current executable evidence は `projection_view` と `cross_place_projection` の preview floor に限る。
- `P3 close` は emitted place-specific program implementation closeout を意味しない。
- final projection IR、actual emitted place-specific program family、placement optimizer、cross-place equivalence checker、deployment planner は後段に残す。
- `samples/generated/` は reserve path であり、current `P3` では generated artifact policy だけを固定する。

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/20-projection-and-placement-roadmap.md`
- `plan/90-source-traceability.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/hands_on/projection_placement_views_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/projection_placement_plan_01.md`
- `samples/README.md`
- `samples/generated/README.md`
- `docs/reports/0942-projection-placement-executable-widening.md`
- `docs/reports/0947-p3-projection-placement-wording-drift-inspection.md`
- sub-agent findings:
  - `Hypatia` explorer: `P3` open gap = preview floor と emitted-program family の boundary定義不足
  - `Pascal` explorer: wording drift / required file set / overclaim risk inspection

## 4. Actions taken

1. `plan/20` に `P3` close 条件を追加し、preview floor と emitted-program family の boundary fixation が package の本体だと明示した。
2. projection validity report minimum を、exact schema ではなく required category set として固定した。
3. `samples/generated/` reserve policy を projection / placement emitted place-specific program family に接続し、heavy disposable artifact は external workdir 優先、committed generated artifact は bridge evidence に限定する current rule を追加した。
4. `P15` を actual emitted place-specific program family の first actualization tranche として明示し、`UNRESOLVED` のままだった validation wording を current guard に置き換えた。
5. front-door docs、reader-facing docs、snapshot docs、relevant `plan/` / `specs/` を `P3 closed -> P4 next -> P5 reopen next` に同期した。
6. `0947` inspection report を side evidence として残し、本 report では package closeout と queue promotion をまとめた。

## 5. Files changed

- front-door / snapshot docs
  - `README.md`
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
- projection-specific docs / memory
  - `plan/20-projection-and-placement-roadmap.md`
  - `docs/hands_on/projection_placement_views_01.md`
  - `docs/research_abstract/projection_placement_plan_01.md`
  - `samples/generated/README.md`
  - `samples/README.md`
- queue / traceability sync
  - `docs/hands_on/current_phase_closeout_01.md`
  - `docs/research_abstract/mirrorea_future_axis_01.md`
  - `plan/01-status-at-a-glance.md`
  - `plan/11-roadmap-near-term.md`
  - `plan/17-research-phases-and-autonomy-gates.md`
  - `plan/90-source-traceability.md`
  - `specs/11-roadmap-and-workstreams.md`
- reports
  - `docs/reports/0947-p3-projection-placement-wording-drift-inspection.md`
  - `docs/reports/0948-p3-projection-emitted-program-gate-closeout.md`

## 6. Evidence / outputs / test results

- current preview floor:
  - `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug projection --format json`
  - `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json`
  - `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
  - expected: pass
  - confirms:
    - `projection_view` remains helper-local preview only
    - `cross_place_projection` remains report-local preview only
    - authority placement / provider placement stay separate
- preview-floor regression guards:
  - `python3 -m unittest scripts.tests.test_sugoroku_world_samples`
  - `cargo test -p mir-runtime --test clean_near_end_samples`
  - expected: pass
- residual gate / reserve path:
  - `python3 scripts/sugoroku_world_samples.py closeout --format json`
  - `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json`
  - `find samples/generated -maxdepth 3 -type f | sort`
  - expected: pass
  - confirms:
    - closeout still exposes preview-only carriers
    - generated reserve path currently contains only `samples/generated/README.md`
    - actual emitted place-specific program family is still later
- docs / hierarchy / hygiene:
  - `python3 scripts/check_source_hierarchy.py`
  - `python3 scripts/validate_docs.py`
  - `git diff --check`
  - expected: pass

## 7. What changed in understanding

- `P3` の本体は projection implementation 追加ではなく、preview floor と emitted-program family の **boundary fixation** だった。
- generated artifact policy が弱いままだと、`P15` 以降の emitted program actualization と repo committed sample boundary が混ざる。
- `E2E-PROJECTION-TARGET` を target-only のまま維持しつつ `P3` を close できるのは、preview floor と later family の切れ目を docs-first に固定した場合だけである。
- `P4` / `P5` への queue promotion は、projection package が close して初めて自然になる。

## 8. Open questions

- projection validity report の exact field naming / serialization をいつ固定するか。
- emitted place-specific program family を `samples/generated/` に bridge evidence として commit する threshold を何で切るか。
- cross-place equivalence checker の public surface を `P15` でどこまで actualize するか。

## 9. Suggested next prompt

`P4` `TermSignature` registry hardening を進め、helper/runtime/report-local の signature kind / granularity / reserved-kind drift を洗い、`plan/09`、`progress.md`、`tasks.md`、`samples_progress.md`、relevant docs、new report に current inventory rule と stop line を同期してください。helper-local inventory を final public signature schema と混同しないこと。

## 10. Git status

- AGENTS.md updated: no
- `progress.md` updated: yes
- `tasks.md` updated: yes
- `samples_progress.md` updated: yes
- new reports created: yes (`0947`, `0948`)
- git commit / push status at report authoring time: pending after final validation
