---
id: working/WRK-0044
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, meta/proposal-012, meta/proposal-013, meta/proposal-017]
summary: P017 X1 の V1/R1 cross-locus read に限り、external rejection と no-observation を候補仮説とする使い捨て relation-envelope presentation が、全 preregistered integration row を reserved surface なしに条件付きで検査できるかを既存 Lean LAB lane で調べる。Canon schema、transition、identity、proof、runtime は選ばない。
open_items: []
---

# WRK-0044 - P017 X1 minimum relation-envelope coherence

## Classification and authority cut

Standing eligibility: pass

This record reads the pinned Canon cut without changing it; pre-registers one
scope-minimal, disposable `existing-lane-experiment` in the existing
`plan/` Markdown-held Lean lane; names its candidate hypotheses, alternative,
falsifiers, non-effects, and rollback first; and introduces no helper family,
schema, CI/Make surface, evidence lane, or public interface. P017, rather than
the LAB plans, supplies the bounded authorization to define and falsify a
minimum X1 model.

Author: codex

Author fingerprint: not-required-for-L3

Canon anchors: adr/ADR-0014@b15eb514c1f2c9223c35336e3f398d94ff06bd1b:b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323, working/readme@b15eb514c1f2c9223c35336e3f398d94ff06bd1b:5a741218ca6d3a571db6686293401c417693338c379f5ac7aa5708532e599ebf, meta/proposal-012@b15eb514c1f2c9223c35336e3f398d94ff06bd1b:09ea4d6957c320b4d0647806714a1643101c2022b2893ac76ec7de3bf1db73d5, meta/proposal-013@b15eb514c1f2c9223c35336e3f398d94ff06bd1b:4e0ecf7475f20eec85c09d50201d2d2cc29848d480e8382935fe489b43877213, meta/proposal-017@b15eb514c1f2c9223c35336e3f398d94ff06bd1b:65f847f3d57cbbc5dd1f86540964fd5d9a7b6e3fcf13387c2776a08edf8254e3, theory/01-mircore-v0@b15eb514c1f2c9223c35336e3f398d94ff06bd1b:35e2f52bebcf96332d8102e9110446930b7ff807d948737a5859909de34b0f12, theory/02-types-effects-failures@b15eb514c1f2c9223c35336e3f398d94ff06bd1b:40c49504e86162fb065d0f5850c4039d88d08af30da7d12dc2e073c43a107257, theory/03-elaboration@b15eb514c1f2c9223c35336e3f398d94ff06bd1b:2d703895da4f75bf57848275db6ae03e0abe7d56f62a11ef364af8fe22677641, theory/04-ordering-and-cuts@b15eb514c1f2c9223c35336e3f398d94ff06bd1b:70bde483330d3745a8694b15cd75f447b6610513ae66cb1ad4ec1faed274a264, theory/05-authority@b15eb514c1f2c9223c35336e3f398d94ff06bd1b:e06dc5ef0539eb5b87bce71b34d3e8d2ab0638603642e0d9f89581f29d25e6c4, theory/07-observation@b15eb514c1f2c9223c35336e3f398d94ff06bd1b:3b0ed16c0506550e33f25f2d71839cef14e545fb9f51bd7a117e2a9b41f8d239, spec/04-core-ir@b15eb514c1f2c9223c35336e3f398d94ff06bd1b:50c23acf01deedbe5bdb78baeba58053e28c940d8202b6d25bfd1f03546fd950, spec/05-runtime-semantics@b15eb514c1f2c9223c35336e3f398d94ff06bd1b:25749e3b171659fa59e3de6ff49126e15331ef52cf3ba5337ece4c46e72ca06c

LAB inputs: LAB:plan/225-post-wrk0043-fixture-frontier-disposition.md@b15eb514c1f2c9223c35336e3f398d94ff06bd1b:86918290a88915de32179067e4225d5762b05747ffb5921f2839db26314d19e6, LAB:plan/226-post-wrk0043-cross-lane-p0a-preflight.md@b15eb514c1f2c9223c35336e3f398d94ff06bd1b:183f6e4d1a716ae5bd0c0e1c862982c46da14083acc0941f021f7b2ab33c247a, LAB:plan/227-p017-x1-decision-vector-and-choice-neutral-consistency.md@b15eb514c1f2c9223c35336e3f398d94ff06bd1b:9a2c15591ccfd36f6c7258f1373025d011f2685d7a33ed818b8b63398a533d65, LAB:plan/228-p017-x1-minimum-coherence-candidate-selection.md@b15eb514c1f2c9223c35336e3f398d94ff06bd1b:fc4db371904ca945afded4f8287c70a8571e5ddbdb5fa1ddef79dfaf3ade4e2d

Permitted LAB locations: plan, docs/reports

Reserved surfaces: excluded

This record does not define or amend a Canon or reusable shared relation schema,
request identity/equality, Core, Config, SaveObject, transition, occurrence
kind, causal generator, authority/ownership/effect/failure/judgment primitive,
validation algorithm, dynamic failure member/row, source grammar/elaboration,
runtime, adapter, transport, serialization, provider, artifact, compatibility,
wire/API, observation/export surface, theorem/OBL, scenario, conformance,
Gate, Phase, lifecycle, production implementation, or public behavior. Its
later disposable definitions, if any, are `D_K` only and are not a Canon
minimality, carrier, interface, or lifecycle claim.

## Pre-registered working question

Question: At one pinned **Git/document authority-and-evidence cut**, can one
scope-minimal, disposable candidate-local presentation of the P017 X1
relation-state family for V1/R1 cross-locus reads give every preregistered row
below a bounded conditional argument, countermodel, or explicit stop without
an unlisted assumption, vacuous discharge, hidden semantic residence, or
reserved dependency?

Status quo: P017 X1 fixes an explicit relation-valued exchange family with a
dynamic domain and unique pending binding, but adopts no carrier schema,
lifecycle, transition, occurrence presentation, failure member, restore
identity/function, observer projection, source form, runtime, or public
contract. Plans 225--228 are LAB selection evidence only: the fixture-only
line is closed, the proposed restore-quantifier line is duplicate, and this
integrated candidate has been selected but not registered or executed. No
source or result exists at this authority-and-evidence cut.

Candidate-hypothesis ledger: `C` is exactly the pinned Canon anchors.
`H_rejection-external` is a LAB-only choice of P017's permitted treatment:
a rejected raw-delivery candidate is outside the semantic exchange transition
system and leaves only receipt-pending, owner-result availability, and the
accepted-consumption budget unchanged. It introduces no `RawReject`
semantic transition/occurrence, requester failure, adapter semantics, transport
event, or whole-machine stuttering claim. `H_nonvacuity` requires every
row that needs a distinguished request, success, accepted-unconsumed frontier,
consumed frontier, adverse authority case, or restore pair to make that
inhabitance/separation explicit rather than derive it from emptiness,
contradiction, an unreachable frontier, or a direct desired-result assumption.
`H_restore` may use only a candidate-local correspondence parameter; it
may not use definitional equality, a globally stable key, or unqualified
pre-load/restored occurrence equality. `D_K` may contain only disposable
definitions over `C + H_K`; finite keys, equality, totality, injectivity,
typeclass imports, lifecycle encodings, and helper semantics are forbidden
unless explicitly listed in `H_K` and bounded by a row-specific stop rule.

Row ledger and aggregate rule:

| Row | Required bounded account | Decisive falsifier / permitted result |
| --- | --- | --- |
| dynamic residence | dynamic relation domain, exactly one pending administrative binding per in-scope `q`, and no incidental-data merge | hidden evaluator/proof/queue/span/transport residence, shared effective state, or unlisted identity/key assumption; otherwise bounded conditional account or stop |
| M1 and authority | request-bound M1/provenance versus non-authoritative claims, adverse no-owner-mutation distinction | claims/provenance treated as authority, a validation algorithm/failure mapping required, or a named adverse distinction collapses; otherwise bounded conditional account or countermodel |
| branch and type | outstanding plus at-most-one terminal owner branch; typed owner result/failure stays distinct from receipt rejection | terminal overlap, untyped branch/failure, owner mutation on failure, or a failure member/row is required; otherwise bounded conditional account or stop |
| receipt and one-shot use | service/result/semantic receipt/acceptance/consumption stay distinct; external rejection has exactly `H_rejection-external`'s three limited non-effects | semantic raw rejection, receipt/use collapse, second acceptance/consumption, or a rejected-state persistence/failure row is required; otherwise bounded conditional account or countermodel |
| causal basis | every relied-on order maps only to an existing theory/04 generator; no new `H` node for administrative consumption | unnamed order, new generator, occurrence kind, causal cycle, or zero-or-one discipline breach; otherwise bounded mapping or stop |
| observation | no observer projection, export, untyped debug view, or storage-to-observation inference is introduced | any observer datatype/projection/export or storage-derived visibility is needed; otherwise explicit negative scope account only |
| save/load | each live required frontier has candidate-local closure/correspondence without merge, duplicate, reset, revalidation, stale resurrection, or cross-load equality | a Config/SaveObject change, restore function, pre/post equality, global identity, or forbidden restore behavior is required; otherwise bounded correspondence account or countermodel |
| source boundary | no source syntax, inferred correlation, elaboration extension, or new `G_e` row is used | any source form, span/payload/queue/history/transport inference, or `G_e` row is required; otherwise explicit stop-free negative scope account only |

A positive aggregate result is available only when every row has its stated
bounded result and no row depends on an unlisted hypothesis or reserved
surface. Any row countermodel, open row, or explicit reserved-surface stop
defeats the positive aggregate result. A passing aggregate means only
candidate-local conditional compatibility at this cut, never Canon
satisfaction, proof, readiness, or adoption.

Alternative: No such integrated presentation exists at this cut without a
required row remaining open, a P017 distinction collapsing, a vacuous or
unlisted assumption, or a reserved surface. That is a valid stop result, not a
reason to add a schema, transition, restore identity, failure member, source
form, or runtime behavior until the artifact appears to pass.

Expected falsifier: Any hidden relation residence; authority/provenance
collapse; terminal-branch overlap; result/receipt/acceptance/use collapse;
duplicate accepted consumption; semantic raw rejection; missing existing causal
basis; a new occurrence kind/generator; restore merge, duplication, reset,
revalidation, stale resurrection, or pre/post-load equality assumption;
observation/export/debug leak; inferred dynamic correlation; empty or
contradictory/vacuous discharge; required row left open; imported helper that
already encodes a result; or need for any ADR-0014 reserved boundary, P017
stop condition, liveness/fairness/retry/timeout/cancellation/freshness/global
exactly-once property, or OPEN-010 closure.

Rollback / reopen trigger: On any reproducible expected falsifier, set
`Reliance status: frozen`, retain the exact artifact and command evidence only
in the declared LAB locations, and record a forward successor or escalation
bundle. Do not repair the source by introducing a Canon/shared schema,
primitive, transition, identity, failure row, occurrence/generator, restore
function, observer surface, helper, CI/Make surface, or runtime. Reopen only
through a successor when the pinned source cut changes, an assumption must be
broadened, the permitted location is insufficient, or a reserved dependency is
discovered.

## Method and evidence plan

Result class: existing-lane-experiment

Commands: Registration checks before outcome evidence: confirm this record is
committed and pushed; confirm the exact pinned Canon/LAB blob digests; confirm
the registration commit changes only this WRK, direct numbered report, and
allowed operational metadata; and confirm no source exists at
`plan/wrk-0044-p017-x1-minimum-relation-envelope-coherence.md`. Outcome
commands, after registration only: extract the sole `lean` fenced block from
that exact Markdown path into a disposable `mktemp` file; run `lean --trust=0`
on it; print axioms for every retained declaration; audit the source and its
imports for `sorry`, `admit`, `unsafe`, `partial`,
`implemented_by`, `axiom`, `Classical`, `Choice`,
`Quotient`, `Quot.sound`, and `native_decide`; audit all
assumptions against the `C + H_K + D_K` ledger; assert each row's stated
falsifier/result and the aggregate rule; assert the no-observation and
external-rejection negative scope; enforce the evidence-commit allowlist; and
run `git diff --check`. The extracted Lean file, `.olean`, caches, and
command scratch output remain disposable and unretained.

Execution cut: `b15eb514c1f2c9223c35336e3f398d94ff06bd1b` is the authority and
input snapshot. No outcome command may run until this registration is committed
and pushed. The evidence commit may add only the exact expected `plan/` source,
a direct numbered report, and permitted operational metadata. The source may
contain one candidate-local relation-envelope presentation only; it is not a
stable module, data model, schema, validator input, runtime implementation, or
public interface. A later metadata-only commit may append the owning evidence
commit and exact artifact digest without rewriting this preregistration.

Non-claims: A positive result establishes neither a canonical minimality or
unique factorization, a positive X1 model, satisfiability/reachability of Mir,
delivery/fairness/termination, validation/fail-closed semantics, typed failure
semantics, owner-mutation behavior, semantic receipt transition, consumption
representation, restore identity/function, authority enforcement, observation
policy, Core/Config/SaveObject rule, theorem/OBL result, scenario conformance,
implementation readiness, runtime behavior, or public claim. Plan 227's
R/B/T/U/C/L labels are traceability axes only, never an imposed common record,
interface, predicate vocabulary, or lifecycle enumeration.

## Results and review

Reliance status: not-promoted

Positive evidence: At the pinned authority/input cut, the sole extracted
candidate-local source passed Lean 4.29.1 with `--trust=0`. It gives five
non-exhaustive pre-load/restored witness pairs, explicit local pending-binding
conditions, separate static frontier facts, and five opaque correspondence
facts. All eleven retained theorem reports had no axioms. This is only a static
conditional account: its witness separation, pending uniqueness/non-sharing,
frontier facts, one-shot premises, and correspondence facts remain explicit
premises, not derived global behavior.

Negative evidence: No registered typed falsifier occurred. The initial
unretained two-pair sketch was underpopulated for P017's five required
save/load frontiers and was not materialized. The retained five-pair source has
one Lean block, no imports, and clean scans for placeholders, unsafe/classical/
quotient/native facilities and the audited reserved-surface vocabulary. It
does not define a finite identity/key carrier, lifecycle, transition, restore
function, causal relation/generator, observer, source form, transport, runtime,
or `SaveObject` representation. The causal row remains a textual conditional
mapping to existing theory/04 generator families; it proves no order or
reachability.

Evidence artifacts: LAB:plan/wrk-0044-p017-x1-minimum-relation-envelope-coherence.md@8223e754b800121a13249b5640306ac268b188ac:83ca22f480970bb5f63884bcb330c8d67bd90f617ec380f64962f4aefda44867

Evidence commits: 8223e754b800121a13249b5640306ac268b188ac

Impact / non-effects: This record is normative only about a reversible L3
research boundary and procedure. The retained source changes no settled theory
or implementation surface and does not establish P017 satisfaction, a positive
X1 model, satisfiability/reachability, validation/fail-closed semantics, typed
failure semantics, owner-mutation behavior, a semantic receipt transition,
consumption over executions or restored continuations, authority enforcement,
observation policy, SaveObject/load closure, a Core/Config rule, theorem/OBL,
scenario conformance, implementation readiness, or public behavior.

Independent review: not-required-for-L3

## Supersession

Supersession: none
