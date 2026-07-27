---
id: working/WRK-0027
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, spec/02-surface-grammar, spec/03-static-semantics, theory/01-mircore-v0, theory/06-existence-fallback, scenarios/SCN-08, meta/proposal-015]
summary: SCN-08 の scalar `room_anchor` と terminal `default_pose` について、表示済み Surface/Core/static source が明示的な宣言・解決対応を既に供給するかを literal comparison で検査する。scalar representation、grammar、Core、fallback policy は選ばない。
open_items: []
---

# WRK-0027 - SCN-08 scalar terminal correspondence

## Classification and authority cut

Standing eligibility: pass
Author: codex
Author fingerprint: not-required-for-L3
Canon anchors: adr/ADR-0014@5f194168a323e5465420e0735dbee6da81055af4:b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323, spec/02-surface-grammar@5f194168a323e5465420e0735dbee6da81055af4:7d97fb4e77f493c3e0be4dbffdee64f206936c1aca1d5c535c19195f81f592b1, spec/03-static-semantics@5f194168a323e5465420e0735dbee6da81055af4:1f708b65993bd3f3b9ae96cb3752f3bfc269b746514a35e459ae034fb124b634, theory/01-mircore-v0@5f194168a323e5465420e0735dbee6da81055af4:35e2f52bebcf96332d8102e9110446930b7ff807d948737a5859909de34b0f12, theory/06-existence-fallback@5f194168a323e5465420e0735dbee6da81055af4:3da20d43a0a87ec8417a4519700777adea141f499e2627f433927ce975a086c8, scenarios/SCN-08@5f194168a323e5465420e0735dbee6da81055af4:95fb9b7f4929657bb594c0f837885fdb992e39dd5fcf98c574cc3d5afa9addb1, meta/proposal-015@5f194168a323e5465420e0735dbee6da81055af4:e8b016be00bf4dd9bc8204451b7d72a871fc4fd29a88d7f4cdbb5090619f7745
LAB inputs: LAB:plan/199-selected-semantic-composition-and-inference-boundary.md@5f194168a323e5465420e0735dbee6da81055af4:2eef76ec041614d404e15078390e61cf961214f9e09a88369074650194b6d72c
Permitted LAB locations: plan, docs/reports
Reserved surfaces: excluded

## Pre-registered working question

Question: In the pinned displayed sources, do SCN-08's scalar declaration
`state room_anchor: Pose`, chain reference to `room_anchor`, and terminal
`default_pose` have a declared Surface/Core/static correspondence for scalar
state, initialization/default evidence, and chain-target resolution? Or does a
literal comparison establish only that the scenario and P015 require a later
explicit correspondence, while the displayed state declarations are indexed?
Status quo: SCN-08 uses participant-indexed `live_pose`, scalar `room_anchor`,
and terminal `default_pose`. The displayed Surface state production, static
state account, and MirCore declaration form each show an indexed state. The
fallback theory supplies lineage conditions for chain options. P015 records
that scalar terminal/default correspondence is required and unresolved, while
forbidding a hidden membership key, type-derived default, or unbound terminal.
Alternative: The pinned texts already define a scalar declaration/reference
scope, terminal/default declaration/resolution, or a conservative elaboration
that covers SCN-08 without the prohibited hidden forms.
Expected falsifier: Any pinned digest differs; a registered source marker is
absent; the literal comparison finds such an existing correspondence; or
retaining its result requires choosing a scalar representation, grammar/Core
amendment, initialization/default semantics, target-resolution policy,
diagnostic, OBL/theory/11 status, SCN/Gate/Phase change, helper/schema/CI, or
public contract.
Rollback / reopen trigger: On any falsifier set `Reliance status` to `frozen`,
retain only reproducible procedure evidence, and do not repair or rerun this
record. Escalate rather than repair if a follow-up chooses a scalar Core
declaration, finite-domain elaboration, terminal/default representation,
fallback or chain-target semantics, grammar/diagnostic, theorem/OBL status,
SCN/Gate/Phase, implementation, or public interface.

## Method and evidence plan

Result class: literal-transcription
Commands: test -s mirrorea_canon/spec/02-surface-grammar.md && test -s mirrorea_canon/spec/03-static-semantics.md && test -s mirrorea_canon/theory/01-mircore-v0.md && test -s mirrorea_canon/theory/06-existence-fallback.md && test -s mirrorea_canon/scenarios/SCN-08-avatar-fallback.md && test -s mirrorea_canon/meta/proposals/PROPOSAL-015-surface-fallback-and-return-closure.md; rg -n -F 'StateDecl     ::= "state" Ident "[" Ident ":" Keyspace "]" ":" Type' mirrorea_canon/spec/02-surface-grammar.md; rg -n -F 'state x[k:K]' mirrorea_canon/theory/01-mircore-v0.md; rg -n -F 'state room_anchor: Pose' mirrorea_canon/scenarios/SCN-08-avatar-fallback.md; rg -n -F 'default_pose' mirrorea_canon/scenarios/SCN-08-avatar-fallback.md mirrorea_canon/meta/proposals/PROPOSAL-015-surface-fallback-and-return-closure.md; git diff --check
Execution cut: `5f194168a323e5465420e0735dbee6da81055af4` is the authority/input snapshot. Execute every outcome command only after this registration is committed and pushed. The evidence commit may add only `plan/wrk-0027-scn08-scalar-terminal-correspondence.md`, its `plan/00-index.md` entry, a direct numbered report, allowed working-record metadata/control files, and no helper, schema, CI/Make, runtime, parser, checker, theory, contract, or public artifact. A later metadata-only commit may append the exact evidence commit and artifact digest without rewriting this pre-registration.
Non-claims: This does not select a scalar representation, change Surface
grammar/static semantics/MirCore, define initialization/default behavior,
resolve chain targets, alter fallback lineage, infer a value from a type, add a
hidden membership key, alter a diagnostic/failure row, discharge an OBL, or
change theory/11, SCN, Gate/Phase, conformance, runtime, transport, API, or
public behavior.

## Results and review

Reliance status: not-promoted
Positive evidence: After registration `dfbe31d3d2b75ebaab6182240e80769ff6e95048`
was pushed, every registered source-existence, marker, and worktree command
exited 0. The displayed Surface marker is the indexed `StateDecl`; the MirCore
marker is indexed `state x[k:K]`; SCN-08 contains scalar `room_anchor` and its
chain contains `default_pose`; P015 records the explicit-correspondence and
no-hidden-default boundary. The retained source comparison is reproducible in
the cited LAB artifact.
Negative evidence: No registered falsifier occurred. The pinned comparison
does not provide a scalar declaration/reference correspondence, terminal/default
declaration/resolution, scalar owner/store/well-formedness form, or permission
to use a hidden membership key, type-derived default, or unbound terminal. This
is a bounded observation about the displayed sources, not a claim that another
future representation is impossible or that SCN-08 is invalid.
Evidence artifacts: LAB:plan/wrk-0027-scn08-scalar-terminal-correspondence.md@a09568819c28fbad764e15b139e3cbde3e942e5d:104ba9fdbd13accaaf768204e82c623256edf83de2e0ae744723825b2aa5010b
Evidence commits: a09568819c28fbad764e15b139e3cbde3e942e5d
Impact / non-effects: The retained Plan artifact records only the literal
boundary that SCN-08's scalar/terminal notation needs an explicit later
correspondence before a model relies on it. It selects no scalar Core form or
finite-domain elaboration, changes no scenario or fallback law, and changes no
grammar, static semantics, Core, OBL, lifecycle, implementation, or public
behavior.
Independent review: not-required-for-L3

## Supersession

Supersession: none
