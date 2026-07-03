# plan/80 - G1 diagnostic carrier inventory

## Purpose

This file inventories the gap between the canon Diagnostic carrier and current
LAB diagnostic evidence, especially for E-ROW-shaped Surface elaboration
diagnostics.

This is LAB repository memory. It does not implement a final diagnostic ABI,
does not state or prove OBL-024/025, does not claim explanation soundness or
completeness, does not claim conformance, and does not edit canon.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- Canon diagnostic theory:
  `mirrorea_canon/theory/10-diagnostics.md`
- Canon diagnostic format:
  `mirrorea_canon/spec/07-diagnostics-format.md`
- Canon proof-status ledger:
  `mirrorea_canon/theory/11-metatheory-ledger.md`
- LAB E-ROW alignment:
  `plan/79-g1-erow-diagnostic-alignment.md`
- LAB Rust carrier:
  `crates/mir-ast/src/textual_alpha.rs`
- LAB Surface elaboration carrier:
  `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- LAB helper projection:
  `scripts/surface_mir_samples.py`

If this LAB inventory conflicts with canon, canon wins.

## Canon carrier fields

Canon `theory/10` and `spec/07` require a Diagnostic carrier with at least:

| Canon field | Required reading |
|---|---|
| `id` | spec/07 error id such as `E-ROW-001` |
| `severity` | `error` or `warn` |
| `span` | source span with file/range/line-column shape; every error has at least one span |
| `rule_instance` | named judgment rule whose premise failed |
| `failed_premise` | the failed premise with bindings |
| `missing_evidence[]` | declared item(s) or evidence whose absence caused rejection |
| `suggested_repair[]` | machine-readable edit suggestions when available |
| `refs[]` | ADR / THM / CON / spec links for human explanation |

Canon `theory/10` also fixes the blame direction: the diagnostic blames a
named failed premise, and underdeclared cases blame the missing declaration
site rather than only the use site.

## Current LAB carrier inventory

| Current layer | Present fields / evidence | Missing relative to canon |
|---|---|---|
| Rust `TextualMirDiagnostic` | `code`, `message`, `span`; `SourceSpan` has `start`, `end`, `line`, `column` | canon `id`, `severity`, `rule_instance`, `failed_premise`, `missing_evidence`, `suggested_repair`, `refs`; file-bearing / byte-range / line-column span shape is not final |
| Surface elaboration report | `diagnostics: Vec<TextualMirDiagnostic>`, `accepted`, generated Core IR, source span rows | same diagnostic carrier gaps; no final rule/premise replay evidence |
| Surface remote request evidence | `required_failures`, `declared_failures`, `failure_row_complete`, `generated_from`, `source_span` | useful missing-evidence inputs, but not emitted as a Diagnostic carrier |
| Surface helper raw JSON | full `diagnostics` and `core_ir` from the elaboration example | raw shape is helper output, not final diagnostic ABI |
| Surface helper projection | `diagnostic_codes`, `remote_request_summaries.failure_row_complete`, source-span entity kinds, obligation codes | helper projection omits full diagnostic object and cannot prove explanation soundness |
| Expected JSON rows | expected `diagnostic_codes`, incomplete failure-row summaries, source-span entity kinds, `final_public_api_frozen: false` | no canon ID field, no repair rows, no refs, no multi-span declaration/use split |
| Plan/report evidence | source hierarchy, canon mapping, overclaim guards | not executable diagnostic ABI |

## E-ROW-specific gap

Current E-ROW-shaped LAB evidence uses helper-local code:

```text
generated_failure_not_declared
```

Against canon requirements, the current gap is:

| Canon expectation | Current LAB status | Gap |
|---|---|---|
| E-ROW-001 / E-ROW-002 ID | helper-local string only | needs canon-shaped id or parallel canon-id field |
| failed rule | implicit row-containment check in code | needs rule-instance field such as elaboration row containment premise |
| failed premise | implicit `generated ⊆ declared fails` false condition | needs machine-readable failed-premise payload |
| missing evidence | inferable from incomplete row and generated failure set | not emitted as `missing_evidence[]` |
| suggested repair | human-inferable add-to-fails-row repair | not emitted as `suggested_repair[]` |
| refs | canon refs known in docs | not emitted as diagnostic `refs[]` |
| span | `TextualMirDiagnostic.span` exists; generated source-span sidecars exist | final file/range/multi-span shape and declaration-site span are not fixed |

## Safe next implementation shape

If a later package changes code, prefer an additive carrier shape rather than a
breaking replacement:

| Field | Candidate role |
|---|---|
| `legacy_code` | preserve current helper code while tests migrate |
| `canon_id` | `E-ROW-001`, `E-ROW-002`, etc. |
| `severity` | `error` for current rejection rows |
| `primary_span` | current span |
| `related_spans` | declaration/use-site split when available |
| `rule_instance` | named elaboration rule / premise family |
| `failed_premise` | structured premise id plus bindings |
| `missing_evidence` | missing failure families / declaration items |
| `suggested_repair` | repair family plus target declaration |
| `refs` | canon spec/theory IDs |

This is only a candidate shape. It is not a final ABI and should not be treated
as a public contract until explicitly promoted.

## OBL-024 / OBL-025 prerequisite reading

| Obligation | What this inventory provides | What remains missing |
|---|---|---|
| OBL-024 explanation soundness | lists the fields needed to connect diagnostics to actual failed premises | no replay relation, no proof, no statement draft |
| OBL-025 explanation completeness | identifies `suggested_repair[]` as required for single-edit repairs | no repair generation, no ranking, no proof, no statement draft |

## Open questions

- Should future helper JSON expose both `legacy_code` and `canon_id` during
  migration?
- Which rule-instance vocabulary should be used for Surface elaboration
  failures before final theorem statements exist?
- How should declaration-site and use-site spans be represented without
  freezing final JSON shape too early?
- Should E-ROW repair rows be added before broader diagnostic families are
  inventoried?

## Next safe packages

1. Additive LAB diagnostic carrier prototype for E-ROW only, with Rust/Python
   tests and legacy code preserved, if executable evidence is desired next.
2. OBL-024 statement-shape inventory, still without a Lean file or proof.
3. Broader diagnostic-family inventory for E-AUTH / E-IDX / E-PATCH once E-ROW
   carrier direction is understood.

## Non-claims

- No canon edit.
- No final diagnostic ABI.
- No final diagnostic message wording.
- No localization decision.
- No OBL-024 statement.
- No OBL-024 proof.
- No OBL-025 statement.
- No OBL-025 proof.
- No explanation soundness claim.
- No explanation completeness claim.
- No C-static, C-runtime, or C-distributed conformance claim.
- No G0 exit.
- No G1 exit.
- No T1 transition.
- No T2 transition.
- No runtime behavior claim.
