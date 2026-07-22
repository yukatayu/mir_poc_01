---
id: working/WRK-0017
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, theory/01-mircore-v0, theory/02-types-effects-failures, theory/11-metatheory-ledger]
summary: exact current-L2 Lean foundation の二 constructor Capability に限り、任意 CaptureSet 間の captureSubset に対する proposition-valued excluded-middle theorem を明示局所消去で構成できるかを検査する可逆なL3 record。WRK-0016のdata-valued declaration routeは再開せず、OBL、checker、Core、APIは変更しない。
open_items: []
---

# WRK-0017 - Local predicate proposition-decidability boundary

## Classification and authority cut

Standing eligibility: pass
Author: codex
Author fingerprint: not-required-for-L3
Canon anchors: adr/ADR-0014@3d530c6b4763f122d04f0d21c9c242b1b028c601:b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323, theory/01-mircore-v0@3d530c6b4763f122d04f0d21c9c242b1b028c601:35e2f52bebcf96332d8102e9110446930b7ff807d948737a5859909de34b0f12, theory/02-types-effects-failures@3d530c6b4763f122d04f0d21c9c242b1b028c601:40c49504e86162fb065d0f5850c4039d88d08af30da7d12dc2e073c43a107257, theory/11-metatheory-ledger@3d530c6b4763f122d04f0d21c9c242b1b028c601:0b423aa49c984386233ae30e958d0b9eda8a36ef6b93a5717ef577ee0455fbd1
LAB inputs: LAB:plan/174-local-predicate-proposition-decidability-selection.md@3d530c6b4763f122d04f0d21c9c242b1b028c601:a1c118468a23f793a7620a2c72318e51bed6cc343dda4c7982985390257b294c, LAB:samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean@3d530c6b4763f122d04f0d21c9c242b1b028c601:dd3eeffcd5dc4ed0b90496f4b048941f6d3aede7e5142f07a8385de1581c1b64, LAB:samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.md@3d530c6b4763f122d04f0d21c9c242b1b028c601:dd5905a24294f0b3394cfba483b3faf911a2216c6f9edac25fb17ff47c6d77a4
Permitted LAB locations: plan, samples/lean
Reserved surfaces: excluded

## Pre-registered working question

Question: At the pinned LAB base, can Lean compile one source-visible proposition-valued theorem named `capture_subset_excluded_middle_constructive` with exact target `captureSubset lhs rhs ∨ ¬ captureSubset lhs rhs` for arbitrary `lhs rhs : CaptureSet`, using only explicit elimination of `Capability.roomHistory` and `Capability.ephemeralToken` plus primitive Bool equality decisions? The theorem is not a `Decidable` value, instance, helper API, checker rule, or OBL result.
Status quo: WRK-0016 froze before its decision body was tested: a persistent source-visible theorem target of type `Decidable (...)` is not a proposition and would require an excluded data-valued declaration. Plan 174 selected the proposition target as a forward successor because it tests explicit body feasibility without reviving that declaration/persistence route. Existing capture positive/negative lemmas compile but are fixed examples, not all-input excluded-middle evidence.
Alternative: The proposition theorem may still require `Classical`, a generic finite carrier, `Fintype`, `Finset`, choice, an import, a value declaration, an instance, a new helper, a predicate change, or an unselected theorem/diagnostic/checker interface. The opaque generic-domain attempt may also compile or fail for an unrelated reason. In any such case the result cannot support the intended exact closed-carrier body reading.
Expected falsifier: Lean cannot compile the named theorem; its marked tail contains `def`, `abbrev`, `opaque`, `instance`, `example`, `axiom`, an import, an admission, unsafe declaration, `Classical`, choice, `Fintype`, `Finset`, a generic carrier/helper, or predicate change; the proof requires an excluded mechanism; or the opaque generic-domain theorem succeeds without an explicit finite interface. Any such result freezes this successor rather than changing the source, checker, language, or Canon.
Rollback / reopen trigger: On a reproducible falsifier, set Reliance status to frozen, retain only declared failure evidence in the permitted LAB locations, and reopen only with a narrower successor or separately scoped owner/canon escalation. Do not repair WRK-0016, add a data-valued declaration, turn the theorem into a `Decidable` value, introduce generic finite machinery, attach a diagnostic witness, or identify the result with OBL-003/Line-1. Escalate if a follow-up needs Core/checker semantics, grammar/API policy, public theorem interface, proof obligation placement, contract, Gate, Phase, conformance, runtime, or public claim.

## Method and evidence plan

Result class: existing-lane-experiment
Commands: python3 -c "from pathlib import Path; text=Path('samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean').read_text(); marker='/-! WRK-0017'; name='capture_subset_excluded_middle_constructive'; assert marker not in text and name not in text"; lean --version; lean --trust=0 samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean; python3 -c "from pathlib import Path; text=Path('samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean').read_text(); marker='/-! WRK-0017'; assert text.count(marker) == 1; tail=text.split(marker, 1)[1]; required=('capture_subset_excluded_middle_constructive','captureSubset lhs rhs ∨ ¬ captureSubset lhs rhs'); forbidden=('import ','def ','abbrev ','opaque ','instance ','example ','classical','Classical','Fintype','Finset','Choice','choice','noncomputable','axiom','sorry','admit','unsafe','partial','implemented_by'); assert all(token in tail for token in required); assert not any(token in tail for token in forbidden)"; python3 -c "import pathlib, subprocess, tempfile; source='universe u\\nexample {alpha : Type u} (lhs rhs : alpha -> Bool) : (forall x, lhs x = true -> rhs x = true) ∨ ¬ (forall x, lhs x = true -> rhs x = true) := by\\n  by_cases h : forall x, lhs x = true -> rhs x = true\\n  · exact Or.inl h\\n  · exact Or.inr h\\n'; path=tempfile.NamedTemporaryFile(suffix='.lean', delete=False); path.write(source.encode()); path.close(); result=subprocess.run(['lean','--trust=0',path.name], text=True, capture_output=True); pathlib.Path(path.name).unlink(); assert result.returncode != 0 and 'Decidable' in (result.stdout + result.stderr)"; git diff --check; python3 scripts/validate_docs.py; python3 scripts/check_source_hierarchy.py; (cd mirrorea_canon && python3 meta/build-index.py --check)
Execution cut: `3d530c6b4763f122d04f0d21c9c242b1b028c601` is the authority/input snapshot. Execute the first outcome command only after this registration commit is committed and pushed. The evidence commit may add only the marked proposition theorem tail and matching explanation in `samples/lean/foundations/`, allowed working-record metadata/control files, and a direct numbered report. It must append its exact full commit to `Evidence commits`; no result is retained merely by an unmanifested source edit.
Non-claims: This does not construct a `Decidable` value, define or select a global instance or data-valued declaration, establish a generic finite interface, or add a reusable value helper/API. It does not define or change a MirCore checker, OBL-003, Line-1, unified judgment, Core, grammar, type/effect/failure/capability semantics, diagnostics, witnesses, authority, transport, contract, runtime, scheduler, standard I/O primitive, Gate, Phase, conformance, workflow, implementation, or public completion. It adds no production helper, schema, CI, Make target, or runtime behavior.

## Results and review

Reliance status: not-promoted
Positive evidence: pending
Negative evidence: pending
Evidence artifacts: none
Evidence commits: none
Impact / non-effects: This registration fixes only the exact proposition target, closed-carrier explicit-elimination method, generic-domain adverse control, semantic declaration boundary, and stop line. It records no Lean outcome and has no Canon, OBL, lifecycle, API, implementation, or workflow effect.
Independent review: not-required-for-L3

## Supersession

Supersession: none
