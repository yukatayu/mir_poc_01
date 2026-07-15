# plan/155 - T0/G0 governance-profile proposal routing

## status

LAB repository-memory and execution-routing note. The canon-resident draft proposal is
`mirrorea_canon/meta/proposals/PROPOSAL-002-t0-g0-governance-profile.md`.
This note does not adopt that proposal, alter canon, emit a profile result, or
move G0/T0.

## purpose

P111 records the owner-authorized, bounded successor to P109: prepare a
canon-reviewable response to G0-EXIT-001 without treating documentation checks
as implementation conformance. The target is a proposal only; no new helper or
runtime is introduced during T0.

## owner inputs and exact boundary

The owner confirmed the core/domain, domain-handler, grant-only authority, and
`.mir` source-authority directions. The owner selected the T0-specific
governance-profile route and waived a separate semantic/historical
LAB-demotion audit at this checkpoint. `PROPOSAL-002` records these inputs.

The following are deliberately still open:

- G0-D1: collective acceptance or deferral of the five named ADRs, GLOSSARY
  baseline, and current LAB-demotion evidence as the G0 substantive criteria;
- adoption, rejection, or revision of the exact canonical profile definition;
- G0-D3: human approval or deferral of G0 exit and identification of its
  effective canonical ADR/ledger record.

Neither an owner preference for profile route nor a future profile `pass`
result approves a Gate exit.

## P111 package plan

| Step | Output | Completion condition |
| --- | --- | --- |
| 1 | `PROPOSAL-002` | defines the boundary, inputs, non-claims, alternatives, and requested owner decisions |
| 2 | LAB state synchronization | P109 and the human-facing control surfaces distinguish recorded inputs from effective canon decisions |
| 3 | review and validation | canon index, documentation/source-hierarchy checks, focused review, and report pass without a profile implementation claim |
| 4 | owner/canon review | owner adopts, rejects, or revises the profile; this is outside P111 close |

## adoption path after P111

If the owner adopts the proposal, the canonical transaction must first define
the exact profile in a plan-level governance source, update `plan/01` to name
its T0 role, and record the required owner-approved decision/ADR and
CHANGELOG/ledger changes. `spec/06` remains SCN-only; `architecture/03` may
gain only the consequential tool contract. Before a JSON producer exists, that
transaction must either identify an already-authorized one-off derived artifact
that creates no committed helper/evidence/report family, or explicitly resolve
the `plan/02` moratorium conflict. Its result must bind the effective profile,
evaluated revision, owner records, LAB evidence revision, and artifact digest;
it remains distinct from SCN conformance and precedes, rather than substitutes
for, G0-D3.

## non-claims

P111 does not establish G0 exit, T1 entry, C-static/C-runtime/C-distributed
conformance, a theorem/proof result, a runnable sample, an implementation
tool, a public grammar/API, or a product/runtime state.
