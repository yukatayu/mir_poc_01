# Plan 198 - T0/G0 governance-profile v2 evaluation

## Role and authority

This is LAB repository memory for the completed O0 package. The normative
definition is `mirrorea_canon/plan/01-phases.md` under the amended ADR-0013.
This note neither accepts G0-D3 nor changes a Gate, Phase, SCN, OBL, proof,
implementation, or public-readiness state.

## Fixed history

- Canon adoption/source commit `C2`:
  `0ee3fdec553de31252a37478fc4a31f507221258`.
- The sole v2 artifact commit `B2`, its direct child:
  `39c85e9f99ae45bcb097939a543b39dbb7bb9b81`.
- Authorized artifact path:
  `plan/198-t0-g0-governance-profile-v2-evaluation.json`.
- v1 remains byte-preserved nonconforming historical evidence at `plan/155`;
  its whole-file SHA-256 is
  `0ad49fa84cd766165c5f28bee4dda9a8794f674873e072bf1919eba9027ca943`.

`B2` has one parent, adds exactly the designated JSON path, and the v2 path is
absent from every pre-`B2` ancestor. The artifact binds `C2` Git blobs and has
digest `b3748cb62569e25da4eaabf124eea97e97e98147185e433557575127b33895e4`.

## Evaluation result

| Check | Result | Reading |
| --- | --- | --- |
| `g0-substantive-owner-record` | `pass` | ADR-0013 retains the original G0-D1 acceptance record. |
| `g0-source-hierarchy-controls` | `fail` | The fixed source-hierarchy pin and three fixed LAB notice pins no longer match `C2`. |
| `g0-demotion-audit-scope` | `pass` | ADR-0013 retains the original G0-D4 waiver wording. |
| Root result | `fail` | Mechanically derived from the three checks. |

The mismatching current blobs are `mirrorea_canon/meta/source-hierarchy.md`,
`CANON.md`, `README.md`, and `AGENTS.md`. `mir_hilight.html` and
`samples/clean-near-end/README.md` still match their pins. This is a
byte-level control result, not a newly inferred semantic or historical audit.

## Consequence and next stop line

The one permitted v2 artifact has now been consumed. It is a valid `fail`, not
a malformed artifact, and it cannot support G0-D3 acceptance or G0 exit. The
official state remains T0.

Any proposal to rebase the fixed controls, interpret their drift, or authorize
another artifact is a new owner/Canon decision. It is not implied by O0, this
note, or the failed result. ADR-0013 permits a concrete drift to reopen a
separately scoped audit, but does not reopen one automatically.

## Scoped drift audit (2026-07-28)

The owner selected the recommended first step: audit the drift before any
proposal to re-pin controls. The audit compared the four mismatching blobs at
the accepted evidence cut `6f96ce17e74173ca5d86ed76cee3db75d60dcbfe` with
the version-2 source cut `0ee3fdec553de31252a37478fc4a31f507221258`.

| Control | Change class | Audit result |
| --- | --- | --- |
| `meta/source-hierarchy` | Canon process wording | Adds ADR-0014's bounded working-theory route while retaining `canon > LAB`; it neither changes Mir semantics nor promotes LAB evidence. |
| `CANON.md` | LAB entry-point wording | Points readers to the same bounded route and its reserved boundaries; it introduces no competing authority source. |
| `README.md` | LAB reader/operational guidance | Adds a project overview link and describes the existing research-governance boundary; it does not amend a Canon claim. |
| `AGENTS.md` | agent operating guidance | Corrects Oracle wrapper preference and spells out existing ADR-0014 limits; it does not grant agents a reserved edit. |

The four changes are intentional governance/readability drift, not a change to
the accepted substantive ADR set, MirCore, an SCN expectation, an OBL, a Gate,
or a Phase. The audit therefore finds no semantic reinterpretation to correct.
It does **not** re-pin a control, create another artifact, change the root
result, or make G0-D3 acceptance available. A normal Canon rebase proposal
remains a separately recordable owner decision if a future valid `pass` route
is desired.

## Non-claims

No G0-D3 acceptance, G0 exit, T1 entry, I1 authorization, SCN conformance,
C-static/C-runtime/C-distributed result, proof/OBL movement, implementation
authorization, or public readiness follows from this record.
