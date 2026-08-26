# progress

最終更新: 2026-08-27 01:09 JST

**Canon notice:** `mirrorea_canon/` is normative. Everything outside
`mirrorea_canon/` is LAB; canon wins. This file is a concise LAB snapshot and
creates no Canon, Gate, Phase, proof, or conformance decision.

## document role

`docs/project-status.md` is the concise human control view. The sole current
execution roadmap is
`plan/249-mirrorea-i2-systems-foundation-current-roadmap.md`. Closed Plan 247
retains the accepted M0--M10 regression history and is not a parallel queue.

## project axis

```text
正しい理論に基づき、正しく hot-plug でき、Place をまたいで
実行・通信・検証・可視化できる仮想空間システム
```

Mir, Mirrorea, PrismCascade, and the Typed-Effect Wiring Platform stay
separable. World/Game/Avatar/Bird remain sample or library vocabulary, never
Mir Core primitives.

## final ideal

```text
ordinary .mir source
→ semantic check / elaboration
→ ownership / authority / dependency / effect / failure / lifetime
→ per-locus executable artifacts + generated communication
→ process/network execution
→ typed trace / diagnostics / devtools
→ checked save/load / patch / hot-plug
→ View/provider/browser/renderer projection
→ persistent virtual-space system
```

The active program covers the in-process systems-foundation segment only.

## current milestone position

| Axis | Current status | Startability |
| --- | --- | --- |
| Logical specification | Accepted finite M10 semantics, the SYS-1 internal lifecycle, and the SYS-2 ST/OW1 ordering/authority-generation refinement are fixed for their exact finite scopes. Theory remains T1; broad I1/I2 acceptance is unclaimed | **着手可能** for bounded SYS-3 projection |
| User-facing specification | One ordinary source remains semantic authority; Plan 249 requires generated per-locus artifacts/plans but does not freeze final CLI spelling, grammar, API, ABI, or wire | **着手可能** for internal projection diagnostics; **後段依存** for public surface |
| Implementation / operation | Source cut `920d3fe0...` runs selected ST/OW1 kernel behavior with actual M8 linearization and acknowledged M9 generation visibility. No per-locus generator or actual locus dispatch exists yet | **着手可能** for SYS-3; SYS-4 is next |

Normative sources: `mirrorea_canon/adr/ADR-0026.md`,
`mirrorea_canon/adr/ADR-0027.md`, `mirrorea_canon/adr/ADR-0028.md`,
`mirrorea_canon/plan/01-phases.md`, and
`mirrorea_canon/theory/11-metatheory-ledger.md`. LAB execution source:
`plan/249-mirrorea-i2-systems-foundation-current-roadmap.md`.

Current direct blocker: checked global Core does not yet deterministically
produce independently executable locus programs and complete generated
communication/effect/observation/persistence plans. SYS-3 must preserve owner,
source/Core provenance, failure/effect/authority, relation/fallback lineage,
designated non-reexecution, and the SYS-2 backend requirements without a
handwritten interface or runtime semantic reconstruction. SYS-3 is active;
SYS-4 remains next.

## milestone map

| Milestone | Capability | Position | Direct evidence/consumer |
| --- | --- | --- | --- |
| M0--M10 | finite theory + deterministic I1+ reference profile | closed baseline | ADR-0025 / Report 2591 |
| SYS-0 | baseline, authority, one goal/control path | **completed / closed**; Report 2592 | SYS-1 |
| SYS-1 | runtime kernel/conformance separation; internal carrier/effect seam | **completed / closed**; cut `94e3707c...`, Report 2593 | SYS-2/3 |
| SYS-2 | ST/OW1 concurrency, live M9 generation, bounded ordering refinement | **completed / closed**; cut `920d3fe0...`, Report 2594 | SYS-3/4 |
| SYS-3 | per-locus artifacts and generated plans | **active** | SYS-4 |
| SYS-4 | in-process generated dispatch | **next**, after SYS-3 | SYS-5 |
| SYS-5 | typed devtools + four-locus toy world | pending | SYS-6 |
| SYS-6 | finite I2 assurance/lifecycle closeout | pending | SYS-7 |
| SYS-7 | inactive I3 goal/entry contract only | pending/terminal | future owner program only |

## line snapshots

### Product Alpha line

Historical runnable LAB evidence only. It is not the active program or a final
product contract.

### Operational Suite line

Historical bounded operational roots remain evidence. SYS-2 changed no
runnable sample path, user command, debug surface, or sample classification.

### Mir Language line

M6/M7 ordinary source → checked Core remains the accepted finite source path.
SYS-3 projects that Core; SYS-2 added no Surface memory-order vocabulary and
did not reopen final grammar.

### PoseGraph line

Existing pose/renderer evidence is not Mir Core. The SYS-5 relation example is
headless sample behavior, not a View/renderer product.

### Projection/Backend line

ST and bounded OW1 are executable internal backend evidence. Executable
per-locus projection does not yet exist; SYS-3 must produce artifacts/plans
without exposing the concrete mailbox or worker layout.

### Engine/Provider line

Providers remain typed adapters and non-owners. The designated remote-input
effect lifecycle now has selected ST/OW1 ordering and source-owner-derived
result evidence, but no generic registry or generated handler plan yet.

## validation floor

| Changed layer | Required command family |
| --- | --- |
| Canon metadata/docs | regenerate/check `mirrorea_canon/INDEX.json`, `make docs`, `git diff --check` |
| runtime kernel/backend | focused red/green boundary tests plus changed-crate and SYS-1/M10 regression |
| bounded model | exact bound/completeness, replayable edge-removal counterexamples, evidence classification |
| projection/dispatch | deterministic artifact/edge tests, no-manual-edge/no-direct-store negatives, replay |
| proof | `lean --trust=0` and placeholder/axiom scan only when a Lean claim changes |
| milestone close | independent review, one report, commit/push, clean worktree, remote parity |

## non-claims

No broad PHASE-I1 exit, official I2 entry/exit, C-distributed/socket/WAN,
public grammar/API/ABI/wire, public carrier freeze, durable distributed
persistence, production/publication, final browser/View renderer, general OBL
discharge, arbitrary DAG/scheduler/memory/data-race theorem, multi-owner OW,
lock-free runtime, or I3 implementation is claimed.

## user decision items vs research-discovery items

| Kind | Item | Current handling |
| --- | --- | --- |
| Self-driven | SYS-0 baseline/goal alignment | completed; Report 2592 retained |
| Self-driven | SYS-1 internal kernel/carrier/effect seam | completed at `94e3707c...`; runtime-monitored, Report 2593 |
| Self-driven | SYS-2 ST/OW1 concurrency/refinement | completed at `920d3fe0...`; OBL-058 bounded model + OBL-059 runtime, Report 2594 |
| Self-driven | SYS-3 projection/artifact generation | active; deterministic checked-Core-derived artifacts/plans |
| Research discovery | finite relation-DAG extension boundary | select one conservative pressure case in SYS-3; no arbitrary theorem |
| Research discovery | exact broad-I1 carrier-freeze residual | OPEN-026/027 + full carrier freeze retained; no criteria weakening |
| Owner decision | public API/ABI/wire freeze | reserved; not a current blocker |
| Owner decision | real transport / production / publication | reserved; SYS-7 remains entry-contract only |

## macro phase map

| Macro | Focus | Current position | Weight | Self-drive |
| --- | --- | --- | --- | --- |
| 0 | governance/repository memory | ADR-0026/Plan 249 active; Plan 247 closed; SYS-0--SYS-2 synchronized | medium | SYS close sync is self-driven |
| 1 | semantics/shared model | accepted finite semantics + internal lifecycle + bounded ST/OW1 refinement | heavy | SYS-3 consumes without reopening general theory |
| 2 | parser-free evidence | historical only; must not become new architecture | medium | maintenance only |
| 3 | source/checker/runtime | M10 source baseline; kernel/backend prerequisites closed | heavy | SYS-3 active; SYS-4 next |
| 4 | executable samples | no SYS runnable sample change yet | medium | SYS-5 after SYS-4 |
| 5 | theorem/model-check | OBL-058 bounded + OBL-059 runtime; earlier classes retained | heavy | SYS-3 projection evidence next |
| 6 | distributed fabric | per-locus generation/dispatch not yet realized | heavy | SYS-3 active, SYS-4 next |
| 7 | toolchain/backend | ST/OW1 internal backend ready; projector/runtime/devtools later | heavy | SYS-3 then SYS-4/5 |
| 8 | applications | four-locus toy is future sample consumer | heavy | SYS-5 only; no Core promotion |

## feature maturity rows

| Feature | Evidence status | Remaining gate | Startability |
| --- | --- | --- | --- |
| multi-node/fabric | logical multi-locus M10 + single-owner threaded backend evidence | generated artifact + actual endpoint dispatch | **着手可能** SYS-3, then SYS-4 |
| robustness via contracts/theorem/model-check | exact finite M3--M10 classes plus OBL-058/059 | projection/dispatch/I2 finite assurance | **着手可能** SYS-3 |
| dynamic attach/detach/DAG evolution | finite patch/relation evidence | artifact/dispatch lifecycle and one finite DAG pressure | **着手可能** bounded SYS-3 pressure; full path later |
| `atomic_cut` / higher-level ordering / memory order | high-level edges + deterministic ST + bounded OW1/model mapping | generated multi-locus use; general memory remains deferred | **後段依存** SYS-4/6 |
| executable sample corpus | existing active roots and M10 commands unchanged | generated four-locus toy | **後段依存** SYS-5 |
| Mir core/runtime kernel | crate-private kernel + ST/OW1 + acked M9 generation visibility | per-locus placement and actual endpoint dispatch | **着手可能** SYS-3 |
| Mirrorea projection/fabric | preservation boundary only | per-locus generation and actual dispatch | **着手可能** SYS-3 |
| typed-effect handler seam | source-owner-derived remote result/consume in ST/OW1 | generated `EffectHandlerPlan` and dispatch | **着手可能** SYS-3 |
| PrismCascade | separate performance kernel | no I2 integration required | deferred |
| View/browser/renderer | BND-007 horizon/historical LAB | final product/API work | deferred beyond SYS-5 headless view |
| upper applications | LAB consumers | no domain Core promotion | deferred |

## recent log

- 2026-08-27 01:09 JST: SYS-2 source cut `920d3fe0...` passed 27/27 focused
  tests and preserved SYS-1/M10/full-runtime checks; four review lanes accepted
  the corrected ST/OW1, actual M8 ordering, M9 ack-before-publish, and bounded
  model scope. OBL-058 is `model-checked-bounded`, OBL-059 is
  `runtime-monitored`; SYS-3 is active and SYS-4 next while theory T1 and broad
  PHASE-I1/I2 acceptance remain unchanged.
- 2026-08-26 23:09 JST: SYS-1 source cut `94e3707c...` and independent
  semantics/code-quality ACCEPT were mirrored through PROPOSAL-030/ADR-0027.
  The crate-private owner/designated-input lifecycle is runtime-monitored;
  SYS-2 became active while theory T1 and broad lifecycle stayed unchanged.
- 2026-08-26 20:13 JST: final SYS-0 review accepted with no P0/P1/P2;
  integration cut `350e04b4...` was pushed with clean remote parity.
- 2026-08-05 15:53 JST: accepted M10 R5 finite source-first profile at static
  26/26 and runtime 47/47, with no broad I1/I2/public/product claim.
