# plan/119 - G0 remaining claim-family drilldown priority map

## Purpose

This file is LAB repository memory.

It classifies the remaining `plan/70` LAB-to-canon claim-family rows after the
ordinary Surface assignment row was drilled down in `plan/118`. Its purpose is
to keep future autonomous work from treating every `plan/70` row as an immediate
drilldown target.

This file does not edit canon, does not close G0, does not close any G1..G7
gate, does not move proof-obligation status, and does not promote a new
implementation package.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- LAB claim-family ledger: `plan/70-lab-to-canon-reconciliation-ledger.md`
- LAB ordinary-assignment drilldown:
  `plan/118-g0-g1-ordinary-assignment-claim-family-drilldown.md`
- Snapshot task map: `tasks.md`
- Snapshot progress map: `progress.md`

If this priority map conflicts with canon, canon wins. If it conflicts with
`plan/70`, treat `plan/70` as the broader ledger and this file as the narrower
post-`plan/118` scheduling note.

## Classification vocabulary

| Priority | Meaning |
|---|---|
| `closed-for-now` | Canon already carries the authoritative point, or LAB evidence is only historical. Do not drill down unless a touched document has stale wording. |
| `support-only` | Useful as supporting evidence for another gate, but not a standalone drilldown now. |
| `later-gate` | Belongs to a later G2..G7 gate. Do not open before the prerequisite gate context is stable unless a human asks. |
| `evidence-only` | Runnable/helper/proof-stub evidence. Cite exact commands/results only; do not turn into canon or implementation status. |
| `process-only` | Operating practice or advisory input. Keep it out of semantic gate status. |
| `open-on-demand` | May deserve a future drilldown when a G0 close decision or human review needs exact citations. |

## Remaining row priority table

| `plan/70` row family | Gate / layer reading | Priority now | Reason |
|---|---|---|---|
| Source hierarchy and governance | G0 / process | `closed-for-now` | Canon source hierarchy and ADR-0012 already carry the normative rule. Drill down only for stale wording audits. |
| Project axis and non-axis vocabulary | G0 / vocabulary | `closed-for-now` | Canon vocabulary already rejects old core-primitive readings. Use focused wording cleanup only if touched docs drift. |
| Surface/Core/Trace/Verification/Projection/Domain separation | G0 / architecture | `closed-for-now` | Canon strata and boundary contracts already carry the rule. Drilldown is unnecessary unless a doc collapses systems. |
| Indexed state, role admission, capability grants, witnesses, stale rejection | G3 authority/admission | `later-gate` | Important, but G3 exit depends on authority/admission theory after G1 and G2 context is stable. |
| Reads, writes, occurrences, dependencies, graph families | G1 support / G4 later | `support-only` | The ordinary-assignment slice is already covered by `plan/118`; broader occurrence / observation semantics should wait for their own gate pressure. |
| Lifetime, lease, freshness, fallback, load/rollback, stale non-resurrection | G2 existence/fallback | `later-gate` | This is the first later semantic gate after G1, but opening it now would mix ordinary assignment with existence/fallback theory. |
| Cut/save-load, activation cut, durable state, stale non-resurrection | G5 cut/save | `later-gate` | Depends on earlier G1/G2/G3 context and should not be inferred from alpha save/load evidence. |
| Authentication, authorization, membership, capability, witness, freshness, admission | G3 authority/admission | `later-gate` | Canonized principle remains important, but details are open and should not be collapsed into transport or role labels. |
| Observation, debug, telemetry, redaction, retention, source-span diagnostics | G4 effects/observation | `later-gate` | Current G1 E-ROW work uses diagnostic evidence, but final observation / telemetry ABI belongs later. |
| Projection, backend, provider adapter, FFI/packet/schema inventory | G6 projection/backend | `later-gate` | Meaning-preservation and backend boundary work should wait until preservation targets are crisp. |
| Source patch pipeline, compatibility, admission, activation cut | G7 hot-plug | `later-gate` | Hot-plug is a capstone correctness operation, not the next theory target. |
| Product Alpha, operational product suite, Full System V1, Surface P-SURF, sample dashboard, helper reports | implementation / sample evidence | `evidence-only` | Cite exact command and report results only. Never read as canon implementation-state completion. |
| Lean proof stubs and statement drafts | proof evidence | `evidence-only` | Current statement drafts and guards are LAB compile-check evidence; proof / ledger status lives only in canon `theory/11`. |
| Reversed Library, PrismCascade, Typed-Effect Wiring Platform, upper app docs | separability / upper systems | `closed-for-now` | Canon separability is already fixed. Product details remain open and should not be folded into Mir core work. |
| Oracle / sub-agent consultation outputs | process | `process-only` | Advisory results must be mirrored into repo memory before use and judged against canon evidence. |

## Immediate scheduling result

No remaining `plan/70` row should be drilled down immediately by default.

The ordinary-assignment row was the correct first drilldown because it is the
current G1 pressure case and already had a bounded LAB chain in `plan/71..78`
and `plan/117`. The remaining rows are either:

- already carried by canon and only need stale wording cleanup when touched;
- supporting evidence for G1 rather than a separate target;
- later-gate material that should wait for prerequisite gate context;
- evidence-only rows that should be cited as results, not status; or
- process-only advisory practice.

## Safe next choices

| Choice | When it is safe | Non-claim |
|---|---|---|
| Focused stale wording audit | A touched LAB doc risks re-promoting legacy `specs/`, helper closeout, or old vocabulary to canon. | No broad historical rewrite. |
| Narrow G1 read/write/dependency support drilldown | A concrete G1 ordinary-assignment vocabulary gap remains after `plan/118`, and the package stays limited to read/write/dependency support for assignment consequences. | No observation semantics, event-model restatement, runtime graph machinery, G4 movement, or standalone gate target. |
| OBL-001/020/021 refinement | Review finds a concrete missing predicate, overfit, or guard weakness. | Lean remains compile-check-only unless canon ledger changes through process. |
| E-ROW / OBL-024 / OBL-025 continuation | The package stays within diagnostic/explanation evidence and does not widen executable behavior without a documented gate. | No diagnostic/repair ABI freeze, proof discharge, or G1 exit. |
| G2 fallback preflight | G1 ordinary-assignment statement boundaries are stable enough to avoid mixing assignment and existence/fallback theory. | No fallback completion or lifetime proof. |
| Canon proposal | A real canon wording gap is found and human review is expected. | Proposal only; no direct canon edit without decision. |

## Focused stale wording audit follow-up

2026-07-04 audit follow-up: a narrow source-hierarchy wording pass corrected
legacy LAB memory in `plan/01`, `plan/07`, `plan/09`, `plan/19`, and
`plan/57`; root reading order in `README.md`; compressed snapshot wording in
`Documentation.md` and `samples_progress.md`; the read/write/dependency
follow-up in `plan/70`; and the `plan/119` provenance row in
`plan/90-source-traceability.md`. The corrected reading is now explicit:
`mirrorea_canon/` is canon, legacy `specs/` are LAB evidence / historical
package-line memory, Surface alpha rows are evidence-closed rather than final
runtime / transport status, and code / helper artifacts are not semantic source
of truth.

This follow-up does not edit canon, does not rewrite historical LAB evidence
wholesale, and does not change any gate, proof, conformance, implementation, or
sample status.

2026-07-04 validator follow-up: `scripts/validate_docs.py` now includes a
source-hierarchy wording lint that checks `CANON.md`, root/snapshot docs,
`samples/README.md`, `.docs/`, `docs/hands_on/`, `docs/research_abstract/`,
and `plan/` for stale `specs/`-as-normative wording. It intentionally excludes
historical reports, legacy `specs/`, and archived research material. The first
lint-backed pass also corrected reader-facing LAB wording in
`docs/hands_on/README.md`, selected `docs/research_abstract/*.md`, `plan/19`,
`plan/50`, and `plan/58`.

This validator follow-up is a guardrail only. It does not edit canon, change
gate status, move proof obligations, claim conformance, or change runnable
sample status.

## Required non-claims

- No canon edit.
- No canon L0/L1 decision.
- No G0 exit.
- No G1..G7 exit.
- No T0 -> T1 transition.
- No OBL status movement.
- No proof discharge.
- No C-static, C-runtime, or C-distributed conformance claim.
- No implementation-state completion.
- No final grammar/API/Core IR/diagnostic/repair/runtime/transport/projection
  ABI freeze.
- No promotion of helper/sample/report/Lean compile-check evidence to canon.

## Close condition

This file is closed when `plan/70`, `plan/00-index.md`,
`plan/90-source-traceability.md`, snapshot docs, report, and validators are
synchronized.

Close condition is priority-map-only: no canon edit, no gate exit, no proof, no
implementation change, and no runnable sample status change.
