# plan/123 - G1 SCN-01 visibility negative actualization

## Purpose

This file is LAB repository memory.

It records the first targeted SCN static-gap actualization after
`plan/122-g1-scn-exact-static-slice-manifest.md`: an exact SCN-01-shaped
negative fixture for the visible-write `VisibilityDenied` failure-row
underdeclaration.

This file does not edit canon, does not move G0/G1 gate status, does not
discharge OBL-001 / OBL-024 / OBL-025, does not claim C-static conformance, and
does not change runtime, transport, diagnostic, repair, or Core IR ABI status.

## Decision

The next package should not refine the LAB OBL-001 Lean statement first unless
`plan/122` exposes a missing abstract predicate.

Local reread found that `samples/lean/lab-statements/obl001/THM001StatementDraft.lean`
already keeps the relevant abstract hooks:

- `AllRhsReadsRecorded`
- `RequestCarriesDependencyEvidence`
- `VisibleWriteConsequencesExplicit`
- `GeneratedFailuresContained`
- `RequestCarriesFailureContainment`

Therefore the smaller safe ratchet is to actualize the exact SCN-01 static
negative gap identified by `plan/122`, not to widen or rename the OBL-001
statement vocabulary.

Oracle consult `you-are-reviewing-the-mirrorea` recommended doing an OBL-001
predicate / statement-boundary audit before adding new SCN negatives. The same
answer also judged that the likely outcome would be no new predicate, because
the current LAB OBL-001 draft already has abstract dependency and visible
consequence hooks.

This package therefore records an explicit local deviation from the preferred
ordering: `ELAB-17` was already a small exact fixture actualization with no
production logic change, no Lean vocabulary change, and no canon/status claim.
The Oracle concern is still accepted as a sequencing guard: do not add another
SCN fixture before the OBL-001 boundary is audited against `ELAB-11`, `ELAB-12`,
and `ELAB-17`.

## Actualized row

New row:

```text
ELAB-17
samples/full-system-v1-surface/elaboration/
  elab-17-scn01-visibility-failure-row-negative/
```

Source shape:

```text
BrowserClient[self] {
  when roll(draw: Int64)
    fails MissingCapability, MissingWitness, RouteUnavailable, StaleMembership {
    World { player[self].position = player[self].position + draw }
  }
}
```

This is the `ELAB-11` / SCN-01 positive source shape with only
`VisibilityDenied` removed from the declared failure row.

## Evidence fixed by ELAB-17

`ELAB-17` is exact current executable LAB evidence for this static negative:

- the row is rejected;
- legacy diagnostic code is `generated_failure_not_declared`;
- LAB canon classification is `E-ROW-002`;
- missing evidence is exactly `VisibilityDenied`;
- request kind is `write`;
- generated request is owner-directed from `role:BrowserClient` to `World`;
- state/key/field are `player[self].position`;
- generated source is `nested_place_block`;
- the RHS same-field dependency remains recorded;
- generated `message_envelope`, `auto_publish`, and `auto_observe` rows remain
  visible in the helper projection;
- non-final `lab_diagnostic_details` include request context, failure-row
  context, OBL-024 diagnostic-soundness projection carrier, report-local replay
  anchor, and one LAB-only `add-to-fails-row` repair item.

## Relation to plan/122

This changes one `plan/122` classification:

| SCN-01 bullet | Previous classification | Current classification |
|---|---|---|
| negative: removing `VisibilityDenied` yields E-ROW-002 at assignment span | structural support only via `ELAB-10` | exact current executable evidence via `ELAB-17`; `ELAB-10` remains structural read/observe support |

Other `plan/122` boundaries remain unchanged:

- `ELAB-11` remains the exact positive SCN-01 evidence center.
- `ELAB-12` remains the exact positive SCN-02 evidence center.
- SCN runtime bullets remain explicit gaps / out of scope.
- SCN-02 direct-local-write rejection is still structural support only.

## Non-claims

- No canon edit.
- No G0 exit.
- No T0 -> T1 transition.
- No G1 exit.
- No OBL-001 completion or proof discharge.
- No OBL-024 / OBL-025 completion or proof discharge.
- No C-static / C-runtime / C-distributed conformance claim.
- No runtime request serving, store mutation, occurrence ordering, admission
  lifecycle, stale-membership runtime failure, or distributed transport claim.
- No final Diagnostic JSON, repair payload, association key, replay, Core IR,
  source map, runtime, transport, or public API freeze.
- No general visibility-repair support beyond the exact singleton
  `VisibilityDenied` `add-to-fails-row` LAB evidence shape.
- No promotion of `World`, `BrowserClient`, `Player`, `position`, or `roll`
  into Mir core vocabulary.

## Next allowed move

After this row, the next package should be the narrow OBL-001 boundary audit
recommended by Oracle: confirm whether the current abstract predicates can carry
`ELAB-11`, `ELAB-12`, and `ELAB-17` without importing LAB carrier names.

After that audit, the narrowest remaining SCN static gap is SCN-02 direct-local
write rejection, if it becomes a blocker for the G1 bridge.

OBL-001 statement refinement should remain reserve-only unless review finds
that the current abstract predicates cannot carry the `ELAB-11`, `ELAB-12`, and
`ELAB-17` evidence without overfitting to helper JSON.
