# Report 0920 — repository layer map and staged restructuring

## 1. Title and identifier

- Identifier: `0920`
- Title: `repository layer map and staged restructuring`

## 2. Objective

Mir / Mirrorea の current progress を踏まえ、
repo を layer-aware に読むための current map と staged migration plan を作る。
high-risk な crate rename / module move / command-path move は避け、
docs / samples / plan / snapshot を low-risk に整理する。

## 3. Scope and assumptions

- `specs/` は規範正本、`plan/` は repository memory、`docs/reports/` は historical evidence として扱う。
- current task では final public package structure を固定しない。
- current repo-local alpha の runnable path を壊す physical move は行わない。
- `samples/not_implemented/` は planned skeleton family であり、archive でも active runnable sample でもない。
- storage device の format / mount / cleanup はこの task では行わない。既存 state を audit して記録する。

## 4. Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `sub-agent-pro/mirrorea_future_plan_full_handoff_2026-04-24.md`
- `sub-agent-pro/mirrorea_phase_samples_progress_storage_handoff_2026-04-24.md`
- `sub-agent-pro/codex_sugoroku_runtime_attachment_handoff_2026-04-23.md`
- `sub-agent-pro/codex_clean_mir_near_end_completion_with_new_samples_2026-04-22.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/04-mir-core.md`
- `specs/05-mirrorea-fabric.md`
- `specs/07-typed-effects-wiring-platform.md`
- `specs/08-cross-system-relations.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/03-decision-strengths-and-boundaries.md`
- `plan/04-core-semantics-current-l2.md`
- `plan/06-surface-notation-status.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/14-glossary-and-boundary-rules.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `plan/91-maintenance-rules.md`

## 5. Actions taken

### audit commands

- `git status --short`
- `git branch --show-current`
- `find . -maxdepth 3 -type d | sort`
- `find crates -maxdepth 2 -type d 2>/dev/null | sort`
- `find samples -maxdepth 3 -type d 2>/dev/null | sort`
- `find docs -maxdepth 3 -type d 2>/dev/null | sort`
- `find specs -maxdepth 3 -type d 2>/dev/null | sort`
- `find plan -maxdepth 2 -type f 2>/dev/null | sort`
- `cargo metadata --no-deps --format-version 1`
- `df -h`
- `lsblk -f`
- `findmnt`
- `du -sh . .git target .cargo .lake 2>/dev/null || true`

### Dirty state at task start

- task 開始時の worktree は dirty だった。
- principal reason:
  - `0919 LayerSignature` first-cut の code/docs closeout diff が未 commit のまま残っていた。
- task 開始時に modified だった principal files:
  - `AGENTS.md`
  - `README.md`
  - `Documentation.md`
  - `scripts/sugoroku_world_samples.py`
  - `scripts/tests/test_sugoroku_world_samples.py`
  - `crates/mir-runtime/src/clean_near_end.rs`
  - `crates/mir-runtime/src/bin/mir-clean-near-end.rs`
  - `crates/mir-runtime/tests/clean_near_end_samples.rs`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
  - `plan/11-roadmap-near-term.md`
  - `docs/research_abstract/mirrorea_future_axis_01.md`
  - `docs/research_abstract/hands_on_sugoroku_detail.md`
  - `docs/research_abstract/hands_on_sugoroku_sample_matrix.md`
  - `samples/clean-near-end/sugoroku-world/README.md`
- task 開始時に untracked だった principal files:
  - `docs/reports/0919-layer-signature-system-first-cut.md`
  - `sub-agent-pro/mirrorea_phase_samples_progress_storage_handoff_2026-04-24.md`

### current tree summary

- top-level active roots:
  - `crates/`
  - `docs/`
  - `plan/`
  - `samples/`
  - `scripts/`
  - `specs/`
  - `sub-agent-pro/`
- workspace crates are still flat; `cargo metadata` reports 12 workspace members.
- current docs roots:
  - `docs/research_abstract/`
  - `docs/reports/`
  - `docs/diagrams/`
- current sample roots:
  - active: `samples/clean-near-end/`
  - base corpus: `samples/current-l2/`
  - proof/mechanization: `samples/lean/`
  - planned skeleton: `samples/not_implemented/`
  - prototype reference: `samples/prototype/`
  - archive: `samples/old/`
  - generated reserve: `samples/generated/`
- root cleanup candidates remain visible:
  - `handson_tmp`
  - `旧資料_*`

### layer classification

| Area | Current reading | Current location | Migration risk |
|---|---|---|---|
| Mir language substrate | parser / AST carrier | `crates/mir-ast` | high |
| Mir verification substrate | semantics / theorem / model-check bridge | `crates/mir-semantics` | high |
| current runner / runtime | CLI / sample execution / report-local evidence | `crates/mir-runtime` | high |
| Mirrorea runtime placeholders | future runtime/control boundaries | `crates/mirrorea-core`, `crates/mirrorea-control` | medium |
| adapter / host boundary placeholder | future host adapters | `crates/engine-abi` | medium |
| visualization placeholder | future editor / visualization lane | `crates/mir-lsp` | medium |
| PrismCascade lane | separate subsystem placeholder | `crates/prism-*` | medium |
| shared utility lane | cross-subsystem placeholder | `crates/shared-*` | low |
| active sample suite | current canonical runnable samples | `samples/clean-near-end/` | medium |
| base source corpus | fixed-subset current-L2 source corpus | `samples/current-l2/` | low |
| planned skeleton family | docs-first future sample skeleton | `samples/not_implemented/` | low |
| prototype / archive / generated | historical or reserve paths | `samples/prototype/`, `samples/old/`, `samples/generated/` | low |
| active runner / helper stack | front-door checks, sample helpers, storage/env | `scripts/` | medium |
| reader-facing docs | summary / hands-on / explainer | `docs/research_abstract/` | low |
| repository memory / normative / evidence | plan / specs / reports | `plan/`, `specs/`, `docs/reports/` | low |

### target layer map

current repository memory principal is now `plan/19-repository-map-and-taxonomy.md`.
The current target map is conceptual, not a final public package freeze.

```text
repo/
  specs/                    # normative
  plan/                     # repository memory
  docs/                     # reader-facing summary + reports
  crates/                   # flat workspace, conceptually separated lanes
  samples/                  # active/base/planned/prototype/archive/generated
  scripts/                  # active runners + helper/support + storage/env + tests
  sub-agent-pro/            # handoff only
```

Conceptual lanes:

- Mir language substrate
- Mir verification substrate
- Mirrorea runtime substrate
- external adapter / host boundary
- visualization / debug / telemetry
- samples / hands-on / examples
- docs / specs / reports / planning

### changes made

- `AGENTS.md` に repository organization discipline を追加した。
- `README.md` と `Documentation.md` に layer-aware repo reading の導線を追加した。
- `samples_progress.md` を required structure に寄せ、active runnable rows と planned/spec-only rows を分けた。
- `plan/19-repository-map-and-taxonomy.md` を追加し、current repo map と staged migration plan を repository memory に昇格した。
- `samples/README.md`、`samples/generated/README.md`、`scripts/README.md` を追加し、sample/script taxonomy を reader-facing に固定した。
- `samples/current-l2/README.md` と `samples/not_implemented/README.md` の stale references を除去した。
- `docs/research_abstract/repository_layer_structure_01.md` を追加し、reader-facing な repo layer summary を置いた。
- `specs/11-roadmap-and-workstreams.md` に repository-structure staging addendum を追加した。
- `specs/10-open-questions.md` に staged physical migration / final package structure distinction の open question を追加した。
- `progress.md`、`tasks.md`、`plan/01`、`plan/08`、`plan/09`、`plan/12`、`plan/17`、`plan/90`、`plan/91` を current reading に合わせて同期した。

### files moved and intentionally not moved

#### files moved

- none

#### files intentionally not moved

- `crates/mir-ast`
- `crates/mir-semantics`
- `crates/mir-runtime`
- `crates/mirrorea-*`
- `crates/prism-*`
- `crates/engine-abi`
- `scripts/current_l2_*`
- `samples/not_implemented/avatar-fairy-follow/`
- `samples/prototype/current-l2-dynamic-attach-detach/`
- `mir_hilight.html`
- root cleanup candidates:
  - `handson_tmp`
  - `旧資料_*`

reason:

- current repo-local alpha の runnable path を壊すリスクが高い
- final public package structure が未決
- wrapper / alias なしの move は active command path を壊しやすい

### samples_progress status

- `samples_progress.md` exists and now follows the required dashboard structure:
  - `Legend`
  - `Summary`
  - `Active sample matrix`
  - `Base corpus / planned / spec-only matrix`
  - `E2E samples`
  - `Build/storage environment`
  - `Current blockers`
  - `Recent validation`
  - `Historical / archived samples`
- active runnable rows と planned/spec-only rows を分けたため、`samples/not_implemented/` と active suite の混同を抑えられる。

### AGENTS.md update

- updated: yes
- added:
  - repository organization discipline
  - active sample directory rule
  - historical sample archive rule
  - generated artifact separation
  - `samples_progress.md` update rule
  - no thick fake E2E wrappers
  - external workdir discipline for heavy artifacts

### Storage audit

- root filesystem:
  - `/dev/vda2`
  - `99G`
  - about `32G` free at audit time
- external workdir:
  - `/dev/vdb1`
  - ext4 `mirrorea-work`
  - mounted at `/mnt/mirrorea-work`
- repo `target/`:
  - symlink to `/mnt/mirrorea-work/cargo-target`
- existing storage helpers present:
  - `scripts/env/mirrorea_storage_env.sh`
  - `scripts/storage/setup_mirrorea_workdisk_root.sh`
  - `scripts/storage/detach_prepare.sh`
  - `scripts/storage/cleanup_disposable_artifacts.sh`
- this task did not format, mount, or clean any device.

## 6. Evidence / outputs / test results

- audit / classification:
  - `cargo metadata --no-deps --format-version 1`
  - pass
  - evidence:
    - 12 workspace members
    - flat crate layout remains intact
- hierarchy / docs:
  - `python3 scripts/check_source_hierarchy.py`
  - pass
  - evidence:
    - `required: 23`
    - `present: 23`
    - `missing: 0`
  - `python3 scripts/validate_docs.py`
  - pass
  - evidence:
    - `Documentation scaffold looks complete.`
    - `Found 918 numbered report(s).`
- active suite:
  - `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  - pass
  - evidence:
    - `matrix.total_samples = 16`
  - `python3 scripts/current_l2_guided_samples.py closeout --format json`
  - pass
  - evidence:
    - `active_sample_root = samples/clean-near-end`
  - `python3 scripts/clean_near_end_samples.py smoke-all --format json`
  - pass
  - evidence:
    - family counts:
      - `typing = 5`
      - `order-handoff = 6`
      - `model-check = 3`
      - `modal = 2`
- Sugoroku vertical slice:
  - `python3 scripts/sugoroku_world_samples.py check-all`
  - pass
  - evidence:
    - `sample_count = 10`
    - `failed = []`
  - `python3 scripts/sugoroku_world_samples.py closeout --format json`
  - pass
  - evidence:
    - `layer_signature_kinds = [membership, runtime_trace, verification]`
  - `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug layers`
  - pass
  - evidence:
    - `LAYER SIGNATURES`
    - active helper layers:
      - `verification`
      - `runtime_trace`
- tests:
  - `python3 -m unittest scripts.tests.test_sugoroku_world_samples`
  - pass
  - evidence:
    - `Ran 17 tests`
    - `OK`
  - `cargo test -p mir-ast`
  - pass
  - `cargo test -p mir-runtime --test clean_near_end_samples`
  - pass
  - `cargo test -p mir-runtime`
  - pass
  - `cargo test -p mir-semantics`
  - pass
- runtime closeout:
  - `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
  - pass
  - evidence:
    - `transport_provider_boundary`
  - `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json`
  - pass
  - evidence:
    - `layer_signatures` contains:
      - `auth_authority_witness`
      - `transport_provider_boundary`
      - `verification_model_check`
- whitespace / diff hygiene:
  - `git diff --check`
  - pass
- git commit / push status:
  - main closeout commit は作成済み
  - main closeout commit は `origin/main` へ push 済み
- missing helper:
  - `python3 scripts/avatar_follow_samples.py closeout`
  - missing
  - evidence:
    - `scripts/avatar_follow_samples.py not present`
  - current reading:
    - `samples/not_implemented/avatar-fairy-follow/` exists as planned skeleton only

### Reviewer completion status

- reviewer completion: yes
- reviewer run 1:
  - stale dashboard/package header
  - reader-facing taxonomy drift
  - `tasks.md` rough estimate 欠落
  - report ordering mismatch
  - all addressed in the same task
- reviewer run 2:
  - report metadata (`dirty state`, `reviewer completion`) 欠落
  - `samples_progress.md` section list mismatch
  - both addressed in the same task
- reviewer run 3:
  - no findings
- final closeout evidence:
  - `python3 scripts/check_source_hierarchy.py`
  - `python3 scripts/validate_docs.py`
  - `git diff --check`
  - all pass after the final reviewer fix cycle

## 7. What changed in understanding

- current repo は flat workspace / flat script root のままでも、docs-first taxonomy を加えれば layer-aware に十分読める。
- risk が高いのは path move そのものではなく、active runnable path と public contract の誤読である。したがって今回の principal gain は **動かすこと** ではなく **どこをまだ動かさないかを明文化したこと** にある。
- `samples/not_implemented/` は archive ではなく planned skeleton family として維持する方が、phase 8 以降の sample ladder を reader-friendly に保てる。
- `samples/generated/` は将来の non-Lean generated sample artifact の reserve とし、heavy disposable artifact は external workdir を優先する方が root disk discipline と整合する。

## 8. Open questions

- UNRESOLVED: `samples/not_implemented/` を later に `samples/planned/` のような名前へ physical move するか。
- UNRESOLVED: `docs/research_abstract/` hands-on 群を later に `docs/hands_on/` へ split するか。
- UNRESOLVED: root cleanup candidates を `docs/history/` へ寄せる staged move をどの package で行うか。
- UNRESOLVED: `scripts/current_l2_*` helper 群を `samples/` / `validation/` / `visualization/` へ rebucket する場合、どこまで wrapper / alias を先行させるか。
- UNRESOLVED: future crate split を `mirrorea-core` / `mirrorea-runtime` / `mirrorea-adapters` / `mirrorea-visualization` のどこまで切るか。

## 9. Suggested next prompt

`MessageEnvelope / Auth seam` package を進めてください。`auth none` baseline を壊さず、transport / authentication / authorization / membership / capability / witness を分離した minimal carrier を docs-first + helper-local first cut で追加し、Sugoroku helper と clean near-end runtime report の両方に narrow evidence を出してください。
