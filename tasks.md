# Current Task Map (LAB)

最終更新: 2026-07-31 21:52 JST

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

**現在の自律 research package はありません。** official lifecycle は `T0`、v2 profile の
唯一の fresh artifact は valid `fail`、G0-D3 は defer、OBL-001..028 は `open` です。
最初の公式 blocker は fixed-control drift の owner/Canon disposition です。valid `pass` route
が将来別途認可・評価・digest accept されるまで G0 exit / T1 entry は起きません。

P016 は narrow T2、separate I1-readiness/bootstrap、C-static formal entry の方向を記録済み
ですが、selected statement-level semantics を bind する profile/authorization は未作成です。
P017 X1 は owner-accepted のままです。WRK-0045 `frozen / DEFER` と Plan 245
`NO-SUCCESSOR` は predicate-only A-Sigma L3 line を閉じるだけで、P017 X1 本体を閉じません。
Plan 227 と Plans 230--245 が choice-neutral inventory を既に持つため、同種の自律 L3
successor/inventory を新しい package にしません。

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
| 1 | fixed-control drift disposition | `O`; pin 維持/defer 又は normal Canon rebase proposal | official T1 blocker; not autonomous |
| 2 | valid `pass` / G0-D3 exit route | `O` with `A/R` evidence preparation; authorized artifact、exact evaluation、digest acceptance、exit record | Macro 0/1; triggered after 1 |
| 3 | selected semantic integration / shared model | `A/R`, then `O`; non-opaque Core/Config/Step/WellFormed/elaboration/history relations and adverse cases | Macro 1/5 middle; after T1 entry and relevant semantic selections |
| 4 | T1 statement and profile package | `A/R`, then `O`; exact OBL-001/020/021, SCN explanation, canonical profile | official T2 blocker; after 3 |
| 5 | narrow T2 skeleton and G5 package | `A/R`, then `O`; import-bearing OBL-020/021/002 skeletons and separate G5 predicates/relations | Macro 5 late; after 4 |
| 6 | P016 I1-readiness/profile package | `O` with `A/R`; after selected statement-level semantics and narrow T2 evidence, bind all-SCN scope, ledger mapping, C-static wording, and any moratorium exception | after 5; no early accepted lifecycle/profile contract |
| 7 | I1 authorization | `O` with `A/R` readiness evidence; explicit bootstrap authorization then later C-static formal entry | after 6; implementation is still unauthorized now |
| on demand | fresh ADR-0014 candidate screen | `A/R`; nonduplicate literal/conditional candidate has standing eligibility, consumer, falsifier, non-effects, rollback trigger | Macro 1 reserve; never create merely to extend P017/lifecycle inventory |

## self-driven macro phase reading

| Macro | Current reading | Startability |
| --- | --- | --- |
| 0 repository memory / governance | Canon hierarchy, derived views, reports, Plans 196/197 are available | maintenance and decision-packet preparation |
| 1 semantic kernel | directions and bounded evidence exist; shared proof-facing composition is incomplete | new ADR-0014 candidate only on a genuine trigger; no current P017 L3 successor or lifecycle inventory package |
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
| semantic composition | shared model and later statements | ordinary Canon selection of exact domain, identity, receipt, failure, scalar, totality relations | finite/conditional LAB evidence does not select semantics |
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
- WRK-0045 predicate-only A-Sigma L3-line closure / P017 ordinary-design boundary:
  `plan/227-p017-x1-decision-vector-and-choice-neutral-consistency.md`,
  `plan/245-post-wrk0045-no-successor-ordinary-x1-handoff.md`,
  `docs/reports/2568-post-wrk0045-autonomous-frontier-reconciliation.md`.
- Runnable evidence dashboard: `samples_progress.md`.
