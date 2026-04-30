# 1035 — authority-witness and delegated-RNG temperature alignment

## Objective

`specs/examples/476` と `477` の current active evidence / historical compare-anchor / package-local queue memory を current repo-level queue authority と整合させ、active docs が stale self-driven line を再昇格しないようにする。

## Scope and assumptions

- scope は docs-only maintenance closeout に限る。
- 規範正本の追加決定は行わない。
- final public witness schema、final public provider receipt schema、combined public contract、distributed fairness / randomness provider は引き続き kept-later gate とする。
- current queue authority は `progress.md` / `tasks.md` を正とし、`specs/examples/476/477` の package-local next-line wording は compare-anchor memory として冷やす。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `.docs/progress-task-axes.md`
- `.docs/continuous-task-policy.md`
- `.docs/current-l2-source-sample-authoring-policy.md`
- `AGENTS.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/04-mir-core.md`
- `specs/05-mirrorea-fabric.md`
- `specs/06-prismcascade-positioning.md`
- `specs/07-typed-effects-wiring-platform.md`
- `specs/08-cross-system-relations.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `plan/00-index.md`
- relevant `plan/` memory already in force for current-L2 / order-handoff maintenance lane
- `samples/README.md`
- `scripts/README.md`
- `sub-agent-pro/mir_poc_01_research_handoff_2026-04-30.md`
- `specs/examples/476-current-l2-auditable-authority-witness-strengthening-actualization.md`
- `specs/examples/477-current-l2-delegated-rng-service-practical-actualization.md`

## Actions taken

1. Re-read `specs/examples/476/477` against current `progress.md` / `tasks.md` queue authority.
2. Kept the already-applied active-evidence cooling for historical `p07/p08/p05/p09` compare-anchor labels versus current clean-near-end `06/01/05` + order-handoff family evidence.
3. Rewrote `specs/examples/476/477` current actualization cut wording so historical `p07/p08/p05/p09` labels remain compare-anchor memory while current runnable evidence stays on clean-near-end `06/01/05`.
4. Rewrote `specs/examples/476/477` `next self-driven line` sections so they remain package-memory only and no longer promote a current repo-level self-driven implementation line.
5. Updated `progress.md` current maintenance lane and recent log.
6. Updated `tasks.md` current task-level status so this family no longer appears as the next optional maintenance line.

## Commands run

- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 06_auditable_authority_witness --format json`
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 01_authorized_roll_publish_handoff --format json`
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
- `python3 scripts/clean_near_end_samples.py run order-handoff --format json`
- `python3 scripts/clean_near_end_samples.py closeout --format json`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `git diff --check`
- `git status --short`

## Files changed

- `specs/examples/476-current-l2-auditable-authority-witness-strengthening-actualization.md`
- `specs/examples/477-current-l2-delegated-rng-service-practical-actualization.md`
- `progress.md`
- `tasks.md`
- `docs/reports/1035-authority-witness-and-delegated-rng-temperature-alignment.md`

## Evidence / outputs / test results

- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 06_auditable_authority_witness --format json`
  - pass
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 01_authorized_roll_publish_handoff --format json`
  - pass
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
  - pass
- `python3 scripts/clean_near_end_samples.py run order-handoff --format json`
  - pass
- `python3 scripts/clean_near_end_samples.py closeout --format json`
  - pass
- `python3 scripts/check_source_hierarchy.py`
  - pass
- `python3 scripts/validate_docs.py`
  - pass
- `git diff --check`
  - pass
- `git status --short`
  - before report authoring: only intended edits in `specs/examples/476`, `specs/examples/477`, `progress.md`, `tasks.md`

## Reviewer findings and follow-up

- reviewer `Ramanujan` identified that `progress.md` / `tasks.md` overstated this package's fresh evidence as including Sugoroku / network / model-check replacement runs. Follow-up: narrowed the wording to current clean-near-end order-handoff evidence and repo-level queue authority.
- reviewer `Ramanujan` identified that `specs/examples/476/477` still left `p07` / `p09` sounding like current reached samples inside `current actualization cut`. Follow-up: rewrote those bullets so they are explicit historical compare-anchor memory, while current runnable evidence stays on clean-near-end `06/01/05`.

## What changed in understanding

- `476/477` の stale point は active evidence row だけではなく、package-local `next self-driven line` が current repo queue authority と collapse して見えることだった。
- current repo-level queue authority が `progress.md` / `tasks.md` に集約されているため、active `specs/examples/` は package-memory next line を compare-anchor memory として明示した方が温度管理として一貫する。
- この family には current blocker になる maintenance line は残っておらず、残件は wider stale-docs sweep か `U1` user decision gate に戻る。

## Open questions

- `U1` actual commitment 前にまだ current-active docs で同種の queue-temperature drift が残っている family があるかは、別 package の maintenance sweep で再確認してよい。
- `specs/examples/476/477` の package-memory wording を更に圧縮するかは reader-facing density の tradeoff があるため、今回は行っていない。

## Suggested next prompt

`U1` 待ちのまま自走を続けるなら、current-active docs に残る queue-temperature drift や stale compare-anchor wording を narrow maintenance package として継続点検する。

## plan/ 更新の有無

- 更新不要

## progress.md 更新の有無

- 更新した

## tasks.md 更新の有無

- 更新した

## samples_progress.md 更新の有無

- 更新不要

## skipped validations and reasons

- full validation floor は未実行。今回は docs-only maintenance closeout であり、対象が `specs/examples/476/477` と snapshot docs に限られるため、focused clean-near-end evidence + docs floor を優先した。
- `samples_progress.md` / `plan/` / `specs/11` / `specs/12` は current authority drift がなく、同 package では未編集とした。

## commit / push status

- report authoring時点では未実行。same-package closeout で commit / push を行う。

## sub-agent session close status

- reviewer `Ramanujan` を使用し、2 件の medium finding を取り込んだ後に session を close した。
