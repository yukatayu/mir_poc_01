# Plan 172 - standing-autonomy lane correspondence checkpoint

## Role and authority

This is LAB repository memory and an escalation bundle for the finite
autonomous research horizon in `plan/158`. It does not amend ADR-0014, the
working annex, validator root-policy/root-admission enforcement, Canon theory,
or any sample/runtime behavior. The numbered-plan registry is synchronized so
this plan remains documented. Canon remains normative.

## Trigger and evidence cut

At source cut `21a00d77`, the P-SURF-05 second-admission candidate had stopped
before registration because `scripts/validate_docs.py` rejects its actual
`crates/mir-semantics` and `samples/full-system-v1-surface` inputs. The
following permitted-root screen selected no candidate for that run. No
outcome-producing P-SURF command, `working/WRK-0015`, or allowlist change
exists.

This checkpoint reads the following evidence without promoting it:

| Cut / source | Relevant fact |
| --- | --- |
| `1041505a` | introduced ADR-0014, the working annex, and the initial exact validator root guardrail together. |
| `66229add` / `plan/170` | corrected the prior permanent-global-whitelist reading: each record declares its own existing documented LAB lane. |
| `0dcc9dd3` / R-2344 | added the bounded Product Alpha computational guardrail without changing Canon; R-2344 calls the tuple an executable guardrail rather than Canon. |
| `80d0ee99` | P-SURF registration preflight stop and permitted-root no-candidate selection. |
| `21a00d77` | current committed source cut before this checkpoint. |

## Established facts

1. The current validator's exact-root rejection is real and deliberate
   fail-closed operational behavior.
2. ADR-0014 and `working/README.md` require an *existing documented LAB lane*
   and record-local permitted locations, but do not name an exhaustive root
   catalog or its amendment authority.
3. `samples/full-system-v1-surface/README.md` documents role admission as an
   actualized LAB evidence lane. This alone does not establish that the
   P-SURF-05 candidate is eligible under every ADR condition.
4. The recent selection filters for material non-duplication, exact command,
   and live downstream decision are LAB prioritization filters. They are not
   recorded here as additional standing-eligibility requirements. A candidate
   must still be concrete, pre-registrable, non-reserved, and evidence-backed.
5. Current source review, the clean-suite review, and temporary Oracle review
   identify no concrete candidate that can be registered now without deciding
   this correspondence or another reserved boundary.

## Unresolved correspondence

Two readings remain compatible with the available evidence:

| Reading | Consequence |
| --- | --- |
| Closed-catalog | The validator tuple is the owner-controlled authoritative catalog of permitted existing LAB lanes. P-SURF-05 remains outside the delegated route until an owner/canon change. |
| Guardrail/cache | The tuple is a fail-closed implementation catalog. An independently documented lane omitted from it may be a validator under-permission that requires a bounded, reviewed correction before any record is opened. |

Neither reading is selected by this plan. In particular, the Git authorship of
the validator commits and prior LAB reports do not substitute for an explicit
owner/canon disposition.

## Current disposition

The current validator remains fail-closed. Do not execute or reuse the
preliminary P-SURF command, create `WRK-0015`, bypass the validator, widen the
tuple, or treat the stop as a checker falsifier.

Package C of the finite `plan/158` research ratchet is intentionally closed at
this checkpoint. This closes only the current autonomous source cut; it does
not close ADR-0014 research, T0/G0, later implementation work, or any public
goal.

## Owner checkpoint question

> Is the validator tuple the closed authoritative catalog of existing permitted
> LAB lanes, or a fail-closed implementation catalog whose omissions may be
> corrected after bounded documentation and review?

No immediate action is presumed. Defer is a valid owner disposition.

## Branch consequences

| If selected | Later required action |
| --- | --- |
| Closed-catalog | Identify the catalog and its amendment authority canonically before a new lane is admitted; keep P-SURF-05 stopped meanwhile. |
| Guardrail/cache | Define auditable documented-lane admission criteria, then narrowly change validator/tests through the ordinary process before any fresh P-SURF preregistration. |
| Defer | Preserve the stop and re-screen only a concrete candidate already accepted by the current validator or a different reserved boundary is resolved. |

## Reopen evidence

This checkpoint should be superseded by any of the following, rather than
reinterpreted in place:

- explicit owner/canon catalog correspondence disposition;
- a concrete current-validator-permitted candidate dossier with pinned inputs,
  alternative/falsifier, non-effects, and no reserved choice;
- evidence that a documented current lane has a new reproducing regression
  that can be isolated without a new helper, schema, CI/Make surface, public
  interface, or production behavior; or
- an already-fixed proof-facing interface that makes a prior reserved theory
  candidate literal and non-reserved.

## Non-claims

This plan does not claim the tuple is accidental or canonically exhaustive;
that P-SURF-05 is eligible or ineligible under ADR-0014 as a whole; an
allowlist defect; checker/runtime behavior; a theorem/OBL, Gate, Phase,
conformance, sample, workflow, or public status change.
