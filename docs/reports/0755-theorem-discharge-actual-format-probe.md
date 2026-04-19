# 0755 — theorem discharge actual-format probe

## Objective

`theorem discharge public-contract actual format` mixed gate を、final public theorem contract に誤昇格させずに helper-local actual-format probe まで actualize する。

## Scope and assumptions

- `specs/` を規範正本、`plan/` を repository memory、`docs/reports/` を時系列証跡として扱う。
- 今回 fixed するのは helper-local actual-format probe であり、actual discharge transport / public theorem contract / concrete theorem prover brand / proof object public schema は fixed しない。
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
- `specs/examples/440-current-l2-notebook-consumer-threshold-and-discharge-reserve-note.md`
- `specs/examples/446-current-l2-theorem-discharge-transport-and-public-contract-later-gate-framing-note.md`
- `specs/examples/465-current-l2-theorem-discharge-prefloor-and-public-contract-mixed-gate-note.md`
- `specs/examples/470-current-l2-theorem-first-experimental-pilot-actualization.md`
- `specs/examples/474-current-l2-theorem-prover-experimental-binding-preflight.md`
- `specs/examples/475-current-l2-principal-theory-spine-and-lean-first-proof-roadmap.md`
- `specs/examples/478-current-l2-model-check-second-line-concretization.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`

## Actions taken

1. theorem discharge actual-format probe 用の failing test を起点に、support helper の missing surface を確認した。
2. `current_l2_source_sample_emitted_artifact_support.rs` に theorem discharge actual-format probe struct / builder / helper refs を追加した。
3. `e5 / p06 / p07 / p08` reached、`p05` guard-only の theorem discharge actual-format probe test を green に戻した。
4. actual-format probe を `specs/examples/479` に actualization note として追加した。
5. `Documentation.md`、`progress.md`、`tasks.md`、relevant `plan/`、relevant `specs/` を `479` と remaining mixed-gate reading に同期した。
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
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `specs/examples/479-current-l2-theorem-discharge-actual-format-probe.md`

## Commands run

```bash
df -h .
free -h
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
cargo test -p mir-runtime --test current_l2_theorem_discharge_actual_format_probe
```

RED:

```text
error[E0432]: unresolved imports
CurrentL2SourceSampleTheoremDischargeActualFormatProbe
build_current_l2_source_sample_theorem_discharge_actual_format_probe
```

GREEN:

```text
running 5 tests
test result: ok. 5 passed; 0 failed
```

Representative validation:

```bash
cargo test -p mir-runtime \
  --test current_l2_theorem_discharge_actual_format_probe \
  --test current_l2_theorem_discharge_prefloor \
  --test current_l2_theorem_first_pilot_actualization \
  --test current_l2_theorem_prover_binding_preflight \
  --test current_l2_model_check_second_line_concretization \
  --test current_l2_authoritative_room_vertical_slice_actualization \
  --test current_l2_auditable_authority_witness_strengthening \
  --test current_l2_delegated_rng_service_practical_actualization \
  --test current_l2_source_sample_runner \
  --test current_l2_operational_cli
python3 scripts/current_l2_source_sample_regression.py inventory
python3 scripts/current_l2_source_sample_regression.py regression
python3 scripts/validate_docs.py
git diff --check
```

Representative output:

```text
test result: ok. 68 passed; 0 failed
authored inventory confirmed
all regression commands passed
Documentation scaffold looks complete.
Found 754 numbered report(s).
git diff --check: no output
```

## Evidence / outputs / test results

- theorem discharge actual-format probe は `e5 / p06 / p07 / p08` reached、`p05` guard-only で machine-check 済み。
- transport preview refs と public-contract preview refs は、row-local review unit / discharge-entry reserve / symbolic evidence refs を壊さずに合成できた。
- repo-local emitted artifact refs は theorem-first pilot actualization と同じ floor に留まっている。
- current principal self-driven queue は依然 `none` だが、theorem line の remaining mixed gate は `actual-format probe 前` ではなく `actual discharge transport / public theorem contract 本体判断` に narrowed した。

## What changed in understanding

- theorem discharge line は compare floor / pilot / preflight だけでなく、actual-format probe まで helper-local actualization しても public theorem contract へ誤昇格しない。
- current mixed gate は `format を全く持たない` のではなく、brand-neutral preview / boundary refs を持ちながら final adoption を止める形に narrowed できる。
- next reopen line は theorem discharge actual-format probe ではなく、settled property language / concrete tool seam か、actual discharge transport / public theorem contract 本体判断である。

## Open questions

- actual discharge transport をどの mixed gate で helper-local probe から actual adoption へ上げるか。
- public theorem contract と proof object public schema をどこで分離 / 接続するか。
- settled property language / concrete tool seam を theorem line より先に reopen するか、同列で扱うか。
- final public verifier contract を theorem/model-check/runtime policy のどこに切るか。

## Suggested next prompt

`settled property language / concrete tool seam mixed gate を helper-local actualization まで進め、theorem discharge actual-format probe と対になる current recommendation / retained alternatives / stop line を docs と tests に反映してください。`
