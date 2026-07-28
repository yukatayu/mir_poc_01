---
id: working/WRK-0030
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, theory/01-mircore-v0, theory/05-authority, meta/proposal-012, meta/proposal-013]
summary: Plan 200 C2-A の source-owned request/authority/occurrence/replay wording を六つの WRK-local question label に source-tag して記録し、一 label の fact/comparison を別 label の答えとして扱わない documentary non-substitution を検査する。field partition、identity、binding、attempt、replay relation は定義・選択しない。
open_items: []
---

# WRK-0030 - C2-A source-tagged anti-collapse vocabulary

## Classification and authority cut

Standing eligibility: pass
Author: codex
Author fingerprint: not-required-for-L3
Canon anchors: adr/ADR-0014@74a276f4de2c62c6459482299d6d322ed3e11065:b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323, theory/01-mircore-v0@74a276f4de2c62c6459482299d6d322ed3e11065:35e2f52bebcf96332d8102e9110446930b7ff807d948737a5859909de34b0f12, theory/05-authority@74a276f4de2c62c6459482299d6d322ed3e11065:e06dc5ef0539eb5b87bce71b34d3e8d2ab0638603642e0d9f89581f29d25e6c4, meta/proposal-012@74a276f4de2c62c6459482299d6d322ed3e11065:09ea4d6957c320b4d0647806714a1643101c2022b2893ac76ec7de3bf1db73d5, meta/proposal-013@74a276f4de2c62c6459482299d6d322ed3e11065:4e0ecf7475f20eec85c09d50201d2d2cc29848d480e8382935fe489b43877213
LAB inputs: LAB:plan/199-selected-semantic-composition-and-inference-boundary.md@74a276f4de2c62c6459482299d6d322ed3e11065:14092df4d0c3b841915c59985032588ee56e6ea5322d472270325876a9000734, LAB:plan/200-reanchored-semantic-composition-research-plan.md@74a276f4de2c62c6459482299d6d322ed3e11065:eb2f17532f0b41f1bb875b6e053edb5a470a6bba91e657a794b073e93c33e942, LAB:plan/wrk-0028-r0-common-cut-fact-manifest.md@2b4a89801b3d30442426926d6aff96b1d709874a:23c7668615d35f8ee82c85db8f5e73f779badeb7db57f7b94990c63a3bc8e478
Permitted LAB locations: plan, docs/reports
Reserved surfaces: excluded

## Pre-registered working question

Question: At the pinned cut, can cited source spans be recorded under the
six WRK-local question labels `C2A.PAYLOAD_QUESTION`,
`C2A.VALIDATION_CLAIMS_QUESTION`, `C2A.BINDING_QUESTION`,
`C2A.REQUEST_UNIT_QUESTION`, `C2A.ATTEMPT_UNIT_QUESTION`, and
`C2A.REPLAY_CLASSIFICATION_QUESTION`, while each substantive row retains its
exact source, status, and source-owned subject? The sole retained conclusion
may be documentary non-substitution: placement of a fact or comparison under
one local question label does not answer a different local question label. No
label may denote a Canon object, carrier, field set, occurrence, predicate, or
relation, and no object-level equality, disequality, membership, function,
uniqueness, freshness, cardinality, causal, persistence, or classification
relation may be introduced.
Status quo: theory/01 displays Core request terms, queues, request emission,
and owner service wording. theory/05 distinguishes role claim from authority,
states post-admission message facts, and rejects copied/replayed capability
references. P012 records bounded value-flow and occurrence directions; P013
records bounded M1 validation-context direction. WRK-0028 already retains that
these sources do not select payload equality, request identity, binding carrier,
service-attempt identity, or replay policy. It does not provide a source-tagged
anti-substitution index for the distinct source-owned subjects.
Alternative: Retain no common local labels. Keep each source's terminology
separate and defer all cross-source question indexing until an owner-authorized
C2-B, C2-C, or C2-D semantic candidate exists.
Expected falsifier: Removing repeated WRK-0028 material leaves no independent
sense-separation result; a row requires deciding whether request components are
payload or claims; a local label must denote a Canon object or current field;
an entry requires equality/identity, an issuance/occurrence/queue/source-span
anchor, a functional/unique/persistent/authoritative binding, service-attempt
cardinality, or any replay/duplicate/retry/retransmission/restoration
classification. Freeze also if a role claim is collapsed with validation claims
or authority, V1 result binding is reused as C2 binding, copied/replayed
capability references are generalized to request replay, a causal edge becomes
an identity equation, P012/P013 detail is treated as a current Core/rule, the
cut/digest differs, or new helper/schema/validator/CI/Make surface, contract,
SCN, OBL, Gate, Phase, runtime, wire, or public claim is required.
Rollback / reopen trigger: On any falsifier set `Reliance status: frozen`,
retain only reproducible procedure evidence, and do not repair or rerun this
record. A later Canon cut requires a forward successor for a current-source
claim. Escalate rather than repair if work needs a field partition, semantic
request/attempt identity, binding/replay/persistence relation, Core/judgment,
source reconciliation, or any reserved surface.

## Method and evidence plan

Result class: literal-transcription
Commands: test ! -e plan/wrk-0030-c2a-source-tagged-anti-collapse-vocabulary.md; test -s mirrorea_canon/adr/ADR-0014.md && test -s mirrorea_canon/theory/01-mircore-v0.md && test -s mirrorea_canon/theory/05-authority.md && test -s mirrorea_canon/meta/proposals/PROPOSAL-012-mircore-value-flow-and-occurrence-identity.md && test -s mirrorea_canon/meta/proposals/PROPOSAL-013-post-admission-request-validation-context.md; sha256sum mirrorea_canon/adr/ADR-0014.md mirrorea_canon/theory/01-mircore-v0.md mirrorea_canon/theory/05-authority.md mirrorea_canon/meta/proposals/PROPOSAL-012-mircore-value-flow-and-occurrence-identity.md mirrorea_canon/meta/proposals/PROPOSAL-013-post-admission-request-validation-context.md; git diff --check
Execution cut: `74a276f4de2c62c6459482299d6d322ed3e11065` is the authority/input
snapshot. Run every outcome command only after this registration is committed
and pushed. The evidence commit may add only
`plan/wrk-0030-c2a-source-tagged-anti-collapse-vocabulary.md`, its
`plan/00-index.md` entry, a direct numbered report, allowed working-record
metadata/control files, and no helper, schema, validator, CI/Make surface,
parser, checker, theory, contract, runtime, or public artifact. The result is
ordinary Markdown, not a stable schema, data model, validator input, or
downstream interface. A later metadata-only commit may append the exact
evidence commit and artifact digest without rewriting this pre-registration.
Non-claims: This does not determine, choose, define, alter, reconcile, or close
a request field partition; structural, source, Core-AST, serialized, hash, or
observational equality; request/attempt/occurrence identity; an identity anchor;
binding relation; authority proof; attempt cardinality; replay/duplicate/retry/
acknowledgement/retransmission/idempotence policy; save/load/rollback behavior;
Core form/judgment; generated edge; occurrence/history schema; Diagnostic;
SCN; OBL/theory status; Gate/Phase; lifecycle; runtime; wire; serialization;
API; or public contract. It does not treat claims as authority, equal claims as
identity, same capref as same request, copied/replayed caprefs as request replay,
P012/P013 direction as current Core, or a missing source relation as a Canon
prohibition. It is not proof, conformance, implementation readiness, or a
machine-consumed artifact.

## Results and review

Reliance status: not-promoted
Positive evidence: The registered absence marker, five non-empty source checks,
five pinned SHA-256 checks, and `git diff --check` passed. LAB evidence retains
six source-tagged local questions and documentary non-substitution only.
Negative evidence: No registered falsifier occurred. The artifact introduces no
field partition, equality/identity, occurrence/queue anchor, binding relation,
attempt cardinality, or replay classifier.
Evidence artifacts: LAB:plan/wrk-0030-c2a-source-tagged-anti-collapse-vocabulary.md@8dcfc17a8a28adf507257cac791a08761dbfd5f6:bf27394c0b914c51987a34d6342181e93125c4fd2abc09b9d275dd820a409721
Evidence commits: 8dcfc17a8a28adf507257cac791a08761dbfd5f6
Impact / non-effects: The retained result is only a documentary index over
source-owned senses and the non-substitution of its local question labels. It
does not define a semantic vocabulary, carrier, equality, identity, binding,
attempt, or replay classifier; it does not alter any existing Canon rule.
Independent review: not-required-for-L3

## Supersession

Supersession: none
