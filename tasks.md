# Current Task Map (LAB)

最終更新: 2026-08-05 16:34 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project
direction, theory, ADRs, conformance, and process. Everything outside
`mirrorea_canon/` is LAB: evidence, history, implementation, and operational
notes. If LAB text conflicts with canon, canon wins.

## document role

これは repository-wide **current task map** である。規範判断は
`mirrorea_canon/`、長期比較は `plan/`、task の証跡は `docs/reports/` に置く。
履歴 plan の「次」を active task に戻さない。

## current promoted package

この legacy heading の `promoted` は documentation validator が要求する current LAB
frontier の意味であり、Canon / L2 / Gate / Phase promotion ではない。

**Closed: M10 I1+ conformance / closeout.** ADR-0015 の M0--M10 bounded program
inside this repository is complete. There is no remaining autonomous semantic
package inside M0--M10. The next program/public/distributed/product direction is
owner-defined and must not be invented from the M10 evidence.

Accepted R5 evidence: commit `23f5a8130334bf0c8516d51e9dcea38b92f50db1`, tree
`d8a296fac7a94a37da92563d5feeeeaa96dbc682`, output SHA256 reproduced twice
`083523518fdae0a111522f49b148c818ca0d5c21b4b7cc4f34dd476f10d172e7`,
static 26/26, runtime 47/47, mismatch 0, missing 0, anchor true, waiver null,
reviewer ACCEPT no P0/P1/P2.

Official lifecycle remains `T1`. M10 accepted the finite deterministic I1+
reference profile only; it does not claim broad PHASE-I1 exit, I2 activation,
final public ABI/wire/carrier freeze, C-distributed, transport, production,
public product completion, or general theorem discharge.

Sources: `mirrorea_canon/adr/ADR-0025.md` and
`plan/247-mir-theory-v0-i1plus-current-roadmap.md`.

## ordered self-driven packages

| Order | Current task package | Authority / completion signal | Macro / rough estimate |
| --- | --- | --- | --- |
| M0 | Canon / agent / governance bootstrap | closed; Report 2581 | Macro 0; complete |
| M1 | Constitution | closed; ADR-0016 / Report 2582 | Macro 1; complete |
| M2 | semantic-assertion T0/G0 closeout | closed; ADR-0017 / reproduced v3 pass | Macro 0/1; complete |
| M3 | evaluation / materialization | closed; ADR-0018 / finite evidence | Macro 1/5; complete |
| M4 | maintained relation / late projection | closed; ADR-0019 / finite evidence | Macro 1/5/6; complete |
| M5 | shared formal model / metatheory | closed; ADR-0020 / exact finite evidence | Macro 1/5; complete |
| M6 | Surface | closed; ADR-0021 / finite grammar-classification evidence | Macro 1/3; complete |
| M7 | checker / elaborator | closed; OBL-049 exact finite evidence | Macro 3/5/7; complete |
| M8 | deterministic runtime | closed; OBL-050..057 exact finite/bounded/runtime evidence | Macro 3/5; complete |
| M9 | auth / verification | closed; OBL-026 exact Lean and OBL-028 bounded model | Macro 1/5/7; complete |
| M10 | conformance / closeout | accepted/closed; R5 finite I1+ profile and reviewer ACCEPT | Macro 0/2/4/5; complete |
| Post-program | owner-defined next direction | no autonomous package selected | requires owner input |

## self-driven macro phase reading

| Macro | Current reading | Startability |
| --- | --- | --- |
| 0 repository memory / governance | M0--M10 closeout snapshots synchronized | maintenance only |
| 1 semantic kernel | finite v0/I1+ line accepted through M10 | no new autonomous line |
| 2 parser-free validation | historical LAB compatibility evidence exists | maintenance only |
| 3 compile-ready actualization | finite I1+ reference profile accepted | post-program direction required |
| 4 executable sample expansion | active sample roots remain LAB evidence | maintenance only |
| 5 theorem/model-check bridge | finite M3--M9 evidence retained; proof ledger unchanged | post-program direction required |
| 6 distributed fabric | not in M0--M10 deterministic I1+ target | owner-defined future |
| 7 toolchain/backend | bounded support accepted; public interface open | owner-defined future |
| 8 applications | user-defined worlds remain LAB consumers | deferred; not a Core milestone |

## user decision gates

| Item | Impact | Main options | Current recommendation |
| --- | --- | --- | --- |
| Post-M0--M10 direction | selects the next roadmap and authority boundary | new owner roadmap; pause after closeout | require owner direction before new autonomous semantic work |
| OPEN-030 / carrier boundary | carrier/public ABI/wire readiness | define/freeze a scoped carrier; defer; or split research from public contract | do not infer freeze from M10 |
| Broad PHASE-I1 exit / I2 activation | lifecycle and next implementation scope | accept separate phase record; define I2 entry; defer | not claimed by M10; owner decision required |
| Public/product deployment | irreversible external contract | publish/deploy under explicit scope; defer | owner-reserved |
| North Star / safety/privacy / Core-domain promotion | project guarantee | change by explicit decision only; preserve current boundary | owner-reserved |

## research discovery items

これらは active user decision ではない。Owner が新 program/direct consumer を定義した後に
初めて、bounded research package として選別する。

| Item | Research must establish | Stop / discard condition |
| --- | --- | --- |
| Carrier boundary / OPEN-030 | exact carrier role, public/non-public line, compatibility and falsifier | public ABI/wire freeze without owner direction |
| I2 entry | process-internal multi-place scope and relation to accepted I1+ profile | treating M10 as automatic I2 authorization |
| General theorem work | proof statement, evidence class, correspondence to finite profile | moving OBL status without actual evidence |
| Public sample/catalog widening | user-facing scope and support contract | relabeling helper/profile evidence as product completion |

## maintenance tasks

- Preserve `canon > LAB`; derived snapshots do not create owner decisions.
- Keep older plans/reports/WRKs as historical LAB memory.
- Keep Report 2591 as the M10 closeout record and do not create extra closeout
  reports for snapshot-only maintenance.
- Do not open a new WRK unless a new owner-defined direct consumer exists.
- Run resource checks before heavy generated work; keep disposable artifacts
  outside the repo.

## non-promoted references

- Normative program and lifecycle:
  `mirrorea_canon/adr/ADR-0015.md`,
  `mirrorea_canon/plan/01-phases.md`,
  `mirrorea_canon/plan/02-operating-model.md`.
- M10 boundary:
  `mirrorea_canon/adr/ADR-0025.md`,
  `mirrorea_canon/spec/11-m10-i1plus-conformance.md`,
  `mirrorea_canon/spec/06-conformance.md`.
- Closeout:
  `docs/reports/2591-mir-theory-v0-i1plus-milestone-10-conformance-closeout.md`.
- Proof status:
  `mirrorea_canon/theory/11-metatheory-ledger.md`.
- Runnable sample dashboard:
  `samples_progress.md`.

No new Gate/Phase exit, broad PHASE-I1 exit, I2 activation, general OBL
discharge, final public contract, production deployment, or product completion
is claimed by this task map.
