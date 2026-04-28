# Report 0946 — Typed External Boundary Residual Planned Review

- Date: 2026-04-28 09:34 JST
- Author / agent: Codex
- Scope: `P2` Typed external boundary residual planned family review closeout
- Decision levels touched: `L1` roadmap / stop-line clarification, `L2` helper closeout and repository memory synchronization

## 1. Objective

`EXT-01` / `EXT-02` / `EXT-05` を active executable sample に昇格させずに、
current repo で確認可能な **indirect anchor / reopen criterion / kept-later gate** を固定する。
同時に、typed external helper current floor の validation / formatting bug を潰し、
`progress.md` / `tasks.md` / `samples_progress.md` / relevant docs / `plan/25` を
`P2 close` 読みに同期する。

## 2. Scope and assumptions

- normative spec の正本は `specs/` であり、この report は evidence と repository memory 更新の記録である。
- `P2 close` は host-facing adapter implementation complete を意味しない。
- current executable floor は `scripts/typed_external_boundary_samples.py` による synthetic preview helper subset (`EXT-03` / `EXT-04`) である。
- `EXT-01` / `EXT-02` / `EXT-05` は residual planned family のまま残し、public adapter API / exact host schema / final visualization service contract は fixed しない。

## 3. Documents consulted

- `sub-agent-pro/mirrorea_next_stage_full_plan_handoff_2026-04-27.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/25-typed-external-boundary-executable-roadmap.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/hands_on/typed_external_boundary_canaries_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/typed_external_boundary_adapter_plan_01.md`
- `samples/not_implemented/typed-external-boundary/README.md`
- `scripts/typed_external_boundary_samples.py`
- `scripts/tests/test_typed_external_boundary_samples.py`
- sub-agent findings:
  - `Ramanujan` explorer: current typed external boundary floor / residual family gaps
  - `Poincare` explorer: package-close snapshot synchronization targets

## 4. Actions taken

1. typed external helper current behavior を再確認し、default pretty `list` / `check-all` / `closeout` が payload shape mismatch で壊れていることを再現した。
2. failing tests を先に追加し、pretty formatting と `residual_review_matrix` closeout surface を regression 化した。
3. `scripts/typed_external_boundary_samples.py` に `RESIDUAL_REVIEW_MATRIX` を追加し、`closeout()` と pretty formatter を修正した。
4. `plan/25`、typed external docs、planned sample README を更新し、`EXT-01` / `EXT-02` / `EXT-05` の indirect anchor / reopen criterion / kept-later gate を固定した。
5. `README.md`、`Documentation.md`、`progress.md`、`tasks.md`、`samples_progress.md`、relevant `plan/` / `specs/` docs を `P2 closed -> P3 next -> P4 reopen next` に同期した。
6. `samples_progress.md` に helper regression / indirect-anchor validation を追加した。

## 5. Files changed

- code / tests
  - `scripts/typed_external_boundary_samples.py`
  - `scripts/tests/test_typed_external_boundary_samples.py`
- repository memory / roadmap / front-door docs
  - `README.md`
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
  - `specs/11-roadmap-and-workstreams.md`
  - `plan/01-status-at-a-glance.md`
  - `plan/11-roadmap-near-term.md`
  - `plan/17-research-phases-and-autonomy-gates.md`
  - `plan/25-typed-external-boundary-executable-roadmap.md`
  - `docs/hands_on/current_phase_closeout_01.md`
  - `docs/hands_on/typed_external_boundary_canaries_01.md`
  - `docs/research_abstract/mirrorea_future_axis_01.md`
  - `docs/research_abstract/typed_external_boundary_adapter_plan_01.md`
  - `samples/not_implemented/typed-external-boundary/README.md`
- report
  - `docs/reports/0946-typed-external-boundary-residual-planned-review.md`

## 6. Evidence / outputs / test results

- helper regression and closeout surface:
  - `python3 -m unittest scripts.tests.test_typed_external_boundary_samples`
  - expected: pass
  - confirms:
    - pretty `list` / `check-all` / `closeout` no longer crash
    - `closeout()` emits `residual_review_matrix`
- typed external helper current floor:
  - `python3 scripts/typed_external_boundary_samples.py list`
  - `python3 scripts/typed_external_boundary_samples.py check-all`
  - `python3 scripts/typed_external_boundary_samples.py closeout`
  - `python3 scripts/typed_external_boundary_samples.py closeout --format json`
  - `python3 scripts/typed_external_boundary_samples.py run EXT-03 --debug envelopes --format json`
  - `python3 scripts/typed_external_boundary_samples.py run EXT-03 --debug visualization --format json`
  - `python3 scripts/typed_external_boundary_samples.py run EXT-04 --debug failures --format json`
  - expected: pass
  - confirms:
    - current executable subset remains `EXT-03` / `EXT-04`
    - envelope / visualization / failure debug lanes remain typed helper-local preview
    - `residual_review_matrix` is exposed in closeout output
- residual family indirect anchors:
  - `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes --format json`
  - `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug projection --format json`
  - `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json`
  - `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
  - expected: pass
  - confirms:
    - `EXT-01` current anchor is `provider_boundary`
    - `EXT-02` current anchors include visualization / telemetry / projection bridge lanes
    - `EXT-05` current anchors include visualization redaction lanes
- docs / hierarchy / hygiene:
  - `python3 scripts/check_source_hierarchy.py`
  - `python3 scripts/validate_docs.py`
  - `git diff --check`
  - expected: pass

## 7. What changed in understanding

- `P2` close に必要だったのは adapter 実装ではなく、residual planned family の **reopen condition を evidence anchor と結び直すこと** だった。
- typed external helper の default pretty path が壊れていたため、closeout readability 自体が不十分だった。これは docs-only gap ではなく実バグだった。
- `EXT-01` / `EXT-02` / `EXT-05` は current repo で何も確認できない planned family ではなく、indirect anchor を通じて reopen criterion を ratchet できる状態にある。
- current promoted next line は `P3` projection / placement residual emitted-program gate、next reopen point は `P4` `TermSignature` hardening と読める。

## 8. Open questions

- `EXT-01` の console-shaped scenario を `provider_boundary` anchor 上で widening するとき、どの程度まで host cue を書いても stdio builtin 誤読を避けられるか。
- `EXT-02` の overlay route は projection / visualization / host boundary のどこまでを `P3` で持ち、どこからを `P12` / `P15` / `P18` に残すべきか。
- `EXT-05` を standalone sample に戻す条件を、visualization service contract 未固定のままどこまで sharpen できるか。

## 9. Suggested next prompt

`P3` Projection / placement residual emitted-program gate を進め、helper/report-local preview floor (`projection_view`, `cross_place_projection`) と emitted place program / generated artifact / equivalence checker family の stop line を `plan/20`、`progress.md`、`tasks.md`、`samples_progress.md`、relevant docs、new report に同期してください。`P3 close` を final projection IR / optimizer / deployment planner completion と混同しないこと。

## 10. Git status

- AGENTS.md updated: no
- `progress.md` updated: yes
- `tasks.md` updated: yes
- `samples_progress.md` updated: yes
- new report created: yes
- git commit / push status at report authoring time: pending after final verification and diff review
