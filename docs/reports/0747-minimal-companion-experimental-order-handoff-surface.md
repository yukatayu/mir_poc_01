# Report 0747 — minimal companion experimental order handoff surface

- Date: 2026-04-18T02:35:09.072032Z
- Author / agent: Codex
- Scope: syntax / modality current recommendation の後段として、semantically honest な minimal companion surface を helper-local に actualize する。
- Decision levels touched: L2

## 1. Objective

- final grammar を凍らせず、`p07` / `p08` current default room profile を helper-local companion lines に actualize する。

## 2. Inputs consulted

- `faq_007.md`
- `specs/examples/468-current-l2-syntax-modality-convergence-and-current-recommendation.md`
- `specs/examples/471-current-l2-authoritative-room-vertical-slice-emitted-artifact-ratchet.md`
- `tasks.md`
- `progress.md`
- `crates/mir-runtime/tests/support/current_l2_source_sample_emitted_artifact_support.rs`
- `samples/prototype/README.md`

## 3. Actions taken

1. RED として `crates/mir-runtime/tests/current_l2_order_handoff_minimal_companion_surface.rs` を追加し、`p07` / `p08` reached と `p05` guard-only の expected companion lines を固定した。
2. `current_l2_source_sample_emitted_artifact_support.rs` に minimal companion helper route を追加し、vertical-slice helper を起点に semantically honest な lines を組み立てるようにした。
3. `specs/examples/472-current-l2-minimal-companion-experimental-order-handoff-surface.md` を追加し、helper-local companion cut / retained alternatives / stop line を docs-first に固定した。
4. `Documentation.md`、`tasks.md`、`progress.md`、relevant `plan/` / `specs/` を later integrated snapshot で同期した。

## 4. Files changed

- `crates/mir-runtime/tests/current_l2_order_handoff_minimal_companion_surface.rs`
- `crates/mir-runtime/tests/support/current_l2_source_sample_emitted_artifact_support.rs`
- `specs/examples/472-current-l2-minimal-companion-experimental-order-handoff-surface.md`
- `Documentation.md`
- `tasks.md`
- `progress.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `samples/prototype/README.md`

## 5. Commands run and exact outputs

- `cargo test -p mir-runtime --test current_l2_order_handoff_minimal_companion_surface`
  - RED: unresolved imports for minimal companion helper names
  - GREEN: `running 3 tests ... test result: ok. 3 passed; 0 failed`
- later integrated verification:
  - combined mir-runtime test sweep passed
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
- `git diff --check`
  - no output

## 6. Evidence / findings

- `p07` では publication / handoff / late-join-visible-as-past を final grammar に上げず helper-local companion lines へ actualize できる。
- `p08` では rollback / refresh / replay invalidation を same helper-local cut に actualize できる。
- `p05` guarded case は reached せず、companion surface を guard-only floor に残せる。
- low-level `memory_order` exact token や packed metadata wording を import せずとも、current recommendation を executable evidence に結び付けられる。

## 7. Changes in understanding

- `specs/examples/468` の syntax recommendation は prose-only comparison に留める必要はなく、helper-local companion cut までは actualize できる段階にあった。
- vertical-slice helper を起点にすることで、minimal companion を order/handoff default profile から乖離させずに保てる。
- 規範判断として追加したのは L2 helper-local surface cut であり、final parser grammar や final modal foundation adoption は変更していない。

## 8. Open questions

- helper-local companion surface をどの mixed gate まで helper-local に保つか。
- final parser grammar へ移る前に low-level exact surface をどこまで排除し続けるか。
- final modal foundation adoption をどの package で reopen するか。

## 9. Suggested next prompt

- `specs/examples/472` を前提に theorem-prover experimental binding preflight、または `auditable_authority_witness` strengthening を self-driven に進めてください。
