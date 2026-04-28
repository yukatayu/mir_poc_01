# 0973 — P18 public API / parser grammar gate first-cut closeout

## Objective

`P18` public API / parser grammar gate の **repo-side first cut** を close し、
final parser grammar / final public API を premature に freeze せずに、
freeze checklist / public-boundary inventory / mixed gate と true user-spec hold line の分離を
repo docs / plan / snapshot / dashboard に同期する。

## Scope and assumptions

- この task は final public freeze ではない
- `P18` で閉じるのは repo-side framing だけである
- actual final parser grammar、actual final public parser / checker / runtime / verifier API、
  actual final public adapter / viewer / projection / hot-plug / transport ABI、
  packaging / installed binary / FFI / engine adapter / final shared-space operational catalog は
  still later / post-`P18` true user-spec hold line に残す
- dirty worktree には user 既存変更として
  `crates/mir-ast/examples/current_l2_inspect_request_head_clause_bundle.rs`
  と `crates/mir-ast/src/current_l2.rs` があるため、触れていない

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/03-decision-strengths-and-boundaries.md`
- `plan/06-surface-notation-status.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/14-glossary-and-boundary-rules.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/19-repository-map-and-taxonomy.md`
- `plan/20-projection-and-placement-roadmap.md`
- `plan/21-hotplug-attachpoint-roadmap.md`
- `plan/22-network-transport-roadmap.md`
- `plan/23-compiler-backend-llvm-guardrail-roadmap.md`
- `plan/25-typed-external-boundary-executable-roadmap.md`
- `plan/26-visual-debugger-viewer-roadmap.md`
- `sub-agent-pro/mirrorea_next_stage_full_plan_handoff_2026-04-27.md`
- sub-agent completion notes from `Helmholtz` and `Halley`

## Actions taken

1. `P18` 専用の repository memory として `plan/27-public-api-parser-gate-roadmap.md` を追加した。
2. reader-facing summary / hands-on landing として
   `docs/research_abstract/public_api_parser_gate_plan_01.md`、
   `docs/hands_on/public_api_parser_gate_01.md` を追加した。
3. front door / snapshot / dashboard / near-term roadmap を
   `P18 repo-side first cut close` と
   `post-P18 true user-spec hold line` の読みへ切り替えた。
4. stale package-local next-package wording を `plan/21`、`plan/23`、`plan/26` で整理した。
5. `tasks.md` の `P11` detailed block に残っていた `started` drift を
   `close 済み` へ修正した。
6. `README.md`、`Documentation.md`、`specs/10`、`specs/11` に
   parser/public verifier だけでなく adapter / viewer / projection / hot-plug / transport を含む
   public-boundary mixed gate であることを明示した。
7. reviewer follow-up で、`docs/hands_on/current_phase_closeout_01.md` の remaining mixed gate、
   `plan/01-status-at-a-glance.md` の closed chain、`plan/11-roadmap-near-term.md` の `0972` chronology、
   front-door current-date wording を整合させ、narrow re-review no findings まで確認した。

## Files changed

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/19-repository-map-and-taxonomy.md`
- `plan/21-hotplug-attachpoint-roadmap.md`
- `plan/23-compiler-backend-llvm-guardrail-roadmap.md`
- `plan/26-visual-debugger-viewer-roadmap.md`
- `plan/27-public-api-parser-gate-roadmap.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/public_api_parser_gate_plan_01.md`
- `docs/hands_on/README.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/hands_on/public_api_parser_gate_01.md`
- `docs/reports/0973-p18-public-api-parser-gate-first-cut-closeout.md`

## Evidence / outputs / test results

- `df -h .`
  - pass
  - root disk: `/dev/vda2` `99G` total, `32G` avail, `67%`
- `free -h`
  - pass
  - memory available: `221Mi`, swap free: `18Gi`
- `bash scripts/env/mirrorea_storage_env.sh`
  - pass
  - `MIRROREA_WORKDIR=/mnt/mirrorea-work`
  - `CARGO_TARGET_DIR=/mnt/mirrorea-work/cargo-target`
  - `CARGO_HOME=/mnt/mirrorea-work/cargo-registry-cache`
  - warning only: `/mnt/mirrorea-work/llvm` parent is `root:root` and non-writable
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
  - pass
  - `message_envelope_scope = representative_slice`
  - `signature_lanes = kind / name / evidence_role`
  - `hotplug_scope = helper_local_package_manager_preview`
- `python3 scripts/typed_external_boundary_samples.py closeout --format json`
  - pass
  - `host_boundary_scope = helper_local_synthetic_preview`
  - `host_boundary_lanes = request / receipt / failure / visualization`
  - residual reopen matrix for `EXT-01` / `EXT-02` / `EXT-05` present
- `python3 scripts/network_transport_samples.py closeout --format json`
  - pass
  - `transport_scope = helper_local_process_boundary`
  - `process_boundary_canaries = [NET-02, NET-03, NET-04, NET-05]`
- `python3 scripts/projection_codegen_samples.py closeout --format json`
  - pass
  - `artifact_boundary = committed manifest bridge evidence only; not a final emitted executable program`
  - `generated_bridge_artifact_inventory` present
- `python3 scripts/visual_debugger_viewer_samples.py closeout --format json`
  - pass
  - `prototype_boundary = typed public prototype inventory over helper/runtime surfaces; not a final public viewer API`
  - `viewer_panel_lanes` / `viewer_telemetry_lanes` present
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json`
  - pass
  - `message_envelope_scope = clean_near_end_canonical_inventory`
  - `transport_seams = provider_boundary / audit_trace_boundary`
  - canonical `LayerSignature` / visualization / telemetry inventories present
- `python3 scripts/check_source_hierarchy.py`
  - pass
  - required paths present: `23/23`
- `python3 scripts/validate_docs.py`
  - pass
  - `Documentation scaffold looks complete.`
  - `Found 971 numbered report(s).`
- `git diff --check`
  - pass
- reviewer narrow re-review
  - pass
  - no findings

## What changed in understanding

- `P18` は parser/public API だけの gate ではなく、viewer / adapter / projection / hot-plug / transport /
  auth を含む public-boundary mixed gate として inventory 化すべきである。
- ただし `P18` で閉じるのは actual final freeze ではなく、
  repo-side qualifier / checklist / hold-line split である。
- `P17` storage/backend guardrail も `P18` では productization judgment ではなく
  toolchain adjacency inventory として扱う方が一貫する。

## Open questions

- actual final parser grammar をいつ、どの evidence floor で freeze するか
- actual final public parser / checker / runtime / verifier API をどこまで library-first にするか
- final public adapter / viewer / projection / hot-plug / transport ABI の first shipped surface をどこに切るか
- packaging / installed binary / FFI / engine adapter / final shared-space operational catalog の target をどう選ぶか

## Suggested next prompt

post-`P18` true user-spec hold line を進めたいので、次の 4 点を docs-first option matrix と current recommendation つきで整理してください。

1. installed binary の形は `CLI` / `library` / `engine-adapter` / `hybrid` のどれを first target にするか
2. host integration target は browser / native process / engine / mixed のどれを first target にするか
3. final shared-space operational catalog は minimal subset 維持か、portal / multi-world / fairness / quorum まで含めるか
4. final public surface の first shipped scope は parser/checker/runtime/verifier 優先か、adapter/viewer/projection/hot-plug を同時に入れるか
