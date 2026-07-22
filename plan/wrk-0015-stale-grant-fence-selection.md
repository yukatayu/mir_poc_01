# WRK-0015 stale-grant-fence selection

## Role and authority

This is LAB candidate-selection memory. It is not a `working/WRK-0015`
registration, does not retain an experiment outcome, and does not interpret the
existing source checker as Canon authority, revocation, rejoin, or runtime
semantics. Canon remains normative.

## Candidate question

Can a post-registration, disposable-source experiment in the existing
P-SURF-05 role-admission lane establish only this source-local fact: after a
`stale_message` fences `(principal, target_place)`, does inserting a second
report-level `join` leave the fence in place and cause the next indexed-state
write check to remain rejected with `stale_membership`?

The transformed source must be temporary. It must not become a committed
sample, fixture, helper, schema, runner, or public workflow.

## Source basis

The present checker stores `StaleGrantFence { principal, target_place }`.
Handling `stale_message` removes matching active grants and inserts that fence.
Later report-level admission creates a fresh checker-local epoch/incarnation
and grant, but the static source has no fence-removal operation there. The
write check requires a matching active grant and no matching fence. The
existing tests cover `join -> stale_message -> write reject`, but not a second
`join` between the stale message and write.

These are source-reading observations only. They do not decide what a Canon
epoch, incarnation, fresh evidence, grant lineage, rejoin, or revocation must
mean.

## Candidate comparison

| Candidate | Disposition | Reason |
| --- | --- | --- |
| Disposable P-SURF-05 source-local fence experiment | selected for L3 pre-registration | Existing lane, concrete untested branch, and a bounded command exist. Positive and adverse outcomes are distinguishable without a new retained source surface. |
| OBL-028/revocation theorem candidate | rejected | No Canon-to-source valid-grant or new-evidence relation exists. Treating checker epochs/fences as that relation would select reserved semantics. |
| Runtime repair or new fixture/test | rejected | The experiment may record behavior only. A repair, coverage requirement, or retained fixture would be a separate implementation decision. |
| No further work | rejected for this source-local question | The second-admission branch is distinct from the existing stale-membership test and has a disposable, existing-lane discriminator. |

## ADR-0014 eligibility read

The proposed result class is `existing-lane-experiment`. It can satisfy the
standing predicate only if a new record pins the exact Canon/LAB inputs,
declares the alternative and falsifier before execution, retains no transformed
source, excludes every reserved surface, and records only source-local output.

The proposed record must not treat this selected experiment as a bridge, and
the selection does not narrow other ADR-0014 result classes or future L3
candidates.

## Expected branches and stop line

| Branch | Source-local observation | Required handling |
| --- | --- | --- |
| Positive observation | second admission appears, but the following write remains rejected with `stale_membership` | Retain only the registered command/result evidence; no defect, Canon, or repair conclusion. |
| Falsifier | following write is accepted without `stale_membership`, or the command needs a retained helper/fixture or reserved interpretation | Freeze reliance immediately and retain only reproducible permitted evidence. |

## Registration boundary

The next package may add only a concise `working/WRK-0015-...` L3
pre-registration and required canonical operational metadata. It must be
committed and pushed before a **fresh WRK outcome** command runs. An
unregistered preliminary command was executed during the eligibility screen;
its output is excluded from this selection and can never become WRK evidence.
A later evidence package may update the record forward-only and retain direct
report evidence only if unchanged validation permits it.

## Non-claims

- No OBL-028 counterexample, theory evidence, proof, ledger movement, or
  Canon-valid authority/revocation interpretation.
- No runtime defect, repair obligation, test-coverage requirement, or
  implementation change.
- No contract, SCN, Gate, Phase, conformance, transport, public API, identity,
  authentication, authorization, or membership-lifecycle claim.
