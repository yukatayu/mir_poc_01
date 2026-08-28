# progress

最終更新: 2026-08-28 14:09 JST

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
| Logical specification | Accepted finite M10 semantics and bounded SYS-1--SYS-5 contracts now reach source-derived artifacts, generated-plan-only locus endpoints, selected ST/eligible-OW1 execution, a four-locus headless toy, source-derived leave/fallback/fresh reacquire, and observer-safe source-to-occurrence causality. Theory remains T1; broad I1/I2 acceptance is unclaimed | **着手可能** for SYS-6 assurance |
| User-facing specification | One ordinary source remains semantic authority. The provisional internal `project-loci`, `run-local`, and `inspect` commands plus a short walkthrough expose the bounded local toy and joined causal view. Final grammar, API, ABI, artifact encoding, devtools schema, and wire remain unfrozen | **着手可能** for SYS-6; public surface remains **後段依存** |
| Implementation / operation | Accepted SYS-5 cut `53a21e64...` runs the four-locus workflow through actual generated endpoints and preserves redaction, fallback separation, clone-prepared ST failure atomicity, exact post-leave cut/reacquire lineage, patch, revocation, and verification evidence. Focused 10/27/28/8/17/12/3/4, `mir-runtime --all-targets`, M10 2/4/67, and three reviews passed. No finite SYS-6 conformance profile exists yet | **着手可能** for SYS-6; SYS-7 is next |

Normative sources: `mirrorea_canon/adr/ADR-0026.md` through
`mirrorea_canon/adr/ADR-0031.md`,
`mirrorea_canon/spec/12-sys3-per-locus-projection.md`,
`mirrorea_canon/spec/13-sys4-in-process-generated-dispatch.md`,
`mirrorea_canon/spec/14-sys5-local-toy-devtools.md`,
`mirrorea_canon/architecture/04-runtime-carriers.md`,
`mirrorea_canon/plan/01-phases.md`, and
`mirrorea_canon/theory/11-metatheory-ledger.md`. LAB execution source:
`plan/249-mirrorea-i2-systems-foundation-current-roadmap.md`.

Current direct blocker: SYS-6 must produce one finite source-first I2 profile
and provisional `conform-i2` report over the accepted SYS-3--SYS-5 cuts. It
must independently test projection determinism, artifact owner preservation,
generated-communication completeness, actual ST/selected-OW correspondence,
no hidden/direct remote store or authority/state minting, failure containment,
relation/fallback/designated coherence, save/patch no-stale/no-mutation, and
observer safety. Each result needs exact source/implementation identity,
replay command, evidence class, residual, and non-claim. SYS-6 is active;
SYS-7 is next.

## milestone map

| Milestone | Capability | Position | Direct evidence/consumer |
| --- | --- | --- | --- |
| M0--M10 | finite theory + deterministic I1+ reference profile | closed baseline | ADR-0025 / Report 2591 |
| SYS-0 | baseline, authority, one goal/control path | **completed / closed**; Report 2592 | SYS-1 |
| SYS-1 | runtime kernel/conformance separation; internal carrier/effect seam | **completed / closed**; cut `94e3707c...`, Report 2593 | SYS-2/3 |
| SYS-2 | ST/OW1 concurrency, live M9 generation, bounded ordering refinement | **completed / closed**; cut `920d3fe0...`, Report 2594 | SYS-3/4 |
| SYS-3 | per-locus artifacts and generated plans | **completed / closed**; cut `3013e7fe...`, OBL-060 static-only runtime-monitored, Report 2595 | SYS-4 |
| SYS-4 | in-process generated dispatch | **completed / closed**; cut `22196f93...`, bounded runtime-monitored evidence | SYS-5 |
| SYS-5 | typed devtools + four-locus toy world | **completed / closed**; cut `53a21e64...`, OBL-062 bounded runtime-monitored, Report 2597 | SYS-6 |
| SYS-6 | finite I2 assurance/lifecycle closeout | **active** | SYS-7 |
| SYS-7 | inactive I3 goal/entry contract only | **next / terminal** | future owner program only |

## line snapshots

### Product Alpha line

Historical runnable LAB evidence only. It is not the active program or a final
product contract.

### Operational Suite line

Historical bounded operational roots remain evidence. SYS-5 adds the active
headless sample root `samples/clean-near-end/mirrorea-i2-local-toy/` and the
provisional internal `project-loci`, `run-local`, and `inspect` workflow. It is
a bounded reproducible LAB workflow, not a public product/debug ABI. SYS-6 now
consumes its exact artifacts and trace rather than replacing them with an
expected-result fixture.

### Mir Language line

M6/M7 ordinary source → checked Core remains the finite source path. SYS-3
adds the bounded non-final internal `designated consume E.result at C` clause;
SYS-5 adds optional explicit relation anchor loci for the accepted toy so leave
and reacquire bind to checked locus identity. Neither change freezes final/
public grammar.

### PoseGraph line

Existing pose/renderer evidence is not Mir Core. The SYS-5 relation example is
headless sample behavior, not a View/renderer product.

### Projection/Backend line

The accepted SYS-3 cut provides crate-private, owned placement-specific
fragments and generated plans. SYS-4 executes only those plans through explicit
locus-local endpoints. SYS-5 composes the same projector/runtime into the
four-locus user workflow and observer-safe joined trace without direct remote
store access or handwritten communication. ST owns the whole-fabric cut/
restore and bounded patch profile; OW1 cut/patch remains typed
`BackendIneligible`, and SYS-6 must classify selected ST/OW evidence exactly.

### Engine/Provider line

Providers remain typed adapters and non-owners. SYS-5 consumes the source-bound
owner/designated service and result lifecycle and exposes only reference-safe
observer evidence; it does not add a generic provider registry or transport-
created authority. SYS-6 must preserve that boundary in conformance rows.

## validation floor

| Changed layer | Required command family |
| --- | --- |
| Canon metadata/docs | regenerate/check `mirrorea_canon/INDEX.json`, `make docs`, HTML reader regression, `git diff --check` |
| runtime kernel/backend | focused red/green boundary tests plus changed-crate and SYS-1/M10 regression |
| bounded model | exact bound/completeness, replayable edge-removal counterexamples, evidence classification |
| projection | deterministic artifact/edge tests, explicit designated consume consumer/delivery, owned Core/provenance, malformed/extra/missing/owner-moving/leaking/competing negatives |
| dispatch | no-source-reparse/no-direct-store/no-handwritten-edge tests, ST/OW1 endpoint traces, replay/cut/patch |
| local toy/devtools | four-locus source/project/run/inspect, source-derived leave/fallback/fresh reacquire, joined provenance/redaction, save/restore, accepted/rejected patch, revocation/verification, M10 regression |
| I2 conformance | source-first row inventory, positive/falsifier controls, exact evidence classes/cuts/replay commands/non-claims, independent assurance/lifecycle review |
| proof | `lean --trust=0` and placeholder/axiom scan only when a Lean claim changes |
| milestone close | independent review, one report, commit/push, clean worktree, remote parity |

## non-claims

No broad PHASE-I1 exit, official I2 entry/exit, C-distributed/socket/WAN,
public grammar/API/ABI/wire/devtools schema, public carrier freeze, durable
distributed persistence, production/publication, final browser/View renderer,
general OBL discharge, arbitrary DAG/scheduler/memory/data-race theorem,
production nested relation graph, multi-owner OW, OW1 whole-fabric cut/patch,
arbitrary patch shape, lifecycle-patch commutation, lock-free runtime, SYS-6
conformance acceptance, or I3 implementation is claimed.

## user decision items vs research-discovery items

| Kind | Item | Current handling |
| --- | --- | --- |
| Self-driven | SYS-0 baseline/goal alignment | completed; Report 2592 retained |
| Self-driven | SYS-1 internal kernel/carrier/effect seam | completed at `94e3707c...`; Report 2593 |
| Self-driven | SYS-2 ST/OW1 concurrency/refinement | completed at `920d3fe0...`; OBL-058/059, Report 2594 |
| Self-driven | SYS-3 projection/artifact generation | completed at `3013e7fe...`; OBL-060 static-only runtime-monitored, Report 2595 |
| Self-driven | SYS-4 generated in-process dispatch | completed at `22196f93...`; actual endpoints, selected ST/OW1, retry wrapper, replay, ST cut/patch |
| Self-driven | SYS-5 typed devtools/four-locus toy | completed at `53a21e64...`; OBL-062 bounded runtime-monitored, Report 2597 |
| Self-driven | SYS-6 finite I2 assurance/conformance | active; exact row inventory, falsifiers, evidence classes, lifecycle/non-claims |
| Research discovery | exact designated consumer projection/runtime refinement | closed finite source/Core and SYS-4 endpoint contract; preserve wrapper/legacy M8 distinction |
| Research discovery | exact broad-I1 carrier-freeze residual | OPEN-026/027 + full carrier freeze retained; no criteria weakening |
| Owner decision | public API/ABI/wire freeze | reserved; not a current blocker |
| Owner decision | real transport / production / publication | reserved; SYS-7 remains entry-contract only |

## macro phase map

| Macro | Focus | Current position | Weight | Self-drive |
| --- | --- | --- | --- | --- |
| 0 | governance/repository memory | ADR-0026/Plan 249 active; Plan 247 and SYS-0--SYS-5 closed; SYS-6 active | medium | SYS status sync is self-driven |
| 1 | semantics/shared model | accepted finite semantics + internal lifecycle/backend + source-derived projection/dispatch/local toy | heavy | SYS-6 assurance active; no new semantic frontier |
| 2 | parser-free evidence | historical only; must not become new architecture | medium | maintenance only |
| 3 | source/checker/runtime | M10 source baseline plus kernel/backend/projector, endpoint dispatch, and local workflow closed | heavy | SYS-6 conformance active |
| 4 | executable samples | four-locus headless toy command/view accepted at `53a21e64...` | medium | regression/assurance; public product later |
| 5 | theorem/model-check | OBL-058 bounded; OBL-059--062 finite runtime evidence runtime-monitored | heavy | SYS-6 evidence classification active |
| 6 | distributed fabric | finite generated artifacts cross actual in-process locus endpoints and drive local toy | heavy | SYS-6 assurance active; real transport deferred |
| 7 | toolchain/backend | ST/eligible-OW1 runtime plus provisional project/run/inspect commands | heavy | SYS-6 `conform-i2` active |
| 8 | applications | four-locus toy is accepted sample/library consumer | heavy | SYS-6 assures; no Core promotion |

## feature maturity rows

| Feature | Evidence status | Remaining gate | Startability |
| --- | --- | --- | --- |
| multi-node/fabric | generated artifacts cross actual in-process endpoints and drive the accepted four-locus toy | finite I2 conformance and lifecycle closeout | **着手可能** SYS-6 |
| robustness via contracts/theorem/model-check | exact finite M3--M10, OBL-058--062, SYS-4 runtime, and SYS-5 negative paths | one classified I2 assurance profile | **着手可能** SYS-6 |
| dynamic attach/detach/DAG evolution | source-derived leave/fresh reacquire, ST cut/restore, accepted/rejected patch, test-only DAG pressure | conformance rows; later general evolution | **着手可能** SYS-6 |
| `atomic_cut` / higher-level ordering / memory order | high-level edges + deterministic ST + bounded OW1/model/endpoint mapping + causal view | selected visibility/correspondence rows; general memory deferred | **着手可能** SYS-6 |
| executable sample corpus | active four-locus toy root plus provisional project/run/inspect workflow | `conform-i2`; public/product surface later | **着手可能** SYS-6 |
| Mir core/runtime kernel | kernel + ST/eligible-OW1 + checked projector + endpoint dispatch + local toy | finite assurance profile | **着手可能** SYS-6 |
| Mirrorea projection/fabric | finite artifacts/plans execute across independent locus endpoints with joined trace | completeness/correspondence assurance | **着手可能** SYS-6 |
| typed-effect handler seam | source-bound owner/designated service/result plans plus revocation/verification view | containment/no-mint assurance | **着手可能** SYS-6 |
| PrismCascade | separate performance kernel | no I2 integration required | deferred |
| View/browser/renderer | BND-007 horizon/historical LAB | final product/API work | deferred beyond SYS-5 headless view |
| upper applications | accepted headless toy plus historical LAB consumers | no domain Core promotion; product later | SYS-6 assurance only |

## recent log

- 2026-08-28 14:09 JST: SYS-5 implementation/evidence cut `53a21e64...`
  closed the bounded ordinary-source four-locus toy and typed devtools slice.
  Project/run/inspect execute actual generated endpoints and expose owner RMW,
  designated consume, leave/fallback/presentation-gap/fresh-reacquire,
  save/restore, accepted/rejected patch, revocation, verification, and one
  observer-safe joined causal report. Focused 10/27/28/8/17/12/3/4,
  `mir-runtime --all-targets` (245 library tests plus all integration targets),
  M10 2/4/67, format, warnings-denied Clippy, diff/manual redaction checks, and
  three final reviews passed. OBL-062 is bounded `runtime-monitored`; theory
  stays T1 and broad PHASE-I1/I2 lifecycle remains unaccepted. Fresh full
  workspace tests/Clippy, Canon index 185, hierarchy 799/799, and HTML 8/8
  also passed at closeout. SYS-6 is active and SYS-7 next.
- 2026-08-27 21:06 JST: SYS-4 implementation/evidence cut `22196f93...`
  closed the finite generated-plan-only in-process dispatch profile with
  selected ST/eligible-OW1 correspondence, exact source-to-occurrence lineage,
  one-consume designated retry, fail-closed observer/fault paths, ST
  whole-fabric cut/restore, and bounded checked patch. Focused 99/99, runtime
  179/179, M10 2/4/67, format, scoped warnings-denied Clippy, and diff checks
  passed. Theory stays T1 and broad PHASE-I1/I2 lifecycle remains unaccepted;
  SYS-5 is active and SYS-6 next.
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
