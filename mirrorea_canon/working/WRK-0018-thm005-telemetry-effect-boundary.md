---
id: working/WRK-0018
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, theory/02-types-effects-failures, theory/07-observation, theory/11-metatheory-ledger, arch/02-boundary-contracts]
summary: THM-005 の observer-safe export に対し、既存 IFC Lean foundation 内の有限 toy telemetry source が modeled high state に依存すると low agreement から row equality が導けないことを検査する可逆 L3 record。telemetry semantics、label lattice、export ABI、OBL は変更しない。
open_items: []
---

# WRK-0018 - THM-005 telemetry-effect dependency boundary

## Classification and authority cut

Standing eligibility: pass
Author: codex
Author fingerprint: not-required-for-L3
Canon anchors: adr/ADR-0014@350a7545db5a23480f4bb5f86cca82ab34b9db55:b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323, theory/02-types-effects-failures@350a7545db5a23480f4bb5f86cca82ab34b9db55:40c49504e86162fb065d0f5850c4039d88d08af30da7d12dc2e073c43a107257, theory/07-observation@350a7545db5a23480f4bb5f86cca82ab34b9db55:3b0ed16c0506550e33f25f2d71839cef14e545fb9f51bd7a117e2a9b41f8d239, theory/11-metatheory-ledger@350a7545db5a23480f4bb5f86cca82ab34b9db55:0b423aa49c984386233ae30e958d0b9eda8a36ef6b93a5717ef577ee0455fbd1, arch/02-boundary-contracts@350a7545db5a23480f4bb5f86cca82ab34b9db55:b9ae8932c10e25d4506f8395a1bbd30aaed8d22f6fd2a293f1a0bed022e39cd3
LAB inputs: LAB:plan/177-thm005-telemetry-effect-boundary-selection.md@350a7545db5a23480f4bb5f86cca82ab34b9db55:a550bf79b6772537cace7d3d70ad27a7b9bd4026667c962ba60817391e617968, LAB:samples/lean/foundations/CurrentL2IfcSecretExamples.lean@350a7545db5a23480f4bb5f86cca82ab34b9db55:73920920677791a470725db58e6fa8f2f4c5c012a846c65ad3d6c88e9afe4aaf, LAB:samples/lean/foundations/CurrentL2IfcSecretExamples.md@350a7545db5a23480f4bb5f86cca82ab34b9db55:cbb07f8f75b38da38e7429104099900481c7848861eb5a1fbafd842c81a9ac97
Permitted LAB locations: plan, samples/lean
Reserved surfaces: excluded

## Pre-registered working question

Question: At the pinned LAB base, can Lean compile one marked `WRK0018TelemetryEffectModel` tail in the existing IFC foundation that defines only concrete `Nat`/`Bool` toy configurations and rows, then proves (a) a low-determined toy telemetry function preserves equality of model exports for two low-agreeing configurations, and (b) a fixed high-dependent toy telemetry function has a low-agreeing pair with unequal model exports? The tail is an experiment-local dependency test, not a declared Mir effect, a label rule, an ObservationEvent, or an observer-safe export implementation.
Status quo: theory/07 states both the THM-005 low-agreement policy and the occurrence-DAG-or-declared-telemetry-effect pipeline wording. BND-008 requires devtools rows to be H-derived. T-RESEARCH-013/023 leave the complete low-equivalence and occurrence/telemetry provenance relation unselected. Plan 177 selects the narrower question without choosing how those Canon statements are reconciled.
Alternative: The toy model may require a new label/effect/provenance/export interface, a canonical interpretation of declared telemetry, a generic carrier, an import, classical machinery, existing declassification machinery, a helper, or a change outside the marked tail. Such a result cannot establish the registered bounded dependency observation.
Expected falsifier: Lean cannot compile the marked tail; text before the marker or the companion explanation marker differs from the pinned SHA-256; the tail contains an import, axiom, admission, unsafe/partial declaration, `Classical`, choice, a generic type parameter, `SecurityLabel`, `Labeled`, `CanDeclassify`, `declassify`, or `flowsTo`; it omits the registered low-determined positive or fixed high-dependent adverse theorem; or retaining the result requires a new helper, schema, CI/Make surface, semantic rule, Canon change, or public claim.
Rollback / reopen trigger: On any falsifier, set Reliance status to frozen and retain only permitted failure evidence. Do not repair the tail, infer a telemetry rule, weaken THM-005, identify the toy export with a Canon row, reinterpret BND-008, add an effect/label/provenance/ABI, or move OBL-017/018, the ledger, Gate, Phase, conformance, runtime, or public status. Escalate only if a later question must choose one of those reserved interfaces.

## Method and evidence plan

Result class: existing-lane-experiment
Commands: python3 -c "from pathlib import Path; lean=Path('samples/lean/foundations/CurrentL2IfcSecretExamples.lean').read_text(); note=Path('samples/lean/foundations/CurrentL2IfcSecretExamples.md').read_text(); assert '/-! WRK-0018' not in lean and '## WRK-0018' not in note"; lean --version; lean --trust=0 samples/lean/foundations/CurrentL2IfcSecretExamples.lean; python3 -c "from pathlib import Path; import hashlib; lean=Path('samples/lean/foundations/CurrentL2IfcSecretExamples.lean').read_text(); note=Path('samples/lean/foundations/CurrentL2IfcSecretExamples.md').read_text(); marker='/-! WRK-0018'; note_marker='## WRK-0018'; assert lean.count(marker) == 1 and note.count(note_marker) == 1; prefix, tail=lean.split(marker, 1); note_prefix, note_tail=note.split(note_marker, 1); assert hashlib.sha256(prefix.encode()).hexdigest() == '73920920677791a470725db58e6fa8f2f4c5c012a846c65ad3d6c88e9afe4aaf'; assert hashlib.sha256(note_prefix.encode()).hexdigest() == 'cbb07f8f75b38da38e7429104099900481c7848861eb5a1fbafd842c81a9ac97'; required=('namespace WRK0018TelemetryEffectModel','low_determined_effect_respects_low_agreement','high_dependent_effect_has_fixed_adverse_pair'); forbidden=('import ','axiom','sorry','admit','unsafe','partial','implemented_by','Classical','classical','Choice','choice','universe ','{alpha','SecurityLabel','Labeled','CanDeclassify','declassify','flowsTo'); assert all(token in tail for token in required); assert not any(token in tail for token in forbidden); assert 'experiment-local' in note_tail"; python3 scripts/current_l2_lean_sample_sync.py; git diff --check; python3 scripts/validate_docs.py; python3 scripts/check_source_hierarchy.py; (cd mirrorea_canon && python3 meta/build-index.py --check)
Execution cut: `350a7545db5a23480f4bb5f86cca82ab34b9db55` is the authority/input snapshot. Execute the first outcome command only after this registration commit is committed and pushed. The evidence commit may add only the marked Lean tail, matching explanation, resulting existing Lean manifest if changed by the registered sync, allowed working-record metadata/control files, and a direct numbered report. It must append its exact full commit to `Evidence commits`; no result is retained merely by an unmanifested source edit.
Non-claims: This does not define or select a Canon configuration, low-equivalence, label order/lattice, declassification rule, typed effect, occurrence relation, telemetry semantics, devtools/export ABI, row identity/equality, redaction or retention behavior. It does not refute THM-005, reconcile theory/07 with BND-008, discharge OBL-017/018, alter `theory/11`, change Surface grammar or SCN-08, create a Core primitive, add a production helper/schema/CI/Make target, or affect implementation, Gate, Phase, conformance, runtime, or public completion.

## Results and review

Reliance status: not-promoted
Positive evidence: pending
Negative evidence: pending
Evidence artifacts: none
Evidence commits: none
Impact / non-effects: This registration fixes only the exact concrete toy carrier, low-determined positive control, high-dependent fixed adverse pair, existing lane, and freeze line. It records no Lean outcome and has no Canon, OBL, grammar, scenario, lifecycle, API, implementation, sample workflow, or public effect.
Independent review: not-required-for-L3

## Supersession

Supersession: none
