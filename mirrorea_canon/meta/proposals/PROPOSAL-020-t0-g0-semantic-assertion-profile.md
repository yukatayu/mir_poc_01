---
id: meta/proposal-020
status: L1-fixed
maturity: draft
depends_on: [meta/proposal-014, adr/ADR-0013, adr/ADR-0015, root/design-constitution]
summary: owner-approved M2 scopeで、mutable reader-facing whole-file pinに代わるT0/G0 semantic-assertion profile v3を採用するproposal。
open_items: []
---

# PROPOSAL-020 — T0/G0 semantic-assertion profile v3

## Owner disposition

The owner-approved M2 direction replaces the current T0 interpretation only:
adopt a deterministic revision-bound semantic-assertion profile, preserve v1
and v2 artifacts as history, generate a fresh v3 artifact, and—if it passes
the adopted contract—record v3 digest acceptance, G0-D3 acceptance, G0 exit,
and T1 entry in one milestone record.

## Chosen and rejected profile

The current v2 fixed whole-file control profile is rejected for M2 because its
normal reader-facing maintenance hash drift is not a Mir semantic failure and
its one-off artifact route is already consumed. The selected v3 profile checks
the six bounded assertions in `plan/04`; it binds a committed source revision
but does not re-pin mutable documents to obtain a result.

## Scope and non-effects

This authorizes the v3 profile, its deterministic producer/test/validator,
fresh LAB artifact, exact canonical acceptance record, and associated status
sync. It does not rewrite v1/v2, reaccept D1/D2/D4, claim SCN/proof/runtime/I1
success, weaken a safety/privacy guarantee, promote domain vocabulary to Core,
or fix a public API/ABI/wire/deployment contract.
