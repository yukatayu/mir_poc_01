# 17 — risk register

## R1. artifact completion mistaken for operational readiness

Risk:

- sidecar / expected JSON / closeout report gets read as practical completion.

Mitigation:

- use category vocabulary
- progress tables include first-floor and operational readiness separately

## R2. full dependent type theory overclaim

Risk:

- indexed/refinement finite checker gets described as dependent type completion.

Mitigation:

- define stratified checker / model-check / proof line
- explicitly defer full dependent type theory

## R3. save/load ad-hoc expansion

Risk:

- local snapshot grows into distributed save/load without consistent cut semantics.

Mitigation:

- event DAG / cut / save object / load admissibility spec
- mark distributed durable save/load still later

## R4. auth layer unsound composition

Risk:

- auth / rate-limit silently strengthens preconditions or widens failure rows.

Mitigation:

- contract transformer theory
- explicit contract update + activation cut

## R5. debug leak

Risk:

- devtools export leaks witness/auth/high-label state.

Mitigation:

- observation event model
- labels / redaction / retention
- observer-safe sample requirements

## R6. host I/O mistaken for stdio builtin

Risk:

- EchoText / AddOne gets implemented as privileged stdout/stdin.

Mitigation:

- typed external host boundary spec
- no stdio builtin stop line

## R7. same-session gap remains hidden

Risk:

- product preview bundle is treated as runtime session.

Mitigation:

- define PracticalAlphaSession requirement
- product preview remains preview until same-session workflow exists

## R8. native signature overclaim

Risk:

- signed native package is treated as safe.

Mitigation:

- signature/provenance distinction
- native execution policy with sandbox/capability/audit

## R9. docs bloat without implementation guidance

Risk:

- new specs are abstract but do not guide next packages.

Mitigation:

- include exact α-0.5 / α-0.8 roadmap and sample matrix
- define P-A1-19/20/21 reopen candidates

