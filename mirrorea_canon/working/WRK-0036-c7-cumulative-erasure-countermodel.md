---
id: working/WRK-0036
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, theory/03-elaboration, meta/proposal-012]
summary: C7 の concrete source rule を選ばず、二つの individually fiber-constant local erasure とその common coarsening が paired observation を失う固定有限 countermodel を検査する。choice、quotient、Mir carrier、source inference は除外する。
open_items: []
---

# WRK-0036 - C7 cumulative-erasure countermodel boundary

## Classification and authority cut

Standing eligibility: pass
Author: codex
Author fingerprint: not-required-for-L3
Canon anchors: adr/ADR-0014@1041505a5979591414ef29e4f850e9d6ab52f28a:b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323, theory/03-elaboration@a51ab57b2df121186029dfae09a8206cee1f6702:2d703895da4f75bf57848275db6ae03e0abe7d56f62a11ef364af8fe22677641, meta/proposal-012@fcf5ea613c2153667e1c4a887589fb939692c7a5:09ea4d6957c320b4d0647806714a1643101c2022b2893ac76ec7de3bf1db73d5
LAB inputs: LAB:plan/199-selected-semantic-composition-and-inference-boundary.md@0080b487643e1afab0f596bcfad4ccf822f0dfb4:dc1a054fba2aa9e065ec66553a0c5b8f94544152ae5d864609515cd9f38f9d97, LAB:plan/200-reanchored-semantic-composition-research-plan.md@0080b487643e1afab0f596bcfad4ccf822f0dfb4:248a6486ffc8c07884bfc3bf97c26b24e38fcee7c9f523e6719ad5ca3805c369, LAB:plan/204-wrk0034-semantic-composition-no-candidate-disposition.md@0080b487643e1afab0f596bcfad4ccf822f0dfb4:5303cb990d341ea5d06109e30c469bb59845e48cfd2314c885ded9d6b50f0402, LAB:plan/205-c7-parametric-factorization-candidate-selection.md@0080b487643e1afab0f596bcfad4ccf822f0dfb4:c85cb43c162d1509ee9de183b4b27a0b2ee83d7188a3acdb0f84861269a52bdf, LAB:plan/206-c7-cumulative-erasure-countermodel-candidate-selection.md@0080b487643e1afab0f596bcfad4ccf822f0dfb4:ce8800bff5c9d134f7f4bf3d10f2a971ca0003c5a8a44bacc68c70a84270e990, LAB:plan/wrk-0035-c7-parametric-factorization.md@0080b487643e1afab0f596bcfad4ccf822f0dfb4:8e27a94f876b9db33d6d30cc56b4569f83094b0cc4d17261bd680497327309a3, LAB:docs/reports/2494-c7-cumulative-erasure-candidate-selection.md@0080b487643e1afab0f596bcfad4ccf822f0dfb4:32455d73f0f55a38ad5cb63355ecf820ada61c19431395a3282fe528b53a58ca
Permitted LAB locations: plan, docs/reports
Reserved surfaces: excluded

## Pre-registered working question

Question: Can a fixed artifact-local finite model with a two-point `E`, two
distinct local two-point representation types `S_A` and `S_B`, and `Unit` as
`S_AB` show all of the following without imports, choice, quotients, or a
Mir-specific interpretation: `erase_A : E -> S_A` is fiber-constant for
`observe_A : E -> Bool`; `erase_B : E -> S_B` is fiber-constant for
`observe_B : E -> Bool`; `erase_AB : E -> Unit` is the common coarsening of
both local erasures; and `erase_AB` collides on the paired observation
`fun e => (observe_A e, observe_B e)`? May the sole retained conclusion be
that separately checked local erasures do not in general justify their
simultaneous common coarsening, so a future C7 matrix must check the cumulative
representation directly? Every local type, constructor, function, and theorem
is artifact-local mathematics only; none denotes a Mir source transformation,
source fact, elaborated artifact, source span, ground/provenance, Core object,
observation primitive, request, result, receipt, occurrence, authority,
failure, history, or contract.
Status quo: WRK-0035 retains a single-erasure equivalence between fiber
constancy and pointwise unique realized observation, plus an explicit single
collision and a full-codomain reconstruction warning. It contains no second
erasure, common-coarsening equation, or conclusion about independently checked
omissions. Plan 199 names a future C7 inference/desugaring matrix and
source-to-elaborated evidence equivalence; Plan 200's D12 rejects missing
full-observation differences. Neither document makes an actual omission rule
or defines a cumulative source representation. Plan 206's current-cut duplicate
search found no retained multi-erasure/common-coarsening countermodel.
Alternative: Retain no further theorem because a future C7 matrix may already
require direct checking of every cumulative representation. In that case the
finite model adds no validation decision and is cosmetic theorem churn rather
than a non-duplicate consumer.
Expected falsifier: A statement-equivalent multi-erasure/common-coarsening
countermodel exists at the pinned cut; Plan 199 cannot serve as an actual
matrix consumer; the candidate reduces to renaming WRK-0035's single
collision; the source requires an actual source/elaboration artifact, a
grounds/provenance relation, semantic dependency schema, source rule, or any
selected equality/identity relation; the fixed proof needs `Classical.choice`,
`Classical`, `Quotient`, `Quot.sound`, a decidable/finite interface beyond its
explicit local constructors, a new repository surface, or a nonreproducible
command; or an authority/input digest changes.
Rollback / reopen trigger: On any reproducible falsifier set `Reliance status:
frozen`, retain the procedure/falsifier in the declared LAB lane, and do not
repair this record or recast the countermodel as a source-rule result. A
changed input cut, generalized composition theorem, lattice/order law,
quotient, reconstruction function, actual source transformation, concrete
grounds/artifact model, or a C7 acceptance algorithm needs a forward successor.
Escalate rather than repair if work needs a source/elaboration or observation
contract, Core/judgment, semantic equality/identity, authority/failure/history
relation, SCN, OBL, Gate, Phase, runtime, public interface, helper, schema,
CI/Make surface, or evidence lane.

## Method and evidence plan

Result class: countermodel
Commands: Registration check, run before this record is created: `test ! -e plan/wrk-0036-c7-cumulative-erasure-countermodel.md`. Outcome commands, run only after this registration is committed and pushed: `test -s mirrorea_canon/adr/ADR-0014.md && test -s mirrorea_canon/theory/03-elaboration.md && test -s mirrorea_canon/meta/proposals/PROPOSAL-012-mircore-value-flow-and-occurrence-identity.md && test -s plan/199-selected-semantic-composition-and-inference-boundary.md && test -s plan/200-reanchored-semantic-composition-research-plan.md && test -s plan/204-wrk0034-semantic-composition-no-candidate-disposition.md && test -s plan/205-c7-parametric-factorization-candidate-selection.md && test -s plan/206-c7-cumulative-erasure-countermodel-candidate-selection.md && test -s plan/wrk-0035-c7-parametric-factorization.md && test -s docs/reports/2494-c7-cumulative-erasure-candidate-selection.md`; `sha256sum mirrorea_canon/adr/ADR-0014.md mirrorea_canon/theory/03-elaboration.md mirrorea_canon/meta/proposals/PROPOSAL-012-mircore-value-flow-and-occurrence-identity.md plan/199-selected-semantic-composition-and-inference-boundary.md plan/200-reanchored-semantic-composition-research-plan.md plan/204-wrk0034-semantic-composition-no-candidate-disposition.md plan/205-c7-parametric-factorization-candidate-selection.md plan/206-c7-cumulative-erasure-countermodel-candidate-selection.md plan/wrk-0035-c7-parametric-factorization.md docs/reports/2494-c7-cumulative-erasure-candidate-selection.md`; `rg -n -i -C 3 'eraseA|eraseB|eraseAB|two erasure|multiple erasure|cumulative erasure|common coarsen|mutual omission|joint omission|combined omission|simultaneous omission|coarsening|factorization composition' plan docs/reports mirrorea_canon/working mirrorea_canon/theory mirrorea_canon/spec mirrorea_canon/meta`; `awk 'BEGIN { in_block = 0 } /^```lean$/ { in_block = 1; next } in_block && /^```$/ { exit } in_block { print }' plan/wrk-0036-c7-cumulative-erasure-countermodel.md > "${TMPDIR:-/tmp}/mir-wrk0036-c7-cumulative-erasure-countermodel.lean" && lean --trust=0 "${TMPDIR:-/tmp}/mir-wrk0036-c7-cumulative-erasure-countermodel.lean"`; `rg -n 'sorry|admit|unsafe|partial|implemented_by|Classical|Choice|Quotient|Quot\.sound|^axiom ' "${TMPDIR:-/tmp}/mir-wrk0036-c7-cumulative-erasure-countermodel.lean" && exit 1 || true`; `git diff --check`
Execution cut: `0080b487643e1afab0f596bcfad4ccf822f0dfb4` is the authority/input snapshot.
Run every outcome command only after this registration is committed and pushed.
The evidence commit may add only
`plan/wrk-0036-c7-cumulative-erasure-countermodel.md`, its `plan/00-index.md`
entry, a direct numbered report, allowed working-record metadata/control files,
and no helper, schema, validator, CI/Make surface, parser, checker, theory,
contract, runtime, sample, or public artifact. The Lean source is one fenced
block in that ordinary Markdown artifact and is materialized only to a
disposable temporary file. It is not a stable schema, module, data model,
validator input, or downstream interface. A later metadata-only commit may
append the exact evidence commit and artifact digest without rewriting this
pre-registration.
Non-claims: This does not define, select, identify, instantiate, or authorize
a Mir source transformation, omission, desugaring, normalization, elaboration
rule, elaborated artifact, source span, ground/provenance, Core
constructor/judgment, observation primitive, request/result/receipt/attempt/
occurrence identity, equality/equivalence, Diagnostic, authority, failure,
history, persistence, transport, wire, public contract, general composition
law, cumulative acceptance algorithm, reconstruction function, quotient
carrier, computable or decidable reconstruction, SCN, OBL/theory status,
Gate/Phase/lifecycle, parser/checker/runtime behavior, implementation, API, or
public behavior. It is not proof, conformance, implementation readiness, or a
machine-consumed artifact.

## Results and review

Reliance status: not-promoted
Positive evidence: Every registered outcome command completed after the
registration push. At the pinned authority/input cut, the fenced source proves
two individual fiber-constancy results, both explicit common-coarsening
equations, a paired-observation collision after the common coarsening, and the
negated cumulative fiber-constancy predicate. Lean 4.29 passed the extracted
source at `--trust=0`. `#print axioms` reports no axioms for all six retained
theorems.
Negative evidence: No registered falsifier occurred. The pinned input digests
match the authority cut; the duplicate query found only the expected
selection/registration/evidence references rather than a prior
statement-equivalent result; and the extracted source has no scanned
placeholder, unsafe, classical-choice, quotient, or axiom token. The retained
result is a fixed countermodel and does not state a general composition law.
Evidence artifacts: LAB:plan/wrk-0036-c7-cumulative-erasure-countermodel.md@32de8b2a8a10d0df2e91587199d6ad608a918a19:21f7b1ab6dc5618d9ccb4050ad0358ffb3f428a146ad0f57aee78dfc04937687
Evidence commits: 32de8b2a8a10d0df2e91587199d6ad608a918a19
Impact / non-effects: This record is normative only about its reversible L3
research boundary and procedure. It cannot make individually checked source
facts jointly omittable or make C7 satisfied; it establishes no
semantic/operational contract.
Independent review: not-required-for-L3

## Supersession

Supersession: none
