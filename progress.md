# progress

最終更新: 2026-08-26 23:09 JST

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
| Logical specification | Accepted finite M10 semantics remain the baseline; SYS-0/SYS-1 are closed and ADR-0027 fixes only the I2-internal owner/designated-input lifecycle. Official lifecycle remains T1; broad I1/I2 acceptance is unclaimed | **着手可能** for bounded SYS-2 concurrency/refinement |
| User-facing specification | One ordinary source remains the authority; Plan 249 fixes the eventual build/run/inspect/conform capability but not final CLI spelling, grammar, API, ABI, or wire | **後段依存** for public surface; SYS-2 changes remain backend-internal |
| Implementation / operation | The crate-private kernel is used by ordinary source and generic checked-owner paths at cut `94e3707c...`; no per-locus generator or actual locus dispatch exists yet | **着手可能** for SYS-2 ST/OW refinement; SYS-3 is next |

Normative sources: `mirrorea_canon/adr/ADR-0026.md`,
`mirrorea_canon/adr/ADR-0027.md`,
`mirrorea_canon/plan/01-phases.md`, and
`mirrorea_canon/plan/02-operating-model.md`. LAB execution source:
`plan/249-mirrorea-i2-systems-foundation-current-roadmap.md`.

Current direct blocker: the SYS-1 kernel consumes an immutable final M9
snapshot. SYS-2 must map owner/designated-input order to ST/OW happens-before,
linearization, and revoke/publication/activation/cut visibility, with bounded
edge-removal counterexamples. SYS-2 is active; SYS-3 remains next.

## milestone map

| Milestone | Capability | Position | Direct evidence/consumer |
| --- | --- | --- | --- |
| M0--M10 | finite theory + deterministic I1+ reference profile | closed baseline | ADR-0025 / Report 2591 |
| SYS-0 | baseline, authority, one goal/control path | **completed / closed**; Report 2592 | SYS-1 |
| SYS-1 | runtime kernel/conformance separation; internal carrier/effect seam | **completed / closed**; cut `94e3707c...`, Report 2593 | SYS-2/3 |
| SYS-2 | ST/OW concurrency and memory refinement | **active** | SYS-3/4 |
| SYS-3 | per-locus artifacts and generated communication | **next**, after SYS-2 | SYS-4 |
| SYS-4 | in-process generated dispatch | pending | SYS-5 |
| SYS-5 | typed devtools + four-locus toy world | pending | SYS-6 |
| SYS-6 | finite I2 assurance/lifecycle closeout | pending | SYS-7 |
| SYS-7 | inactive I3 goal/entry contract only | pending/terminal | future owner program only |

## line snapshots

### Product Alpha line

Historical runnable LAB evidence only. It is not the active program or a final
product contract.

### Operational Suite line

Historical bounded operational roots remain evidence. SYS-0/SYS-1 changed no
runnable path, command, debug surface, or sample classification.

### Mir Language line

M6/M7 ordinary source → checked Core remains the accepted finite source path.
SYS-3 will project that checked Core; SYS-1 did not reopen final grammar.

### PoseGraph line

Existing pose/renderer evidence is not Mir Core. The SYS-5 relation example is
headless sample behavior, not a View/renderer product.

### Projection/Backend line

Canon defines the preservation boundary, but executable per-locus projection
does not yet exist. SYS-2 supplies ST/OW refinement; SYS-3 supplies artifacts.

### Engine/Provider line

Providers remain typed adapters and non-owners. SYS-1 selected only a bounded
designated remote-input effect lifecycle, not a generic registry; transport,
auth, projection, and persistence remain separate.

## validation floor

| Changed layer | Required command family |
| --- | --- |
| Canon metadata/docs | regenerate/check `mirrorea_canon/INDEX.json`, `make docs`, `git diff --check` |
| runtime kernel/carrier | focused red/green boundary tests plus all changed-crate tests and M10 regression |
| concurrency | ST/OW focused tests, litmus/model-check falsifiers, warnings-denied Clippy |
| projection/dispatch | deterministic artifact/edge tests, no-manual-edge/no-direct-store negatives, replay |
| proof/model | `lean --trust=0`, placeholder/axiom scan, exact bounded-model classification when claimed |
| milestone close | independent review, one report, commit/push, clean worktree, remote parity |

## non-claims

No broad PHASE-I1 exit, official I2 entry/exit, C-distributed/socket/WAN,
public grammar/API/ABI/wire, public carrier freeze, durable distributed
persistence, production/publication, final browser/View renderer, general OBL
discharge, arbitrary DAG/scheduler/memory theorem, lock-free runtime, or I3
implementation is claimed.

## user decision items vs research-discovery items

| Kind | Item | Current handling |
| --- | --- | --- |
| Self-driven | SYS-0 baseline/goal alignment | completed; accepted integration cut and Report 2592 retained |
| Self-driven | SYS-1 internal kernel/carrier/effect seam | completed at `94e3707c...`; runtime-monitored, Report 2593 |
| Self-driven | SYS-2 ST/OW concurrency/refinement | active; preserve abstract order and detect removed-edge counterexamples |
| Research discovery | exact broad-I1 carrier-freeze residual | OPEN-026/027 + full carrier freeze retained; no criteria weakening |
| Research discovery | safe OW primitives and finite memory abstraction | select in active SYS-2; no Surface `memory_order_*` |
| Research discovery | finite DAG extension boundary | select in SYS-3 with one pressure case; no arbitrary theorem |
| Owner decision | public API/ABI/wire freeze | reserved; not a current blocker |
| Owner decision | real transport / production / publication | reserved; SYS-7 remains entry-contract only |

## macro phase map

| Macro | Focus | Current position | Weight | Self-drive |
| --- | --- | --- | --- | --- |
| 0 | governance/repository memory | ADR-0026/Plan 249 active; Plan 247 closed | medium | SYS close sync is self-driven |
| 1 | semantics/shared model | accepted finite semantics + closed narrow internal carrier | heavy | SYS-2 ordering refinement self-driven |
| 2 | parser-free evidence | historical only; must not become new architecture | medium | maintenance only |
| 3 | source/checker/runtime | M10 baseline; kernel separation closed; concurrency active | heavy | SYS-2 active; SYS-3 next |
| 4 | executable samples | no SYS sample change yet | medium | SYS-5 after SYS-4 |
| 5 | theorem/model-check | finite evidence retained; concurrency obligations next | heavy | SYS-2 in sequence |
| 6 | distributed fabric | per-locus generation/dispatch not yet realized | heavy | SYS-3/4 after prerequisites |
| 7 | toolchain/backend | kernel boundary available; projector/runtime/devtools later | heavy | SYS-2 then SYS-3--5 |
| 8 | applications | four-locus toy is future sample consumer | heavy | SYS-5 only; no Core promotion |

## feature maturity rows

| Feature | Evidence status | Remaining gate | Startability |
| --- | --- | --- | --- |
| multi-node/fabric | logical multi-locus M10 evidence only | generated artifact + actual endpoint dispatch | **後段依存** SYS-3/4 |
| robustness via contracts/theorem/model-check | finite M3--M10 classes retained | SYS-2/6 exact refinement evidence | **後段依存** |
| dynamic attach/detach/DAG evolution | finite patch/relation evidence | artifact/dispatch lifecycle and finite DAG pressure | **後段依存** SYS-3/4/5 |
| `atomic_cut` / higher-level ordering / memory order | high-level edges + deterministic ST + narrow kernel lifecycle | OW mapping and falsifiers | **着手可能** in active SYS-2 |
| executable sample corpus | existing active roots and M10 commands unchanged | generated four-locus toy | **後段依存** SYS-5 |
| Mir core/runtime kernel | crate-private kernel on ordinary source/generic OwnerEvent; 13 focused tests | revoke/publication visibility under OW | **着手可能** in active SYS-2 |
| Mirrorea projection/fabric | Canon boundary only | per-locus generation and actual dispatch | **後段依存** SYS-3/4 |
| typed-effect handler seam | bounded designated remote-input request/result/consume lifecycle | ST/OW effect ordering; no generic registry | **着手可能** in active SYS-2 |
| PrismCascade | separate performance kernel | no I2 integration required | deferred |
| View/browser/renderer | BND-007 horizon/historical LAB | final product/API work | deferred beyond SYS-5 headless view |
| upper applications | LAB consumers | no domain Core promotion | deferred |

## recent log

- 2026-08-26 23:09 JST: SYS-1 source cut `94e3707c...` and independent
  semantics/code-quality ACCEPT were mirrored through PROPOSAL-030/ADR-0027.
  The crate-private owner/designated-input lifecycle is runtime-monitored;
  SYS-2 is active and SYS-3 next while theory T1 and broad PHASE-I1/I2
  acceptance remain unchanged.
- 2026-08-26 20:13 JST: final SYS-0 review accepted with no P0/P1/P2; integration
  cut `350e04b4...` was pushed with clean `HEAD == origin/main` parity. SYS-0
  is closed, SYS-1 is active, and SYS-2 is next without changing official T1
  or broad PHASE-I1/I2 lifecycle acceptance.
- 2026-08-26 19:31 JST: first SYS-0 close review returned REJECT with no P0;
  authority-entry, primary HTML reader, current-state timing, and exact-command
  evidence corrections were applied and locally validated. SYS-0 remains
  closing and SYS-1 remains next until repeat review plus commit/push/parity.
- 2026-08-26 18:45 JST: owner direction was recorded through PROPOSAL-029 /
  ADR-0026; Plan 249 became the sole current roadmap, SYS-0 aligned the M10
  baseline without changing official T1/broad I1/I2 acceptance, with SYS-1
  planned as the next kernel/carrier goal. Pre-edit M10 focused groups
  67+2+4+3+5 and agent-config validation passed; post-edit close evidence is
  tracked in Report 2592.
- 2026-08-06 12:15 JST: refreshed the human overview against finite M10
  acceptance while preserving the lifecycle/public-product boundary.
- 2026-08-05 15:53 JST: accepted M10 R5 finite source-first profile at static
  26/26 and runtime 47/47, with no broad I1/I2/public/product claim.
