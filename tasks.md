# Current Task Map (LAB)

最終更新: 2026-08-05 01:59 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project
direction, theory, ADRs, conformance, and process. Everything outside
`mirrorea_canon/` is LAB: evidence, history, implementation, and operational
notes. If LAB text conflicts with canon, canon wins.

## document role

これは repository-wide **current task map** である。規範判断は
`mirrorea_canon/`、唯一の実行 queue は
`plan/247-mir-theory-v0-i1plus-current-roadmap.md`、長期比較はその他の
`plan/`、task の証跡は `docs/reports/` に置く。履歴 plan の「次」を active task に
戻さない。

## current promoted package

この legacy heading の `promoted` は documentation validator が要求する current LAB
frontier の意味であり、Canon / L2 / Gate / Phase promotion ではない。

**Active: M10 I1+ conformance / closeout.** M0--M9 は closed であり、M0--M10
program 内に次の semantic milestone はない。M10 close 後の public / distributed / product
direction は owner が新たに決める。根拠は `mirrorea_canon/adr/ADR-0015.md`、
`mirrorea_canon/plan/02-operating-model.md`、
`plan/247-mir-theory-v0-i1plus-current-roadmap.md` である。

M0 は start revision `b9dcaa054c548112a7977776723418559b8ba8b2` から bootstrap
payload `be5928a168fd519c05867fba2746ddd833a3bde5` を push して閉じた。M1 は
`root/design-constitution` と ADR-0016 を採用し、SCN-02 owner-side RMW と
semantic/presentation fallback を整合させた。M2 は revision-bound semantic-assertion v3 の
reproduced `pass` digestを受理し、G0-D3、G0 exit、T1 entry を順に適用した。M3--M8 は
有限の evaluation/materialization、relation/projection、shared model、ordinary Surface、
checker/elaborator、single-process runtime を source-first evidence と independent review で閉じた。

M9 は M8 direct admission を変えず、その source/identity/map/residual を lossless に保持する
private seam として、MembershipAuth、CapabilityAuth、non-transparent ContractUpdate、
attach/remove/revocation、finite refinement、bounded model、Lean evidence provenance と
invalidation を閉じた。29 focused M9 tests、workspace test、changed-crate format / all-target
Clippy、`--trust=0` Lean compile が evidence である。`OBL-026` は exact finite
`lean-proved`、`OBL-028` は一 subject/capability・depth 4・
`{admit, grant, revoke, use, reacquire}` の reachable-state graph に限る
`model-checked-bounded` である。これは general theorem、arbitrary auth composition、
official SCN conformance、public ABI/wire、transport、production の claim ではない。

現在の direct blocker は、**同じ ordinary `.mir` source** を実際に
`parse → check → elaborate → M8/M9 runtime → trace → projection` へ通す fresh release
profile である。これは C-static / C-runtime、waiver なしの SCN-01..10 10/10、
source→Core→trace→projection correspondence、fresh checkout reproduction、independent review を
一つの authoritative path として示さなければならない。過去 report、expected JSON、又は
fixture 名を束ねる wrapper は代替にならない。M10 はまだ pass していない。

official lifecycle は M2 close 時点から `T1` のままである。General OBL-001..025 / 027 は
`intentionally-deferred`、M3--M8 finite obligations は各 exact scope の既存分類を保つ。
M9 の OBL-026 は exact finite `lean-proved`、OBL-028 は exact bounded
`model-checked-bounded` である。SCN-01..10 official conformance claim は M10 evidence cut
まで unchanged である。

## ordered self-driven packages

ADR-0015 program 内では、owner-reserved condition に到達しない限り、各 milestone を
acceptance evidence と independent review で直列に閉じる。`A` = autonomous writer、
`R` = independent reviewer、`O` = orchestrator integration。

| Order | Current task package | Authority / completion signal | Macro / rough estimate |
| --- | --- | --- | --- |
| M0-A--C | Canon / agent / governance bootstrap | closed; ADR-0015 cut、role contract、Plan 247、review/validation/push evidence in Report 2581 | Macro 0; complete |
| M1 | Constitution | closed; `root/design-constitution`、ADR-0016、Report 2582 | Macro 1; complete |
| M2 | semantic-assertion T0/G0 closeout | closed; `plan/248` reproduced pass, independent review, and ADR-0017 acceptance applied | Macro 0/1; complete |
| M3 | evaluation / materialization | closed; ADR-0018/theory-13, finite Lean/bounded model/Rust trace, independent re-review | Macro 1/5; complete |
| M4 | maintained relation / late projection | closed; ADR-0019/theory-14/SCN-12, finite Lean/Rust evidence, final independent review | Macro 1/5/6; complete |
| M5 | shared formal model / metatheory | closed; finite shared model、13 Rust tests、exact OBL-040..047 Lean evidence、bounded/runtime correspondence | Macro 1/5; complete |
| M6 | Surface | closed; bounded grammar/AST/span/classification、parser/classifier tests、OBL-048 evidence | Macro 1/3; complete |
| M7 | checker / elaborator | closed; M6-preserving source-first finite route、OBL-049、M7 fixture matrix（SCN conformanceではない） | Macro 3/5/7; complete |
| M8 | deterministic runtime | closed; one checked artifact、explicit state/effect/failure trace、deterministic replay、cut/save/patch、exact OBL-050..056 Lean evidence | Macro 3/5; complete |
| M9 | auth / verification | closed; contract-transformer seam、Membership/CapabilityAuth、ContractUpdate、revocation、finite refinement and bounded-model evidence; OBL-026/028 exact classifications | Macro 1/5/7; complete |
| M10 | conformance / closeout | `A/R/O`; fresh same-source SCN-01..10 C-static/C-runtime release profile、correspondence、fresh checkout, non-claims and independent review | Macro 0/2/4/5; **active; heavy** |

次の semantic milestone は current milestone の normative rule/non-effect、reference
behavior、positive/negative evidence、formal classification、independent review、validation、
report、commit/push/parity が閉じるまで開始しない。

## self-driven macro phase reading

| Macro | Current reading | Startability |
| --- | --- | --- |
| 0 repository memory / governance | M0 bootstrap closed; Plan 247 is sole queue; M10 is closeout consumer | maintenance only |
| 1 semantic kernel | M1--M9 accepted; no new semantic candidate family is open | maintenance only |
| 2 parser-free validation | bounded LAB compatibility evidence exists | M10 same-source reproduction consumer |
| 3 compile-ready actualization | M8 checked-artifact runtime and M9 private extension seam are accepted | M10 authoritative composition only |
| 4 executable sample expansion | active sample roots remain runnable LAB | M10 conformance fixture selection only; no relabel |
| 5 theorem/model-check bridge | finite M3--M9 evidence closed; general proof remains deferred | M10 inventory/correspondence only |
| 6 distributed fabric | not in M0--M10 deterministic I1+ target | deferred / future owner direction |
| 7 toolchain/backend | M9 bounded typed auth/verification evidence is closed | M10 release validation only |
| 8 applications | user-defined worlds remain LAB consumers | deferred; not a Core milestone |

## user decision gates

現在、M10 を止める owner decision item はない。ADR-0015 が owner-approved objective、
M0--M10 order、bounded execution authority を記録しているため、旧 tasks にあった
fixed-control、C1/C2、Gate/ledger 一般の「常時 owner 待ち」は current queue から外す。
M10 evidence により program を閉じた後の方向だけが owner decision になる。

| Item | Impact | Main options | Current recommendation |
| --- | --- | --- | --- |
| ADR-0015 owner-reserved condition | North Star、safety/privacy weakening、domain-to-Core promotion、mandatory v0 non-goal、irreversible public API/ABI/wire、deployment/publication、tied irreversible choice、user data/secret risk | stop and present exact alternatives/evidence; or remain inside current scope | no trigger observed; continue M10 |
| post-M10 direction | next public/distributed/product program | new owner roadmap; retain completed bounded program | decide only after M10 evidence cut |

## research discovery items

これらは user decision request ではない。指定 milestone 内で current proposal と一つの
smallest viable alternative を falsifier で比較する。

| Milestone | Research must establish | Stop / discard condition |
| --- | --- | --- |
| M2 | semantic assertion identity、evidence binding、negative control、acceptance separation | historical artifact rewrite or premature lifecycle claim |
| M3--M8 | finite shared calculus、Surface、checker/elaborator、runtime and exact evidence classes | general proof/runtime/conformance/public-contract overclaim |
| M9 | typed auth/verification extension | M8 regression failure、authority-grant conflation、untyped ContractUpdate、stale capability after removal/revocation、hidden residual success、base-semantics redefinition、untyped observation leak |
| M10 | same ordinary-source authoritative release path and exact completion/non-completion boundary | any unreproduced required evidence, stale current reference, fake E2E wrapper, SCN waiver, or mismatch in source/Core/trace/projection correspondence |

## maintenance tasks

- Preserve `canon > LAB`; Plan 247 cannot invent lifecycle or proof facts.
- Keep Plans 196 / 197 / 246 and all older plans/reports/WRKs as historical LAB memory.
- Maintain one active milestone, one report per milestone by default, and one independent review.
- Update Plan 247 and derived status at milestone close; `samples_progress.md` changes only if its
  sample path / command / classification / blocker changes.
- Do not open a new WRK unless it has a current-milestone direct consumer, blocker reduction,
  explicit falsifier, milestone-report exclusion reason, and adoption/discard rule.
- Run resource checks before heavy generated work; keep disposable artifacts outside the repo.

## non-promoted references

- Normative program and lifecycle:
  `mirrorea_canon/adr/ADR-0015.md`,
  `mirrorea_canon/plan/01-phases.md`,
  `mirrorea_canon/plan/02-operating-model.md`.
- Sole current queue:
  `plan/247-mir-theory-v0-i1plus-current-roadmap.md`.
- Historical lifecycle / I1 / semantic comparison memory, not queues:
  `plan/196-t0-t2-implementation-entry-roadmap.md`,
  `plan/197-i1-bootstrap-decision-and-readiness-audit.md`,
  `plan/246-goal-first-semantic-integration-and-i1-entry.md`.
- Proof status:
  `mirrorea_canon/theory/11-metatheory-ledger.md`.
- Runnable sample dashboard:
  `samples_progress.md`.

No new Gate/Phase exit、general OBL discharge、official SCN/conformance pass、I1 implementation、
final public contract、production deployment、又は product completion をこの task map は主張しない。
