---
id: working/WRK-0033
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, meta/proposal-012]
summary: Plan 202 の V1/R1 administrative binding と one-slot machine presentation を、opaque LAB correlation と explicit matching/single-use/failure assumptions の下で比較する conditional-lemma research。Mir pending/request/occurrence identity、Core/history/runtime は定義・選択しない。
open_items: []
---

# WRK-0033 - V1/R1 presentation-refinement boundary

## Classification and authority cut

Standing eligibility: pass
Author: codex
Author fingerprint: not-required-for-L3
Canon anchors: adr/ADR-0014@ddabd97bb3e13df51ede3ba00ead626600e1011a:b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323, meta/proposal-012@ddabd97bb3e13df51ede3ba00ead626600e1011a:09ea4d6957c320b4d0647806714a1643101c2022b2893ac76ec7de3bf1db73d5
LAB inputs: LAB:plan/187-mircore-value-flow-and-occurrence-decision-packet.md@ddabd97bb3e13df51ede3ba00ead626600e1011a:360e9da45be15a3bcf5f2f4a638af082cb85a1b1f115661f76bcc99cd6154575, LAB:plan/193-post-admission-validation-context-literature-and-counterexample-memo.md@ddabd97bb3e13df51ede3ba00ead626600e1011a:82e36cee61cc92311dc93b373c80182d43de4524684b07d8e6f78fd6c6cb94da, LAB:plan/199-selected-semantic-composition-and-inference-boundary.md@ddabd97bb3e13df51ede3ba00ead626600e1011a:e32df1fef4ee1b539c240f04ddb67ad61591b2610a13a1ba69c2e88d6b8de1e0, LAB:plan/200-reanchored-semantic-composition-research-plan.md@ddabd97bb3e13df51ede3ba00ead626600e1011a:9772919cfccf16f464f686674a4ea2557afd6499cc6d1c4af83b8d6f9cf2a750, LAB:plan/202-v1-r1-presentation-refinement-candidate-selection.md@ddabd97bb3e13df51ede3ba00ead626600e1011a:65167bbdd7794706651d61a11fff2f16e165ffb5fc378974eae7eee85ea2eb37, LAB:docs/reports/2478-v1-r1-presentation-refinement-selection.md@ddabd97bb3e13df51ede3ba00ead626600e1011a:80d63c8ec891753b42f392152191bb07da88215bbde230918904bf28a0d511cd
Permitted LAB locations: plan, docs/reports
Reserved surfaces: excluded

## Pre-registered working question

Question: At the pinned cut, can a finite LAB-only administrative-binding
presentation and a finite LAB-only one-slot machine presentation be compared
under all of these explicit assumptions: a waiting state has one opaque
`LAB$Correlation`; only a matching reply may resume it; that reply is consumed
at most once; success and terminal failure are disjoint; and save/load,
authority, redaction, multi-slot state, and source-elaboration are absent? The
sole positive result may be a conditional lemma that the two presentations have
the same waiting/success/failure classification under those assumptions. The
sole negative results may be finite counterexamples when matching, single-use,
or failure exclusion is weakened. `LAB$Correlation` must not denote a Mir
request, receipt, attempt, occurrence, queue item, identity, or wire token.
Status quo: P012 records V1 restricted result binding and R1 explicit typed
reply/receipt as directions. It permits an evaluation-frame or machine-state
presentation only as an explicit equivalent presentation of V1. Plan 187
records that such a comparison must preserve trace equivalence, local
decomposition, ownership/non-copying, success/failure resumption locus, and no
hidden communication. Current Canon and retained LAB do not select a pending
unit, correlation relation, result payload, failure family, persistence rule,
or source-elaboration correspondence.
Alternative: Do not retain a presentation comparison. Defer both presentations
until an ordinary Canon design selects the missing correlation, pending-control,
failure, persistence, and source-elaboration boundaries.
Expected falsifier: An identical pinned-cut conditional comparison is found; a
single model clause needs a Mir carrier, Core form/rule, request/attempt/
occurrence identity, result payload, pending queue, failure-family choice,
save/load schema, authority/redaction policy, source-elaboration relation, or
new helper/schema/validator/CI/Make surface/evidence lane; or the finite model
cannot show a classification difference after weakening matching, single-use,
or failure exclusion. Freeze also if a pinned input/digest changes or a result
is stated beyond the listed assumptions.
Rollback / reopen trigger: On any falsifier set `Reliance status: frozen`,
retain only reproducible procedure evidence, and do not repair or rerun this
record. A changed Canon/LAB cut requires a forward successor. Escalate rather
than repair if the work needs a semantic carrier, source syntax/elaboration,
Core/judgment, history/persistence, authority/transport contract, SCN, OBL,
Gate, Phase, runtime, or public interface.

## Method and evidence plan

Result class: conditional-lemma
Commands: Registration check, run before this record is created: `test ! -e plan/wrk-0033-v1r1-presentation-refinement.md`. Outcome commands, run only after this registration is committed and pushed: `test -s mirrorea_canon/adr/ADR-0014.md && test -s mirrorea_canon/meta/proposals/PROPOSAL-012-mircore-value-flow-and-occurrence-identity.md && test -s plan/187-mircore-value-flow-and-occurrence-decision-packet.md && test -s plan/193-post-admission-validation-context-literature-and-counterexample-memo.md && test -s plan/199-selected-semantic-composition-and-inference-boundary.md && test -s plan/200-reanchored-semantic-composition-research-plan.md && test -s plan/202-v1-r1-presentation-refinement-candidate-selection.md && test -s docs/reports/2478-v1-r1-presentation-refinement-selection.md`; `sha256sum mirrorea_canon/adr/ADR-0014.md mirrorea_canon/meta/proposals/PROPOSAL-012-mircore-value-flow-and-occurrence-identity.md plan/187-mircore-value-flow-and-occurrence-decision-packet.md plan/193-post-admission-validation-context-literature-and-counterexample-memo.md plan/199-selected-semantic-composition-and-inference-boundary.md plan/200-reanchored-semantic-composition-research-plan.md plan/202-v1-r1-presentation-refinement-candidate-selection.md docs/reports/2478-v1-r1-presentation-refinement-selection.md`; `rg -n -C 3 'restricted administrative binding|evaluation-frame|machine-state|matching receipt|one-shot|failure|resumption|no hidden communication' mirrorea_canon/meta/proposals/PROPOSAL-012-mircore-value-flow-and-occurrence-identity.md plan/187-mircore-value-flow-and-occurrence-decision-packet.md plan/202-v1-r1-presentation-refinement-candidate-selection.md`; `awk 'BEGIN { in_block = 0 } /^```lean$/ { in_block = 1; next } in_block && /^```$/ { exit } in_block { print }' plan/wrk-0033-v1r1-presentation-refinement.md > "${TMPDIR:-/tmp}/mir-wrk0033-v1r1-presentation-refinement.lean" && lean --trust=0 "${TMPDIR:-/tmp}/mir-wrk0033-v1r1-presentation-refinement.lean"`; `git diff --check`
Execution cut: `ddabd97bb3e13df51ede3ba00ead626600e1011a` is the authority/input
snapshot. Run every outcome command only after this registration is committed
and pushed. The evidence commit may add only
`plan/wrk-0033-v1r1-presentation-refinement.md`, its `plan/00-index.md` entry,
a direct numbered report, allowed working-record metadata/control files, and
no helper, schema, validator, CI/Make surface, parser, checker, theory,
contract, runtime, sample, or public artifact. The Lean source is a fenced
block in that ordinary Markdown evidence artifact and is materialized only to a
disposable external temporary file. It is not a stable schema, module, data
model, validator input, or downstream interface. A later metadata-only commit
may append the exact evidence commit and artifact digest without rewriting this
pre-registration.
Non-claims: This does not determine, choose, define, alter, reconcile, or close
a Mir pending object, request/reply/receipt/attempt/occurrence identity,
correlation relation, result type/payload/provenance, `Delta`/`Gamma` carrier,
continuation/evaluation-context form, source syntax/elaboration, queue,
scheduler, failure family, persistence/save/load/rollback behavior,
authority/redaction/transport/wire behavior, Core form/judgment, history/
causal edge/DAG mapping, Diagnostic, SCN, OBL/theory status, Gate/Phase/
lifecycle, parser/checker/runtime behavior, API, or public contract. It does
not treat same final classification as full trace equivalence, or a matching/
single-use/failure assumption as a fact that source syntax may omit. It is not
proof, conformance, implementation readiness, or a machine-consumed artifact.

## Results and review

Reliance status: not-promoted
Positive evidence: Every registered outcome command passed at the pinned cut.
The finite `presentation_refinement` theorem relates the six administrative
states and four reply labels to their one-slot machine presentations under the
registered assumptions. The one-shot and failure checks pass for both
presentations. The three adversarial checks distinguish swapped reply,
duplicate reply, and failure-then-success when matching, single-use, or
failure-exclusion is respectively weakened.
Negative evidence: No registered falsifier occurred. The source inputs and
SHA-256 digests match the authority cut; the source query did not require a
semantic carrier; Lean 4.29.1 passed the 133-line fenced source at `--trust=0`;
and no helper, schema, validator, CI/Make surface, evidence lane, or reserved
surface was introduced. The finite result is not generalized beyond its stated
assumptions.
Evidence artifacts: LAB:plan/wrk-0033-v1r1-presentation-refinement.md@37d2fd00a01aa5cf302f0293f0b6be51a337b217:6347a2b4603e485c3e040302fc69a54746a4aecf7c4180d597729688859fc4fd
Evidence commits: 37d2fd00a01aa5cf302f0293f0b6be51a337b217
Impact / non-effects: The retained result is only a finite conditional
presentation comparison and three finite adverse distinctions. It does not
select V1/R1 syntax, a pending/correlation/payload/failure/persistence carrier,
source elaboration, or any semantic/operational contract.
Independent review: not-required-for-L3

## Supersession

Supersession: none
