# 0786 — theorem Lean-first non-production stub pilot actualization

## Objective

theorem-first external integration target を brand-neutral preflight だけに止めず、
review-unit principal を保ったまま
Lean-first の non-production emitted stub pilot を helper-local actualization する。

## Scope and assumptions

- `specs/` を規範正本として扱う。
- Lean-first は current recommendation だが、final adopted theorem prover brand ではない。
- actual Lean tool execution、actual discharge transport、public theorem contract、proof object public schema、final public verifier contract は今回 fixed しない。
- representative corpus は `e5 / p06 / p07 / p08` reached、`p05` guard-only に置く。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/470-current-l2-theorem-first-experimental-pilot-actualization.md`
- `specs/examples/474-current-l2-theorem-prover-experimental-binding-preflight.md`
- `specs/examples/475-current-l2-principal-theory-spine-and-lean-first-proof-roadmap.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `sub-agent-pro/codex_theory_handoff_2026-04-18.md`

## Actions taken

1. RED 先行で `crates/mir-semantics/tests/current_l2_lean_theorem_stub_support.rs` と `crates/mir-runtime/tests/current_l2_theorem_lean_stub_pilot_actualization.rs` を追加し、missing support module / missing runtime builder による compile failure を確認した。
2. `crates/mir-semantics/examples/support/current_l2_lean_theorem_stub_support.rs` を追加し、proof notebook review unit から Lean-first theorem stub artifact を生成する helper-local support を実装した。
3. `crates/mir-semantics/examples/current_l2_emit_lean_theorem_stub.rs` を追加し、review-unit artifact JSON から Lean stub artifact JSON を emit する non-production CLI example を実装した。
4. `crates/mir-runtime/tests/support/current_l2_source_sample_emitted_artifact_support.rs` に Lean-first stub pilot actualization route を追加し、preflight route の reached/guarded boundaryを保ったまま repo-local emitted stub refs を machine-check できるようにした。
5. `specs/examples/508-current-l2-theorem-lean-first-nonproduction-stub-pilot-actualization.md` を新設し、current recommendation / retained alternatives / stop line / non-goal を actual package として固定した。
6. `Documentation.md`、`tasks.md`、`progress.md`、relevant `plan/` / `specs/` / `samples/prototype/README.md` / `docs/research_abstract/` を同期した。

## Evidence / outputs / test results

- RED:
  - `cargo test -p mir-semantics --test current_l2_lean_theorem_stub_support`
  - missing support module `current_l2_lean_theorem_stub_support.rs`
  - `cargo test -p mir-runtime --test current_l2_theorem_lean_stub_pilot_actualization`
  - unresolved imports
    - `CurrentL2SourceSampleTheoremLeanStubPilotActualization`
    - `build_current_l2_source_sample_theorem_lean_stub_pilot_actualization`
- focused GREEN:
  - `cargo test -p mir-semantics --test current_l2_lean_theorem_stub_support`
  - `4 passed`
  - `cargo test -p mir-runtime --test current_l2_theorem_lean_stub_pilot_actualization`
  - `5 passed`
- local tool availability check:
  - `command -v lean`
  - not found
  - `command -v lake`
  - not found

## What changed in understanding

- theorem-first external integration target は、brand-neutral preflight の次段として Lean-first non-production emitted stub pilot まで self-driven に actualize できる。
- concrete theorem prover brand を public contract に上げなくても、review-unit first / repo-local emitted stub refs first の concrete brand pressure は machine-check できる。
- actual external Lean execution が未導入でも、current repo では helper-local emitted artifact line を一段前へ進められる。

## Open questions

- actual Lean tool execution をどの mixed gate で reopen するか。
- emitted stub を actual discharge transport / public theorem contract / proof object public schema のどこへ繋ぐか。
- Lean-first stub pilot を result-object / payload / verifier contract reopen threshold とどう接続するか。
- Rocq/Coq + Iris fallback をどの pressure で helper-local compare package に戻すか。

## Suggested next prompt

`theorem Lean-first non-production stub pilot actualization の次として、actual Lean tool execution を伴わない artifact-conformance package に進むか、それとも order-handoff / shared-space final mixed gate をさらに narrow に actualize するかを source-backed に選んでください。`
