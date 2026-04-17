# Report 0716 — sample stimuli status map

- Date: 2026-04-17T01:33:27.544211Z
- Author / agent: Codex
- Scope: repo-local status mapping for prior rough syntax stimuli and their current docs / implementation anchors
- Decision levels touched: none; read-only classification over existing L2 / L3 material

## 1. Objective

Map the prior rough syntax / motivating stimuli around dice-owner handoff examples to the current repo state, with emphasis on:

- where each stimulus still appears in current docs
- whether it is a runnable-now candidate, prototype-only, or not-implemented
- what the closest current executable analogue is, if any

### Scope and assumptions

- Classification rule used in this report:
  - `runnable-now candidate` = present in `samples/current-l2/` and accepted by the current source-sample runner / tests
  - `prototype-only` = kept as docs-only motivating scenario or comparison material, without current source-sample / parser-facing support
  - `not-implemented` = exact rough token / macro has no current code or source-sample presence and is explicitly left outside the adopted companion notation
- This task is read-only except for this report.
- `plan/` 更新不要。
- `progress.md` 更新不要。
- `tasks.md` 更新不要。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `plan/06-surface-notation-status.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/08-representative-programs-and-fixtures.md`
- `samples/current-l2/README.md`
- `specs/examples/01-current-l2-surface-syntax-candidates.md`
- `specs/examples/409-current-l2-order-handoff-syntax-stimuli-comparison.md`
- `specs/examples/411-current-l2-order-handoff-adequacy-corpus-and-verification-boundary-matrix.md`
- `specs/examples/442-current-l2-order-handoff-source-surface-wording-reserve-note.md`
- `faq_005.md`
- `docs/reports/0708-faq005-literature-mapping-and-theory-lab-followup.md`
- `crates/mir-runtime/src/current_l2.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_runner.rs`
- `samples/current-l2/e1-place-atomic-cut.txt`
- `samples/current-l2/e21-try-atomic-cut-frontier.txt`
- `samples/current-l2/e22-try-atomic-cut-place-mismatch.txt`

## 3. Actions taken

1. Read the repo-required entry documents and current status snapshot before narrowing to the stimulus family.
2. Searched the repository for exact tokens: `dice_owner`, `roll_dice`, `release(context)`, `atomic`, `update_global_state`.
3. Cross-checked the rough syntax package (`specs/examples/409`) against the later wording reserve note (`specs/examples/442`) and FAQ explanation (`faq_005.md`).
4. Verified the current accepted source-sample surface and runtime-backed `atomic_cut` examples in `samples/current-l2/` and `crates/mir-runtime`.
5. Ran fresh `mir-runtime` source-sample runner tests to confirm the current executable subset.

## 4. Files changed

- `docs/reports/0716-sample-stimuli-status-map.md`
- `plan/` 更新不要
- `progress.md` 更新不要
- `tasks.md` 更新不要

## 5. Commands run and exact outputs

- `date '+%Y-%m-%d %H:%M:%S %Z'`
  - `2026-04-17 10:30:19 JST`
- `rg -n --hidden -S 'dice-owner|roll_dice|release\(context\)|update_global_state|atomic\b|dice owner|release context|roll dice|update global state' .`
  - key hits: `specs/examples/409-current-l2-order-handoff-syntax-stimuli-comparison.md`, `specs/examples/442-current-l2-order-handoff-source-surface-wording-reserve-note.md`, `faq_005.md`, plus `atomic_cut` implementation / sample files
- `rg -n -S 'release\(context\)|roll_dice|update_global_state|dice_owner' crates samples`
  - no matches, exit `1`
- `rg -n -S 'atomic\[' crates samples`
  - no matches, exit `1`
- `rg -n -S '^\s*atomic_cut$' samples/current-l2 crates/mir-runtime/src/current_l2.rs crates/mir-runtime/tests/current_l2_source_sample_runner.rs`
  - hits:
    - `samples/current-l2/e1-place-atomic-cut.txt:8`
    - `samples/current-l2/e21-try-atomic-cut-frontier.txt:8`
    - `samples/current-l2/e22-try-atomic-cut-place-mismatch.txt:9`
- `cargo test -p mir-runtime --test current_l2_source_sample_runner`
  - `running 16 tests`
  - `... current_l2_source_sample_runner_accepts_named_e1_sample ... ok`
  - `... current_l2_source_sample_runner_accepts_named_e21_sample ... ok`
  - `... current_l2_source_sample_runner_accepts_named_e22_sample ... ok`
  - `test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`

## 6. Evidence / findings

| stimulus | current docs that discuss it | status | rationale |
|---|---|---|---|
| `dice_owner` / dice-owner handoff | `specs/examples/409` stimulus B/C, `specs/examples/411` adequacy corpus id 1, `specs/examples/442`, `faq_005.md` section 10 | `prototype-only` | current repo keeps it as a motivating authoritative-room scenario and wording example, but `rg` found no `dice_owner` hits under `crates/` or `samples/`; it is not in the accepted source-sample surface |
| `roll_dice(...)` | `specs/examples/409` stimulus A/B, `faq_005.md` section 10 prose | `prototype-only` | same status as `dice_owner`: current explanation / comparison material only, with no source-sample or runtime implementation anchor |
| `release(context)[ ... ]` | `specs/examples/409` stimulus A; indirectly replaced by relation-family explanation in `specs/examples/442` and `faq_005.md` | `not-implemented` | exact token appears only in the non-normative rough-syntax comparison package; current wording reserve explicitly keeps final handoff syntax OPEN and uses relation-family vocabulary instead |
| exact rough `atomic[...] { ... }` / `atomic { ... }` | `specs/examples/409` stimuli B/C, `specs/examples/442`, `faq_005.md` | `prototype-only` | the exact rough `atomic` surface is not adopted and has no code/sample hits, but it has a live executable analogue in `atomic_cut` |
| `atomic_cut` current analogue | `plan/06`, `specs/examples/01`, `samples/current-l2/README.md`, `samples/current-l2/e1/e21/e22`, `crates/mir-runtime/src/current_l2.rs`, `crates/mir-runtime/tests/current_l2_source_sample_runner.rs` | `runnable-now candidate` | current companion notation includes `atomic_cut`; the runner accepts `e1-place-atomic-cut`, `e21-try-atomic-cut-frontier`, and `e22-try-atomic-cut-place-mismatch`; fresh test run passed |
| `update_global_state { ... }` | `specs/examples/409` stimulus D, `specs/examples/442` wording comparison, `faq_005.md` rough-syntax note | `not-implemented` | current docs explicitly treat it as room-macro / transaction-like comparison material and drop macro adoption from the current package; no code/sample hits exist |

### supporting notes

- `specs/examples/409-current-l2-order-handoff-syntax-stimuli-comparison.md` explicitly labels A-D as `non-normative stimuli` and says they are `not adopted surface forms`.
- `specs/examples/442-current-l2-order-handoff-source-surface-wording-reserve-note.md` keeps rough A-D as `comparison material`, rejects fence-like public wording and room-macro adoption for the current package, and says final source-surface handoff syntax is still later.
- `faq_005.md` converts the dice-owner example into a docs-first explanation table based on relation families (`publication_order`, `observation_order`, `witness_order`, `finalization_order`) and explicitly says the old `roll_dice` / `dice_owner` sketches should be explained as intended structure, not as writable current syntax.
- The only source-backed executable nucleus in this family today is `atomic_cut`, not `release(context)`, `atomic{}`, or `update_global_state{}`.

## 7. Changes in understanding

- The repo did not discard the dice-owner scenario; it preserved it as theory-lab adequacy corpus and FAQ explanation material.
- The repo did discard the old rough handoff surface as current syntax. `release(context)`, `atomic{}`, and `update_global_state{}` survive only as comparison stimuli, not as parser- or runner-facing forms.
- The live implementation boundary is narrower than the rough sketches suggest: current execution reaches `atomic_cut`, not a room-level handoff DSL.

## 8. Open questions

- If the user wants “prior” to include pre-report history, the next step would be a git-history sweep for the first appearance of stimuli A-D before `specs/examples/409`.
- If the user wants a closest-current executable analogue for the full dice-owner scenario, that would require a second pass across shared-space / authoritative-room docs; this report only confirms that no such source sample is implemented today.

## 9. Suggested next prompt

Trace rough syntax stimuli A-D backward to the first report / commit that introduced them, then show how each one was translated into the current relation-family wording (`publication_order`, `witness_order`, `authority_serial_transition_family`, `atomic_cut`).
