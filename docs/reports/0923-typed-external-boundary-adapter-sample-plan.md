# 0923 Typed External Boundary Adapter Sample Plan

## Objective

phase 9 `Typed external boundary / adapter` を、active code path を壊さずに
docs-first / sample-first で repo に固定する。

## Scope and assumptions

- current scope は planned sample family、evidence anchor、validation、stop line の固定である。
- final public adapter API、browser/network/VR host schema、real transport widening は固定しない。
- `EXT-01..05` の name は 2026-04-24 handoff 由来の working sample ID であり、規範正本ではない。
- worktree には unrelated current-L2 CLI formatting diff が残っているため、この package では stage / commit に含めない。

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/07-typed-effects-wiring-platform.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/hands_on_sugoroku_07_message_envelope_auth.md`
- `docs/reports/0921-message-envelope-auth-seam-first-cut.md`
- `docs/reports/0922-visualization-protocol-first-cut.md`

## Actions taken

1. phase 9 planned family `samples/not_implemented/typed-external-boundary/README.md` を追加した。
2. `EXT-01..05` の working sample ID、goal、current evidence anchor、stop line を文書化した。
3. `samples_progress.md` に `PH9`、`EXT-01..05`、`E2E-ADAPTER-BOUNDARY` の docs-first progress row を追加した。
4. README / Documentation / progress / tasks / plan / reader docs を、next promoted package が projection / placement である current snapshot に同期した。
5. unrelated current-L2 CLI formatting diff は読み取りのみとし、この package には含めないと明示した。

## Files changed

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `samples/not_implemented/README.md`
- `samples/not_implemented/typed-external-boundary/README.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `docs/research_abstract/typed_external_boundary_adapter_plan_01.md`
- `docs/reports/0923-typed-external-boundary-adapter-sample-plan.md`

## Evidence / outputs / test results

- `python3 scripts/check_source_hierarchy.py`
  - pass
- `python3 scripts/validate_docs.py`
  - pass
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
  - pass
- `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes --format json`
  - pass

## What changed in understanding

- phase 9 は real adapter 実装を急がなくても、planned family と evidence anchor を固定するだけで
  projection / placement や hot-plug の議論に必要な boundary を先に揃えられる。
- current repo では `provider_boundary` と `local_queue` が phase 9 の最小 anchor として十分に使える。

## Open questions

- final public adapter API / serialization schema をどこで固定するか。
- browser/network/VR host family を phase 9 sample ladder にどう割り当てるか。
- adapter failure typed result を current report-local carrier にいつ actualize するか。

## Suggested next prompt

`Projection / placement` package を進めてください。system-wide source と place-specific program の区別、validity checklist、server / participant / adapter / visualizer split、report / progress / tasks / samples_progress 同期まで同じ task で閉じてください。
