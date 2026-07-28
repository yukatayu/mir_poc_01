# progress

最終更新: 2026-07-28 09:01 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project
direction, theory, ADRs, conformance, and process. Everything outside
`mirrorea_canon/` is LAB: evidence, history, implementation, and operational
notes. If LAB text conflicts with canon, canon wins.

## document role

`docs/project-status.md` is the human control view, `tasks.md` is the current
work map, `plan/` is detailed repository memory, and `docs/reports/` is
immutable task evidence. This file creates no normative decision.

## project axis

```text
正しい理論に基づき、正しく hot-plug でき、Place をまたいで
実行・通信・検証・可視化できる仮想空間システム
```

Mir, Mirrorea, PrismCascade, and the Typed-Effect Wiring Platform remain
separable. `World` and `Game` are user-defined concepts, not Mir primitives.

## final ideal

```text
.mir -> Surface parse/elaboration -> Core typed IR and obligations
     -> checker/runtime -> projection/deployment -> provider/View evidence
```

Communication, authority, failure, observation, and evolution stay explicit
at typed boundaries. Authentication is not transport identity; visualization
is not an untyped debug leak.

## current milestone position

| Axis | Status | Next boundary | Startability |
| --- | --- | --- | --- |
| Logical specification | official `T0`; v2 profile is adopted but its sole artifact is valid `fail`; P004/008/012/013/015 directions are recorded; WRK-0028 retains a current-cut C0/C2 provenance reading and completes C0-A only at that cut; WRK-0029 retains C0-B only as an opaque conditional DAG; all OBL rows `open` | C2-A equality research, C1/C3--C7 candidate comparison, T1/T2 profiles, shared formal model | conditional |
| User-facing specification | bounded notation/scenario/sample evidence; v0 direction is Participant-only with explicit scalar terminal and excluded `return`; WRK-0027 confirms correspondence is not implicitly supplied by displayed indexed rules | exact grammar/domain, scalar candidate comparison, rejection diagnostic | autonomous research then Canon process |
| Implementation / operation | bounded Surface/current-L2/Product Alpha/Full System/operational/Lean evidence is runnable | P016 profile/authorization, exact target fragment, C-static timing | later dependency |

Current exact blockers:

1. The v2 artifact derives `fail` because the fixed source-hierarchy control
   and three fixed LAB notice controls drifted. The scoped audit finds
   governance/readability changes only; O0 still did not authorize rebase or a
   retry.
2. G0-D3 remains deferred and cannot consume the v2 `fail`; no G0 exit / T1
   entry record exists.
3. T1/T2 lack canonical phase profiles.
4. WRK-0024 falsifies the inference that owner-serial submitted writes alone
   provide SCN-02 atomic read-dependent behavior. WRK-0025/0026 are frozen on
   their registered commands, not on C0/C2 semantic results. WRK-0027 retains
   the C6 source boundary: SCN-08 scalar/terminal correspondence is not supplied
   by the displayed indexed rules, but no scalar representation is selected.
   Snapshot/evaluation/pending semantics, request/replay identity, served/admission
   facets, scalar candidate comparison, and total domain remain to be bound.
5. No accepted shared Core/Config/Step/WellFormed/elaboration/history model
   exists for T1/T2 proof-facing packages. WRK-0028 retains only a source-local
   C0/C2 fact manifest: it confirms proposal directions are not current rules
   and does not make a shared model accepted.
6. P016 records bootstrap then C-static formal entry, but Canon has no
   lifecycle profile or phase/conformance reconciliation yet.

The last source-cut screen selected no new WRK from its reviewed delta. That
is a LAB priority result, not a permanent restriction on ADR-0014. A fresh
literal-transcription or conditional-lemma candidate may proceed only after
its own standing-eligibility preflight.

Sources: `mirrorea_canon/plan/01-phases.md`,
`mirrorea_canon/theory/11-metatheory-ledger.md`,
`plan/196-t0-t2-implementation-entry-roadmap.md`,
`plan/197-i1-bootstrap-decision-and-readiness-audit.md`.

## milestone map

| Phase | Primary aim | Current position | Self-drive |
| --- | --- | --- | --- |
| T0 | vocabulary, decisions, G0 | official current; v2 profile validly reports fixed-control drift | research/preparation yes; exit no |
| T1 | calculus, G1-G3 statements, final SCN expectations | no official entry | ADR-0014-eligible composition research after recorded directions |
| T2 | OBL-020/021/002 skeletons and G5 statements | later | skeleton research after accepted model; exit no |
| I1 | single-process reference implementation | later | blocked on owner-selected authorization route and C-static timing |
| I2 | in-process multi-locus | later | blocked on I1 |
| I3 | real transport | later; first real LAN phase | blocked on I2 and transport ADR |
| I4-I6 | persistence/patch, View, distributed persistence/federation | later | sequential dependency |

Current Canon T2 is narrower than proven I1 readiness. P016 records the
narrow-T2 direction: a separately accepted I1-readiness / bootstrap record
must bind all-SCN / G0-G7 statement-level criteria, OBL-003/027 classification,
C-static timing, and scoped production authorization. The actual profile and
Canon reconciliation remain open; this does not imply carrier freeze or
C-runtime conformance.

## line snapshots

### Product Alpha line

`samples/product-alpha1/` has bounded package check/run, attach, save/load,
native bundle, and two-process Docker TCP evidence. `.mir` is not its direct
execution input. It is not final ABI, WAN federation, distributed durability,
or official implementation status.

### Operational Suite line

The WorldCore through TwoShard/Gradient suite exercises documented bounded
contracts and evidence. It is not final shared-space catalog or distributed
durable execution.

### Mir Language line

Surface parser, indexed-state checks, elaboration, admission, source patch,
static devtools, and a bounded computational fragment are runnable LAB
evidence. Final grammar, public error contract, and Canon-aligned common proof
model remain open.

### PoseGraph line

PoseGraph has bounded sample evidence. Its performance-sensitive kernel remains
separate from Mir runtime semantics.

### Projection/Backend line

Projection, deployment, and backend/provider artifacts are bounded LAB
evidence or planned boundaries. BND-006 and later implementation phases govern
their normative realization.

### Engine/Provider line

Provider and engine adapters are typed external boundaries. They do not define
world semantics, authentication, authorization, or Mir core.

## validation floor

| Evidence | Current command |
| --- | --- |
| docs and source hierarchy | `python3 scripts/validate_docs.py`; `python3 scripts/check_source_hierarchy.py` |
| Canon metadata | `python3 meta/build-index.py --check` from `mirrorea_canon/` |
| Surface LAB | `python3 scripts/surface_mir_samples.py check-all --format json` |
| Lean statement shapes | direct `lake env lean` under `samples/lean/lab-statements/` |
| runnable dashboards | commands in `samples_progress.md` |

Run the changed layer's focused validation plus docs/index/hierarchy checks.
Passing LAB evidence does not move a Gate, Phase, or OBL.

## non-claims

No Gate/Phase exit, OBL discharge, final proof, conformance result, final
grammar/API/ABI, production implementation, real federation, distributed
durable save/load, or public product completion is claimed.

## user decision items vs research-discovery items

| Kind | Item | Current handling |
| --- | --- | --- |
| Owner / Canon | fixed-control drift | scoped audit complete; decide whether to retain pins/defer or start a normal rebase proposal; no silent rebase or retry |
| Owner / Canon | G0-D3 | unavailable until a valid `pass` artifact exists; current v2 `fail` has no exit effect |
| Owner / Canon | T1/T2/I1 lifecycle contract | P016 direction is recorded; define actual profiles, Gate-to-ledger mapping, proof-skeleton evidence class, and phase/conformance wording |
| Research | selected semantic composition | WRK-0028 completed the C0/C2 source-role re-anchor and subsumes C0-A at its pinned cut; WRK-0029 retained C0-B as an opaque conditional DAG; pre-register C2-A, then compare C1/C3--C7 at one source cut before a shared model |
| Research evidence | WRK-0024 C1 countermodel | owner-serial writes do not alone imply atomic read-dependent update; no repair selected |
| Research evidence | WRK-0027 C6 source comparison | SCN-08's scalar/terminal needs explicit correspondence; no invalidity or representation conclusion |
| Owner / Canon | resulting Canon amendments | only after C0--C7 identifies a minimum rule/profile change; do not infer one from a proposal record |
| Research | conservative statement preflight | test ADR-0014 eligibility; open L3 only for non-duplicate existing-lane literal/conditional evidence |
| Research after decisions | shared model and Gate packages | compare, formalize, falsify, validate, review, and prepare acceptance packets |
| Later dependency | runtime, conformance, transport, federation | do not preempt the theory/lifecycle contract |

## macro phase map

| Macro | Focus | Current position | Weight | Self-drive |
| --- | --- | --- | --- | --- |
| 0 | repository memory and governance | current snapshots and Plan 196 synchronized | light | yes |
| 1 | semantic kernel | directions recorded; proof-facing composition incomplete | heavy | C0--C7 research, then existing-lane packages |
| 2 | parser-free validation | compatibility anchors runnable | medium | maintenance/reproduction |
| 3 | compile-ready actualization | bounded Surface/Full System evidence exists | heavy | production widening deferred |
| 4 | sample expansion | active roots runnable | heavy | maintenance before I1 |
| 5 | theorem/model-check bridge | drafts, countermodels, frozen/not-promoted WRKs exist | heavy | conservative L3 preflight now; main line after shared model |
| 6 | distributed fabric | later | heavy | after I1/I2 |
| 7 | toolchain/backend | bounded LAB only | heavy | later |
| 8 | applications | bounded samples only | heavy | outside T0-T2 critical path |

## feature maturity rows

| Feature | Evidence status | Remaining gate | Startability |
| --- | --- | --- | --- |
| multi-node / fabric | bounded local/two-process LAB | I2/I3, transport and distributed contracts | later dependency |
| contracts / theorem / model-check | statement drafts, countermodels, L3/frozen evidence, dependency audits | accepted shared model, exact statements/skeletons, ledger status | conservative preflight only |
| dynamic attach/detach / DAG-safe evolution | bounded LAB sample evidence | G7 and implementation contract | later dependency |
| `atomic_cut` / ordering | Canon theory plus bounded hook/cut evidence | noncircular G5 model and OBL-009..014 statements | after model choice |
| executable sample corpus | runnable active roots | conformance and public workflow | maintenance |
| Mir language foundation | bounded parse/check/elaboration/compute evidence | exact Surface/Core fragment, P016 profile/authorization, C-static timing | composition research then Canon process |
| Mirrorea fabric | bounded alpha/runtime evidence | I2/I3 semantics and transport ADR | later dependency |
| Typed-Effect Wiring | typed adapter evidence | public contract and authority integration | later dependency |
| PrismCascade | separable bounded sample/kernel evidence | dedicated semantics/performance contract | later dependency |
| upper applications | user-defined sample worlds | stable lower-layer workflow | later dependency |

## recent log

- 2026-07-28 09:01 JST: WRK-0029 retained only the conditional fact that a
  rank-increasing opaque `Lex -> Parse -> Static -> WS -> Terminal` graph is
  acyclic. It defines none of those roles and selects no grammar, static
  semantics, `WellScoped` predicate, outcome, Diagnostic, or Core relation;
  C2-A is the next candidate.

- 2026-07-28 08:48 JST: Oracle advisory and local cut comparison established
  that C0-A would duplicate WRK-0028's source-authority provenance span. It is
  `complete-by-R0` only at that pinned cut; C0-B is the next bounded
  conditional-lemma candidate and selects no grammar, `WellScoped` predicate,
  Diagnostic, outcome, or Core relation.

- 2026-07-28 08:31 JST: WRK-0028 retained a current-cut, source-local C0/C2
  manifest. Current grammar/theory wording and bounded proposal directions are
  distinguished without selecting a semantic reconciliation, identity, or
  shared model.

- 2026-07-28 06:23 JST: a temporary GPT-5.6 Sol Pro review, checked against
  current Canon, recommended a common-cut re-anchor and staged C0/C2/C1/C3--C7
  candidate comparison. Plan 200 records this as LAB execution order only; no
  grammar, Core, identity carrier, failure family, or shared model was chosen.

- 2026-07-28 06:04 JST: committed/pushed WRK-0027, then ran its registered
  source comparison. SCN-08's scalar `room_anchor` and `default_pose` are not
  given a silent correspondence by the displayed indexed Surface/Core rules;
  P015's explicit-correspondence boundary remains open. No scalar candidate or
  SCN invalidity was selected.

- 2026-07-28 05:57 JST: froze WRK-0026 immediately after its registered token
  audit required the absent contiguous phrase `copied/replayed requests` in
  P013. This is a command falsifier, not a conclusion about M1/replay; C2
  remains open and any restart needs a new pre-registration.

- 2026-07-28 05:52 JST: froze WRK-0025 immediately after its registered token
  audit required `CallArgs` in displayed `spec/02`, where it does not occur.
  The token exists only in P004's candidate EBNF. No C0 inventory conclusion
  was retained; a new pre-registration is required rather than repairing or
  rerunning the frozen record.

- 2026-07-28 05:44 JST: committed/pushed WRK-0024 pre-registration, then
  reproduced its finite Lean countermodel at `--trust=0`. Two stale replies can
  lead to serial writes 7 then 6 from HP 10, unlike owner-side sequential
  damage 3 then 4. This is a non-implication boundary, not a Canon execution,
  SCN change, or snapshot/pending design choice.

- 2026-07-28 05:35 JST: recorded the owner-accepted P004/P008/P012/P013/P015/
  P016 directions without changing a Core rule, SCN, Gate, Phase, OBL, or
  implementation status. A temporary GPT-5.6 Sol Pro review and local source
  check found that the directions are not yet composition-closed; Plan 199 now
  orders the C0--C7 countermodel and safe-inference research before a shared
  operational model.

- 2026-07-28 05:13 JST: audited the four v2 fixed-control mismatches against
  the accepted evidence cut. They are intentional source-hierarchy and agent/
  reader governance changes, not Mir semantics, SCN, OBL, Gate, or Phase drift.
  No pin was rebased, artifact regenerated, or lifecycle claim advanced.
- 2026-07-28 04:30 JST: independently reviewed the post-v2 autonomous theory
  frontier with GPT-5.6 Sol Pro. The v2 `fail` is fixed-control document drift
  only; after candidate-specific comparison, no new ADR-0014 L3 record was
  identified and no Lean experiment was repeated. The no-candidate result is a
  current-cut LAB selection result, not a claim that Plan 195 narrows the
  standing Canon predicate or permanently closes future research.
- 2026-07-28 03:51 JST: applied owner O0 through PROPOSAL-014 and the ADR-0013
  v2 amendment, then recorded exactly one direct-child v2 artifact. Independent
  Git-blob, JSON-shape, RFC 8785 digest, and topology validation passed. The
  artifact is valid `fail` because four fixed controls drifted; v1 remains
  nonconforming history. G0-D3, G0 exit, T1, I1, OBL, conformance, and
  implementation status did not move.
- 2026-07-28 00:41 JST: completed an I1 bootstrap/readiness audit with a
  planner, an independent reviewer, and GPT-5.6 Sol Pro Oracle. It found no
  current official start path and isolated the C-static entry/exit tension.
  Plan 197 recommends a separate I1-readiness record only if the owner selects
  narrow T2; integrated and phase-contract alternatives remain open. The first
  owner checkpoint remains the T0 profile/artifact route. No Canon, Gate/Phase,
  OBL, sample, implementation, or public status moved.
- 2026-07-27 19:38 JST: audited the path from official T0 through T2 and the
  I1 implementation entrance with independent sub-agent and GPT-5.6 Sol Pro
  review. Plan 196 now separates official owner-reserved transitions from
  autonomous ADR-0014 research, defines the dependency DAG and package gates,
  and records that current Canon T2 is narrower than all-SCN I1 readiness. No
  Canon, Gate/Phase, OBL, sample, implementation, or public status moved.
- 2026-07-26 23:54 JST: added the Japanese HTML orientation map and Mermaid
  source; browser, print, Mermaid, Product Alpha, and operational validations
  passed. This was a reader-facing LAB map only.
- 2026-07-25 01:50 JST: completed the whole-theory source audit; identified
  the T0 `pass` / `derived-pass` conflict and retained all semantic/formal
  boundaries as decision requests or reserved interfaces.
