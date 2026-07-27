# tasks

最終更新: 2026-07-28 00:41 JST

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
all OBL rows remain `open`. The last autonomous source-cut screen selected no
new `WRK-####` from its reviewed delta. That is a LAB priority disposition,
not a permanent narrowing of ADR-0014.

`plan/196-t0-t2-implementation-entry-roadmap.md` is the T0--T2 execution map;
`plan/197-i1-bootstrap-decision-and-readiness-audit.md` is the current I1
start audit. Their conclusion is conditional:

- owner inputなしでofficial T2 exitまで連続自走することはできない。
- owner input前でも、既存Canonのliteral transcription / conditional lemmaだけで
  閉じる候補のADR-0014 eligibility preflightは自走できる。
- ownerがreserved boundaryを選んだ後も、各packageがADR-0014のstanding
  predicateとexisting-lane条件を個別に満たす範囲で、次のowner checkpointまでの
  研究、形式化、反例、bounded validation、review、report、commit/pushは自走できる。
- current CanonのT2条件だけでは、userが意図するI1 bootstrap/readinessと同義にならない。
- `spec/06`のC-static entryとphase tableのI1 exit表記を、bootstrap recordなしに
  implementation convenienceで読み替えてはならない。

Immediate blocker:

1. T0 profileの`pass` / `derived-pass`矛盾。
2. 旧artifactが矛盾したsource revisionを自己bindしているため、単純な文言修正では
   artifact continuityが成立しないこと。
3. G0-D3がexplicit deferであること。

Sources: `mirrorea_canon/plan/01-phases.md`,
`mirrorea_canon/adr/ADR-0014.md`,
`plan/196-t0-t2-implementation-entry-roadmap.md`,
`plan/197-i1-bootstrap-decision-and-readiness-audit.md`.

Current LAB recommendation is a versioned correction: retain v1 as historical
evidence, define profile v2 with `pass`, authorize one fresh v2 artifact, and
keep G0-D3 acceptance separate.

## ordered self-driven packages

`O` = owner/canon action、`A` = autonomous agent package、`R` = independent
review。

| Order | Work unit | Authority / completion signal | Macro / rough estimate |
| --- | --- | --- | --- |
| 0 | T0-T2 planning audit | `A/R`; closed when Plan 196 and snapshots agree with Canon | Macro 0/1 checkpoint; current package |
| 0A | Conservative statement preflight | `A/R`; inspect literal/conditional candidates, open a WRK only if every ADR-0014 condition and a non-duplicate consumer pass | Macro 1/5 early; autonomous, no official movement |
| 1 | T0 profile revision packet | `A` prepares, `O` decides; result literal, versioning, old artifact, one-off route are unambiguous | Macro 0/1 early; one decision package |
| 2 | Fresh T0 evaluation | `A/R`; exact Git blobs, ordering, RFC 8785 digest, three checks, non-claims pass | Macro 0/1 early; bounded |
| 3 | G0-D3 exit decision | `O`; exact digest acceptance and canonical exit record | official T1 entry blocker |
| 4 | Lifecycle/profile contract | `A` prepares, `O` decides; Gate status mapping, T1/T2 profiles, proof-skeleton meaning, narrow T2/I1-readiness relation, bootstrap/C-static timing are exact | Macro 0/1 middle; one design/decision package |
| 5 | Semantic decision closure | `A` prepares and `O` decides; P013 comparison starts only after M1/M2/MD disposition | Macro 1 early; several decision records |
| 6 | Shared formal model | `A/R`, then `O` integration; non-opaque Core/Config/Step/WF/elaboration/history relations and adverse cases exist | Macro 1/5 middle; heavy |
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
| T0 profile contract | profile validity | v2; v1 corrigendum; clarify | v2 with `pass`; retain v1 as nonconforming historical evidence |
| T0 artifact route | source-bound artifact continuity | authorize one fresh v2 artifact; preserve old only; clarify | one fresh v2 artifact, no inferred G0 exit |
| G0-D3 | official T1 entry | accept exact fresh digest; continue defer | decide only after fresh evaluation |
| T2 / I1 relation | narrow T2 or integrated I1 readiness; bootstrap/C-static timing | narrow T2; integrated profile; phase-contract amendment | if narrow route is selected, separate readiness/authorization; C-static is formal entry and remains I1-exit evidence |

### Before a shared formal model

| Item | Impact | Options | Current recommendation |
| --- | --- | --- | --- |
| PROPOSAL-008 | elaboration outcome existence | A/B/C/D | A: separate totality obligation |
| PROPOSAL-012 | value flow, read receipt, write/admission occurrence | V/R/S/A families | V1/R1/SW1/conditional A2, followed by compatibility review |
| PROPOSAL-013 | post-admission validation context | M1/M2/MD | owner first selects M1/M2/MD; then test only the selected family against adverse cases |
| PROPOSAL-004 | exact Surface v0 closure | A/B/C | A Participant-only closure |
| OPEN-005 / SCN-08 | chain syntax and scalar/indexed scenario reading | select one coherent source/scenario form | settle before G2 statement identity |
| Surface `return` | exact v0 fragment | specify Core/elaboration; exclude; defer | exclude unless an immediate canonical scenario needs it |
| Gate/Phase status contract | official exits | map existing status vocabulary and define T1/T2 profiles | required before any exit packet |

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
- Statement identity: `plan/180-t1-t2-statement-identity-dependency-closure-audit.md`.
- Whole-theory reconciliation: `plan/whole-theory-foundation-audit-20260725.md`.
- Last autonomous source-cut screen: `docs/reports/2433-post-audit-autonomous-rescreen.md`.
- Runnable LAB classification: `samples_progress.md`.
- Historical research evidence: `plan/156-t0-t2-research-autonomy-envelope.md`.
