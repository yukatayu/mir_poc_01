---
id: working/WRK-0034
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, meta/proposal-012]
summary: Plan 203 の固定 WRK-0033 presentation を変更せず、opaque LAB reply の有限列に対する translation-preservation と local-observation equality だけを conditional lemma として検査する。Mir trace、pending/request/occurrence identity、Core/history/runtime、source inference は定義・選択しない。
open_items: []
---

# WRK-0034 - V1/R1 finite-sequence presentation boundary

## Classification and authority cut

Standing eligibility: pass
Author: codex
Author fingerprint: not-required-for-L3
Canon anchors: adr/ADR-0014@1553bcc8fd140ad5ca98f5d7294fd802f776c7f1:b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323, meta/proposal-012@1553bcc8fd140ad5ca98f5d7294fd802f776c7f1:09ea4d6957c320b4d0647806714a1643101c2022b2893ac76ec7de3bf1db73d5
LAB inputs: LAB:plan/187-mircore-value-flow-and-occurrence-decision-packet.md@1553bcc8fd140ad5ca98f5d7294fd802f776c7f1:360e9da45be15a3bcf5f2f4a638af082cb85a1b1f115661f76bcc99cd6154575, LAB:plan/199-selected-semantic-composition-and-inference-boundary.md@1553bcc8fd140ad5ca98f5d7294fd802f776c7f1:28bc8ce37ab94c76e5b60a01d5a914c9a3e73ed819516b72d7d4d7953c4c13d1, LAB:plan/200-reanchored-semantic-composition-research-plan.md@1553bcc8fd140ad5ca98f5d7294fd802f776c7f1:c3532536469f5f0a2eaf3f3de02e36685c8dc7fbed411fc0914668bd6aff612f, LAB:plan/202-v1-r1-presentation-refinement-candidate-selection.md@1553bcc8fd140ad5ca98f5d7294fd802f776c7f1:3509613656d1618c03e6e9f6fc7d8bcdaa2ade87e0395c571d5302c9287f7e14, LAB:plan/wrk-0033-v1r1-presentation-refinement.md@1553bcc8fd140ad5ca98f5d7294fd802f776c7f1:6347a2b4603e485c3e040302fc69a54746a4aecf7c4180d597729688859fc4fd, LAB:plan/203-v1-r1-finite-sequence-candidate-selection.md@1553bcc8fd140ad5ca98f5d7294fd802f776c7f1:05fed931105a2ec6fac1b4d15db5a81da5e3305eb3edd2abe266a24a79088ba0, LAB:docs/reports/2483-wrk0034-finite-sequence-candidate-selection.md@1553bcc8fd140ad5ca98f5d7294fd802f776c7f1:6f80544e6344237d56e0168a7428067515718b49c32e2e7cc1411ba196685336
Permitted LAB locations: plan, docs/reports
Reserved surfaces: excluded

## Pre-registered working question

Question: At the pinned cut, keeping the 133-line WRK-0033 `AdminState`,
`MachineState`, `LabReply`, `adminStep`, `machineStep`, `toMachine`, local
observations, and matching/single-use/failure-exclusion assumptions exactly
unchanged, does the fixed translation commute with one step and consequently
preserve final local observation after an arbitrary finite `List.foldl` of
opaque LAB replies? The sole positive result may be those two conditional
lemmas. A reply list is mathematical input only; it must not denote a Mir
delivery trace, scheduler, history, queue, persistence mechanism, request,
attempt, occurrence, or semantic correlation identity.
Status quo: WRK-0033 retains finite local observation equality for every one
state/reply pair and three adverse distinctions when matching, single-use, or
failure exclusion is weakened. Plan 203 found no retained arbitrary-finite-list
preservation theorem at the pinned cut. P012 permits an explicit equivalent
machine presentation of V1 but requires a later design to settle the actual
carrier, pending control, correlation, failure, cut/save/load, and source
boundaries. Plan 187 requires full trace equivalence only for a real machine
presentation; the current finite LAB comparison is not that result.
Alternative: Retain no finite-sequence theorem. Treat the existing one-step
comparison as its complete bounded result and defer all further presentation or
source-inference work until an ordinary Canon design selects the missing
semantics.
Expected falsifier: An equivalent arbitrary-finite-list theorem already exists
at the pinned cut; direct translation preservation fails; the list result
requires changing any fixed state, reply, transition, translation, observation,
matching, single-use, or failure-exclusion clause; it requires multi-slot,
payload/provenance, authority, redaction, save/load, scheduler, transport,
history, reachability, request/attempt/occurrence identity, a Mir carrier, or
a new helper/schema/validator/CI/Make surface/evidence lane; or the result is
stated as trace equivalence, C3 completion, source inference, grammar,
implementation, conformance, or public behavior. Freeze also if an input digest
changes or an outcome command is not reproducible.
Rollback / reopen trigger: On any falsifier set `Reliance status: frozen`,
retain the reproducible procedure/falsifier in the existing LAB lane, and do
not repair this record or its fixed model. A changed source cut requires a
forward successor. Escalate rather than repair if work needs a semantic carrier,
source/elaboration rule, Core/judgment, history/persistence, authority/transport
contract, SCN, OBL, Gate, Phase, runtime, or public interface.

## Method and evidence plan

Result class: conditional-lemma
Commands: Registration check, run before this record is created: `test ! -e plan/wrk-0034-v1-r1-finite-sequence-refinement.md`. Outcome commands, run only after this registration is committed and pushed: `test -s mirrorea_canon/adr/ADR-0014.md && test -s mirrorea_canon/meta/proposals/PROPOSAL-012-mircore-value-flow-and-occurrence-identity.md && test -s plan/187-mircore-value-flow-and-occurrence-decision-packet.md && test -s plan/199-selected-semantic-composition-and-inference-boundary.md && test -s plan/200-reanchored-semantic-composition-research-plan.md && test -s plan/202-v1-r1-presentation-refinement-candidate-selection.md && test -s plan/wrk-0033-v1r1-presentation-refinement.md && test -s plan/203-v1-r1-finite-sequence-candidate-selection.md && test -s docs/reports/2483-wrk0034-finite-sequence-candidate-selection.md`; `sha256sum mirrorea_canon/adr/ADR-0014.md mirrorea_canon/meta/proposals/PROPOSAL-012-mircore-value-flow-and-occurrence-identity.md plan/187-mircore-value-flow-and-occurrence-decision-packet.md plan/199-selected-semantic-composition-and-inference-boundary.md plan/200-reanchored-semantic-composition-research-plan.md plan/202-v1-r1-presentation-refinement-candidate-selection.md plan/wrk-0033-v1r1-presentation-refinement.md plan/203-v1-r1-finite-sequence-candidate-selection.md docs/reports/2483-wrk0034-finite-sequence-candidate-selection.md`; `rg -n -C 3 'presentation_refinement|List\.foldl|finite[- ]sequence|arbitrary finite|trace equivalence|trace-equivalence' plan docs/reports mirrorea_canon/working`; `awk 'BEGIN { in_block = 0 } /^```lean$/ { in_block = 1; next } in_block && /^```$/ { exit } in_block { print }' plan/wrk-0034-v1-r1-finite-sequence-refinement.md > "${TMPDIR:-/tmp}/mir-wrk0034-v1r1-finite-sequence-refinement.lean" && lean --trust=0 "${TMPDIR:-/tmp}/mir-wrk0034-v1r1-finite-sequence-refinement.lean"`; `rg -n 'sorry|admit|axiom|unsafe|partial|implemented_by|Classical|Choice' "${TMPDIR:-/tmp}/mir-wrk0034-v1r1-finite-sequence-refinement.lean" && exit 1 || true`; `git diff --check`
Execution cut: `1553bcc8fd140ad5ca98f5d7294fd802f776c7f1` is the authority/input snapshot.
Run every outcome command only after this registration is committed and pushed.
The evidence commit may add only `plan/wrk-0034-v1-r1-finite-sequence-refinement.md`,
its `plan/00-index.md` entry, a direct numbered report, allowed working-record
metadata/control files, and no helper, schema, validator, CI/Make surface,
parser, checker, theory, contract, runtime, sample, or public artifact. The
Lean source is one fenced block in that ordinary Markdown artifact and is
materialized only to a disposable external temporary file. It is not a stable
schema, module, data model, validator input, or downstream interface. A later
metadata-only commit may append the exact evidence commit and artifact digest
without rewriting this pre-registration.
Non-claims: This does not determine, choose, define, alter, reconcile, or close
a Mir trace, pending object, request/reply/receipt/attempt/occurrence identity,
correlation relation, result type/payload/provenance, `Delta`/`Gamma` carrier,
continuation/evaluation-context form, source syntax/elaboration or inference,
queue, scheduler, failure family, persistence/save/load/rollback behavior,
authority/redaction/transport/wire behavior, Core form/judgment, history/
causal edge/DAG mapping, Diagnostic, SCN, OBL/theory status, Gate/Phase/
lifecycle, parser/checker/runtime behavior, API, or public contract. It does
not treat a finite list result as full trace equivalence or as an omitted source
fact. It is not proof, conformance, implementation readiness, or a
machine-consumed artifact.

## Results and review

Reliance status: not-promoted
Positive evidence: Every registered outcome command passed at the pinned cut.
The 182-line finite Lean source preserves `toMachine` across one fixed step and
then across every finite `List.foldl` of the same opaque reply labels; the
fixed local observations agree after the two runs. Its first 133 lines are
byte-identical to WRK-0033. Lean 4.29.1 passed at `--trust=0` with no output.
Negative evidence: The RED draft failed exactly because `rfl` cannot reduce
arbitrary opaque state/reply inputs; finite case analysis is required. This was
a proof-authoring check, not a semantic falsifier. No registered falsifier
occurred. The input digests match; no semantic carrier, source form, helper,
schema, validator, CI/Make surface, evidence lane, or reserved surface was
introduced. The result is not generalized beyond the fixed finite assumptions.
Evidence artifacts: LAB:plan/wrk-0034-v1-r1-finite-sequence-refinement.md@dc66f08237acd11e4de722cd67a42fae0b26e1eb:0e3eb3513f39afb241f796248737fc4a9f66665986fd32e143503991a71b820b
Evidence commits: dc66f08237acd11e4de722cd67a42fae0b26e1eb
Impact / non-effects: This record is normative only about its reversible,
fixed-model research boundary and procedure. It establishes neither a Mir
trace nor V1/R1's eventual presentation, source inference, or any
semantic/operational contract.
Independent review: not-required-for-L3

## Supersession

Supersession: none
