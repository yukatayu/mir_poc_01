---
id: working/WRK-0035
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, theory/03-elaboration, meta/proposal-012]
summary: C7 の concrete source rule を選ばず、任意の local erase/observe function に対する fiber constancy と range 上の pointwise unique observation の構成的 conditional lemma を検査する。choice、quotient、Mir carrier、source inference は除外する。
open_items: []
---

# WRK-0035 - C7 parametric factorization boundary

## Classification and authority cut

Standing eligibility: pass
Author: codex
Author fingerprint: not-required-for-L3
Canon anchors: adr/ADR-0014@1041505a5979591414ef29e4f850e9d6ab52f28a:b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323, theory/03-elaboration@a51ab57b2df121186029dfae09a8206cee1f6702:2d703895da4f75bf57848275db6ae03e0abe7d56f62a11ef364af8fe22677641, meta/proposal-012@fcf5ea613c2153667e1c4a887589fb939692c7a5:09ea4d6957c320b4d0647806714a1643101c2022b2893ac76ec7de3bf1db73d5
LAB inputs: LAB:plan/199-selected-semantic-composition-and-inference-boundary.md@281754e83f9f3b753a24ab116fdc4d9ad622d21e:7b3b3273c3eef031ffb94a27f554ce87bedf430400d5d1485d76f4c960352131, LAB:plan/204-wrk0034-semantic-composition-no-candidate-disposition.md@281754e83f9f3b753a24ab116fdc4d9ad622d21e:97956bd6d099d6fae52264c0990ae047f9dd66b02009735bc04bcab12dac4f24, LAB:plan/205-c7-parametric-factorization-candidate-selection.md@281754e83f9f3b753a24ab116fdc4d9ad622d21e:17527b7c8620d86f93af613f31a8d6a06c573b355783e17c3a70654056c5a1bb, LAB:docs/reports/2489-c7-parametric-factorization-candidate-selection.md@281754e83f9f3b753a24ab116fdc4d9ad622d21e:6be21f9ddaa3b41dd4fd382e56ae5cd71bc4749228bb15df35b3695208b9ad6b
Permitted LAB locations: plan, docs/reports
Reserved surfaces: excluded

## Pre-registered working question

Question: For arbitrary universe-polymorphic local types `E`, `S`, and `O`, and
uninterpreted local functions `erase : E -> S` and `observe : E -> O`, is
fiber constancy (`erase x = erase y -> observe x = observe y`) constructively
equivalent to the pointwise proposition that every `s` with a preimage under
`erase` has exactly one realized `O` observation? May an explicit collision
(`erase x = erase y` and `observe x != observe y`) refute both predicates, and
may one fixed `Unit`/`Bool`/`Bool` countermodel reject unique reconstruction by
a function over all of `S`? The sole positive result may be those pointwise
conditional lemmas. `E`, `S`, `O`, `erase`, and `observe` are artifact-local
mathematical parameters only; none denotes a Mir source, elaborator, artifact,
observation primitive, request, result, receipt, occurrence, authority,
failure, history, or contract.
Status quo: Plan 199 records C7's non-normative design constraint that an
omitted fact must be uniquely determined and retained with inspectable grounds
by an elaborated artifact. It supplies no factorization theorem, no source
omission rule, and no concrete mapping. Plan 204 closes only the fixed
WRK-0034 presentation line and Plan 205's pinned duplicate search found no
retained theorem with this parametric range-observation statement. WRK-0005 is
a fixed actual-outcome all-pairs relation and expressly not `ExistsUnique`;
WRK-0017 is a frozen warning that generic classical decision can hide
`Classical.choice`/`Quot.sound` dependencies.
Alternative: Treat the C7 wording as sufficient design guidance and retain no
Lean theorem. A generic statement may be a definitional restatement, may need
choice/quotient machinery, or may have no non-reserved consumer before a
concrete source/elaboration design exists.
Expected falsifier: A statement-equivalent theorem exists at the pinned cut;
the pointwise theorem needs `Classical.choice`, `Classical`, `Quotient`,
`Quot.sound`, decidable equality, finiteness, a Mir-specific assumption, or a
selected equality/identity relation; a local parameter must be interpreted as a
source/semantic contract; the proof establishes only a full-codomain function
claim; a declared command is not reproducible; or the source changes outside
the permitted LAB package. Freeze also if the input digest changes.
Rollback / reopen trigger: On any reproducible falsifier set `Reliance status:
frozen`, retain the procedure/falsifier in the declared LAB lane, and do not
repair this record or turn a pointwise theorem into a different one. A changed
input cut, global function package, quotient formulation, relation-valued
generalization, executable reconstruction, or concrete Mir instantiation needs
a forward successor. Escalate rather than repair if work needs a source rule,
source/elaboration or observation contract, semantic equivalence/identity,
authority/failure/history relation, Core/judgment, SCN, OBL, Gate, Phase,
runtime, public interface, helper, schema, CI/Make surface, or evidence lane.

## Method and evidence plan

Result class: conditional-lemma
Commands: Registration check, run before this record is created: `test ! -e plan/wrk-0035-c7-parametric-factorization.md`. Outcome commands, run only after this registration is committed and pushed: `test -s mirrorea_canon/adr/ADR-0014.md && test -s mirrorea_canon/theory/03-elaboration.md && test -s mirrorea_canon/meta/proposals/PROPOSAL-012-mircore-value-flow-and-occurrence-identity.md && test -s plan/199-selected-semantic-composition-and-inference-boundary.md && test -s plan/204-wrk0034-semantic-composition-no-candidate-disposition.md && test -s plan/205-c7-parametric-factorization-candidate-selection.md && test -s docs/reports/2489-c7-parametric-factorization-candidate-selection.md`; `sha256sum mirrorea_canon/adr/ADR-0014.md mirrorea_canon/theory/03-elaboration.md mirrorea_canon/meta/proposals/PROPOSAL-012-mircore-value-flow-and-occurrence-identity.md plan/199-selected-semantic-composition-and-inference-boundary.md plan/204-wrk0034-semantic-composition-no-candidate-disposition.md plan/205-c7-parametric-factorization-candidate-selection.md docs/reports/2489-c7-parametric-factorization-candidate-selection.md`; `rg -n -i -C 3 'FiberConstant|UniqueObsOnRange|UniqueReconstructible|NegativeWitness|factorization criterion|fiber-constancy|unique reconstruction over the range' plan docs/reports mirrorea_canon/working mirrorea_canon/theory mirrorea_canon/spec mirrorea_canon/meta`; `awk 'BEGIN { in_block = 0 } /^```lean$/ { in_block = 1; next } in_block && /^```$/ { exit } in_block { print }' plan/wrk-0035-c7-parametric-factorization.md > "${TMPDIR:-/tmp}/mir-wrk0035-c7-parametric-factorization.lean" && lean --trust=0 "${TMPDIR:-/tmp}/mir-wrk0035-c7-parametric-factorization.lean"`; `rg -n 'sorry|admit|unsafe|partial|implemented_by|Classical|Choice|Quotient|Quot\.sound|^axiom ' "${TMPDIR:-/tmp}/mir-wrk0035-c7-parametric-factorization.lean" && exit 1 || true`; `git diff --check`
Execution cut: `281754e83f9f3b753a24ab116fdc4d9ad622d21e` is the authority/input snapshot.
Run every outcome command only after this registration is committed and pushed.
The evidence commit may add only `plan/wrk-0035-c7-parametric-factorization.md`,
its `plan/00-index.md` entry, a direct numbered report, allowed working-record
metadata/control files, and no helper, schema, validator, CI/Make surface,
parser, checker, theory, contract, runtime, sample, or public artifact. The
Lean source is one fenced block in that ordinary Markdown artifact and is
materialized only to a disposable temporary file. It is not a stable schema,
module, data model, validator input, or downstream interface. A later
metadata-only commit may append the exact evidence commit and artifact digest
without rewriting this pre-registration.
Non-claims: This does not define, select, identify, instantiate, or authorize a
Mir source form, omission, desugaring, normalization, elaboration rule,
elaborated artifact, Core constructor/judgment, observation primitive,
request/result/receipt/attempt/occurrence identity, equality/equivalence,
Diagnostic, authority, failure, history, persistence, transport, wire, public
contract, global reconstruction function, quotient carrier, computable or
decidable reconstruction, provenance/grounds, SCN, OBL/theory status,
Gate/Phase/lifecycle, parser/checker/runtime behavior, implementation, API, or
public behavior. It is not proof, conformance, implementation readiness, or a
machine-consumed artifact.

## Results and review

Reliance status: not-promoted
Positive evidence: Every registered outcome command completed after the
registration push. At the pinned authority/input cut, the fenced source proves
the constructive equivalence of fiber constancy and pointwise unique realized
observation on `range erase`; an explicit collision refutes both predicates;
and the fixed `Unit`/`Bool`/`Bool` model rejects unique reconstruction by a
function on all of `S`. Lean 4.29 passed the extracted source at `--trust=0`.
`#print axioms` reports no axioms for the equivalence, both collision
refutations, and the full-codomain non-uniqueness theorem.
Negative evidence: No registered falsifier occurred. The pinned input digests
match the authority cut; the duplicate query found only the expected
selection/registration/evidence references rather than a prior
statement-equivalent result; and the extracted source has no scanned
placeholder, unsafe, classical-choice, quotient, or axiom token. The retained
result remains pointwise and does not construct a reconstructor.
Evidence artifacts: LAB:plan/wrk-0035-c7-parametric-factorization.md@e3bd47217365acbfe2d861de7e2377d06ba61d14:8e27a94f876b9db33d6d30cc56b4569f83094b0cc4d17261bd680497327309a3
Evidence commits: e3bd47217365acbfe2d861de7e2377d06ba61d14
Impact / non-effects: This record is normative only about its reversible L3
research boundary and procedure. It cannot make C7 satisfied or make a fact
omittable; it establishes no semantic/operational contract.
Independent review: not-required-for-L3

## Supersession

Supersession: none
