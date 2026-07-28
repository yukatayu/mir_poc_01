---
id: working/WRK-0037
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, theory/01-mircore-v0, theory/04-ordering-and-cuts, theory/05-authority, meta/proposal-012, meta/proposal-013]
summary: C2-B/C3 の Canon carrier を選ばず、二つの opaque request atom、direct staged projection、injective restore を持つ固定有限 B-primary experiment が incidental identity なしに一つの coherent presentation を持つかを検査する。Core、Config、history、SaveObject、source rule、runtime は除外する。
open_items: []
---

# WRK-0037 - C2-B/C3 B-primary opaque-anchor experiment

## Classification and authority cut

Standing eligibility: pass
Author: codex
Author fingerprint: not-required-for-L3
Canon anchors: adr/ADR-0014@1041505a5979591414ef29e4f850e9d6ab52f28a:b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323, meta/proposal-012@fcf5ea613c2153667e1c4a887589fb939692c7a5:09ea4d6957c320b4d0647806714a1643101c2022b2893ac76ec7de3bf1db73d5, meta/proposal-013@fcf5ea613c2153667e1c4a887589fb939692c7a5:4e0ecf7475f20eec85c09d50201d2d2cc29848d480e8382935fe489b43877213, theory/01-mircore-v0@a51ab57b2df121186029dfae09a8206cee1f6702:35e2f52bebcf96332d8102e9110446930b7ff807d948737a5859909de34b0f12, theory/04-ordering-and-cuts@a51ab57b2df121186029dfae09a8206cee1f6702:70bde483330d3745a8694b15cd75f447b6610513ae66cb1ad4ec1faed274a264, theory/05-authority@a51ab57b2df121186029dfae09a8206cee1f6702:e06dc5ef0539eb5b87bce71b34d3e8d2ab0638603642e0d9f89581f29d25e6c4
LAB inputs: LAB:plan/208-c2b-c3-value-flow-design-preparation.md@e73c1fdc439ee977c272d7c415f36911b1304b2d:84380a9d2f9929f4ffe5d48f4baf74083fb00cc34d30132e43c45f48f1ddef55, LAB:plan/209-c2b-c3-relation-obligation-audit.md@e73c1fdc439ee977c272d7c415f36911b1304b2d:50dc299076df7844f3dd2fe641bbd65a57269d305743556bda07525c588faefa, LAB:plan/210-c2b-c3-family-a-b-instantiation-audit.md@6303d2ef40dc1ba00a71bca44436b51180b470d1:4cba73fdbb245b16bf9fdd312609401518abaae6c96273923e1bf861e1548ffe, LAB:plan/211-c2b-c3-b-primary-opaque-anchor-candidate-selection.md@f4c3e2bbac7b5c0ad57d23452d68ce080dd5bb87:d072a5430cbac1df52be216921f6f0e15c3d6819175e25872cb048430654585c, LAB:docs/reports/2503-c2b-c3-b-primary-candidate-selection.md@83cb649893cbf9bd1f175da8b40b4671d23c79b1:50dd2c4359105a3664684ff635b56e655adcf5eb4e0712d8b1ce72f2a624ef8a
Permitted LAB locations: plan, docs/reports
Reserved surfaces: excluded

## Pre-registered working question

Question: Can one fixed, artifact-local finite model use two locally distinct
opaque request atoms `q0` and `q1`, equal incidental observations, direct
q-indexed staged projections, and an explicit injective restore renaming to
realize one successful reply/receipt/resume/dependency branch and one terminal
failure branch without a second nominal attempt identity? The model must make
pending phase, validation outcome, reply, receipt, failure, held `Gamma`/`Delta`
disposition, consumed result/provenance, dependency ground, and restored state
explicit local functions. No function or theorem denotes a Mir request identity,
Core carrier, Config/history/SaveObject field, occurrence equality, authority,
runtime behavior, source form, or public contract.
Status quo: P012 V1/R1 requires a later package to make one-shot result use and
separate typed reply/receipt explicit, while leaving the carrier open. P013 M1
leaves request-instance/occurrence identity and encoding open. Plans 208--210
establish that B cannot derive correlation, receipt, consumption, or load
identity from the existing DAG or incidental request data. Plan 211 selects
only this reversible B2-OPAQUE experiment for an existing `plan/` lane.
Alternative: Retain no B-primary experiment because a finite model cannot add
a non-duplicate discriminator without selecting unregistered Core or persistent
state. In that event, do not materialize source; return to the owner/Canon
design boundary or compare Family C only after a concrete B failure.
Expected falsifier: The candidate aliases equal-incidental `q0`/`q1`; recovers
reply, receipt, failure, or dependency from ancestry/payload/span/locus/
transport/queue position; accepts a duplicate, late, or wrong-locus receipt;
allows failure with a matching success continuation, dependent occurrence, or
mutation; loses phase/provenance/held context/dependency after the stated
restore; requires a second nominal attempt/exchange identity; treats q, claims,
receipt, locus, or provenance as authority; needs selected Canon equality,
Core/Config/history/SaveObject/queue/wire state, OPEN-010/011 closure, a helper,
schema, CI/Make surface, source rule, THM/OBL, Gate/Phase, runtime, or public claim.
Rollback / reopen trigger: On any reproducible falsifier, set `Reliance status:
frozen`, retain the procedure and falsifier only in the declared LAB locations,
and do not repair this record into a positive B result. A changed authority cut,
actual carrier/identity selection, a Family C comparison, source/elaboration
artifact, contract, theorem/OBL, helper/schema/CI surface, runtime, or public
behavior requires a forward successor or ordinary Canon escalation.

## Method and evidence plan

Result class: existing-lane-experiment
Commands: Registration check, run before this record is created: `test ! -e mirrorea_canon/working/WRK-0037-c2b-c3-b-primary-opaque-anchor.md`. Outcome commands, run only after this registration is committed and pushed: `test -s mirrorea_canon/adr/ADR-0014.md && test -s mirrorea_canon/meta/proposals/PROPOSAL-012-mircore-value-flow-and-occurrence-identity.md && test -s mirrorea_canon/meta/proposals/PROPOSAL-013-post-admission-request-validation-context.md && test -s mirrorea_canon/theory/01-mircore-v0.md && test -s mirrorea_canon/theory/04-ordering-and-cuts.md && test -s mirrorea_canon/theory/05-authority.md && test -s plan/208-c2b-c3-value-flow-design-preparation.md && test -s plan/209-c2b-c3-relation-obligation-audit.md && test -s plan/210-c2b-c3-family-a-b-instantiation-audit.md && test -s plan/211-c2b-c3-b-primary-opaque-anchor-candidate-selection.md && test -s docs/reports/2503-c2b-c3-b-primary-candidate-selection.md`; `sha256sum mirrorea_canon/adr/ADR-0014.md mirrorea_canon/meta/proposals/PROPOSAL-012-mircore-value-flow-and-occurrence-identity.md mirrorea_canon/meta/proposals/PROPOSAL-013-post-admission-request-validation-context.md mirrorea_canon/theory/01-mircore-v0.md mirrorea_canon/theory/04-ordering-and-cuts.md mirrorea_canon/theory/05-authority.md plan/208-c2b-c3-value-flow-design-preparation.md plan/209-c2b-c3-relation-obligation-audit.md plan/210-c2b-c3-family-a-b-instantiation-audit.md plan/211-c2b-c3-b-primary-opaque-anchor-candidate-selection.md docs/reports/2503-c2b-c3-b-primary-candidate-selection.md`; `rg -n -i -C 3 'B2-OPAQUE|opaque request|opaque anchor|q-indexed|injective restore|incidental identity|request occurrence anchor|Family B' plan docs/reports mirrorea_canon/working mirrorea_canon/theory mirrorea_canon/spec mirrorea_canon/meta`; `awk 'BEGIN { in_block = 0 } /^```lean$/ { in_block = 1; next } in_block && /^```$/ { exit } in_block { print }' plan/wrk-0037-c2b-c3-b-primary-opaque-anchor-experiment.md > "${TMPDIR:-/tmp}/mir-wrk0037-c2b-c3-b-primary-opaque-anchor.lean" && lean --trust=0 "${TMPDIR:-/tmp}/mir-wrk0037-c2b-c3-b-primary-opaque-anchor.lean"`; `rg -n 'sorry|admit|unsafe|partial|implemented_by|Classical|Choice|Quotient|Quot\.sound|^axiom ' "${TMPDIR:-/tmp}/mir-wrk0037-c2b-c3-b-primary-opaque-anchor.lean" && exit 1 || true`; `git diff --check`
Execution cut: `83cb649893cbf9bd1f175da8b40b4671d23c79b1` is the authority/input snapshot. Run every outcome command only after this registration is committed and pushed. The evidence commit may add only `plan/wrk-0037-c2b-c3-b-primary-opaque-anchor-experiment.md`, a direct numbered report, declared control files, and no helper, schema, validator, CI/Make surface, parser, checker, theory, contract, runtime, sample, or public artifact. The fenced Lean source is materialized only to a disposable temporary file. It is not a stable module, schema, data model, validator input, or downstream interface. A later metadata-only commit may append the exact evidence commit and artifact digest without rewriting this pre-registration.
Non-claims: This does not select A, B, or C; define any Mir request/attempt/occurrence identity, equality, or correlation; add a Core constructor/judgment, Config/history/SaveObject/IR/queue/wire field, reply/receipt/failure occurrence kind, authority relation, restore law, source grammar/elaboration/omission rule, delivery/retry/fairness/timeout/cancellation behavior, implementation, API, contract, THM/OBL, SCN, Gate, Phase, conformance, lifecycle, or public claim. It is finite L3 evidence only, not a proof, family adequacy result, or implementation readiness.

## Results and review

Reliance status: not-promoted
Positive evidence: Every registered outcome command completed after the
registration push. At the pinned authority/input cut, the fenced finite source
proves distinct equal-incidental request atoms, no total left inverse from the
listed incidental record recovering both atoms, explicit staged records,
involutive local reindexing of every frontier/view, unique receipt and resume
extensions, rejected receipt/resume combinations, terminal failure exclusion
including local mutation, grounded dependency, and local non-authority. Lean
4.29 passed the extracted source at `--trust=0`; `#print axioms` reports no
axioms for every retained theorem.
Negative evidence: No registered falsifier occurred. The pinned input digests
match the authority cut; the duplicate query found the expected
selection/registration/evidence references rather than a prior
statement-equivalent result; and the extracted source has no scanned
placeholder, unsafe, classical-choice, quotient, or axiom token. The retained
result is one finite table, not a general recovery, restore, or carrier result.
Evidence artifacts: LAB:plan/wrk-0037-c2b-c3-b-primary-opaque-anchor-experiment.md@99f468d6d5e415ed05f90b77c2b37956102fdc36:839ffda0e4c01fb1dab476598b97f658a8f85e27d8ce2547ab6a8c49e8662739
Evidence commits: 99f468d6d5e415ed05f90b77c2b37956102fdc36
Impact / non-effects: This record is normative only about the reversible L3 research boundary and its procedure. The finite pass establishes only that one explicitly bounded candidate did not trigger its registered falsifiers; it cannot make B a Canon carrier, define request identity, authorize a downstream design, or establish a recovery/persistence/inference rule.
Independent review: not-required-for-L3

## Supersession

Supersession: none
