# Report 0951 — P4 TermSignature registry hardening re-review

## Objective

`P4` `TermSignature` registry hardening diff を narrow scope で再レビューし、semantic regression、stale queue wording、overclaim の有無だけを確認する。

## Scope and assumptions

- 対象は user 指定の `P4` diff scope に限定する。
- style-only comment は扱わない。
- 実装修正は行わず、review evidence と residual risk だけを記録する。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/00-index.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/14-glossary-and-boundary-rules.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `samples/clean-near-end/sugoroku-world/README.md`
- `docs/reports/0950-p4-term-signature-registry-hardening.md`
- `scripts/sugoroku_world_samples.py`
- `scripts/tests/test_sugoroku_world_samples.py`
- `crates/mir-runtime/src/clean_near_end.rs`
- `crates/mir-runtime/tests/clean_near_end_samples.rs`

## Actions taken

1. Read the repository front-door and normative docs in the required order, then narrowed to the `P4` scope files.
2. Inspected the actual diff to isolate the `TermSignature` registry hardening changes from unrelated repo work.
3. Checked runtime/helper code paths for:
   - provenance preservation via `(kind, name, evidence_role)`
   - removal of `history` / `witness-field` / `proof-obligation` from active `TermSignature` kinds
   - helper/runtime closeout scope split
4. Checked snapshot/docs/report wording for:
   - `P4` closed, `P5` next, `P6` reopen-next
   - no overclaim toward final public signature / message / adapter contracts
5. Spot-checked live JSON outputs to confirm the docs match emitted closeout data.

## Files changed

- `docs/reports/0951-p4-term-signature-rereview.md`
- `progress.md` 更新不要
- `tasks.md` 更新不要
- `samples_progress.md` 更新不要
- `plan/` 更新不要

## Commands run

```bash
git status --short
git diff --stat
git diff --name-only
wc -l README.md Documentation.md progress.md tasks.md plan/00-index.md plan/09-helper-stack-and-responsibility-map.md plan/14-glossary-and-boundary-rules.md specs/10-open-questions.md specs/11-roadmap-and-workstreams.md docs/research_abstract/mirrorea_future_axis_01.md docs/hands_on/current_phase_closeout_01.md samples/clean-near-end/sugoroku-world/README.md docs/reports/0950-p4-term-signature-registry-hardening.md
git diff -- scripts/sugoroku_world_samples.py scripts/tests/test_sugoroku_world_samples.py crates/mir-runtime/src/clean_near_end.rs crates/mir-runtime/tests/clean_near_end_samples.rs README.md Documentation.md progress.md tasks.md samples_progress.md plan/09-helper-stack-and-responsibility-map.md plan/14-glossary-and-boundary-rules.md specs/10-open-questions.md specs/11-roadmap-and-workstreams.md docs/research_abstract/mirrorea_future_axis_01.md docs/hands_on/current_phase_closeout_01.md samples/clean-near-end/sugoroku-world/README.md docs/reports/0950-p4-term-signature-registry-hardening.md
ls specs
sed -n '1,260p' README.md
sed -n '1,340p' Documentation.md
sed -n '1,220p' specs/00-document-map.md
sed -n '1,220p' specs/01-charter-and-decision-levels.md
sed -n '1,220p' specs/02-system-overview.md
sed -n '1,240p' specs/03-layer-model.md
sed -n '1,260p' specs/09-invariants-and-constraints.md
sed -n '1,220p' plan/00-index.md
sed -n '1,220p' progress.md
rg -n "TermSignature|signature_lanes|signature_scope|signature_evidence_roles|next queue|current next|local queue|stale reconnect|P4|P5|P6" progress.md tasks.md plan/09-helper-stack-and-responsibility-map.md plan/14-glossary-and-boundary-rules.md specs/10-open-questions.md specs/11-roadmap-and-workstreams.md docs/research_abstract/mirrorea_future_axis_01.md docs/hands_on/current_phase_closeout_01.md samples/clean-near-end/sugoroku-world/README.md docs/reports/0950-p4-term-signature-registry-hardening.md
sed -n '820,1260p' crates/mir-runtime/src/clean_near_end.rs
sed -n '2060,2165p' scripts/sugoroku_world_samples.py
sed -n '120,190p' scripts/tests/test_sugoroku_world_samples.py
sed -n '130,430p' crates/mir-runtime/tests/clean_near_end_samples.rs
sed -n '1,160p' docs/reports/0950-p4-term-signature-registry-hardening.md
python3 scripts/sugoroku_world_samples.py closeout --format json
cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json
cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json
date '+%Y-%m-%d %H:%M JST'
```

## Evidence / outputs / test results

- 2026-04-28 10:25 JST に再レビューを実施。
- helper closeout は `signature_lanes = ["kind", "name", "evidence_role"]`、`signature_scope = "representative_slice"`、`signature_kinds = ["effect", "property", "relation", "transition", "witness"]`、`reserved_signature_kinds = ["message", "adapter", "layer"]` を返した。
- runtime closeout は `signature_lanes = ["kind", "name", "evidence_role"]`、`signature_scope = "clean_near_end_canonical_inventory"`、`signature_kinds = ["effect", "property", "relation", "transition", "witness"]`、`reserved_signature_kinds = ["message", "adapter", "layer"]` を返した。
- runtime sample `05_delegated_rng_service` は `effect:rng` を `source_decl` と `effect_row_constraint` の両 provenance で保持し、`history` / `witness-field` / `proof-obligation` は `term_signatures` ではなく dedicated field 側に残っていた。
- scope 内 docs の queue wording は `P4` closed、`P5` next、`P6` reopen-next に揃っており、review scope 内で stale queue wording は見つからなかった。
- scope 内 docs / report wording は helper-local / report-local carrier を final public signature / message / adapter contract へ昇格しておらず、critical overclaim は見つからなかった。
- Critical findings: none.

## What changed in understanding

- `P4` diff の本質は wording refresh ではなく、runtime report carrier を active kind family に戻しつつ provenance 粒度を 3-lane へ修正した点にある。
- scope 内の docs 同期は、その実装差分を超えていない。closeout scope split と stop line wording は code output と整合している。

## Open questions

- low-severity residual gap として、runtime test は closeout inventory の exact kind set を押さえているが、per-sample `term_signatures` で `history` / `witness-field` / `proof-obligation` が再混入しないことを直接 negative assertion してはいない。

## Suggested next prompt

`P5` `LayerSignature` system hardening diff が出たら、同じ narrow-scope review で helper/runtime naming、`requires / provides / transforms / checks / emits / laws` の semantic drift、`VerificationLayer` overclaim、queue wording を再確認してください。
