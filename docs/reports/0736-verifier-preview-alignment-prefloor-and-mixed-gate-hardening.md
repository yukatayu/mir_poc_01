# Report 0736 — verifier preview alignment pre-floor and mixed-gate hardening

## 1. Title and identifier

- Report ID: `0736`
- Title: `verifier preview alignment pre-floor and mixed-gate hardening`

## 2. Objective

typed / theorem / model-check mixed gate のうち、
helper-local `verification_preview` / `artifact_preview` を final public verifier contract に昇格させずに、
prototype bucket を含む representative corpus で compare/validation floor を 1 段厚くする。

## 3. Scope and assumptions

- scope は current L2 current line に限定する。
- mainline public surface は widen しない。
- fixture-backed emitted-artifact route は source-authored fixture chain の evidence として保持する。
- prototype bucket も含む current compare floor が必要だが、final public contract adoption はしない。

## 4. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/381-current-l2-model-check-concrete-carrier-first-actualization-ready-source-sample-emitted-verification-artifact-wiring-comparison.md`
- `specs/examples/446-current-l2-theorem-discharge-transport-and-public-contract-later-gate-framing-note.md`
- `specs/examples/447-current-l2-model-check-property-language-and-tool-binding-later-gate-framing-note.md`
- `specs/examples/453-current-l2-sample-verification-preview-and-prototype-second-tranche.md`
- `specs/examples/454-current-l2-artifact-preview-and-underdeclared-source-gap-note.md`
- `specs/examples/459-current-l2-verifier-boundary-and-typed-theorem-model-check-current-first-line-integration.md`
- `specs/examples/462-current-l2-theory-line-near-end-closeout-and-mixed-gate-only-reading.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `samples/prototype/README.md`

## 5. Actions taken

1. RED として `crates/mir-runtime/tests/current_l2_verifier_preview_alignment.rs` を追加し、preview summary と typed artifact side support の literal alignment を代表 sample で確認する failing test を作った。
2. systematic debugging で failure を切り分けた。
   - prototype bucket は existing fixture-backed emitted-artifact route helper が直接は cover しない
   - current authored sample では CLI preview が kebab-case `sample_id`、formal hook / typed artifact side が underscore `fixture_id` を使っていた
3. fix は existing fixture-backed emitted-artifact route を無理に public-looking に広げる方向ではなく、sample-local preview-aligned typed artifact route を test/support helper-local compare floor として追加する方向に絞った。
4. `crates/mir-runtime/tests/support/current_l2_source_sample_emitted_artifact_support.rs` に sample-local preview-aligned typed artifact route を追加し、runtime/static/guarded/prototype sample を `sample_id` aligned subject ref と symbolic evidence refs で compare できるようにした。
5. failing alignment test を new helper route に合わせて GREEN にした。
6. `specs/examples/463` を追加し、preview summary / fixture-backed emitted route / sample-local preview-aligned route の current relation と mixed-gate pre-floor judgment を docs-first に固定した。
7. `Documentation.md`、`progress.md`、`tasks.md`、relevant `plan/`、`docs/research_abstract/`、`samples/prototype/README.md`、`plan/90-source-traceability.md` を更新し、snapshot / memory / traceability を同期した。

## 6. Files changed

- Added:
  - `crates/mir-runtime/tests/current_l2_verifier_preview_alignment.rs`
  - `specs/examples/463-current-l2-verifier-preview-alignment-prefloor-and-public-contract-mixed-gate-note.md`
  - `docs/reports/0736-verifier-preview-alignment-prefloor-and-mixed-gate-hardening.md`
- Modified:
  - `crates/mir-runtime/tests/support/current_l2_source_sample_emitted_artifact_support.rs`
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `plan/00-index.md`
  - `plan/01-status-at-a-glance.md`
  - `plan/08-representative-programs-and-fixtures.md`
  - `plan/09-helper-stack-and-responsibility-map.md`
  - `plan/10-roadmap-overall.md`
  - `plan/11-roadmap-near-term.md`
  - `plan/12-open-problems-and-risks.md`
  - `plan/17-research-phases-and-autonomy-gates.md`
  - `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
  - `plan/90-source-traceability.md`
  - `docs/research_abstract/README.md`
  - `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
  - `samples/prototype/README.md`
  - `specs/00-document-map.md`
  - `specs/10-open-questions.md`
  - `specs/11-roadmap-and-workstreams.md`
  - `specs/12-decision-register.md`

## 7. Commands run

- RED / root-cause capture:
  - `cargo test -p mir-runtime --test current_l2_verifier_preview_alignment`
    - first run: `FAILED`
    - mismatches:
      - runtime prototype rows: CLI preview `reached` vs existing fixture-backed route `guarded_not_reached`
      - static underdeclared row: CLI `subject_ref = e5-underdeclared-lineage` vs fixture-backed route `subject_ref = e5_underdeclared_lineage`
- Evidence gathering:
  - `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-typed-proof-model-check/p06-typed-proof-owner-handoff.txt --format json`
  - `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample e5-underdeclared-lineage --host-plan crates/mir-ast/tests/fixtures/current-l2/e2-try-fallback.host-plan.json --format json`
- GREEN:
  - `cargo test -p mir-runtime --test current_l2_verifier_preview_alignment`
    - `5 passed`
  - `cargo test -p mir-runtime --test current_l2_source_sample_emitted_artifact_wiring`
    - `9 passed`
  - `cargo test -p mir-runtime --test current_l2_operational_cli`
    - `12 passed`
  - `python3 scripts/validate_docs.py`
    - `Documentation scaffold looks complete.`
    - `Found 735 numbered report(s).`
  - `git diff --check`
    - no output

## 8. Evidence / outputs / test results

- RED failure 自体が、existing fixture-backed route を prototype bucket compare floor にそのまま使えないことを示した。
- `p06` の CLI JSON は `runtime_try_cut_cluster` reached、`rollback_cut_non_interference`、`fixture:p06-...` / `runtime_cluster:p06-...` を示した。
- `e5` の CLI JSON は `fixture_static_cluster` reached、`canonical_normalization_law` / `no_re_promotion` を示した。
- new alignment harness は `e5 / p05 / p06 / p07 / p08` を current representative corpus として green になった。
- existing fixture-backed emitted-artifact route test も `9 passed` で維持されたため、old source-authored fixture chain は壊していない。
- current operational CLI test も `12 passed` で維持されたため、public shell concern は変えていない。

## 9. What changed in understanding

- existing fixture-backed emitted-artifact route は source-authored fixture chain evidence としては正しいが、prototype bucket compare floor の source of truth ではない。
- helper-local `verification_preview` / `artifact_preview` を public contract へ上げる前に、sample-local preview-aligned typed artifact route を別 helper に切る方が current repo boundary と整合する。
- current mixed gate で進めるべきことは「final contract adoption」ではなく、「どの helper-local floor が drift なく representative corpus を cover するか」の hardening である。

## 10. Open questions

- theorem discharge transport / public-contract seam 自体は still later である。
- first settled property language / concrete tool seam も still later である。
- sample-local preview-aligned route を later public layer へ昇格するかは未決である。
- stronger typed surface actual adoption もこの task では reopen していない。

## 11. Suggested next prompt

`specs/examples/463` を前提に、theorem discharge transport / public-contract と model-check property-language / concrete tool seam のどちらを next mixed-gate concretization package として reopen するかを比較し、必要なら tiny non-production compare helper か sample-side evidence を足してください。
