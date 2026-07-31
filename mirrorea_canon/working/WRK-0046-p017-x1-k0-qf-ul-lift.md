---
id: working/WRK-0046
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, meta/proposal-012, meta/proposal-013, meta/proposal-017]
summary: P017 X1 の K0 external-rejection branch に限り、q-fibered disposable Spent predicate を一つの finite linear experimental lineage で restore-preservation と比較する source-free L3 preregistration。Canon state、identity、transition、persistence、proof、runtime は選ばない。
open_items: []
---

# WRK-0046 - P017 X1 K0 q-fiber U/L lifting

## Classification and authority cut

Standing eligibility: pass

P017's recorded X1 direction and ADR-0014 authorize reversible research in an
existing LAB lane. This record itself changes neither authority source nor
Canon. It pre-registers one source-free, finite, candidate-local U/L conditional
lemma. It is a separate candidate, not a repair, successor presentation, or
consumer of the frozen WRK-0045 predicate-only A-Sigma result.

Author: codex

Author fingerprint: not-required-for-L3

Canon anchors: adr/ADR-0014@246f2b1cbafe072ed4a3a11d202609c3401eb6ad:b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323, working/readme@246f2b1cbafe072ed4a3a11d202609c3401eb6ad:5a741218ca6d3a571db6686293401c417693338c379f5ac7aa5708532e599ebf, working/WRK-0045@246f2b1cbafe072ed4a3a11d202609c3401eb6ad:6268e4ccfc3a61b5c4027d65b3c5e0985223739d8861d10e66a80854bf772577, meta/proposal-012@246f2b1cbafe072ed4a3a11d202609c3401eb6ad:09ea4d6957c320b4d0647806714a1643101c2022b2893ac76ec7de3bf1db73d5, meta/proposal-013@246f2b1cbafe072ed4a3a11d202609c3401eb6ad:4e0ecf7475f20eec85c09d50201d2d2cc29848d480e8382935fe489b43877213, meta/proposal-017@246f2b1cbafe072ed4a3a11d202609c3401eb6ad:65f847f3d57cbbc5dd1f86540964fd5d9a7b6e3fcf13387c2776a08edf8254e3, theory/01-mircore-v0@246f2b1cbafe072ed4a3a11d202609c3401eb6ad:35e2f52bebcf96332d8102e9110446930b7ff807d948737a5859909de34b0f12, theory/02-types-effects-failures@246f2b1cbafe072ed4a3a11d202609c3401eb6ad:40c49504e86162fb065d0f5850c4039d88d08af30da7d12dc2e073c43a107257, theory/04-ordering-and-cuts@246f2b1cbafe072ed4a3a11d202609c3401eb6ad:70bde483330d3745a8694b15cd75f447b6610513ae66cb1ad4ec1faed274a264, theory/05-authority@246f2b1cbafe072ed4a3a11d202609c3401eb6ad:e06dc5ef0539eb5b87bce71b34d3e8d2ab0638603642e0d9f89581f29d25e6c4, theory/07-observation@246f2b1cbafe072ed4a3a11d202609c3401eb6ad:3b0ed16c0506550e33f25f2d71839cef14e545fb9f51bd7a117e2a9b41f8d239

LAB inputs: LAB:plan/227-p017-x1-decision-vector-and-choice-neutral-consistency.md@246f2b1cbafe072ed4a3a11d202609c3401eb6ad:9a2c15591ccfd36f6c7258f1373025d011f2685d7a33ed818b8b63398a533d65, LAB:plan/230-p017-x1-first-ordinary-design-card-preflight.md@246f2b1cbafe072ed4a3a11d202609c3401eb6ad:6c0408002e6c273eacc26dd51878ced49511ae8ac4de5b3f4b488547c1630906, LAB:plan/231-k0-rl-factorization-preflight.md@246f2b1cbafe072ed4a3a11d202609c3401eb6ad:e36348ef753f53136f7e82392cffe0c9676bcd21a23d38f078b2ace6fd18eabd, LAB:plan/245-post-wrk0045-no-successor-ordinary-x1-handoff.md@246f2b1cbafe072ed4a3a11d202609c3401eb6ad:73c9c48b4ffa393e86af29ee767b1b93d5bd81017d14df356c3d4a991528a8da, LAB:docs/reports/2570-post-wrk0045-k0-ul-candidate-re-screen.md@246f2b1cbafe072ed4a3a11d202609c3401eb6ad:48eb1f2e6b2129c4fbf72ea78ffc8f9686c0ec6863c4aa8f060dc783497d54d3

Permitted LAB locations: plan, docs/reports

Reserved surfaces: excluded

The terms `state`, `restore`, `consume`, `AcceptedSuccess`, and `Spent` below
are experiment-local opaque carriers or predicates. They do not define or amend
Canon `Config`, `SaveObject`, history, identity/equality, relation schema,
transition, occurrence, causal generator, authority, ownership, effect,
failure, judgment, `Gamma`/`Delta`, source/elaboration form, runtime, adapter,
transport, serialization, observation/export, theorem/OBL, scenario,
conformance, Gate, Phase, lifecycle, production implementation, or public
behavior. No helper family, schema, CI/Make surface, evidence lane, or public
interface is introduced.

## Pre-registered working question

Question: For one finite linear sequence of candidate-local endpoint pairs
`(state_i, q_i)` with exactly one supplied four-endpoint restore-relation witness
between `(state_pre, q_pre)` and `(state_post, q_post)`, do opaque per-state
`AcceptedSuccess(state, q)`, disposable `Spent(state, q)`, guarded non-restore
consume edges, and explicit `Spent` preservation across every lineage edge imply
at most one counted consume edge in that sequence?

Status quo: P017 X1 requires later ordinary work to address accepted restricted
use and no reset/re-enable after restore, but selects neither consumption
representation nor a restore mechanism. Plan 230 leaves `H_K0-U` and `H_K0-L`
open; Plan 231 leaves the final primitive-versus-uniquely-derived classification
open. WRK-0045 is frozen for a different branch-to-binding premise and does not
derive this finite mixed-lineage property. No file exists yet at
`plan/wrk-0046-p017-x1-k0-qf-ul-lift.md`.

Candidate-hypothesis ledger: `C` is exactly the pinned Canon anchors. `H_accept`
is an uninterpreted candidate-local premise `AcceptedSuccess(state, q)` at each
visited endpoint; it does not generate, preserve, or type receipt, matching,
acceptance, result availability, authority, or a transition. `H_spent` is the
disposable predicate `Spent(state, q)`, not semantic state, a stored field,
budget, key, branch identity, or final primitive. `H_line` supplies the finite
linear endpoint sequence and partitions its edges exclusively into non-restore
ordinary edges and one designated restore edge. Every counted consume is a
non-restore edge. `H_consume` requires `AcceptedSuccess` and `not Spent` at its
source and establishes `Spent` at its target. `H_ordinary-frame` preserves
`Spent` only for the non-restore ordinary edges in this supplied finite lineage.
`H_restore-edge` is one supplied four-endpoint relation witness between
`(state_pre, q_pre)` and `(state_post, q_post)` without asserting a map,
equality, key, functionality, injectivity, global identity, restore function,
actual load, or persistence representation. `H_restore-preservation` is the
A0-only premise that this supplied edge preserves `Spent`. `H_A0-nonvacuity`
supplies one consume, one restore, post-restore `AcceptedSuccess`, and preserved
`Spent`. `H_A1-adverse` instead supplies the exact A1 two-consume fixture after
the designated restore edge, with post-restore `AcceptedSuccess` and `not
Spent`; it is not an A0 premise.

`D_K` may name endpoint, edge, and counted-consume views over the declared
finite lineage only. It may not add a relation schema, closed-world fact,
identity, key, receipt, branch, field, storage format, state machine, or
theorem premise equivalent to at-most-one use.

Alternative: A0 retains `H_restore-edge`, `H_restore-preservation`, and
`H_A0-nonvacuity`, then attempts the stated conditional lemma. A1 retains the
same edge and every common premise but removes only `H_restore-preservation`; it
must satisfy `H_A1-adverse` with the pre-registered two-consume trace, including
post-restore `AcceptedSuccess` and `not Spent`. A1 is an omission/reset control,
not a final "derived" representation. A true uniquely-derived alternative needs
a separately pinned derivation and consumer.

Expected falsifier: Freeze the record if a two-consume trace satisfies all A0
premises, including restore preservation. Also freeze or escalate if the source
needs receipt semantics, branch/request identity or equality, a key, a restore
function, actual load/persistence, `Gamma`/`Delta`, an occurrence or causal
rule, Core, a helper/module/schema/CI surface, or an at-most-one/use-functionality
premise. Failure to produce the exact A1 adverse control is `DEFER`/freeze, but
is not the true A0 falsifier. Do not treat the expected A1 control as an A0
falsifier.

Rollback / reopen trigger: On any reproducible A0 falsifier or reserved-surface
dependency, set `Reliance status: frozen`, retain only the exact LAB artifact
and command evidence, and record `DEFER` or an ordinary Canon escalation. Do
not repair the record by adding a key, identity, receipt, storage, transition,
or global invariant. A forward record requires a materially distinct source
delta, consumer, and falsifier; WRK-0045 is neither rewritten nor superseded.

## Method and evidence plan

Result class: conditional-lemma

Commands: Before outcome evidence, confirm this record is committed and pushed;
verify every pinned Canon/LAB digest against this record's parent cut; verify the
registration commit changes only this WRK, `MAP.md`, `INDEX.json`, the allowed
current status metadata, and its direct numbered report; and confirm the future
source path remains absent. Only after registration may a later package place
one Markdown-held Lean block at
`plan/wrk-0046-p017-x1-k0-qf-ul-lift.md`, extract it to a disposable `mktemp`
file, run `lean --trust=0`, print axioms for retained declarations, scan the
source/imports for prohibited assumptions, run the A0 and A1 controls, and
perform the documented diff/allowlist checks. Scratch source, artifacts, caches,
and outputs remain disposable until separately retained as evidence.

Non-claims: This record establishes neither a P017 X1 model nor actual
admissible-load closure, every restored continuation, no merge/no duplicate,
totality, functionality, injectivity, global exactly-once, receipt semantics,
matching, authority, failure, `Gamma`/`Delta`, actual transition, occurrence,
causal order, Config/SaveObject rule, theorem/OBL status, scenario conformance,
Gate/Phase movement, implementation readiness, runtime behavior, or public
claim.

## Results and review

Reliance status: not-promoted

Positive evidence: not-run

Negative evidence: not-run

Evidence artifacts: none

Evidence commits: none

Impact / non-effects: This source-free registration retains only a reversible
question and no outcome evidence. It changes no settled theory or implementation
surface and cannot be used as an L2 position, proof, Gate/Phase movement, or
implementation authorization.

Independent review: not-required-for-L3

## Supersession

Supersession: none
