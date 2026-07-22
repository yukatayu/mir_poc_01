---
id: working/WRK-0016
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, theory/01-mircore-v0, theory/02-types-effects-failures, theory/11-metatheory-ledger]
summary: 既存 Lean foundation の二 constructor Capability に限り、任意 CaptureSet 間の captureSubset を明示的な局所消去で構成的に決定できるかを検査する可逆な L3 record。OBL、MirCore の decidability、checker、grammar、carrier、API は変更しない。
open_items: []
---

# WRK-0016 - Local predicate constructivity boundary

## Classification and authority cut

Standing eligibility: pass
Author: codex
Author fingerprint: not-required-for-L3
Canon anchors: adr/ADR-0014@e24f78d485f30932740d410f10526b0a1a8e9f33:b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323, theory/01-mircore-v0@e24f78d485f30932740d410f10526b0a1a8e9f33:35e2f52bebcf96332d8102e9110446930b7ff807d948737a5859909de34b0f12, theory/02-types-effects-failures@e24f78d485f30932740d410f10526b0a1a8e9f33:40c49504e86162fb065d0f5850c4039d88d08af30da7d12dc2e073c43a107257, theory/11-metatheory-ledger@e24f78d485f30932740d410f10526b0a1a8e9f33:0b423aa49c984386233ae30e958d0b9eda8a36ef6b93a5717ef577ee0455fbd1
LAB inputs: LAB:plan/173-local-predicate-constructive-decidability-selection.md@e24f78d485f30932740d410f10526b0a1a8e9f33:e581b01d67552de63894248e96642a4da37e7f6f46eed52605450352fb2f7bfc, LAB:samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean@e24f78d485f30932740d410f10526b0a1a8e9f33:dd3eeffcd5dc4ed0b90496f4b048941f6d3aede7e5142f07a8385de1581c1b64, LAB:samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.md@e24f78d485f30932740d410f10526b0a1a8e9f33:dd5905a24294f0b3394cfba483b3faf911a2216c6f9edac25fb17ff47c6d77a4
Permitted LAB locations: plan, samples/lean
Reserved surfaces: excluded

## Pre-registered working question

Question: At the pinned LAB base, can Lean construct a named non-instance `Decidable (captureSubset lhs rhs)` for arbitrary `lhs rhs : CaptureSet` in the exact `CurrentL2FiniteIndexFirstLayer` foundation, using only explicit elimination of the existing `Capability.roomHistory` and `Capability.ephemeralToken` constructors plus primitive Bool equality decisions? `outlives` and `remoteCallAllowed` receive named local decision terms only as positive controls. The source must not introduce a generic finite-carrier abstraction or a global instance.
Status quo: The exact LAB foundation already defines a two-constructor `Capability`, function-valued `CaptureSet`, `captureSubset`, `outlives`, and `remoteCallAllowed`, but it has no named all-input constructive decision term for `captureSubset`. Plan 173 selected this question because it is distinct from the recorded finite failure-row and OBL-005 evidence, while it remains a helper-local experiment rather than an OBL-003 or MirCore result.
Alternative: The proof may require a generic finite-domain interface, `Fintype` or `Finset`, `Classical` choice, a global `Decidable` instance, an imported helper, a new reusable abstraction, or a change to the predicate/API. The opaque-domain control may also show that closed-carrier elimination, rather than arbitrary function codomain decidability, is doing the work. Any of these conditions prevents the intended local constructivity reading.
Expected falsifier: Lean cannot compile the named `capture_subset_decidable_constructive` term; the term requires a forbidden generic carrier/classical/global-instance mechanism; the marked source tail contains a new definition, instance, import, axiom, admission, unsafe declaration, or reusable helper; or an opaque arbitrary-domain predicate receives the same all-input decision without an explicit finite interface. Any such result falsifies the selected boundary and freezes this record rather than expanding the language or helper surface.
Rollback / reopen trigger: On a reproducible falsifier, set Reliance status to frozen, retain only declared failure evidence in the permitted LAB locations, and reopen only with a narrower successor or a separately scoped owner/canon escalation. Do not add a Mir primitive, a generic finite-carrier library, a global instance, a checker rule, a theorem/OBL interpretation, or a repair to make the result pass. Escalate if a follow-up needs Core decidability, user-facing syntax, typeclass/API policy, a general finite collection, authority semantics, an OBL/Line-1 claim, a contract, Gate, Phase, conformance, runtime, or public claim.

## Method and evidence plan

Result class: existing-lane-experiment
Commands: python3 -c "from pathlib import Path; text=Path('samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean').read_text(); marker='/-! WRK-0016'; names=('outlives_decidable_control','capture_subset_decidable_constructive','remote_call_allowed_decidable_control'); assert marker not in text and not any(name in text for name in names)"; lean --version; lean --trust=0 samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean; python3 -c "from pathlib import Path; text=Path('samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean').read_text(); marker='/-! WRK-0016'; assert text.count(marker) == 1; tail=text.split(marker, 1)[1]; required=('outlives_decidable_control','capture_subset_decidable_constructive','remote_call_allowed_decidable_control'); forbidden=('import ','def ','instance ','classical','Classical','Fintype','Finset','Choice','choice','noncomputable','axiom','sorry','admit','unsafe','partial','implemented_by'); assert all(name in tail for name in required); assert not any(token in tail for token in forbidden)"; python3 -c "import pathlib, subprocess, tempfile; source='universe u\\nexample {alpha : Type u} (lhs rhs : alpha -> Bool) : Decidable (forall x, lhs x = true -> rhs x = true) := by\\n  infer_instance\\n'; path=tempfile.NamedTemporaryFile(suffix='.lean', delete=False); path.write(source.encode()); path.close(); result=subprocess.run(['lean','--trust=0',path.name], text=True, capture_output=True); pathlib.Path(path.name).unlink(); assert result.returncode != 0 and 'Decidable' in (result.stdout + result.stderr)"; git diff --check; python3 scripts/validate_docs.py; python3 scripts/check_source_hierarchy.py; (cd mirrorea_canon && python3 meta/build-index.py --check)
Execution cut: `e24f78d485f30932740d410f10526b0a1a8e9f33` is the authority/input snapshot. Execute the first outcome command only after this registration commit is committed and pushed. The evidence commit may add only the marked source tail and matching explanation in `samples/lean/foundations/`, allowed working-record metadata/control files, and a direct numbered report. It must append its exact full commit to `Evidence commits`; no result is retained merely by an unmanifested source edit.
Non-claims: This does not define or select a MirCore decidability rule, a generic finite carrier, a `Fintype`/`Finset` abstraction, a typeclass or global instance, a public library/API, user syntax, type checking, elaboration, checker behavior, runtime behavior, authority/capability semantics, transport, authentication, contract, diagnostic, Core carrier, grammar, effect, scheduler, or standard I/O primitive. It does not prove, discharge, reinterpret, or move OBL-003 or any other OBL, Line-1, theorem, theory/11 status, Gate, Phase, conformance, sample-workflow, implementation, or public-completion status. It adds no production helper, schema, CI, Make target, or runtime behavior.

## Results and review

Reliance status: not-promoted
Positive evidence: pending
Negative evidence: pending
Evidence artifacts: none
Evidence commits: none
Impact / non-effects: This registration fixes only the experiment's exact inputs, local proof shape, controls, falsifiers, and stop line. It records no Lean outcome and has no Canon, OBL, lifecycle, API, implementation, or workflow effect.
Independent review: not-required-for-L3

## Supersession

Supersession: none
