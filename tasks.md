# Current Task Map (LAB)

最終更新: 2026-08-01 12:43 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project
direction, theory, ADRs, conformance, and process. Everything outside
`mirrorea_canon/` is LAB: evidence, history, implementation, and operational
notes. If LAB text conflicts with canon, canon wins.

## document role

これは repository-wide **current task map** であり、履歴や候補の索引ではありません。
規範判断の正本は `mirrorea_canon/`、詳細な比較と時系列の repository memory は `plan/`、
task ごとの不変な証跡は `docs/reports/` にあります。ここでは「今すぐ自走できる package」、
「research で判明させること」、「owner が判断すること」を分けます。

## current promoted package

この legacy heading の `promoted` は documentation validator が要求する current LAB
frontier の意味であり、Canon/L2/Gate/Phase の promotion ではありません。

**Official critical path remains the owner-controlled fixed-control disposition.**
WRK-0046 R1 registration and R2 evidence/link are completed parallel-reserve
packages. The record remains `L3-open` / `not-promoted`; it is neither a Gate
input nor a critical-path dependency. No promoted autonomous research package,
successor, inventory extension, or lifecycle package is selected. Official
lifecycle is `T0`, the v2 profile's sole fresh artifact is valid `fail`, G0-D3
is defer, and OBL-001..028 are `open`.
最初の公式 blocker は fixed-control drift の owner/Canon disposition です。valid `pass` route
が将来別途認可・評価・digest accept されるまで G0 exit / T1 entry は起きません。

P016 は narrow T2、separate I1-readiness/bootstrap、C-static formal entry の方向を記録済み
ですが、selected statement-level semantics を bind する profile/authorization は未作成です。
P017 X1 は owner-accepted のままです。WRK-0045 `frozen / DEFER` と Plan 245
`NO-SUCCESSOR` は predicate-only A-Sigma L3 line を閉じるだけで、P017 X1 本体を閉じません。
独立再査読により、K0 external-rejection branch の q-fibered spent/use と local restore
preservation に限る一件だけが、Plans 230--231 の未消化 U/L consumer を持つと判明しました。
WRK-0046 の R1/R2 packages は完了したが、record は `L3-open` / `not-promoted` のままです。
A0 は registered preservation premises の下で two-consume を排除し、A1 は omission/reset
control を構成した。receipt、identity、actual persistence、Core、Gamma/Delta、OBL、実装は
選ばず、同種の inventory 拡張や WRK-0045 の修復はしません。

**Parallel semantic mainline:** S2-A は `plan/246-goal-first-semantic-integration-and-i1-entry.md`
と Report 2577 で完了した LAB comparison である。D0/D3/D4 は LAB candidate、`C1-A-r`
（target-owner RMW）/ `C1-B`（determined value）と `C2-A-r`（P017 X1 の
candidate-specific extension）/ defer は
ordinary Core/SCN amendment choice であり、現行 Canon の semantics ではない。SCN-02 の
two dependencies と read/visibility authority の baseline reconciliation を含め、owner/Canon
selection 前に formal model、prototype、runtime は作らない。official T0、G0-D3、OBL status、
Canon Core は変えない。fixed-control drift は official lifecycle blocker だが、S2-A completion
の前提ではなかった。

```text
T0 -- owner fixed-control disposition -- authorized valid pass / G0-D3
   -- T1 entry -- selected semantic integration -- T1 statements/profile
   -- narrow T2 skeleton/G5 -- separate I1 readiness/bootstrap -- I1 authorization
```

根拠: `mirrorea_canon/plan/01-phases.md`,
`mirrorea_canon/adr/ADR-0013.md`,
`mirrorea_canon/theory/11-metatheory-ledger.md`,
`plan/196-t0-t2-implementation-entry-roadmap.md`,
`plan/245-post-wrk0045-no-successor-ordinary-x1-handoff.md`。

## ordered self-driven packages

`O` = owner/Canon action、`A` = autonomous agent package、`R` = independent review。
「triggered」は source delta 又は前段の owner decision が到着するまで着手しないことを表します。

| Order | Work unit | Authority / completion signal | Macro / rough estimate |
| --- | --- | --- | --- |
| 0 | snapshot and evidence maintenance | `A/R`; current task map と derived views が Canon cut と一致 | Macro 0; small; current only when status changes |
| S1 | goal-first authority-aware packet | `A/R`; D0/D3/D4 candidate と C1-A/C2-A amendment hypothesis、authority/rollback/no-effects を整理 | review-corrected: `plan/246-goal-first-semantic-integration-and-i1-entry.md`; no Canon effect |
| S2-A | bounded comparison and amendment packet | `A/R`; C1-A-r/C1-B/defer と C2-A-r/defer、adverse trace、SCN-01..10 impact、exact amendment surface を comparison 済み | **complete**: `plan/246-goal-first-semantic-integration-and-i1-entry.md` / Report 2577; no Canon effect |
| S2-B | selected shared kernel model | `O`, then `A/R`; selected Core/SCN surface を non-opaque model、permitted Lean/prototype checks にする | Macro 1/5 middle; **blocked on owner/Canon choice**: SCN-02 reconciliation と C1/C2 amendment selection |
| S3 | candidate-local statement preparation | `A/R` after S2-B; selected model を statement drafts/SCN explanation に接続 | Macro 1/5 late; no ledger/OBL/profile change |
| CP-1 | fixed-control drift disposition | `O`; pin 維持/defer 又は normal Canon rebase proposal | official T1 blocker; not autonomous |
| CP-2 | valid `pass` / G0-D3 exit route | `O` with `A/R` evidence preparation; authorized artifact、exact evaluation、digest acceptance、exit record | Macro 0/1; triggered after CP-1 |
| R1 (parallel reserve) | `WRK-0046 X1-K0-QF-UL-LIFT` L3 registration | `A/R`; exact cut, alternative, ablation falsifier, no-effects, rollback, existing `plan/` Lean lane only | closed registration stage: source-free at that cut; current evidence state is tracked by R2; neither Gate input nor CP dependency |
| R2 (parallel reserve) | bounded q-fiber mixed-trace evidence | `A/R`; one Markdown-held Lean source, `lean --trust=0`, axiom scan, A0/A1 control, immutable evidence link | closed: evidence commit `7e4b01eb` linked by `c05653c4`; `L3-open` / `not-promoted`; neither Gate input nor critical-path dependency |
| CP-3 | ordinary selection of C1/C2 amendment surface | `A/R`, then `O`; reconcile SCN-02 dependencies/read authority, select C1-A-r/C1-B/defer and C2-A-r/defer, then freeze ordinary proposal before S2-B | Macro 1 middle; current semantic blocker after S2-A |
| CP-4 | T1 statement and profile package | `A/R`, then `O`; exact OBL-001/020/021, SCN explanation, canonical profile | official T2 blocker; after CP-3 |
| CP-5 | narrow T2 skeleton and G5 package | `A/R`, then `O`; import-bearing OBL-020/021/002 skeletons and separate G5 predicates/relations | Macro 5 late; after CP-4 |
| CP-6 | P016 I1-readiness/profile package | `O` with `A/R`; after selected statement-level semantics and narrow T2 evidence, bind all-SCN scope, ledger mapping, C-static wording, and any moratorium exception | after CP-5; no early accepted lifecycle/profile contract |
| CP-7 | I1 authorization / autonomous stop | `O` with `A/R` readiness evidence; explicit bootstrap authorization then later C-static formal entry | I1-entry condition: close out and stop before the first implementation package; do not silently start it |
| on demand | fresh ADR-0014 candidate screen | `A/R`; nonduplicate literal/conditional candidate has standing eligibility, consumer, falsifier, non-effects, rollback trigger | Macro 1 reserve; never create merely to extend P017/lifecycle inventory |

## self-driven macro phase reading

| Macro | Current reading | Startability |
| --- | --- | --- |
| 0 repository memory / governance | Canon hierarchy, derived views, reports, Plans 196/197 are available | maintenance and decision-packet preparation |
| 1 semantic kernel | S2-A comparison complete; no shared model yet | owner/Canon reconciliation and selection before formal model/prototype |
| 2 parser-free validation | compatibility anchors are runnable | reproduce and maintain; not a Gate substitute |
| 3 compile-ready actualization | bounded Surface/Full System evidence exists | production widening waits for authorization |
| 4 sample expansion | runnable samples exist | maintenance only before I1 |
| 5 theorem/model-check bridge | drafts/countermodels exist; no Canon-aligned shared model | post-selection research line |
| 6 distributed fabric | later | blocked on I1/I2 |
| 7 toolchain/backend | bounded LAB evidence only | later; public contract unselected |
| 8 applications | user-defined worlds/samples as LAB evidence | outside current T0--T2 critical path |

## user decision gates

| Item | Impact | Main options | Current recommendation |
| --- | --- | --- | --- |
| fixed-control drift | whether a future valid `pass` route can exist | retain pins/defer; normal Canon rebase proposal | scoped audit found governance-only drift; no silent rebase/retry |
| G0-D3 | official T1 entry | accept a future valid `pass` digest; continue defer | current `fail` is ineligible |
| lifecycle / I1 boundary | profile, ledger mapping, phase/conformance wording, authorization | materialize P016's narrow T2 + separate readiness; explicit Canon reopen for another route | P016 direction is the recorded path; do not create a duplicate autonomous map |
| C1/C2 amendment surface | shared model and later statements | reconcile SCN-02's two dependencies/read authority; choose C1-A-r/C1-B/defer and C2-A-r/defer | C1-A-r conditionally and C2-A-r directionally recommended; LAB evidence does not select semantics |
| T1/T2/I1 acceptance | official phase movement | accept later evidence/profile/record; keep current state | do not decide before their direct evidence exists |

## research discovery items

| Item | Research must establish | Stop condition |
| --- | --- | --- |
| Shared elaboration model | exact input/output, value flow, equality, Diagnostic, request/result relation | any unselected Core/occurrence/contract choice |
| Ergonomic inference | source omission preserves elaborated authority/failure/history evidence | ambiguity or non-reconstructible semantic fact |
| Global OBL-020 model | complete step-family coverage, frame/freshness, safe H insertion, owner seriality | opaque predicate or missing rule family |
| G2/G3 model | normalization, lineage/lease/reacquire, mutation-to-use/owner-local relation | unresolved grammar, scenario identity, validation context, or event identity |
| G5 model | saved predicate, restore relation, live-state postcondition, checker/Z-cycle correspondence | success premise contains the desired conclusion |
| Proof skeleton criterion | exact Lean artifact and ledger-status interpretation | hidden axiom, `True` stub, or status overclaim |
| I1 readiness matrix | all-SCN interfaces and G4/G6/G7 / OBL-003/027 classification | Canon has not yet supplied the selected semantic/evidence cut |

Research stops and prepares an escalation bundle when it needs an L0/L1 choice, a Core/external
contract, SCN/Gate/Phase, `theory/11` wording/status, or a new moratorium-protected lane.

## maintenance tasks

- Preserve the Canon/LAB hierarchy and do not repair or replay frozen WRK records.
- Update `docs/project-status.md`, `progress.md`, `tasks.md`, and `samples_progress.md` only when
  their owned status changes; record every nontrivial task in a new `docs/reports/` file.
- For broad status, critical-path, roadmap, phase-recut, or lifecycle-inventory work, obtain a
  Canon-first read-only `planner` review before editing and before package close. It must check the
  current blocker, direct consumer, authority boundary, evidence, and stop/reopen trigger.
- Before heavy work, inspect disk/memory/external workdir. Run focused evidence, Canon index,
  source hierarchy, documentation, diff, and secret checks appropriate to the package.
- Commit with `--no-gpg-sign`, push every completed package, and verify `HEAD == origin/main`.

## non-promoted references

- Canon lifecycle and gates: `mirrorea_canon/plan/00-gates.md`,
  `mirrorea_canon/plan/01-phases.md`.
- Research authority: `mirrorea_canon/adr/ADR-0014.md`,
  `mirrorea_canon/working/README.md`.
- Proof status: `mirrorea_canon/theory/11-metatheory-ledger.md`.
- Current critical-path memory: `plan/196-t0-t2-implementation-entry-roadmap.md`,
  `plan/197-i1-bootstrap-decision-and-readiness-audit.md`,
  `plan/180-t1-t2-statement-identity-dependency-closure-audit.md`,
  `plan/199-selected-semantic-composition-and-inference-boundary.md`.
- Goal-first autonomous semantic mainline: `plan/246-goal-first-semantic-integration-and-i1-entry.md`.
- WRK-0045 predicate-only A-Sigma L3-line closure / P017 ordinary-design boundary:
  `plan/227-p017-x1-decision-vector-and-choice-neutral-consistency.md`,
  `plan/245-post-wrk0045-no-successor-ordinary-x1-handoff.md`,
  `docs/reports/2568-post-wrk0045-autonomous-frontier-reconciliation.md`.
- WRK-0046 non-promoted finite conditional evidence:
  `mirrorea_canon/working/WRK-0046-p017-x1-k0-qf-ul-lift.md`,
  `plan/wrk-0046-p017-x1-k0-qf-ul-lift.md`,
  `docs/reports/2572-wrk0046-p017-x1-k0-qf-ul-lift-execution.md`,
  `docs/reports/2573-wrk0046-positive-conditional-evidence-metadata-link.md`.
- Runnable evidence dashboard: `samples_progress.md`.
