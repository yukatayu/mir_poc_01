# 0928 Hands On Docs Closeout

## Objective

`hands-on docs / closeout` package を close し、current runnable floor、remaining mixed gate、remaining true user-spec gate、next queue を reader-facing に再同期する。

## Scope and assumptions

- current scope は docs / snapshot / report の closeout である。
- new runtime behavior、new sample behavior、final public API は追加しない。
- low-risk change として `docs/hands_on/` landing page を追加するが、既存の長い hands-on 文書は `docs/research_abstract/` に残す。
- `specs/10-open-questions.md` と `plan/17-research-phases-and-autonomy-gates.md` は current package で実質変更がないため更新しない。
- worktree には unrelated current-L2 CLI diff が残っているため、この package でも stage / commit に含めない。

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00-document-map.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/19-repository-map-and-taxonomy.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/reports/0927-compiler-backend-llvm-preparation.md`

## Actions taken

1. `docs/hands_on/README.md` を追加し、current hands-on landing page と existing long-form hands-on の関係を明示した。
2. `docs/hands_on/current_phase_closeout_01.md` を追加し、current runnable floor、debug lanes、stop line、remaining mixed gate、remaining true user-spec gate、next queue を command-first で整理した。
3. `README.md`、`Documentation.md`、`docs/research_abstract/README.md`、`docs/research_abstract/mirrorea_future_axis_01.md` を更新し、hands-on closeout が close したことと、新しい landing page への導線を追加した。
4. stale reader-facing drift を同じ package で吸収し、`docs/research_abstract/typed_external_boundary_adapter_plan_01.md` の old queue reference を queue-neutral に戻し、Sugoroku hands-on docs に `debug envelopes` / `debug visualization` の current surface を追記した。
5. `specs/00-document-map.md` と `specs/11-roadmap-and-workstreams.md` を更新し、`docs/hands_on/` landing page と current promoted next package の移行を current map / roadmap に反映した。
6. `plan/01-status-at-a-glance.md`、`plan/11-roadmap-near-term.md`、`plan/19-repository-map-and-taxonomy.md` を更新し、`hands-on docs / closeout` closeout と next promoted package を repository memory に反映した。
7. `progress.md`、`tasks.md`、`samples_progress.md` を更新し、current snapshot と next queue を `Network transport` executable widening 側へ進めた。

## Files changed

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00-document-map.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/19-repository-map-and-taxonomy.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/typed_external_boundary_adapter_plan_01.md`
- `docs/research_abstract/hands_on_sugoroku_sample_matrix.md`
- `docs/research_abstract/hands_on_sugoroku_detail.md`
- `docs/hands_on/README.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/reports/0928-hands-on-docs-closeout.md`

## Evidence / outputs / test results

- `python3 scripts/check_source_hierarchy.py`
  - pass
- `python3 scripts/validate_docs.py`
  - pass
- `python3 scripts/current_l2_guided_samples.py closeout --format json`
  - pass
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
  - pass
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json`
  - pass
- `git diff --check`
  - pass

## What changed in understanding

- `docs/hands_on/` は physical reorg を伴わずに actualize できる。landing page だけ追加すれば、existing `docs/research_abstract/` hands-on の導線を壊さず current closeout を短く読める。
- current phase closeout で重要なのは、新機能追加ではなく **sample / helper / report / progress / tasks / docs が同じ current line を指すこと** である。

## Open questions

- `Network transport` executable widening を helper widening だけで始めるか、runtime side canary まで同時に入れるか。
- avatar fairy follow の representative slice を Sugoroku helper extension として置くか、専用 helper に切るか。
- `docs/research_abstract/` 内の long-form hands-on を `docs/hands_on/` へ後でどこまで移すか。

## Suggested next prompt

`Network transport executable widening` package を進めてください。`plan/22-network-transport-roadmap.md` の `NET-01..05` から最小の loopback canary を選び、helper / runtime / docs / `samples_progress.md` / report を同一 task で更新し、auth/transport separation evidence を維持してください。
