# Current Task Map (LAB)

最終更新: 2026-08-04 18:56 JST

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

**Active: M8 deterministic runtime. Next: M9 auth/verification.** 根拠は
`mirrorea_canon/adr/ADR-0015.md`、
`mirrorea_canon/plan/02-operating-model.md`、
`plan/247-mir-theory-v0-i1plus-current-roadmap.md` である。

M0 は start revision `b9dcaa054c548112a7977776723418559b8ba8b2` から bootstrap
payload `be5928a168fd519c05867fba2746ddd833a3bde5` を push して閉じた。M1 は
`root/design-constitution` と ADR-0016 を採用し、SCN-02 owner-side RMW と
semantic/presentation fallback を整合させ、independent review と one correction cycle を
pass した。M2 は revision-bound semantic-assertion v3 の reproduced `pass` digestを受理し、
G0-D3、G0 exit、T1 entry を順に適用した。M3 は finite `EvalPlan`、owner RMW、
release-admitted causal receipt、designated publish/consume を Lean/bounded model/Rust traceと
independent reviewで閉じた。M4 は ADR-0019/theory/14/SCN-12 の finite relation-first
calculusを閉じた。M5 は ADR-0020/theory/15 の finite concrete
`SurfaceFragment → Core | Diagnostic` shared modelを閉じた。M6 は ADR-0021/spec/01--04 の
bounded ordinary grammar、span-rich AST、total M5-aligned classificationを閉じた。M7 は M6
meaningを変更せず、ordinary `.mir` の `parse → M6 full classification retention → finite check
→ typed Core/effects/obligations/residuals/source map` routeを閉じた。8 AST tests、11 classifier
tests、22 M7 pipeline tests、full `mir-ast` / `mir-semantics` suites、format/clippy、OBL-049の16
exact finite `--trust=0` Lean theoremsがevidenceである。10-rowはM7 fixture matrixでありSCN
official conformanceではない。official lifecycleは`T1`、v1/v2はhistorical evidenceのままで
ある。現在のdirect blockerはM7 checked outputをbypassなしに消費するM8 deterministic runtimeで
あり、direct consumerはM9である。

M2 close 時点で official lifecycle は `T1`。これにより I1 は開始しない。General
OBL-001..028 は `intentionally-deferred`、finite M3 OBL-029..032 は `lean-proved`、033は
`model-checked-bounded`、034は`runtime-monitored`、finite M4 OBL-035..039、M5 OBL-040..047、
M6 OBL-048、M7 OBL-049は exact scope の `lean-proved`である。SCN-01..10のofficial
conformance claimはunchangedであり、M8 runtime implementationが次のactive scopeである。

## ordered self-driven packages

ADR-0015 program 内では、owner-reserved condition に到達しない限り、各 milestone を
acceptance evidence と independent review で直列に閉じる。`A` = autonomous writer、
`R` = independent reviewer、`O` = orchestrator integration。

| Order | Current task package | Authority / completion signal | Macro / rough estimate |
| --- | --- | --- | --- |
| M0-A--C | Canon / agent / governance bootstrap | closed; ADR-0015 cut、role contract、Plan 247、review/validation/push evidence in Report 2581 | Macro 0; complete |
| M1 | Constitution | closed; payload `aa0771ec` pushed with review/validation evidence in `root/design-constitution`、ADR-0016、Report 2582 | Macro 1; complete |
| M2 | semantic-assertion T0/G0 closeout | closed; `plan/248` reproduced pass, independent review, and ADR-0017 acceptance applied | Macro 0/1; complete |
| M3 | evaluation / materialization | closed; ADR-0018/theory-13, finite Lean/bounded model/Rust trace, independent re-review | Macro 1/5; complete |
| M4 | maintained relation / late projection | closed; ADR-0019/theory-14/SCN-12, finite Lean/Rust evidence, final independent review | Macro 1/5/6; complete |
| M5 | shared formal model / metatheory | closed; finite shared model、13 Rust tests、exact OBL-040..047 Lean evidence、bounded/runtime correspondence | Macro 1/5; complete |
| M6 | Surface | closed; bounded grammar/AST/span/classification、3 parser/11 classifier tests、OBL-048 evidence | Macro 1/3; complete |
| M7 | checker / elaborator | closed; M6-preserving source-first finite route、OBL-049、M7 fixture matrix（SCN conformanceではない） | Macro 3/5/7; complete |
| M8 | deterministic runtime | `A/R/O`; M7 checked output、explicit state/effect/failure trace、deterministic replay、cut/save/patch | Macro 3/5; active; heavy |
| M9 | auth / verification | `A/R/O`; typed auth/authority/verification、negative security evidence、M8 regression | Macro 1/5/7; next after M8; heavy |
| M10 | conformance / closeout | `A/R/O`; fresh full evidence matrix、clean-clone reproduction、claims/non-claims、program report/parity | Macro 0/2/4/5; after M9; heavy |

次の semantic milestone は current milestone の normative rule/non-effect、reference
behavior、positive/negative evidence、formal classification、independent review、validation、
report、commit/push/parity が閉じるまで開始しない。

## self-driven macro phase reading

| Macro | Current reading | Startability |
| --- | --- | --- |
| 0 repository memory / governance | M0 bootstrap closed; Plan 247 is sole queue | maintenance only |
| 1 semantic kernel | M1--M7 accepted; M8 is the current checked-Core runtime package | **着手可能**: runtime without M7 bypass or semantic drift |
| 2 parser-free validation | bounded LAB compatibility evidence exists | maintenance only; M10 reproduction consumer |
| 3 compile-ready actualization | M7 checked boundary is accepted | **着手可能**: M8 deterministic runtime |
| 4 executable sample expansion | active sample roots remain runnable LAB | maintenance; no M0 relabel |
| 5 theorem/model-check bridge | finite M3/M4/M5/M6/M7 evidence closed; general proof remains deferred | M8 correspondence consumer |
| 6 distributed fabric | not in M0--M10 deterministic I1+ target | deferred / future owner direction |
| 7 toolchain/backend | M8--M9 bounded reference surfaces only | **後段依存**: M7 |
| 8 applications | user-defined worlds remain LAB consumers | deferred; not a Core milestone |

## user decision gates

現在、M8 を止める owner decision item はない。ADR-0015 が owner-approved objective、
M0--M10 order、bounded execution authority を記録しているため、旧 tasks にあった
fixed-control、C1/C2、Gate/ledger 一般の「常時 owner 待ち」は current queue から外す。
それらの具体内容は該当 milestone が evidence-gated に選び、Canon へ統合する。

owner escalation は次の場合だけ行う。

| Item | Impact | Main options | Current recommendation |
| --- | --- | --- | --- |
| ADR-0015 owner-reserved condition | North Star、safety/privacy weakening、domain-to-Core promotion、mandatory v0 non-goal、irreversible public API/ABI/wire、deployment/publication、tied irreversible choice、user data/secret risk | stop and present exact alternatives/evidence; or remain inside current scope | no trigger observed; continue M8 |
| post-M10 direction | next public/distributed/product program | new owner roadmap; remain closed | decide only after M10 evidence cut |

## research discovery items

これらは user decision request ではない。指定 milestone 内で current proposal と一つの
smallest viable alternative を falsifier で比較する。

| Milestone | Research must establish | Stop / discard condition |
| --- | --- | --- |
| M2 | semantic assertion identity、evidence binding、negative control、acceptance separation | historical artifact rewrite or premature lifecycle claim |
| M3 evaluation/materialization | closed finite calculus | no general proof/runtime/conformance claim |
| M4 relation/projection | closed finite relation-first fragment | no general DAG/runtime/conformance claim |
| M5 | closed finite shared model、statement/implementation identity、proof classes | general proof/runtime overclaim |
| M6 | closed finite Surface grammar/AST/span/classification | runtime/general-theorem/public-contract overclaim |
| M7 | closed source-first checker/elaborator preserving M6 meaning | exact finite evidenceのみ; 10-row fixture matrixをSCN conformanceへ読み替えない |
| M8 | runtime algorithms that consume checked M7 output | parser/classifier bypass、model drift、hidden side table、transport-as-authority、stale resurrection、rejected-patch mutation |
| M9 | typed auth/verification extension | M8 regression failure、authority grant conflation、untyped observation leak |
| M10 | exact repo-local completion claim and remaining deferrals | any unreproduced required evidence or stale current reference |

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

No Gate/Phase exit、OBL discharge、SCN/conformance pass、I1 implementation、final public
contract、production deployment、又は product completion をこの task map は主張しない。
