# plan/90 — source traceability

## 目的

この文書は `plan/` 各ファイルの current source root を compact に追跡する。

## current traceability table

| plan | 主な根拠 source |
|---|---|
| `plan/01-status-at-a-glance.md` | `Documentation.md`、`progress.md`、`tasks.md`、`samples_progress.md`、`specs/05-mirrorea-fabric.md`、`specs/07-typed-effects-wiring-platform.md`、`specs/10-open-questions.md`、`specs/11-roadmap-and-workstreams.md`、`docs/research_abstract/mirrorea_future_axis_01.md`、`sub-agent-pro/mirrorea_phase_samples_progress_storage_handoff_2026-04-24.md`、`docs/reports/0912-*`、`docs/reports/0913-*` |
| `plan/06-surface-notation-status.md` | `samples/clean-near-end/README.md`、`samples/clean-near-end/*.mir`、`specs/10-open-questions.md`、`specs/12-decision-register.md`、`crates/mir-runtime/src/clean_near_end.rs` |
| `plan/08-representative-programs-and-fixtures.md` | `samples/current-l2/`、`samples/clean-near-end/`、`samples/old/2026-04-22-pre-clean-near-end/`、`samples/lean/manifest.json`、`docs/reports/0904-*` |
| `plan/11-roadmap-near-term.md` | `progress.md`、`tasks.md`、`samples_progress.md`、`specs/10-open-questions.md`、`specs/11-roadmap-and-workstreams.md`、`sub-agent-pro/mirrorea_future_plan_full_handoff_2026-04-24.md`、`sub-agent-pro/mirrorea_phase_samples_progress_storage_handoff_2026-04-24.md`、`docs/reports/0912-*`、`docs/reports/0913-*` |
| `plan/12-open-problems-and-risks.md` | `specs/09-invariants-and-constraints.md`、`specs/10-open-questions.md`、`specs/11-roadmap-and-workstreams.md`、`samples_progress.md`、`sub-agent-pro/mirrorea_future_plan_full_handoff_2026-04-24.md`、`sub-agent-pro/mirrorea_phase_samples_progress_storage_handoff_2026-04-24.md`、`docs/reports/0912-*`、`docs/reports/0913-*` |
| `plan/13-heavy-future-workstreams.md` | `specs/10-open-questions.md`、`specs/11-roadmap-and-workstreams.md`、`Documentation.md`、`docs/research_abstract/mirrorea_future_axis_01.md`、`docs/reports/0912-*` |
| `plan/16-shared-space-membership-and-example-boundary.md` | `specs/03-layer-model.md`、`specs/05-mirrorea-fabric.md`、`samples/clean-near-end/order-handoff/`、`samples/clean-near-end/model-check/`、`samples/clean-near-end/sugoroku-world/`、`docs/research_abstract/hands_on_sugoroku_*.md`、`docs/reports/0909-*`、`docs/reports/0912-*` |
| `plan/17-research-phases-and-autonomy-gates.md` | `progress.md`、`tasks.md`、`samples_progress.md`、`plan/11-roadmap-near-term.md`、`sub-agent-pro/mirrorea_future_plan_full_handoff_2026-04-24.md`、`sub-agent-pro/mirrorea_phase_samples_progress_storage_handoff_2026-04-24.md`、`docs/reports/0912-*`、`docs/reports/0913-*` |

## historical note

pre-clean-near-end package chain、old `p..` representative tranche、old problem-bundle trace は
historical source trace として `specs/examples/`、archive、old reports に残す。
current active traceability の正本は clean near-end suite、Sugoroku vertical slice、Mirrorea future-axis integration report である。
