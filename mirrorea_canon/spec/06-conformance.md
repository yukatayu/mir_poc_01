---
id: spec/06-conformance
status: L1-fixed
maturity: draft
depends_on: [spec/05-runtime-semantics, scenarios/readme, plan/01-phases, adr/ADR-0013]
summary: SCN-01..10 を適合性基準として束ねる。適合レベルと合否判定。
open_items: []
---

# 06 — Conformance

The frozen suite scenarios/SCN-01..10 **is** the conformance definition.
Any change to theory or spec that alters an SCN expectation requires an ADR.

Levels:

- **C-static**: parse + check + elaborate every declared SCN source carrier;
  validate every declared finite typed carrier; and establish the exact
  spec/11 carrier correspondence for each frozen expectation. Every positive
  source-derived expectation elaborates with its expected edges/obligations;
  every negative source or typed-carrier variant yields the expected diagnostic
  at its required source span or typed-carrier field reference. (PHASE-I1
  entry.)
- **C-runtime**: run executable accepted carriers under the deterministic
  profile and its typed external action schedule; occurrence rows, verdicts,
  store states at cuts, and projections match their predicates. Runtime-class
  negatives produce the expected explicit failures. (PHASE-I1 exit.)
- **C-distributed**: same expectations with ≥2 OS processes and real
  transport for SCN-01/02/03/06 (PHASE-I3 exit).

Pass = 10/10 at the claimed level, no expectation waivers. Partial claims must
enumerate failing SCN ids. A frozen expectation without an exact correspondence
is a failure, never `N/A` or a waiver. Implementations report as
`conformance: {level, scn_pass: [...], scn_fail: [...], profile_hash}`.

For the selected M10 profile, spec/11 fixes each correspondence row phase to
`static` or `runtime`. `C-static` is the aggregate of all `static` rows;
`C-runtime` additionally requires a passing `C-static` aggregate and all
`runtime` rows. `C-static` / `C-runtime` are not row-phase values.

## M10 finite direct realization

Spec/11 defines the only selected M10 realization of C-static/C-runtime for
the frozen suite. Each SCN correspondence names one primary ordinary `.mir`
source plus finite named negative source variants; a primary source may be
shared across SCNs while retaining its exact identity. SCN-09 additionally has
its finite named candidate patch sources. Every source unit is parsed and
classified through the direct path; an accepted unit is checked and elaborated,
while a negative unit ends only at its typed diagnostic terminal. An executable
accepted unit retains its checked identity across M8 `DeferredToM9`, M9
source-bound resolution, crate-private M9-to-M8 authority inventory, runtime,
and observer-safe projection. Setup prose is a versioned typed profile input,
not source or expected output; expected JSON, reports, fixture names, and
waiver carriers cannot construct a result. Waivers are prohibited and always
empty.

The profile binds version, source revision, every source-unit hash, runtime
inputs, policy stamps, predicates, and `profile_hash`; it requires per-source-unit identity
continuity to its terminal, negative diagnostic/span/no-mutation evidence,
deterministic replay, and fresh-checkout reproduction. It does not require an
identity to be shared across variants. SCN-11/12 remain pressure rows, not
members of this frozen 10/10 claim. A passing profile is separate from an
explicit phase acceptance record.

The M10 carrier table binds each frozen expectation to one or more exact
source, separately checked patch-source, finite typed profile-carrier, profile
context, or schedule-action references. It records program-artifact and
schedule-action provenance separately. A schedule is exogenous typed context
or request only; it cannot create a Core result, authority grant, verdict,
state/relation/fallback mutation, patch declaration, history/projection row,
or expected result. SCN-08's non-Surface three-option fallback uses the
explicit finite typed carrier in spec/11, not a schedule-only substitute.
SCN-09 candidate source plus typed patch intent is checked before compatibility
and derives its reject diagnostics there. SCN-05/07 use only the explicit
source-bound observation-policy carrier for their non-Surface private-policy
checks; those carrier diagnostics are not represented as absent M6/M7 checks.

For the frozen SCN-01/02 owner-directed cases, `Role[self] at L_actor` is the
authority origin and nested `at L_owner` is the evaluation/request site. Their
difference is accepted; only a target state whose declared owner differs from
`L_owner` is rejected. Same-owner RHS reads resolve at `L_owner`, and the
generated `RouteUnavailable` failure remains required.

C-distributed is outside this M10 finite realization and remains unclaimed.

`phase-governance/t0-g0` is defined by `plan/01-phases` under ADR-0013. Its
one-off T0 JSON is outside this conformance definition: it has no conformance
carrier, C-level, or SCN pass/fail claim.
