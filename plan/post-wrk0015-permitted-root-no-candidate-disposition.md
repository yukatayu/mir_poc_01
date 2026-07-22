# Post-WRK-0015 permitted-root candidate disposition

## Role and authority

This is a LAB selection record at source cut `80d0ee99`. It does not create a
`working/WRK-0015` record, execute a command, alter ADR-0014, or change Canon
theory, lifecycle, authority, or runtime semantics. Canon remains normative.

## Question

After the P-SURF-05 stale-fence candidate stopped at its input-location
preflight, does a distinct candidate already exist in a permitted LAB root for
autonomous L3 pre-registration?

## Scope and selection filters

The current validator permits exact LAB roots `plan`,
`samples/clean-near-end`, `samples/current-l2`, `samples/lean`, and
`samples/product-alpha1/computational`. Only the last root may declare a
descendant input location. A candidate in this run also had to have a fresh
exact command, a materially non-duplicative discriminator, a documented live
downstream decision branch, and no reserved-boundary choice.

These are run-specific filters. They do not add a new ADR-0014 rule or prove
that no future eligible candidate can exist.

## Current-cut screen

| Near-miss | Disposition | Reason |
| --- | --- | --- |
| P-SURF-05 stale-fence source/test/sample | policy stop | Its actual inputs are outside permitted roots. This is neither a behavioral result nor a frozen record. |
| P-COMP direct-carrier variants | duplicate / no live branch | WRK-0012/0013 already delimit the one-row carrier observations. The current public carrier collapses the rejection phases, so this screen provides no distinct decision branch. |
| `samples/current-l2` e13/e16/e18/e20/e23 variants | duplicate or reserved | Existing records already cover the available literal gaps; e23 additionally needs an external source choice. |
| Lean / clean-suite parity variants | reserved | Literal theorem skeletons exist, but a semantic mapping or synthetic-role choice would be needed to interpret them. |
| clean-suite detach TODO | reserved | Its lifecycle and migration semantics remain intentionally unfinished rather than an autonomous experiment boundary. |

Planner and explorer screens at this cut found no additional qualified source.
Temporary Oracle review `adr0014-no-candidate-20260722` challenged the screen
and found no substantiated counterexample candidate. Oracle advice is advisory
only.

## Disposition

**No candidate was selected for this run.** At this source cut, no candidate
passed the documented non-duplication, exact-command, live-decision-branch, and
reserved-boundary filters. This is a bounded LAB selection disposition, not an
executed result, behavioral falsifier, frozen WRK, permanent closure, or proof
that no ADR-0014-eligible candidate exists.

## Reopen conditions

Re-screen when a fresh permitted-root discriminator has a documented live
decision fork, or when explicit owner/canon action changes the lane policy or
fixes an otherwise reserved proof interface. Do not reuse the excluded
pre-registration command from the P-SURF-05 selection.

## Non-claims

This record makes no claim about checker behavior, runtime correctness,
authority/revocation/rejoin semantics, theorem/OBL status, conformance,
workflow readiness, Gate/Phase movement, or public completion.
