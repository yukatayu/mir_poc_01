# Risk register

## R1: evidence -> product overclaim

Risk: sidecars/reports pass, but product is not usable.

Mitigation:

- classify as evidence / first-floor / operational / product.
- product α-1 requires clean clone workflow and CLI.

## R2: exact report bundle mistaken for same-session runtime

Risk: devtools/product preview composes reports but does not run.

Mitigation:

- session carrier must be source of product demo behavior.
- exact report bundle may support devtools, not replace runtime.

## R3: quiescent save overclaim

Risk: consistent cut is called "all messages delivered".

Mitigation:

- distinguish R1 vs R2.
- require NoInFlight / AllPlacesSealed / NoPostCutSend.

## R4: auth layer transparent overclaim

Risk: auth/rate-limit silently strengthens contract.

Mitigation:

- explicit contract update or declared failure row.

## R5: native safety overclaim

Risk: signed native package treated as safe.

Mitigation:

- signature = provenance only.
- native execution disabled by default.

## R6: viewer leaks secret data

Risk: observer-safe view exposes raw witness/auth/capability.

Mitigation:

- redaction tests.
- observer/admin separation.

## R7: product α-1 collapses Reversed Library / PrismCascade

Risk: upper-layer goals enter α-1.

Mitigation:

- keep Reversed Library and PrismCascade out of product α-1 implementation scope.

## R8: final grammar freeze too early

Risk: `package.mir.json` or textual `.mir` becomes final grammar accidentally.

Mitigation:

- call it alpha package format.
- document migration/non-final grammar.

## R9: WAN/distributed durability overclaim

Risk: Docker/local result called production network or durable save/load.

Mitigation:

- WAN/federation and R3/R4 are non-goals.

## R10: Codex never stops

Risk: infinite widening.

Mitigation:

- package sequence.
- explicit closeout.
- stop on U1/product decision blocker.
