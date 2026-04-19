# 0788 — theorem Lean-stub representative trace-alignment bridge

## Objective

repo-local theorem artifact-conformance bridge を
authored source sample `e2 / e5` だけで止めず、
`p06 / p07 / p08` を含む representative prototype corpus まで伸ばし、
review unit / Lean stub pair alignment を helper-local actualization floor として固定する。

## Scope and assumptions

- `specs/` を規範正本として扱う。
- actual Lean tool execution は導入しない。
- current package は representative bridge であり、prototype-wide exhaustive alignment ではない。
- `p05` は guard-only contrast に維持する。
- public theorem contract、proof object public schema、cross-tool public artifact-conformance contract は今回 fixed しない。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/508-current-l2-theorem-lean-first-nonproduction-stub-pilot-actualization.md`
- `specs/examples/509-current-l2-theorem-review-unit-to-lean-stub-repo-local-artifact-conformance-bridge.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`

## Actions taken

1. RED 先行で `crates/mir-runtime/tests/current_l2_theorem_lean_stub_trace_alignment_bridge.rs` を追加し、missing struct / builder による compile failure を確認した。
2. `crates/mir-runtime/tests/support/current_l2_source_sample_emitted_artifact_support.rs` に `CurrentL2SourceSampleTheoremLeanStubTraceAlignmentBridge` と `build_current_l2_source_sample_theorem_lean_stub_trace_alignment_bridge` を追加した。
3. review unit side / Lean stub side の `(subject_ref, obligation_kind)` pair を `theorem_trace_alignment_pair:*` ref へ正規化し、exact match を alignment predicate に固定した。
4. representative corpus `e2 / e5 / p06 / p07 / p08` と guard-only `p05` を focused runtime test に追加した。
5. `specs/examples/510-current-l2-theorem-lean-stub-representative-trace-alignment-bridge.md` を新設し、current recommendation / retained alternatives / stop line を source-backed に固定した。
6. relevant `Documentation.md` / `tasks.md` / `progress.md` / `plan/` / `specs/` / traceability docs を representative trace-alignment bridge 前提に同期した。

## Evidence / outputs / test results

- RED:
  - `cargo test -p mir-runtime --test current_l2_theorem_lean_stub_trace_alignment_bridge`
  - unresolved imports
    - `CurrentL2SourceSampleTheoremLeanStubTraceAlignmentBridge`
    - `build_current_l2_source_sample_theorem_lean_stub_trace_alignment_bridge`
- focused GREEN:
  - `cargo test -p mir-runtime --test current_l2_theorem_lean_stub_trace_alignment_bridge`
  - `6 passed`

## What changed in understanding

- theorem artifact-conformance bridge は representative prototype corpus まで self-driven に伸ばせる。
- required predicate は actual Lean execution ではなく、review unit side / Lean stub side の exact pair match で十分に narrow に fixed できる。
- unresolved なのは pair alignment そのものではなく、prototype-wide exhaustive coverage、actual Lean execution、public theorem contract 群である。

## Open questions

- prototype-wide exhaustive alignment をどこで止めるか。
- actual Lean tool execution をいつ reopen するか。
- representative trace alignment を public theorem contract / proof object public schema / cross-tool public artifact-conformance contract にいつ接続するか。

## Suggested next prompt

`representative theorem trace alignment bridge の次として、order-handoff serial-scope sugar reserve surface を actualize するか、witness/provider emitted-contract trace alignment bridge を先に閉じるかを source-backed に選んでください。`
