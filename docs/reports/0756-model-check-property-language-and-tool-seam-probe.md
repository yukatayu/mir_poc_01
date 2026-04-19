# 0756 — model-check property-language and tool-seam probe

## Objective

`settled property language / concrete tool seam` mixed gate を、final property language や actual public checker migration に誤昇格させずに helper-local property/tool-seam probe まで actualize する。

## Scope and assumptions

- `specs/` を規範正本、`plan/` を repository memory、`docs/reports/` を時系列証跡として扱う。
- 今回 fixed するのは helper-local property/tool-seam probe であり、first settled property language / concrete tool brand / actual public checker migration / production checker-runtime-policy contract は fixed しない。
- current principal self-driven queue は `none` のまま維持し、remaining work は mixed gate / reserve integration として読む。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/415-current-l2-model-check-projection-and-property-family-reserve-inventory.md`
- `specs/examples/420-current-l2-model-check-carrier-to-projection-bridge-note.md`
- `specs/examples/464-current-l2-model-check-projection-prefloor-and-property-tool-seam-mixed-gate-note.md`
- `specs/examples/475-current-l2-principal-theory-spine-and-lean-first-proof-roadmap.md`
- `specs/examples/478-current-l2-model-check-second-line-concretization.md`
- `specs/examples/479-current-l2-theorem-discharge-actual-format-probe.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`

## Actions taken

1. model-check second-line concretization の上に property-language / tool-seam probe を重ねる failing test を追加した。
2. `current_l2_source_sample_emitted_artifact_support.rs` に property/tool-seam probe struct / builder / helper refs を追加した。
3. `e5 / p06 / p07 / p08 / p09` reached、`p05` guard-only の property/tool-seam probe test を green に戻した。
4. property/tool-seam probe を `specs/examples/480` に actualization note として追加した。
5. `Documentation.md`、`progress.md`、`tasks.md`、relevant `plan/`、relevant `specs/` を `480` と remaining mixed-gate reading に同期した。
6. `plan/90-source-traceability.md` に addendum を追加した。

## Files changed

- `crates/mir-runtime/tests/support/current_l2_source_sample_emitted_artifact_support.rs`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `specs/examples/480-current-l2-model-check-property-language-and-tool-seam-probe.md`

## Commands run

```bash
cargo test -p mir-runtime --test current_l2_model_check_property_tool_seam_probe
```

RED:

```text
error[E0432]: unresolved imports
CurrentL2SourceSampleModelCheckPropertyToolSeamProbe
build_current_l2_source_sample_model_check_property_tool_seam_probe
```

GREEN:

```text
running 6 tests
test result: ok. 6 passed; 0 failed
```

Representative validation:

```bash
cargo test -p mir-runtime \
  --test current_l2_model_check_property_tool_seam_probe \
  --test current_l2_model_check_second_line_concretization \
  --test current_l2_theorem_discharge_actual_format_probe \
  --test current_l2_theorem_prover_binding_preflight \
  --test current_l2_source_sample_runner \
  --test current_l2_operational_cli
python3 scripts/current_l2_source_sample_regression.py regression
python3 scripts/validate_docs.py
git diff --check
```

Representative output:

```text
test result: ok. 55 passed; 0 failed
all regression commands passed
Documentation scaffold looks complete.
Found 755 numbered report(s).
git diff --check: no output
```

## Evidence / outputs / test results

- property/tool-seam probe は `e5 / p06 / p07 / p08 / p09` reached、`p05` guard-only で machine-check 済み。
- property-language probe refs と tool-seam probe refs は、row-local carrier / property preview / small-cluster projection second line を壊さずに合成できた。
- checker-boundary probe refs により、public checker migration を actual public contract と誤読しない stop line を helper-local manifest 側で保持できた。
- current principal self-driven queue は依然 `none` だが、model-check line の remaining mixed gate は `property/tool-seam probe 前` ではなく `first settled property language / concrete model-check tool brand / actual public checker migration 本体判断` に narrowed した。

## What changed in understanding

- model-check line は compare floor / second-line concretization だけでなく、property/tool-seam probe まで helper-local actualization しても public checker migration へ誤昇格しない。
- theorem discharge actual-format probe と model-check property/tool-seam probe を sibling helper cut に保つことで、Problem 1 の残 mixed gate を theorem side と model-check sideでより対称に読める。
- next reopen line は property/tool-seam probe ではなく、actual discharge transport / public theorem contract か、first settled property language / concrete tool brand の本体判断である。

## Open questions

- actual discharge transport / public theorem contract を model-check line より先に reopen するか。
- first settled property language と concrete model-check tool brand / schema をどこで切り分けるか。
- actual public checker migration と emitted verifier handoff artifact をどこで接続するか。
- final public verifier contract を theorem/model-check/runtime policy のどこに切るか。

## Suggested next prompt

`actual discharge transport / public theorem contract mixed gate を helper-local actualization か actual adoption 判断のどちらまで押せるかを検証し、theorem discharge actual-format probe と property/tool-seam probe の対称性を docs と tests に反映してください。`
