# 0613 — phase6 compile-ready verification and formal hook package

## Objective

Phase 6 front-half の compile-ready checkpoint を閉じる次段として、
selected cargo / smoke gate と theorem-line整合の tool-neutral formal hook first tranche を
source-backed minimum で actualize する。

## Scope and assumptions

- parser side の accepted floor は `mir-ast` stage 1 / stage 2 carrier に留める。
- checker/runtime side の accepted floor は `mir-semantics` program-level entry と `mir-runtime::current_l2` thin orchestrator に留める。
- concrete theorem prover binding、concrete model-check tool binding、parser second tranche widen、final public parser / checker / runtime surfaceは今回 fixed しない。
- `plan/07-parser-free-poc-stack.md` と `docs/research_abstract/README.md` は今回の current line 変更に対して追加 mirror を要しないと判断し、**plan/07 更新不要**、**research_abstract/README 更新不要** とした。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/00-index.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/130-current-l2-theorem-line-narrow-actualization-comparison.md`
- `specs/examples/131-current-l2-theorem-line-evidence-ref-family-comparison.md`
- `specs/examples/133-current-l2-theorem-line-minimum-contract-row-comparison.md`
- `specs/examples/301-current-l2-phase6-actual-parser-ast-carrier-first-tranche-ready-phase6-actual-checker-runtime-skeleton-first-tranche-comparison.md`
- `specs/examples/302-current-l2-phase6-actual-checker-runtime-skeleton-first-tranche-ready-minimal-phase6-actual-checker-runtime-skeleton-first-tranche-threshold.md`
- `docs/reports/0612-phase6-actual-checker-runtime-skeleton-first-tranche-package.md`
- `crates/mir-semantics/examples/current_l2_emit_formal_hook.rs`
- `crates/mir-semantics/examples/support/current_l2_static_gate_support.rs`
- `crates/mir-semantics/examples/support/current_l2_detached_bundle_support.rs`
- `scripts/current_l2_detached_loop.py`

## Actions taken

1. Phase 6 compile-ready checkpoint の next cut を再確認し、docs-only inventory ではなく、selected gate と tool-neutral emitted hook を narrow に固定するのが current minimum だと整理した。
2. `crates/mir-semantics/tests/current_l2_formal_hook_support.rs` を red/green の軸にし、formal hook row core を helper-local string path ではなく、Phase 5 theorem-line と同じ `obligation_kind + typed symbolic evidence_refs` に合わせた。
3. `crates/mir-semantics/examples/support/current_l2_formal_hook_support.rs` を追加し、`DetachedStaticGateArtifact` / `DetachedBundleArtifact` から emitted formal hook artifact を作る pure helper を実装した。subject は `fixture_static_cluster` / `runtime_try_cut_cluster` に留め、obligation は `canonical_normalization_law` / `no_re_promotion` / `rollback_cut_non_interference` に揃えた。
4. reviewer 指摘を受け、source artifact の `schema_version` / `artifact_kind` mismatch を fail-closed に止める validation を formal hook helper に追加し、negative tests も加えた。
5. `scripts/tests/test_current_l2_static_gate_loop.py` と `scripts/tests/test_current_l2_detached_loop.py` に actual `emit_formal_hook` CLI を叩く end-to-end tests を足し、exported JSON shape が theorem-line existing cut と整合することを固定した。
6. `scripts/current_l2_detached_loop.py` の `smoke-formal-hook-static` / `smoke-formal-hook-runtime` を selected smoke gate とし、actual fixture から formal hook artifact までの smoke path を確認した。
7. `specs/examples/303...304`、`Documentation.md`、`progress.md`、`tasks.md`、`specs/00-document-map.md`、`plan/09`、`plan/10`、`plan/11`、`plan/12`、`plan/17`、`plan/90`、`docs/research_abstract/phase6-compile-ready-minimal-actualization.md` を current mainline へ合わせて更新した。

## Files changed

- `crates/mir-semantics/examples/current_l2_emit_formal_hook.rs`
- `crates/mir-semantics/examples/support/current_l2_detached_bundle_support.rs`
- `crates/mir-semantics/examples/support/current_l2_formal_hook_support.rs`
- `crates/mir-semantics/examples/support/current_l2_static_gate_support.rs`
- `crates/mir-semantics/tests/current_l2_detached_bundle_support.rs`
- `crates/mir-semantics/tests/current_l2_formal_hook_support.rs`
- `scripts/current_l2_detached_loop.py`
- `scripts/tests/test_current_l2_static_gate_loop.py`
- `scripts/tests/test_current_l2_detached_loop.py`
- `specs/examples/303-current-l2-phase6-actual-checker-runtime-skeleton-first-tranche-ready-phase6-compile-ready-verification-and-formal-hook-comparison.md`
- `specs/examples/304-current-l2-phase6-compile-ready-verification-and-formal-hook-ready-minimal-phase6-compile-ready-verification-and-formal-hook-threshold.md`
- `specs/00-document-map.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `docs/reports/0613-phase6-compile-ready-verification-and-formal-hook-package.md`

## Commands run and exact outputs

```bash
cargo test -p mir-semantics --test current_l2_formal_hook_support
cargo test -p mir-semantics --test current_l2_static_gate_support
cargo test -p mir-semantics --test current_l2_detached_bundle_support
cargo test -p mir-semantics --test current_l2_minimal_interpreter
cargo test -p mir-runtime
cargo test -p mir-ast
cargo check -p mir-semantics --examples
python3 -m unittest scripts.tests.test_current_l2_static_gate_loop scripts.tests.test_current_l2_detached_loop
python3 scripts/current_l2_detached_loop.py smoke-formal-hook-static e5-underdeclared-lineage --artifact-root /tmp/mir_poc_formal_hook_artifacts --run-label formal-static --overwrite
python3 scripts/current_l2_detached_loop.py smoke-formal-hook-runtime e2-try-fallback --artifact-root /tmp/mir_poc_formal_hook_artifacts --run-label formal-runtime --overwrite
python3 scripts/validate_docs.py
git diff --check
git status --short
```

- `cargo test -p mir-semantics --test current_l2_formal_hook_support`
  - red phase では `examples/support/current_l2_formal_hook_support.rs` missing と off-spec row-shape issue があり、reviewer が theorem-line mismatch を指摘した
  - current green では `5 passed; 0 failed`
- `python3 -m unittest scripts.tests.test_current_l2_static_gate_loop scripts.tests.test_current_l2_detached_loop`
  - current green では `Ran 28 tests ... OK`
- `python3 scripts/current_l2_detached_loop.py smoke-formal-hook-static ...`
  - `static gate artifact: /tmp/mir_poc_formal_hook_artifacts/static-gates/formal-static/e5-underdeclared-lineage.static-gate.json`
  - `formal hook artifact: /tmp/mir_poc_formal_hook_artifacts/formal-hooks/formal-static/e5-underdeclared-lineage.formal-hook.json`
- `python3 scripts/current_l2_detached_loop.py smoke-formal-hook-runtime ...`
  - `bundle artifact: /tmp/mir_poc_formal_hook_artifacts/bundles/formal-runtime/e2-try-fallback.detached.json`
  - `formal hook artifact: /tmp/mir_poc_formal_hook_artifacts/formal-hooks/formal-runtime/e2-try-fallback.formal-hook.json`

## Evidence / outputs / test results

- emitted hook row core は `obligation_kind + typed symbolic evidence_refs` として固定した
- static side emitted hook は `fixture_static_cluster` subject に対して `canonical_normalization_law` / `no_re_promotion` を返す
- runtime side emitted hook は `runtime_try_cut_cluster` subject に対して `rollback_cut_non_interference` を返す
- schema/kind mismatch は support test で reject を確認した
- Python end-to-end tests で actual CLI output JSON shape を固定した

## What changed in understanding

- Phase 6 formal hook first tranche でも、new helper-local contract schema を作るより、Phase 5 theorem-line existing cut を emitted hook に再利用する方が drift が少ない。
- compile-ready checkpoint の formal gate では actual tool binding より先に、selected gate と emitted hook row core の整合を source-backed に閉じることが重要だった。
- detached artifact を Deserialize にしただけでは stale/foreign artifact を弾けないため、schema/kind validation は first tranche でも必要だった。

## Open questions

- concrete theorem prover binding をどの consumer pressure で reopen するか
- concrete model-check tool binding を theorem-side line とどう分けて narrow reopen するか
- compile-ready checkpoint close 後の next reopen point を parser second tranche widen と tool binding のどちらに置くか

## Suggested next prompt

```text
Phase 6 compile-ready checkpoint drift suppression / mirror sweep を進め、specs / progress / tasks / abstract / plan の stale wording を掃除して checkpoint close まで揃えてください。
```
