# tasks

最終更新: 2026-07-28 17:26 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project
direction, theory, ADRs, conformance, and process. Everything outside
`mirrorea_canon/` is LAB: evidence, history, implementation, and operational
notes. If LAB text conflicts with canon, canon wins.

## document role

This is the current LAB task map, not an append-only history and not a Canon
decision record. Detailed comparison and history live in `plan/`; immutable
task evidence lives in `docs/reports/`.

## current promoted package

The current official phase is `T0`. G0 exit and T1 entry are unrecorded, and
all OBL rows remain `open`. The latest autonomous source-cut chain retains
WRK-0036 and then records Plan 207 `no-candidate`. That is LAB evidence and a
local disposition, not an ADR-0014 narrowing or official lifecycle movement.

`plan/196-t0-t2-implementation-entry-roadmap.md` is the T0--T2 execution map;
`plan/197-i1-bootstrap-decision-and-readiness-audit.md` is the current I1
start audit; Plans 199/200 are the selected-direction composition plan, Plan
201 plus WRK-0032 record the completed C5-PRE audit, Plan 202 plus WRK-0033
record the completed bounded presentation comparison, and Plan 203 plus WRK-0034
record the completed fixed finite-sequence comparison. Plan 205 plus WRK-0035
record the retained generic C7 factorization boundary, Plan 206 plus WRK-0036
record the retained cumulative-erasure countermodel, Plan 207 records the
fresh no-candidate disposition, and Plans 208/209 prepare and prefix-locally
audit the C2-B/C3 comparison. Plan 210 compares the A/B instantiation limits.
Their current conclusion is:

- owner inputなしでofficial T2 exitまで連続自走することはできない。
- owner input前でも、既存Canonのliteral transcription / conditional lemmaだけで
  閉じる候補のADR-0014 eligibility preflightは自走できる。
- P004/P008/P012/P013/P015/P016 の direction は記録済みである。各 package が
  ADR-0014 の standing predicate と existing-lane 条件を個別に満たす範囲で、
  composition、形式化、反例、bounded validation、review、report、commit/push は自走できる。
- current CanonのT2条件だけでは、userが意図するI1 bootstrap/readinessと同義にならない。
- `spec/06`のC-static entryとphase tableのI1 exit表記を、bootstrap recordなしに
  implementation convenienceで読み替えてはならない。

Immediate blocker:

1. v2 profile の唯一の artifact は valid `fail`。fixed source-hierarchy と三つの
   LAB notice control は historical pin と一致しないが、scoped audit は統治文書 drift
   のみと確認した。O0 は rebase / retry を許可していない。
2. G0-D3 は explicit defer のままであり、current `fail` は T0 exit evidence にならない。

Sources: `mirrorea_canon/plan/01-phases.md`,
`mirrorea_canon/adr/ADR-0014.md`,
`plan/196-t0-t2-implementation-entry-roadmap.md`,
`plan/197-i1-bootstrap-decision-and-readiness-audit.md`.

The versioned correction has completed: v1 is retained as nonconforming
historical evidence, v2 uses `pass` as its only success literal, and its one
authorized fresh artifact is a valid `fail` on fixed-control drift. There is
no authorized retry or automatic successor. New autonomous research is judged
by ADR-0014's standing predicate. Plan 195's reopening patterns remain useful
conservative LAB selection discipline, not an additional Canon eligibility
gate; a genuinely novel candidate may still receive its own ADR-0014 screen.

## ordered self-driven packages

`O` = owner/canon action、`A` = autonomous agent package、`R` = independent
review。

| Order | Work unit | Authority / completion signal | Macro / rough estimate |
| --- | --- | --- | --- |
| 0 | T0-T2 planning audit | `A/R`; closed when Plan 196 and snapshots agree with Canon | Macro 0/1 checkpoint; completed baseline |
| 0A | Conservative statement preflight | `A/R`; assess every genuinely new literal/conditional candidate against ADR-0014. Fresh evidence and Plan 195 patterns are strong LAB selection signals, but do not replace the Canon predicate. | Macro 1/5 early; autonomous screening, no official movement |
| 1 | T0 profile v2 and fresh evaluation | `O` accepted; `A/R` completed one direct-child artifact. It is valid `fail`, not a lifecycle result | Macro 0/1 complete; no retry authorized |
| 2 | Fixed-control drift disposition | `A/R` scoped audit complete; `O` retains/defer or starts normal Canon rebase proposal | Macro 0/1 current blocker |
| 3 | G0-D3 exit decision | `O`; only after a valid `pass` artifact, exact digest acceptance and canonical exit record | official T1 entry blocker |
| 4 | Lifecycle/profile contract | `A` prepares, `O` decides; Gate status mapping, T1/T2 profiles, proof-skeleton meaning, narrow T2/I1-readiness relation, bootstrap/C-static timing are exact | Macro 0/1 middle; one design/decision package |
| 5 | Selected-direction composition | `A/R` fixed-presentation line closed locally: WRK-0028--0034 retain bounded C0/C2/C3/C5 results and Plan 204 finds no successor there. WRK-0035/0036 retain C7 guards; Plan 207 finds no further L3 candidate; Plans 208--210 prepare relation-first, request-occurrence, and nominal-attempt C2-B/C3 alternatives, staged correlation/validation/failure/restore obligations, and A/B instantiation limits. A/B remain conditional. C4/C5 proper are Canon design, and C0-D/C1/C6 are duplicate or select semantics. C7 source rules remain downstream of concrete uniquely reconstructible semantics and inspectable grounds. WRK-0024/0027 remain bounded evidence and WRK-0025/0026 remain frozen. | Macro 1 early; owner/Canon C2-B/C3 selection, then compatibility design |
| 6 | Shared formal model | `A/R`, then `O` integration; non-opaque Core/Config/Step/WF/elaboration/history relations and Plan 199 adverse cases exist | Macro 1/5 middle; heavy |
| 7 | G1 package | `A/R`, then `O`; exact OBL-001/020/021 statements and SCN-01/02 explanation | Macro 1/5 middle; heavy |
| 8 | G2/G3 package | `A/R`, then `O`; OBL-005..007/015 statements and owner-defined OBL-008 proof/status package share selected carriers | Macro 1/5 middle; heavy |
| 9 | T1 close | `A` prepares, `O` accepts; SCN finalization, profile pass, exact ledger statuses, exit record | official T2 entry blocker |
| 10 | T2 proof-skeleton package | `A/R`, then `O`; OBL-020/021/002 import-bearing skeletons with explicit assumptions/coverage | Macro 5 late; heavy |
| 11 | G5 statement package | `A/R`, then `O`; OBL-009..014 use separate saved predicate, restore relation, live-state postcondition, checker and checkpoint graph | Macro 1/5 late; heavy |
| 12 | I1-readiness matrix | `A/R`; all SCN/Core/G0-G7 interfaces, including OBL-003/027, classified as pre-bootstrap, I1-time, or later | Macro 1/3 boundary; medium |
| 13 | T2 close | `A` prepares, `O` accepts; narrow T2 profile pass, evidence cut, exit record | separate owner-defined I1 readiness / authorization |
| 14 | I1 authorization route | `A` prepares, `O` accepts; narrow-route readiness record, integrated profile, or phase-contract amendment binds fragment, C-static timing, all-SCN profile, carrier/BND baseline, and scoped moratorium lift | no route is current Canon; production starts only after the owner-selected route authorizes it |

Packages 5, 7/8, and 10/11 may have parallel research branches after their
shared prerequisites are fixed. Gate/Phase acceptance remains ordered.

## self-driven macro phase reading

| Macro | Current reading | Self-drive |
| --- | --- | --- |
| Macro 0 repository memory | cockpit, source hierarchy, reports, Plan 196 are available | maintenance and decision-packet preparation |
| Macro 1 semantic kernel | direction is fixed; proof-facing relations are incomplete | owner dispositions unlock only ADR-0014-eligible existing-lane packages |
| Macro 2 parser-free validation | existing compatibility anchors are runnable | reproduce/maintain only; not a Gate substitute |
| Macro 3 compile-ready actualization | bounded Surface/Full System evidence exists | production widening waits for theory/lifecycle authorization |
| Macro 4 sample expansion | bounded runnable roots exist | maintenance only before I1 |
| Macro 5 theorem/model-check bridge | drafts/countermodels exist; no Canon-aligned common model | main post-disposition research line |
| Macro 6 distributed fabric | later | blocked on I1/I2 |
| Macro 7 toolchain/backend | bounded LAB evidence only | later; public contract not selected |
| Macro 8 applications | user-defined worlds/samples exist as LAB evidence | not the T0-T2 critical path |

## user decision gates

### Immediate

| Item | Impact | Options | Current recommendation |
| --- | --- | --- | --- |
| fixed-control drift | whether a new v2 `pass` route can exist | retain/defer; normal Canon rebase proposal | scoped audit found governance-only drift; no silent rebase; O0's one artifact is consumed |
| G0-D3 | official T1 entry | accept a future valid `pass` digest; continue defer | current v2 `fail` is not eligible |
| T2 / I1 relation | narrow T2 or integrated I1 readiness; bootstrap/C-static timing | narrow T2; integrated profile; phase-contract amendment | if narrow route is selected, separate readiness/authorization; C-static is formal entry and remains I1-exit evidence |

### Before a shared formal model

| Item | Impact | Options | Current recommendation |
| --- | --- | --- | --- |
| Plan 199 C0 | elaboration outcome existence | source authority, front-end stage domain, diagnostic abstraction, totality/equality separation | WRK-0028 confirms source-local roles at one cut: current displayed grammar differs from P004/P015 direction, and P008 does not select domain/Diagnostic/OBL details. C0-A is complete-by-R0 only at that cut; WRK-0029 retains C0-B only as an opaque rank-increasing conditional DAG, without defining `WellScoped` or outcomes. |
| Plan 199 C1--C5 | value flow, receipt, service/admission occurrence | Family A relation primary; Family B request-occurrence anchor primary; Family C reserve after an A/B failure | WRK-0024 shows write seriality alone is insufficient. Plans 208--210 prepare C2-B/C3 alternatives, staged validation/reply/receipt/failure and restore obligations, and show that A/B are conditional rather than implicit. Owner/Canon must select identity/correlation/pending/restore rules. Reject hidden identity, receipt correlation, or intermediate mutation |
| Plan 199 C2 | post-admission validation context | payload equality, semantic request identity, admitted-execution binding, replay policy, persistence | WRK-0028 confirms source-local roles: theory/01/05 expose request/authority facts, while P012/P013 remain bounded directions and M1 leaves identity/replay open. WRK-0030 closes C2-A as source-tagged documentary non-substitution only, not an equality matrix or semantic candidate. |
| Plan 199 C6 | Surface/SCN scalar closure | explicit scalar/Core correspondence alternatives | WRK-0027 confirms displayed indexed rules do not silently cover SCN-08's scalar/terminal. Compare distinct scalar Core versus already-declared finite-domain elaboration; no hidden key/default or SCN-invalid inference. |
| Plan 199 C7 | source ergonomics | infer only uniquely reconstructible facts with inspectable grounds | WRK-0035 checks a generic range-only condition; WRK-0036 shows individually checked erasures cannot be composed unchecked. Do not treat either as a source rule; future cumulative representations require direct checking |
| Gate/Phase status contract | official exits | map existing status vocabulary and define T1/T2/I1 profiles | P016 direction recorded; required before any exit/implementation packet |

### Not on the explicit current critical path

- PROPOSAL-003 is organizational, not a semantic `Step`/frame decision.
- PROPOSAL-010 is an overview-wording issue unless locus hierarchy is used.
- PROPOSAL-011 is relevant to OBL-026/overlay cost; move it before T2 only if
  the selected I1-entry profile includes that patch-compatibility guarantee.
- The missing L2 trust anchor blocks delegated L2 promotion, not L3 research
  or direct owner/canon adoption.

## research discovery items

| Item | Research must establish | Stop condition |
| --- | --- | --- |
| Shared elaboration model | exact input/output, value flow, equality, Diagnostic, request/result relation | any unselected Core/occurrence/contract choice |
| Ergonomic inference | source omission preserves elaborated authority/failure/history evidence | ambiguity or non-reconstructible semantic fact |
| Global OBL-020 model | complete step-family coverage, frame/freshness, safe H insertion, owner seriality | opaque predicates or missing rule family |
| G2 chain model | normalization relation, confluence, lineage/lease/reacquire trace | unresolved grammar/scenario identity |
| G3 authority model | mutation-to-use/owner-local relation and all lineage claims | validation context or event identity unselected |
| G5 model | saved predicate, restore relation, post-load liveness, checker and Z-cycle correspondence | success precondition contains desired conclusion |
| Proof skeleton criterion | exact Lean artifact and ledger-status interpretation | hidden axiom, `True` stub, or status overclaim |
| I1-entry matrix | which G4/G6/G7 and all-SCN interfaces must be fixed before implementation | current Canon does not determine one answer |

Routine target selection after owner disposition is not a user gate. A
candidate touching L0/L1, Core/external contracts, SCN/Gate/Phase, or
`theory/11` stops with an escalation bundle.

## maintenance tasks

- Preserve `mirrorea_canon/` as normative and label all `plan/`, `specs/`,
  samples, helpers, and reports as LAB evidence.
- Keep `docs/project-status.md`, `progress.md`, `tasks.md`, and
  `samples_progress.md` synchronized only when their owned dimensions change.
- Do not repair or replay frozen WRK records.
- Before heavy work, recheck disk/memory and the external workdir mount.
- Run focused Lean/sample validation plus Canon index, source hierarchy,
  documentation, diff, and secret checks for each package.
- Commit with `--no-gpg-sign`, push every completed package, and verify
  `HEAD == origin/main`.

## non-promoted references

- Canon lifecycle: `mirrorea_canon/plan/00-gates.md`,
  `mirrorea_canon/plan/01-phases.md`.
- Research authority: `mirrorea_canon/adr/ADR-0014.md`,
  `mirrorea_canon/plan/02-operating-model.md`.
- Proof status: `mirrorea_canon/theory/11-metatheory-ledger.md`.
- Current T0-T2 route: `plan/196-t0-t2-implementation-entry-roadmap.md`.
- Current I1 decision/readiness audit: `plan/197-i1-bootstrap-decision-and-readiness-audit.md`.
- Selected-direction composition and inference boundary: `plan/199-selected-semantic-composition-and-inference-boundary.md`.
- C7 selection and retained L3 evidence: `plan/205-c7-parametric-factorization-candidate-selection.md`,
  `plan/wrk-0035-c7-parametric-factorization.md`.
- C7 cumulative-erasure selection and retained evidence:
  `plan/206-c7-cumulative-erasure-countermodel-candidate-selection.md`,
  `plan/wrk-0036-c7-cumulative-erasure-countermodel.md`.
- Post-WRK-0036 autonomous frontier disposition:
  `plan/207-post-wrk0036-autonomous-frontier-disposition.md`.
- C2-B/C3 value-flow design preparation:
  `plan/208-c2b-c3-value-flow-design-preparation.md`.
- C2-B/C3 relation-obligation audit:
  `plan/209-c2b-c3-relation-obligation-audit.md`.
- C2-B/C3 Family A/B instantiation audit:
  `plan/210-c2b-c3-family-a-b-instantiation-audit.md`.
- Statement identity: `plan/180-t1-t2-statement-identity-dependency-closure-audit.md`.
- Whole-theory reconciliation: `plan/whole-theory-foundation-audit-20260725.md`.
- Last autonomous source-cut screen: `docs/reports/2433-post-audit-autonomous-rescreen.md`.
- Runnable LAB classification: `samples_progress.md`.
- Historical research evidence: `plan/156-t0-t2-research-autonomy-envelope.md`.
