# WRK-0015 stale-fence registration preflight

## Role and authority

This is LAB repository-memory for a failed pre-registration check. It is not a
`working/WRK-0015` record, does not retain a command result, and does not
change Canon theory, authority, revocation, rejoin, diagnostics, or runtime
semantics. Canon remains normative.

## Question checked

Can the selected P-SURF-05 second-admission stale-fence candidate be registered
under the current ADR-0014 working-annex protocol without creating or widening
an evidence lane?

## Preflight evidence

The candidate's exact inputs are the role-admission checker and test under
`crates/mir-semantics/` and the stale-membership source/readme under
`samples/full-system-v1-surface/role-admission/`. Current
`scripts/validate_docs.py` permits WRK LAB inputs only in `plan`,
`samples/clean-near-end`, `samples/current-l2`, `samples/lean`, and
`samples/product-alpha1/computational` (with only the latter allowing
descendant locations). The current source and sample roots are not permitted.

A target-literal search across every permitted root found no matching
role-admission source, stale-message input, or checker artifact for the
registered command. It does not establish that no semantically similar input
exists. A temporary draft record correctly failed its preflight validation for
the input roots; it was removed before any commit. No `working/WRK-0015` record
exists, and no fresh Cargo command ran.

## Disposition

The candidate is **not registrable** under the current L3 route. This is a
governance/input-location stop, not a falsifier of the checker behavior and not
a frozen WRK, because no WRK was created. The preliminary command from the
selection screen remains excluded forever from any future WRK evidence.

## Advisory review

The temporary Oracle consultation `wrk0015-lab-lane-governance-20260722`
independently reached the same conclusion: defer this candidate as an
admission-policy block, rather than treating it as an experiment falsifier or
a frozen record. Its advice is non-normative. It found no evidence that the
excluded roots were already owner/canon-authorized; an actual lane-policy
change therefore remains an owner/canon action.

## Rejected workarounds

| Option | Disposition | Reason |
| --- | --- | --- |
| Put a transcript or digest in `plan/`, then execute the source through `git show` | rejected | It disguises the actual checker/sample input rather than locating retained source/test evidence in a permitted existing lane. |
| Add `crates/` or `samples/full-system-v1-surface/` to the validator allowlist | rejected for autonomous work | It broadens the evidence-lane policy and conflicts with the current moratorium; it is not an L3 record-local adjustment. |
| Use a matching input in an already permitted root | unavailable now | The target-literal preflight search found no matching source/checker artifact. |
| Run the disposable command without a record | rejected | ADR-0014 requires pre-registration before outcome evidence is relied on. |

## Reopen condition

Reopen only if a matching documented input is already available in a current
permitted LAB root, or after an explicit owner/canon action changes the evidence
lane policy. A later candidate must use a new record and fresh input pins; it
cannot reuse the excluded preliminary output.

## Non-claims

This does not claim stale-fence behavior, a runtime defect, a required repair,
test coverage, Canon authority/revocation/rejoin semantics, OBL-028 evidence,
contract/conformance status, SCN/Gate/Phase movement, or public workflow
readiness.
