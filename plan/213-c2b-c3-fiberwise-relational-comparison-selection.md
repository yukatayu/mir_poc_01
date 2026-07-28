# Plan 213 - C2-B/C3 fiberwise relational comparison selection

## Role and authority

This is a **LAB corrective candidate-selection record**. `mirrorea_canon/` remains normative. It does not rewrite WRK-0038's pre-registration or use it as evidence. It selects only a narrower successor candidate after a pre-execution scope review found that a bare `DirectView` does not retain its `(Frontier, Request)` key.

This record selects no Family A/B carrier, occurrence equality, authority, freshness, persistence, recovery, source/elaboration rule, implementation, OBL, SCN, Gate, Phase, conformance, API, or public behavior. A fiber receives its key explicitly; it never recovers a key from a view or incidental data.

## Authority cut and review finding

The corrective selection cut is `78dde80d6cb42acac7c6d80a680beec9edcd7ee1`.

| Input | SHA-256 | Relevance |
| --- | --- | --- |
| ADR-0014 | `b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323` | requires a successor rather than rewriting a registered scope |
| P012 / P013 | `09ea4d6957c320b4d0647806714a1643101c2022b2893ac76ec7de3bf1db73d5` / `4e0ecf7475f20eec85c09d50201d2d2cc29848d480e8382935fe489b43877213` | value-flow and validation directions leave carrier/identity open |
| theory/01 / theory/04 / theory/05 | `35e2f52bebcf96332d8102e9110446930b7ff807d948737a5859909de34b0f12` / `70bde483330d3745a8694b15cd75f447b6610513ae66cb1ad4ec1faed274a264` / `e06dc5ef0539eb5b87bce71b34d3e8d2ab0638603642e0d9f89581f29d25e6c4` | current read-only occurrence, cut, and authority boundaries |
| Plan 212 | `4485ec34a3c170dd2a249b18748d5fed35eed2852f68d5ba991df9a58ff9c5aa` | original finite presentation-comparison selection |
| WRK-0037 artifact | `839ffda0e4c01fb1dab476598b97f658a8f85e27d8ce2547ab6a8c49e8662739` | fixed two-atom table whose keys and transitions must remain unchanged |
| WRK-0038 | `ddcabc21d3be50c43ac651d5ce8cbdd4311d87f00c17da16ecd8d1492228d88c` | unexecuted scope that used an undefined reachable-state/global inverse phrase |

A temporary GPT-5.6 Sol Pro review, response SHA-256 `1e721ec619fce903755949b2d01d28cfc11880019b7a1f2858ff58f2371458fc`, found two material scope gaps: bare `DirectView` values collide across supplied keys, and WRK-0037 defines no reachability closure. Local `lean --trust=0` checks confirmed `directView .awaiting .q0 = directView .awaiting .q1` and `directView .consumed .q0 = directView .failed .q0`. These are review findings, not a Canon semantic result.

## Corrective disposition

WRK-0038 is **not executed**. Its protected pre-registration remains intact and has no evidence artifact or evidence commit. The global/bare-view interpretation is not repaired in place. A successor must compare all ten supplied finite `(Frontier, Request)` cells fiberwise, or return to `no-candidate`.

## Selected successor candidate

`AB-FIBER-REL-PRE` asks whether an independently enumerated relation-first presentation is fiberwise isomorphic to the bundled lookup at **each of all ten registered cells**. Its translation domains are indexed by a supplied frontier and request. It makes no `DirectView -> Request`, incidental-data-to-request, or source-elaboration reconstruction claim.

The relation-first presentation must use the existing finite types and enumerate five direct graph relations without mentioning `DirectView`, any `...At` lookup, receipt/resume lookup, `restore`, or `loadedView` in their definitions: (1) `CellR` has one explicit row for each of ten cells and all view columns; (2) `IncidentalR` has both equal-incidental rows; (3) `ReceiptResultR` has all twenty `Option Frontier` outcomes; (4) `ResumeResultR` has all ten `Option Frontier` outcomes; and (5) `RestoreR` has both swap rows.

The combined receipt-then-resume behavior is derived only from rows 3 and 4. No sixth adjustable transition table, row/profile tag, nominal attempt, missing-proof-as-rejection convention, phase-derived state shortcut, copied `DirectView`, or duplicate isomorphic view structure is permitted.

For supplied `f` and `q`, the successor may use a bundled fiber containing a view equal to `directView f q` and a relation fiber containing explicit columns plus a `CellR f q` proof. It must prove total maps in both directions, both pointwise round trips, exact observation preservation, exact transition graphs including all `none` cases, derived combined behavior, and local restore commutation for every supplied cell. A Lean proof of this finite proposition is not a Canon theorem or general semantic equivalence.

## Falsifiers, stop lines, and non-effects

The successor fails if an independent relation graph cannot be enumerated; any row is missing, extra, nonfunctional, or derives from a bundled lookup; a map or round trip fails; any listed observation, accepted/rejected receipt/resume outcome, derived combined result, or local restore lookup differs; any key is recovered from a bare view or incidental record; or a reserved semantic premise is needed. Check cells, incidental rows, receipt rows, resume rows, derived combined rows, restore rows, then translations in that order. Syntax, extraction, or toolchain failure is inconclusive infrastructure evidence, not a semantic falsifier.

On a typed finite falsifier, freeze reliance on the successor and retain it forward-only. If the graph is a repackaging, close as duplicate; do not enlarge the table. No result affects carrier selection, identity/equality, authority, persistence/recovery, source ergonomics, implementation, ledger status, or public completion. Later ergonomics remain separate and require uniquely reconstructible facts with inspectable grounds, never incidental context.

## Successor workflow

The next package may create `WRK-0039` as a new L3 record only. It must pin this cut; state the all-ten-cell fiber domain; declare the five graph relations, enumeration and isolation checks, exact baseline comparison, and theorem list; and permit only `plan/` and `docs/reports/` evidence. It must keep WRK-0038's pre-registration intact, record that it is unexecuted, and run no source before the successor registration is committed and pushed.
