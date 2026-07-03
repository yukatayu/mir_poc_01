# Report 2132 - G1 E-ROW Repair Payload Inventory

- Date: 2026-07-03 22:22 JST
- Author / agent: Codex
- Scope: LAB-only E-ROW repair payload inventory before code prototype
- Decision levels touched: L1/L2 canon consulted, no canon edit; LAB repository memory updated

## Objective

Inventory a non-final E-ROW repair payload vocabulary for a later diagnostic
detail prototype, without implementing repairs, changing helper output,
freezing a Diagnostic or repair ABI, proving OBL-024/025, claiming explanation
soundness/completeness, claiming conformance, claiming G1 exit, or editing
canon.

## Scope and assumptions

The normative source is `mirrorea_canon/`. This task edits only LAB repository
memory, snapshot docs, validator path lists, and this report.

Working assumption: if the next E-ROW implementation prototype includes
`suggested_repair[]`, it should have a pre-recorded LAB vocabulary for target
row, missing failure, local premise, and non-runtime-success guard. If that is
too risky, a carrier-only prototype should not claim OBL-025 progress.

## Start state / dirty state

The task started from clean `main...origin/main` at commit `a3f87b47 Add G1
OBL025 statement-shape inventory`. A new Discord baseline was recorded with
`discord_notify.py begin --cwd .`.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `AGENTS.md`
- `mirrorea_canon/theory/10-diagnostics.md`
- `mirrorea_canon/spec/07-diagnostics-format.md`
- `plan/79-g1-erow-diagnostic-alignment.md`
- `plan/80-g1-diagnostic-carrier-inventory.md`
- `plan/81-g1-obl024-statement-shape-inventory.md`
- `plan/82-g1-obl025-statement-shape-inventory.md`
- `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- `samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/elab-10-visibility-failure-row-negative/expected/elaboration.json`

## Actions taken

- Added `plan/83-g1-erow-repair-payload-inventory.md`.
- Recorded candidate non-final E-ROW repair payload roles:
  `repair_family`, `diagnostic_family`, `target_kind`, `target_span`,
  `use_span`, `missing_failure`, `required_failures`, `declared_failures`,
  `single_edit_assumption`, `local_premise`, and `non_goal`.
- Recorded E-ROW-001, E-ROW-002, and mixed E-ROW boundaries.
- Recorded current LAB evidence and the fact that helper expected JSON does not
  yet expose repair payloads.
- Integrated read-only sub-agent review, especially the current evidence
  boundary, candidate `applies_to` / target / local-effect roles, and the
  recommendation to implement carrier-only E-ROW diagnostic detail first.
- Updated plan index, source traceability, snapshot docs, and validator path
  lists.
- Started one read-only sub-agent semantic review in parallel.

## Files changed

- `Documentation.md`
- `plan/00-index.md`
- `plan/83-g1-erow-repair-payload-inventory.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `scripts/README.md`
- `scripts/check_source_hierarchy.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/validate_docs.py`
- `tasks.md`
- `docs/reports/2132-g1-erow-repair-payload-inventory.md`

No code, sample, Lean, manifest, `samples_progress.md`, or `mirrorea_canon/`
file was edited.

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
oracle status --hours 2 --limit 10
sed -n '1,240p' plan/82-g1-obl025-statement-shape-inventory.md
sed -n '1,230p' plan/81-g1-obl024-statement-shape-inventory.md
sed -n '1,190p' plan/79-g1-erow-diagnostic-alignment.md
sed -n '1,180p' plan/80-g1-diagnostic-carrier-inventory.md
sed -n '560,670p' crates/mir-semantics/src/surface_to_core_elaboration.rs
jq '{accepted, diagnostic_codes, remote_request_summaries, source_span_entity_kinds, obligation_codes, final_public_api_frozen}' samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/expected/elaboration.json
jq '{accepted, diagnostic_codes, remote_request_summaries, source_span_entity_kinds, obligation_codes, final_public_api_frozen}' samples/full-system-v1-surface/elaboration/elab-10-visibility-failure-row-negative/expected/elaboration.json
date '+%Y-%m-%d %H:%M %Z'
```

Post-report validation commands are listed in the evidence section.

## Evidence / outputs / test results

The package is docs-only. No Rust, Python helper, Lean, sample matrix, expected
JSON, or manifest behavior changed.

Post-report validation:

```bash
python3 scripts/check_source_hierarchy.py
# required: 579
# present: 579
# missing: 0

python3 scripts/validate_docs.py
# Documentation scaffold looks complete.
# Found 1284 numbered report(s).

python3 -m unittest scripts.tests.test_validate_docs
# Ran 20 tests in 0.342s
# OK

git diff --check
# exit 0
```

## What changed in understanding

Current evidence supports a repair payload inventory, but not a repair
guarantee. The next implementation should be carrier-only first: add non-final
canon ID / severity / rule / premise / missing-evidence detail while preserving
legacy output, then add repair rows only after required/declared/missing failure
projection and placeholder-rejection tests exist.

## Open questions

- Should the first implementation include repair rows, or stay carrier-only?
- How should target declaration span be recovered when the current evidence
  only carries request/source spans and source-span sidecars?
- Is adding multiple missing failures to one `fails` row one edit or multiple
  edits?
- Should E-ROW-002 initially prefer add `VisibilityDenied` to `fails` over
  visibility / observe-authority alternatives?

## Suggested next prompt

自走で E-ROW carrier-only diagnostic detail prototype を実装してください。
legacy helper output を保ち、new detail は LAB/non-final として扱い、
`suggested_repair[]` はまだ入れないでください。final ABI、OBL-024/025 proof、
OBL-025 advancement、conformance、G1 exit は主張しないでください。

## Plan update status

更新済み:

- `plan/00-index.md`
- `plan/83-g1-erow-repair-payload-inventory.md`
- `plan/90-source-traceability.md`

## Documentation.md update status

更新済み: Surface Mir line now mentions E-ROW repair payload inventory as LAB
repository memory, not diagnostic/repair ABI freeze, OBL-024/025 discharge,
proof discharge, or G1 exit.

## progress.md update status

更新済み: Added the current E-ROW repair payload inventory note and recent log
entry.

## tasks.md update status

更新済み: Added E-ROW repair payload inventory to current holding state and
promoted carrier-only E-ROW diagnostic detail prototype as the safer next
implementation candidate.

## samples_progress.md update status

`samples_progress.md` 更新不要: no runnable sample, validation command, debug
surface, Lean artifact, manifest entry, expected JSON, or sample workflow
status changed in this package.

## Reviewer findings and follow-up

Read-only sub-agent review found:

- current Rust evidence computes `required_failures`, `declared_failures`, and
  `failure_row_complete`, then emits only legacy
  `generated_failure_not_declared`;
- ELAB-07/10 expected JSON projects only diagnostic code and incomplete-row
  summary, not missing failures or repairs;
- a minimal non-final repair payload should include `repair_family`,
  `applies_to` with legacy code / candidate canon ID / request id, target row
  context, required failures, declared failures before repair, missing
  failures, local effect, and local row-containment scope;
- final diagnostic/repair ABI, edit-script syntax, declaration-site span
  policy, single-edit atomicity, E-ROW-002 alternatives, rule/premise/bindings,
  replay, OBL-024/025 proofs, ordering/equality, conformance, and G1 exit all
  remain OPEN or LAB-only;
- the next implementation should be carrier-only first, adding non-final
  `canon_id`, `severity`, `rule_instance`, `failed_premise`,
  `missing_evidence`, and refs while preserving legacy output;
- repair-bearing should be a later follow-up once projections expose
  required/declared/missing failures and tests can reject placeholder repair
  rows.

These findings were integrated into `plan/83`, `progress.md`, and `tasks.md`.

## Skipped validations and reasons

- Cargo tests / build / clippy: skipped because no Rust code changed.
- Surface helper execution: skipped because no sample matrix, helper behavior,
  or expected output changed.
- Lean validation: skipped because no Lean files or Lean manifest changed.
- Runtime / release checks: skipped because this package makes no runtime,
  product, conformance, or release claim.
- Canon validators: skipped because `mirrorea_canon/` was not edited.
- Oracle advisory review: skipped because recent OBL-024 and OBL-025 Oracle
  reviews already covered the relation boundaries needed for this payload
  inventory; a read-only sub-agent review was sufficient for this narrower
  package.

## Commit / push status

Pending at report update. This package should be committed with
`git commit --no-gpg-sign` and pushed after final diff review.

## Sub-agent session close status

Read-only reviewer sub-agent `019f2824-dfd4-71a2-9219-02a3b20c7a62` was closed
after its findings were integrated.
