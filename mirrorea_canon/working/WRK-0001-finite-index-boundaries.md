---
id: working/WRK-0001
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, theory/02-types-effects-failures]
summary: theory/02 が許す有限 index 断片と CurrentL2FiniteIndexFirstLayer.lean の helper-local 正例・拒否例を再現する L3 pilot。最終型体系、Mir primitive、OBL 状態は変更しない。
open_items: []
---

# WRK-0001 - finite index boundaries

## Classification and authority cut

Standing eligibility: pass
Author: codex
Author fingerprint: not-required-for-L3
Canon anchors: theory/02-types-effects-failures@032a0ac22964b053aa0af29b0ab1a928de88d423:40c49504e86162fb065d0f5850c4039d88d08af30da7d12dc2e073c43a107257
LAB inputs: LAB:samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean@032a0ac22964b053aa0af29b0ab1a928de88d423:dd3eeffcd5dc4ed0b90496f4b048941f6d3aede7e5142f07a8385de1581c1b64, LAB:samples/lean/manifest.json@032a0ac22964b053aa0af29b0ab1a928de88d423:30cb502777060e23092fd159a9ea1671b192b93c726e9bc19bc78a96233b09ae
Permitted LAB locations: plan, samples/lean
Reserved surfaces: excluded

## Pre-registered working question

Question: Does the existing `CurrentL2FiniteIndexFirstLayer.lean` reproducibly express only the three finite index shapes already permitted by theory/02 - a two-point lifetime preorder, finite capture-set inclusion, and a simple numeric remote-call bound - while compiling both its positive and rejecting helper-local lemmas?
Status quo: theory/02 permits those finite index families, while the existing Lean file is classified as a helper-local, non-production proof fragment rather than a final typed calculus or canonical theorem.
Alternative: the file may fail to compile, contain an inadmissible placeholder or additional semantic commitment, or fail to provide the stated concrete rejection cases; then it is not suitable as evidence for this bounded reproduction.
Expected falsifier: Lean fails to check the file, the source contains a proof placeholder or forbidden implementation escape, or its named negative lemmas for step/session lifetime, capture inclusion, or zero budget are absent.
Rollback / reopen trigger: On any falsifier, set Reliance status to frozen, retain the failure evidence, and either supersede with a narrower reproduction or escalate if reconciling it would require a canon or helper-surface decision.

## Method and evidence plan

Result class: reproduction
Commands: lean --version; lean samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean; python3 -c "from pathlib import Path; text = Path('samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean').read_text(); required = ('step_does_not_outlive_session', 'ephemeral_only_not_subset_of_empty', 'room_history_only_not_subset_of_ephemeral_only', 'zero_budget_rejects_remote_call'); forbidden = ('sorry', 'admit', 'axiom', 'unsafe', 'partial', 'implemented_by'); assert all(name in text for name in required); assert not any(token in text for token in forbidden)"
Non-claims: This does not select a Mir lifetime, capture, capability, cost, or authority primitive; prove theory/02; establish a final typed calculus; validate runtime behavior; move theory/11; create conformance; or authorize L2 promotion.

## Results and review

Reliance status: not-promoted
Positive evidence: Lean 4.29.1 checks the existing finite-index helper-local fragment with exit status 0.
Negative evidence: The registered source audit finds the four named local rejection lemmas and no registered placeholder or implementation-escape token.
Evidence artifacts: LAB:plan/wrk-0001-finite-index-reproduction.md@887a0f6cd2de57443f4508c14fbadf4a88f25992:5155ce3de994cc92975a797a2d7ee1b7b79453ff6739a125546c26f000d23972
Evidence commits: 887a0f6cd2de57443f4508c14fbadf4a88f25992
Impact / non-effects: The record reads theory/02 and existing LAB artifacts only. It introduces no helper, schema, CI/Make target, source/runtime API, contract, Gate/Phase action, proof-status change, or public claim. Terminology clarification: the lifetime and capture carriers are finite, while the remote-call parameter uses `Nat`; "finite-index" names this bounded fragment and does not claim finite cardinality for every index.
Independent review: not-required-for-L3

## Supersession

Supersession: none
