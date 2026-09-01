---
id: meta/proposal-039
status: L1-fixed
maturity: reviewed
depends_on: [meta/proposal-037, adr/ADR-0034, adr/ADR-0035, arch/02-boundary-contracts, arch/06-project-product-layers, theory/05-authority, theory/07-observation, theory/13-evaluation-materialization]
summary: ALIGN-2でBrowser/Host package、View/input、provider/raw FFI、resource sandboxの非凍結trust boundaryを採用する提案。
open_items: []
---

# PROPOSAL-039 — ALIGN-2 Browser/Host trust boundaries

## Owner disposition and direct capability

Under PROPOSAL-037 / ADR-0034, accept a responsibility-only contract that lets
a future Browser/Host safely admit third-party Mir packages, connect them to the
Mirrorea fabric, project to Views/providers and return typed input/effects without
making host implementation facts authoritative or freezing a concrete product.

```text
Direct consumer: I3-0, I3-1, I3-5, and NEXT-0's inactive I5 entry contract
Blocker reduced: admission, grant, presentation, input, provider, raw FFI and
  resource enforcement were previously compressed into BND-007 or left unstated
Acceptance use: constrain I3 and future I5 so package/transport/provider/host
  facts never become semantic authority and all reverse paths remain typed
```

## Selected design

Preserve and clarify BND-007 as Runtime/Projection to View. Add BND-010 through
BND-016 for package admission, Browser-to-fabric participation, View-to-renderer,
typed input, typed effect/provider, privileged native/raw FFI, and resource/sandbox
responsibilities. Put the full contract matrix and trust tiers in
`arch/07-browser-host-trust-boundaries`; keep `arch/02` as the concise BND index.
`arch/08-browser-host-security-invariants` fixes shared content/instance/epoch
binding, role separation, use-time revalidation, stale-work, ambiguous-effect,
metadata-redaction, resource-accounting and trusted-computing-base rules.

Trust tiers T0--T4 are explicitly local trust labels, not theory T0--T2 or a
numeric privilege lattice. Package admission is separate from membership,
capability and locus allocation. View/provider may perform presentation-local
computation but never owns authoritative domain semantics or directly mutates
semantic state. T1 checked untrusted packages have no raw FFI; T3 native
integration uses a separate privileged, least-privilege, revocable trust path.

## Smallest alternative and rejection

The smallest alternative was to leave every edge as an unnumbered subclause of
BND-007. It is rejected because it would again combine forward projection with
reverse input, ordinary typed providers with privileged raw native access, and
package admission with semantic grant. Separate stable responsibility IDs allow
later conservative implementation while keeping all concrete schemas private and
provisional.

## Falsifier and acceptance

Primary falsifiers are: content/target substitution or stale verdict reuse;
signature/admission/session/provider identity creates a grant; View/input/provider
directly changes state; T1 reaches raw FFI; revoke/reconnect resurrects stale
authority/work; replay duplicates an effect; enforcement fails open; or payload/
metadata output weakens redaction. Accept only when each BND has a positive
path, typed denial, validation owner, authority consequence, revocation/termination,
observation/redaction rule and explicit non-freeze, and independent security and
semantic review has no P0/P1.

## Non-effects

This proposal clarifies rather than changes the North Star: authoritative domain
semantics stays in Mir while presentation-local computation is allowed outside.
It changes no runtime, sample, scenario, proof/model/OBL, official lifecycle,
transport selection or public/production contract. It selects no package, origin,
signature, sandbox, storage, browser, renderer, Unity/Unreal, FFI, API, ABI, wire,
codec or permission-dialog mechanism. I5 remains inactive.
