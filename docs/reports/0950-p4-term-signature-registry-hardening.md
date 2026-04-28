# Report 0950 — P4 TermSignature registry hardening

## Objective

`P4` `TermSignature` registry hardening を close し、current `TermSignature` carrier の
active kind family、lanes、helper/runtime scope、reserved kind split、provenance rule を
repo current line に同期する。

## Scope and assumptions

- scope は `P4` の current evidence carrier hardening に限定する。
- `P5` `LayerSignature` law surface、`P6` `MessageEnvelope / Auth seam` widening、final public signature schema は対象外とする。
- helper-local / report-local evidence carrier と final public contract を混同しない。

## Documents consulted

- `sub-agent-pro/mirrorea_next_stage_full_plan_handoff_2026-04-27.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/14-glossary-and-boundary-rules.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `samples/clean-near-end/sugoroku-world/README.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/hands_on_sugoroku_sample_matrix.md`
- `docs/research_abstract/hands_on_sugoroku_detail.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `scripts/sugoroku_world_samples.py`
- `scripts/tests/test_sugoroku_world_samples.py`
- `crates/mir-runtime/src/clean_near_end.rs`
- `crates/mir-runtime/tests/clean_near_end_samples.rs`
- read-only explorer outputs from `Popper` and `Wegener`

## Files changed

- `scripts/sugoroku_world_samples.py`
- `scripts/tests/test_sugoroku_world_samples.py`
- `crates/mir-runtime/src/clean_near_end.rs`
- `crates/mir-runtime/tests/clean_near_end_samples.rs`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/14-glossary-and-boundary-rules.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/hands_on_sugoroku_sample_matrix.md`
- `docs/research_abstract/hands_on_sugoroku_detail.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `samples/clean-near-end/sugoroku-world/README.md`
- this report

## Actions taken

1. Compared Sugoroku helper `term_signatures` / `signature_kinds` with clean near-end runtime `term_signatures` / `signature_kinds`.
2. Confirmed the real drift:
   - runtime `TermSignature` had mixed in `history` / `witness-field` / `proof-obligation`
   - runtime deduped by `(kind, name)` and lost provenance
   - docs still flattened `P4` as “next” and left the stop line underspecified
3. Hardened the code surface:
   - added `signature_lanes`, `signature_scope`, `signature_evidence_roles` to Sugoroku helper closeout
   - added the same three fields to clean near-end closeout
   - tightened runtime `TermSignature` active kind family to `effect` / `transition` / `witness` / `relation` / `property`
   - moved `history` / `witness-field` / `proof-obligation` back to their dedicated report fields
   - changed runtime provenance preservation to `(kind, name, evidence_role)`
4. Strengthened tests:
   - helper closeout regression guards for lanes/scope/evidence-role inventory
   - runtime regression guards for canonical kind family and provenance preservation
5. Synced front-door docs, repository memory, snapshots, sample docs, and traceability so that:
   - `P4` is closed
   - `P5` is current promoted next
   - `P6` is next reopen point
6. Applied reviewer follow-up fixes:
   - added a regression guard that `theorem_obligations`, `witness_core_fields`, and `visible_history` remain populated after narrowing runtime `TermSignature`
   - fixed stale queue wording left in `plan/01-status-at-a-glance.md`
   - added the explicit `Files changed` section required by `AGENTS.md`

## Evidence / outputs / test results

Commands run:

```bash
date '+%Y-%m-%d %H:%M JST'
python3 -m unittest scripts.tests.test_sugoroku_world_samples -v
cargo test -p mir-runtime --test clean_near_end_samples
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug signatures --format json
python3 scripts/sugoroku_world_samples.py closeout --format json
cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json
cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
```

Results:

- `python3 -m unittest scripts.tests.test_sugoroku_world_samples -v`
  - pass (`35/35`)
- `cargo test -p mir-runtime --test clean_near_end_samples`
  - pass (`25/25`)
- reviewer follow-up:
  - added `clean_sample_delegated_rng_service_keeps_dedicated_signature_side_fields`
  - reran `cargo test -p mir-runtime --test clean_near_end_samples`
  - pass (`26/26`)
- `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug signatures --format json`
  - helper sample still emits instance/provenance-rich rows such as `sample_transition` and `runtime_witness`
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
  - `signature_lanes = ["kind", "name", "evidence_role"]`
  - `signature_scope = "representative_slice"`
  - `signature_kinds = ["effect", "property", "relation", "transition", "witness"]`
  - `signature_evidence_roles = ["derived_relation", "runtime_witness", "sample_transition", "source_decl", "validation_property"]`
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
  - runtime sample now preserves duplicate provenance rows such as `effect:rng` with both `source_decl` and `effect_row_constraint`
  - `history` / `witness-field` / `proof-obligation` remain available via dedicated fields, not as active `TermSignature` kinds
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json`
  - `signature_lanes = ["kind", "name", "evidence_role"]`
  - `signature_scope = "clean_near_end_canonical_inventory"`
  - `signature_kinds = ["effect", "property", "relation", "transition", "witness"]`
  - `signature_evidence_roles = ["derived_relation", "effect_row_constraint", "order_produced_witness", "sample_property", "source_decl"]`
  - `reserved_signature_kinds = ["message", "adapter", "layer"]`
- `python3 scripts/check_source_hierarchy.py`
  - pass (`present: 23`, `missing: 0`)
- `python3 scripts/validate_docs.py`
  - pass (`Documentation scaffold looks complete.`)
- `git diff --check`
  - pass

## What changed in understanding

- The real `P4` drift was not merely wording. Runtime `TermSignature` had become a mixed carrier for dedicated report fields.
- Closing `P4` credibly required two simultaneous fixes:
  - explicit current inventory metadata (`signature_lanes`, `signature_scope`, `signature_evidence_roles`)
  - narrowing the runtime active kind family back to the same current taxonomy as the helper
- The helper/runtime distinction that remains is now about scope and evidence role, not about conflicting active kind families.

## Open questions

- When should future full `TermSignature` shape widening happen beyond `kind` / `name` / `evidence_role`?
- In which package should reserved `message` / `adapter` / `layer` kinds stop being reserve-only?
- How should future full `TermSignature` widening relate to `P5` `LayerSignature` and `P6` `MessageEnvelope / Auth seam` without collapsing carriers?

## Suggested next prompt

`P5` `LayerSignature` system hardening を進め、helper/runtime/report-local の layer naming、`requires / provides / transforms / checks / emits / laws` 読み、`VerificationLayer` composition の current stop line を `plan/09`、`plan/14`、`progress.md`、`tasks.md`、`samples_progress.md`、relevant docs、new report に同期してください。helper-local inventory を final public layer law schema や hidden verifier builtin と混同しないこと。

## Git status

- commit: pending at report-write time
- push: pending at report-write time
