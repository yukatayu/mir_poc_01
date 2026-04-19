# 0843 — Package 88 source-sample corpus scope/layout and typing default sync

## Objective

Package 88 として、
fixed-subset source-sample corpus の scope / directory / naming / non-goal minimum を
crate-local manifest と docs snapshot に actualize する。

あわせて、user-provided strong typing defaults が current repo line と
矛盾しないことを確認し、checker-adjacent first strong typing layer の読みと
early stronger typed source principal promotion の読みを混同しないように整える。

## Scope and assumptions

- normative source of truth は `specs/`
- repository memory は `plan/`
- `progress.md` / `tasks.md` は snapshot
- `specs/examples/557` の typing default と、今回の user-provided default 群は
  **checker-adjacent first strong typing layer**
  の current target を整理するものであり、
  stronger typed surface の early source-principal promotion を意味しない
- Package 88 の closeout は
  source-corpus policy minimum の actualization に留め、
  representative / fixture / source mapping matrix widening、
  actual sample content widening、
  parser-to-`Program` lowering widening、
  final public parser/checker/runtime surface までは進めない

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `.docs/progress-task-axes.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/315-current-l2-phase6-parser-second-tranche-shared-single-attachment-frame-first-package-ready-fixed-subset-source-sample-corpus-scope-and-file-layout-comparison.md`
- `specs/examples/316-current-l2-fixed-subset-source-sample-corpus-scope-and-file-layout-ready-minimal-fixed-subset-source-sample-corpus-scope-and-file-layout-threshold.md`
- `specs/examples/557-current-l2-first-strong-typing-layer-finite-index-spine-default.md`
- `tasks.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `.docs/current-l2-source-sample-authoring-policy.md`
- `crates/mir-runtime/src/current_l2.rs`
- `scripts/current_l2_source_sample_regression.py`

## Actions taken

1. current docs / plan / snapshot / runtime code を再点検し、
   fixed-subset source-sample corpus scope/layout と typing default の drift を洗い出した。
2. RED で
   `current_l2_fixed_subset_source_sample_corpus_scope_and_file_layout_manifest()`
   が未実装であることを test から確認した。
3. `mir-runtime` に
   `CurrentL2FixedSubsetSourceSampleCorpusScopeAndFileLayoutManifest`
   と getter を追加し、
   `scope_kind + source_cluster_refs + directory_ref + file_layout_ref + file_extension_policy + sample_id_policy + non_goal_refs`
   の minimum cut を actualize した。
4. Package 88 closeout 用の helper-mirror doc
   `specs/examples/561-current-l2-fixed-subset-source-sample-corpus-scope-and-file-layout-threshold-helper-mirror.md`
   を追加した。
5. source-sample policy drift として
   `.docs/current-l2-source-sample-authoring-policy.md`
   の stale `authored fourteen` 読みを `authored sixteen` に直し、
   current initial cluster 6 本も明示した。
6. user-provided strong typing defaults が `specs/examples/557` / `D-144` / `plan/18`
   current line と矛盾しないことを確認し、
   checker-adjacent first-layer target と source-principal promotion の非同一性を
   snapshot / roadmap / open-questions 側に同期した。
7. Package 88 closeout 後の next active line を
   `Package 89 phase6-request-clause-suite publicization comparison`
   に揃えるよう、
   `Documentation.md`、`progress.md`、`tasks.md`、relevant `plan/`、document map、
   decision register、traceability を更新した。

## Evidence / outputs / test results

- resource check
  - `df -h .`
  - `free -h`
- formatting
  - `cargo fmt --all`
  - passed
- red test
  - `cargo test -p mir-runtime --test current_l2_fixed_subset_source_sample_corpus_scope_and_file_layout_manifest current_l2_fixed_subset_source_sample_corpus_scope_and_file_layout_manifest_keeps_minimum_cut -- --exact`
  - failure:
    unresolved import `current_l2_fixed_subset_source_sample_corpus_scope_and_file_layout_manifest`
- green test
  - `cargo test -p mir-runtime --test current_l2_fixed_subset_source_sample_corpus_scope_and_file_layout_manifest current_l2_fixed_subset_source_sample_corpus_scope_and_file_layout_manifest_keeps_minimum_cut -- --exact`
  - passed after manifest/getter actualization
- validation rerun
  - `python3 -m unittest scripts.tests.test_current_l2_source_sample_regression`
  - `python3 scripts/validate_docs.py`
  - `git diff --check`
  - all passed

## What changed in understanding

- user-provided typing policy is **aligned** with the current repo direction,
  provided that “principal target” is read as
  checker-adjacent first strong typing layer の principal target
  and not as early source-principal promotion of a stronger typed surface.
- Package 88 should close on a narrow manifest + helper-mirror cut.
  It does not need to widen into mapping-matrix finalization or parser/public-surface adoption.
- current next self-driven line after Package 88 is no longer source-corpus scope/layout itself.
  It is request-clause-suite publicization comparison on top of the shared single attachment frame cut.

## Open questions

- request-local `require` / `ensure` suite を multiline frame cut の上にどこまで publicize するか
- perform head / span-rich diagnostics / final grammar をどの package まで defer するか
- strong typing side の next widening targets
  - secret key cannot flow to `Public`
  - sanitize removes taint only with proof/capability
  - remote call count `<= 0` check
  - lifetime/capture escape rejected
  を checker-adjacent payload / static rejection / theorem-side reserve widening のどこへ先に置くか

## Suggested next prompt

Package 89 として、request-local `require` / `ensure` suite の current first compare floor を
source-backed に narrow 化し、
shared single attachment frame と source-corpus scope/layout minimum を保ったまま
perform head / diagnostics / final grammar への premature widening を避けて
next parser-side closeout を進めてください。
