# progress

最終更新: 2026-08-27 07:07 JST

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
| Logical specification | Accepted finite M10 semantics, SYS-1 lifecycle, SYS-2 ST/OW1 refinement, and SYS-3 source-derived per-locus projection are fixed at their bounded cuts. The SYS-3 retry row is only a static theory/13-derived SYS-4 refinement contract, not current M8/runtime evidence. Theory remains T1; broad I1/I2 acceptance is unclaimed | **着手可能** for SYS-4 dispatch |
| User-facing specification | One ordinary source remains semantic authority. The bounded internal clause `designated consume E.result at C` names exactly one consumer; topology cannot invent it. Final grammar, API, ABI, artifact encoding, and wire remain unfrozen | **後段依存** for public surface; SYS-4 uses internal artifacts only |
| Implementation / operation | Accepted SYS-3 cut `3013e7fe...` owns per-locus fragments and generated plans; `ded622fe...` is partial history. No endpoint executes them yet. Legacy M8 still rejects a repeated delivery id as `AlreadyConsumed`; SYS-4 must implement carrier-side idempotent return plus first/retry/competing endpoint tests while preserving M8/M10 | **着手可能** for SYS-4; SYS-5 is next |

Normative sources: `mirrorea_canon/adr/ADR-0026.md` through
`mirrorea_canon/adr/ADR-0029.md`,
`mirrorea_canon/spec/12-sys3-per-locus-projection.md`,
`mirrorea_canon/plan/01-phases.md`, and
`mirrorea_canon/theory/11-metatheory-ledger.md`. LAB execution source:
`plan/249-mirrorea-i2-systems-foundation-current-roadmap.md`.

Current direct blocker: SYS-4 must run the accepted artifacts as independent
locus-local stores/queues/endpoints, cross only generated carrier boundaries,
and bind actual runtime occurrences without source reparse, handwritten edges,
or direct cross-locus mutation. It must add the source/Core-bound carrier-side
idempotent return/wrapper with exactly one accepted M8 consume and actual first/
retry/competing-consumer tests, preserve M8/M10 duplicate-delivery behavior,
execute selected ST/OW1 profiles, fail closed, replay deterministically, and
support a process-local whole-fabric cut/save/restore/patch. SYS-4 is active;
SYS-5 is next.

## milestone map

| Milestone | Capability | Position | Direct evidence/consumer |
| --- | --- | --- | --- |
| M0--M10 | finite theory + deterministic I1+ reference profile | closed baseline | ADR-0025 / Report 2591 |
| SYS-0 | baseline, authority, one goal/control path | **completed / closed**; Report 2592 | SYS-1 |
| SYS-1 | runtime kernel/conformance separation; internal carrier/effect seam | **completed / closed**; cut `94e3707c...`, Report 2593 | SYS-2/3 |
| SYS-2 | ST/OW1 concurrency, live M9 generation, bounded ordering refinement | **completed / closed**; cut `920d3fe0...`, Report 2594 | SYS-3/4 |
| SYS-3 | per-locus artifacts and generated plans | **completed / closed**; cut `3013e7fe...`, OBL-060 static-only runtime-monitored, Report 2595 | SYS-4 |
| SYS-4 | in-process generated dispatch | **active** | SYS-5 |
| SYS-5 | typed devtools + four-locus toy world | **next**, after SYS-4 | SYS-6 |
| SYS-6 | finite I2 assurance/lifecycle closeout | pending | SYS-7 |
| SYS-7 | inactive I3 goal/entry contract only | pending/terminal | future owner program only |

## line snapshots

### Product Alpha line

Historical runnable LAB evidence only. It is not the active program or a final
product contract.

### Operational Suite line

Historical bounded operational roots remain evidence. SYS-3 changed no
runnable sample path, user command, debug surface, or sample classification.

### Mir Language line

M6/M7 ordinary source → checked Core remains the finite source path. SYS-3
adds exactly one bounded non-final internal Surface-v0 clause,
`designated consume E.result at C`, so projection can derive a consumer path
without topology inference. This does not freeze final/public grammar.

### PoseGraph line

Existing pose/renderer evidence is not Mir Core. The SYS-5 relation example is
headless sample behavior, not a View/renderer product.

### Projection/Backend line

The accepted SYS-3 cut provides crate-private, owned placement-specific fragments and
generated communication/effect/observation/persistence/source-map plans. ST is
always a semantic requirement; OW1 eligibility remains exactly-one combined
owner/source-owner locus. Its designated consumer path is source-bound and
static; no endpoint executes those artifacts yet.

### Engine/Provider line

Providers remain typed adapters and non-owners. SYS-3 generates source-bound
owner/designated service plans but no generic provider registry; SYS-4 must
materialize only the selected internal carrier lifecycle.

## validation floor

| Changed layer | Required command family |
| --- | --- |
| Canon metadata/docs | regenerate/check `mirrorea_canon/INDEX.json`, `make docs`, HTML reader regression, `git diff --check` |
| runtime kernel/backend | focused red/green boundary tests plus changed-crate and SYS-1/M10 regression |
| bounded model | exact bound/completeness, replayable edge-removal counterexamples, evidence classification |
| projection | deterministic artifact/edge tests, explicit designated consume consumer/delivery, owned Core/provenance, malformed/extra/missing/owner-moving/leaking/competing negatives |
| dispatch | no-source-reparse/no-direct-store/no-handwritten-edge tests, ST/OW1 endpoint traces, replay/cut/patch |
| proof | `lean --trust=0` and placeholder/axiom scan only when a Lean claim changes |
| milestone close | independent review, one report, commit/push, clean worktree, remote parity |

## non-claims

No broad PHASE-I1 exit, official I2 entry/exit, C-distributed/socket/WAN,
public grammar/API/ABI/wire, public carrier freeze, durable distributed
persistence, production/publication, final browser/View renderer, general OBL
discharge, arbitrary DAG/scheduler/memory/data-race theorem, production nested
relation graph, multi-owner OW, lock-free runtime, or I3 implementation is
claimed.

## user decision items vs research-discovery items

| Kind | Item | Current handling |
| --- | --- | --- |
| Self-driven | SYS-0 baseline/goal alignment | completed; Report 2592 retained |
| Self-driven | SYS-1 internal kernel/carrier/effect seam | completed at `94e3707c...`; Report 2593 |
| Self-driven | SYS-2 ST/OW1 concurrency/refinement | completed at `920d3fe0...`; OBL-058/059, Report 2594 |
| Self-driven | SYS-3 projection/artifact generation | completed at `3013e7fe...`; OBL-060 static-only runtime-monitored, Report 2595 |
| Self-driven | SYS-4 generated in-process dispatch | active; actual endpoints, ST/OW1, retry wrapper, replay/cut/patch |
| Research discovery | exact designated consumer projection | closed finite source/Core contract; runtime refinement remains SYS-4 |
| Research discovery | exact broad-I1 carrier-freeze residual | OPEN-026/027 + full carrier freeze retained; no criteria weakening |
| Owner decision | public API/ABI/wire freeze | reserved; not a current blocker |
| Owner decision | real transport / production / publication | reserved; SYS-7 remains entry-contract only |

## macro phase map

| Macro | Focus | Current position | Weight | Self-drive |
| --- | --- | --- | --- | --- |
| 0 | governance/repository memory | ADR-0026/Plan 249 active; Plan 247 and SYS-0--SYS-3 closed; SYS-4 active | medium | SYS status sync is self-driven |
| 1 | semantics/shared model | accepted finite semantics + internal lifecycle/backend + static source-derived projection | heavy | SYS-4 runtime refinement active |
| 2 | parser-free evidence | historical only; must not become new architecture | medium | maintenance only |
| 3 | source/checker/runtime | M10 source baseline; kernel/backend/projector closed; generated dispatch absent | heavy | SYS-4 active; SYS-5 next |
| 4 | executable samples | no new SYS runnable sample yet | medium | SYS-5 after SYS-4 |
| 5 | theorem/model-check | OBL-058 bounded; OBL-059 runtime; OBL-060 static finite runtime-monitored | heavy | SYS-4 runtime evidence next |
| 6 | distributed fabric | complete finite per-locus generation exists; actual dispatch absent | heavy | SYS-4 active |
| 7 | toolchain/backend | ST/OW1 and projector ready; endpoint runtime pending | heavy | SYS-4 active |
| 8 | applications | four-locus toy is future sample consumer | heavy | SYS-5 only; no Core promotion |

## feature maturity rows

| Feature | Evidence status | Remaining gate | Startability |
| --- | --- | --- | --- |
| multi-node/fabric | logical multi-locus + complete finite generated artifacts/plans | actual endpoint dispatch | **着手可能** SYS-4 |
| robustness via contracts/theorem/model-check | exact finite M3--M10 classes plus OBL-058/059/060 bounded evidence | dispatch/I2 assurance | **着手可能** SYS-4 |
| dynamic attach/detach/DAG evolution | finite patch/relation evidence + test-only DAG extension pressure | dispatch lifecycle and whole-fabric patch | **着手可能** SYS-4 |
| `atomic_cut` / higher-level ordering / memory order | high-level edges + deterministic ST + bounded OW1/model mapping | generated multi-locus execution; general memory remains deferred | **着手可能** SYS-4 |
| executable sample corpus | existing active roots and M10 commands unchanged | generated four-locus toy | **後段依存** SYS-5 |
| Mir core/runtime kernel | kernel + ST/OW1 + accepted checked projector | endpoint dispatch | **着手可能** SYS-4 |
| Mirrorea projection/fabric | per-locus fragments and complete finite generated plans | independent-locus execution | **着手可能** SYS-4 |
| typed-effect handler seam | generated source-bound owner/designated service/result plans | dispatch | **着手可能** SYS-4 |
| PrismCascade | separate performance kernel | no I2 integration required | deferred |
| View/browser/renderer | BND-007 horizon/historical LAB | final product/API work | deferred beyond SYS-5 headless view |
| upper applications | LAB consumers | no domain Core promotion | deferred |

## recent log

- 2026-08-27 07:07 JST: corrected SYS-3 source/evidence cut `3013e7fe...`
  closed the bounded source-derived designated-consumer path after M6 metadata,
  missing-producer, and signature-shadow fixes; focused 9/13/25/8/27/7/2/67,
  full runtime/workspace, scoped Clippy, and final reviews passed. OBL-060 is
  static-only `runtime-monitored`; SYS-4 is active and SYS-5 next.
- 2026-08-27 06:02 JST: RED review P1 #2 separated the static
  `ReturnExistingNoNewConsumption` identity/contract from runtime evidence.
  Legacy M8 same-delivery `AlreadyConsumed` and accepted M10 duplicate-delivery
  behavior remain unchanged; SYS-4 now explicitly owns the carrier-side
  idempotent return/wrapper and actual endpoint positive/retry/conflict tests.
- 2026-08-27 05:29 JST: independent close review found the missing
  evaluator→named-consumer E-CONSUME source/Core path. `ded622fe...` is partial
  regression evidence, OBL-060 returned to `intentionally-deferred`, and SYS-3
  reopened for the bounded non-final `designated consume E.result at C`
  AST/M6/M7/projection correction; SYS-4 is next.
- 2026-08-27 04:27 JST: SYS-3 source cut `ded622fe...` passed 25/25 focused
  projection tests and preserved SYS-2 27/27, SYS-1 13/13, Full System V1
  projection 9/9, M10 2/4/67, full runtime/workspace, format/Clippy/diff
  validation. Semantic and code-quality reviews accepted the corrected
  edge→fragment, provenance, observation-idempotence finite contract. OBL-060
  was initially classified `runtime-monitored`; the 05:29 close-review entry
  supersedes that acceptance reading while preserving these partial results.
- 2026-08-27 01:09 JST: SYS-2 source cut `920d3fe0...` passed 27/27 focused
  tests; OBL-058 is `model-checked-bounded`, OBL-059 is `runtime-monitored`.
- 2026-08-26 23:09 JST: SYS-1 source cut `94e3707c...` and independent
  semantics/code-quality ACCEPT were mirrored through PROPOSAL-030/ADR-0027.
- 2026-08-26 20:13 JST: final SYS-0 review accepted; integration cut
  `350e04b4...` was pushed with clean remote parity.
- 2026-08-05 15:53 JST: accepted M10 R5 finite source-first profile at static
  26/26 and runtime 47/47, with no broad I1/I2/public/product claim.
