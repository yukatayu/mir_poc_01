# 15 — sub-agent / reviewer plan

Use sub-agents actively if available. Do not treat a missing sub-agent result as approval.

## Reviewer 1 — verification/type-theory

Focus:

- checker / model-check / proof line separation
- dependent type wording
- Isabelle / Lean relationship
- soundness / bounded completeness
- residual obligation handling

Questions:

- Does the spec overclaim full dependent type theory?
- Does it hide obligations?
- Does it keep finite decidable checker and proof line separate?

## Reviewer 2 — cut/save-load

Focus:

- event DAG
- consistent cut
- save object shape
- load admissibility
- Z-cycle
- memory-order boundary

Questions:

- Is `atomic_cut` still place-local?
- Does save/load avoid ad-hoc snapshot semantics?
- Are distributed/durable claims avoided?

## Reviewer 3 — auth/layer algebra

Focus:

- contract transformer semantics
- transparent overlay vs contract update
- auth stack composition
- rate-limit / debug layer
- signature/provenance boundary

Questions:

- Are auth/rate-limit treated as non-transparent when appropriate?
- Are failure/effect/capability rows preserved?
- Is native signature not overclaimed?

## Reviewer 4 — observability/devtools

Focus:

- typed information-bearing effect
- label / authority / redaction / retention
- session-bound observability
- observer-safe vs admin/debug

Questions:

- Is debug treated as mandatory for operational readiness?
- Are leaks prevented in wording?
- Are exact-report bundle and live/session-bound export separated?

## Reviewer 5 — docs/source hierarchy

Focus:

- specs vs plan vs snapshot hierarchy
- document map updates
- progress percentage wording
- report template

Questions:

- Are new docs linked in correct places?
- Does `progress.md` distinguish first-floor and operational readiness?
- Does samples dashboard avoid overclaim?

## Reviewer 6 — practical-readiness

Focus:

- α-0.5 completion condition
- α-0.8 completion condition
- sample matrix
- developer reproduction flow

Questions:

- Can a new developer tell what to run next?
- Is host-I/O gap captured?
- Is same-session runtime gap captured?

