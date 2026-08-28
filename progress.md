# progress

最終更新: 2026-08-28 18:58 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project
direction, theory, ADRs, conformance, and process. Everything outside
`mirrorea_canon/` is LAB: evidence, history, implementation, and operational
notes. If LAB text conflicts with canon, canon wins. This is a concise LAB
snapshot and creates no Canon, Gate, Phase, proof, or compatibility decision.

## document role

Current lifecycle and semantics come from Canon. Plan 249 is the sole current
execution roadmap, Plan 247 is the closed M0--M10 record, and milestone reports
hold detailed evidence. This file mirrors workflow readiness, remaining gates,
and whether work is self-driven, research-discovered, owner-reserved, or
dependent on a later program.

## project axis

```text
正しい理論に基づき、正しく hot-plug でき、Place をまたいで
実行・通信・検証・可視化できる仮想空間システム
```

Mir, Mirrorea, PrismCascade, and the Typed-Effect Wiring Platform remain
separable. World, Avatar, Bird, and Viewer remain sample/library vocabulary.

## final ideal

```text
ordinary Mir source
-> checking / elaboration
-> ownership / authority / dependency / effect / failure / lifetime
-> per-locus executable artifacts + generated communication
-> process / network execution
-> typed trace / diagnostics / devtools
-> save / load / checked patch / hot-plug
-> View / provider / browser / renderer
-> persistent virtual-space system
```

The accepted I2 boundary reaches actual in-process generated dispatch and typed
observation. Real transport and the later product layers remain outside it.

## current milestone position

| Axis | Current status | Startability |
|---|---|---|
| Logical specification | Ordinary source -> checked Core -> per-locus artifacts -> generated communication -> actual in-process dispatch -> 22-row finite assurance is accepted. Theory stays T1 and broad PHASE-I1 stays unaccepted | **着手可能** only for SYS-7 contract work; general theory is **後段依存** |
| User-facing specification | Provisional `project-loci`, `run-local`, `inspect`, and `conform-i2` expose the bounded workflow. Final grammar, CLI, JSON, API, ABI, artifact, devtools, and wire are unfrozen | runnable regression is **着手可能**; public surface is **要仕様確認/後段依存** |
| Implementation / operation | SYS-6 cut `5429712d...` has 22/22 accepted finite rows and official I2 entry then exit. SYS-7 is the sole active goal and starts no transport | **着手可能** for SYS-7 entry contract only |

```text
Theory: T1
Broad PHASE-I1: unaccepted (OPEN-026/027 + full carrier freeze)
Official I2: entry accepted -> exit accepted (ADR-0032)
ADR-0026 program: SYS-0--SYS-6 completed, SYS-7 active
I3: inactive; OPEN-032 unresolved
```

The `conform-i2` report-local lifecycle bits remain false because runtime
evidence cannot self-authorize Canon. ADR-0032 owns the current lifecycle.

Sources: `mirrorea_canon/adr/ADR-0032.md`,
`mirrorea_canon/plan/01-phases.md`, and
`plan/249-mirrorea-i2-systems-foundation-current-roadmap.md`.

## milestone map

| Milestone | Capability | Position / evidence |
|---|---|---|
| M0--M10 | finite Mir Theory v0 + deterministic I1+ profile | closed; ADR-0025 / Report 2591 |
| SYS-0 | authority and one goal/control path | closed; Report 2592 |
| SYS-1 | kernel/conformance separation + internal carrier | closed at `94e3707c...`; Report 2593 |
| SYS-2 | ST/OW1 + bounded ordering/model | closed at `920d3fe...`; OBL-058/059 |
| SYS-3 | per-locus artifacts + generated plans | closed at `3013e7fe...`; OBL-060 |
| SYS-4 | in-process generated dispatch | closed at `22196f93...`; OBL-061 |
| SYS-5 | four-locus toy + joined devtools | closed at `53a21e64...`; OBL-062 |
| SYS-6 | finite I2 conformance/lifecycle | closed at `5429712d...`; OBL-063 / ADR-0032 |
| SYS-7 | inactive I3 goal/entry contract only | **active / terminal**; future owner program is direct consumer |

## line snapshots

### Product Alpha line

Historical Product Alpha and Full System V1 materials remain LAB consumers and
examples. They are not the current semantic queue and are not public/product
completion evidence for the accepted I2 systems foundation.

### Operational Suite line

The canonical four-locus toy plus `project-loci`, `run-local`, `inspect`, and
`conform-i2` form the current bounded reproducible workflow. The 22-row profile
consumes actual SYS-2--SYS-5 evidence and executed falsifiers; it is not an
expected-JSON facade or stable public tool contract.

### Mir Language line

Ordinary `.mir` source and checked Core remain semantic authority. The bounded
designated-consumer and explicit relation-anchor clauses serve direct finite
consumers and do not freeze the final grammar. Runtime reports, carriers,
workers, transports, and paths cannot mint Core or authority.

### PoseGraph line

The accepted relation fragment preserves explicit A-primary/B-fallback lineage,
consumer-local late projection, presentation-gap nonmutation, leave/fresh
incarnation boundaries, and a test-only extension pressure seam. Arbitrary DAG
theorems and production graph generalization remain deferred.

### Projection/Backend line

Checked Core deterministically creates owned locus fragments and generated
communication/effect/observation/persistence plans. SYS-4 executes those plans
through explicit endpoints. ST is the whole-toy reference; selected OW1 uses a
separate exactly-one-worker ordinary source. Arbitrary fairness, memory model,
and data-race theorems remain deferred.

### Engine/Provider line

The joined devtools and conformance outputs expose typed reference-only causal
evidence with redaction. Provider, worker, transport, report, and receipt
identity remain non-authority. PrismCascade, browser/View/renderer, and broader
provider products remain separable later consumers.

## validation floor

| Changed layer | Required evidence family |
|---|---|
| Canon/docs | regenerated/current INDEX, hierarchy/docs/HTML tests, `make docs`, `git diff --check` |
| SYS-6 | library 25 + CLI 8; exact row/control/provenance/redaction checks |
| preserved systems path | SYS-2 28, SYS-3 28, SYS-4 104, SYS-5 62 |
| M10 baseline | conformance 67 + CLI 4 |
| workspace | all-target tests, format, warnings-denied Clippy |
| lifecycle | independent review of broad I1 residual, I2 entry/exit, and I3 inactivity |

No unrun validation is counted as pass. Helper, report, or runtime monitor is
not promoted into a general theorem or public-product completion claim.

## non-claims

No broad PHASE-I1 exit, Theory T2, C-distributed/socket/WAN, real transport
selection, I3 activation, public grammar/CLI/API/ABI/wire/JSON/devtools schema,
durable distributed persistence, production/publication, browser/View product,
four-locus whole-workflow OW1, arbitrary relation DAG/scheduler/fairness/memory/
data-race theorem, general OBL discharge, exactly-once, lock-free runtime, or
public product completion is claimed.

## user decision items vs research-discovery items

| Class | Item | Current state |
|---|---|---|
| Self-driven | SYS-7 inactive I3 entry contract | active / terminal; no code or transport selection |
| Research discovery | at-most-two candidate transports, failure matrix, ordering mappings, C-distributed gates | record constraints only inside SYS-7 |
| Owner decision | OPEN-032 transport selection and I3 activation | future owner-authorized program |
| Owner decision | public API/ABI/wire freeze, production/publication | reserved; not a SYS-7 blocker |
| Later dependency | broad I1 carrier freeze and general theory | independent residuals; do not weaken criteria |

## macro phase map

| Macro | Focus | Current position | Weight | Self-drive |
|---|---|---|---|---|
| 0 | governance/repository memory | ADR-0032 accepted; SYS-7 active | medium | contract/status only |
| 1 | semantics/shared model | finite semantics through I2 assurance | heavy | maintenance |
| 2 | parser-free evidence | historical only | medium | maintenance |
| 3 | source/checker/runtime | source-first in-process I2 boundary | heavy | I3 contract only |
| 4 | executable samples | toy + conform command reproducible | medium | regression |
| 5 | theorem/model bridge | OBL-058 bounded; OBL-059--063 runtime | heavy | classification maintenance |
| 6 | generated/distributed fabric | in-process accepted; network absent | heavy | entry contract only |
| 7 | toolchain/backend | provisional project/run/inspect/conform | heavy | no public freeze |
| 8 | applications | local toy is sample/library consumer | heavy | no Core promotion |

## feature maturity rows

| Feature/subsystem | Evidence status | Remaining gate | Startability |
|---|---|---|---|
| Mir core/runtime kernel | source/check/project/dispatch assured for finite I2 | broader/general/public widening | maintenance |
| Mirrorea multi-locus fabric | generated artifacts cross in-process endpoints; official I2 exit | real multi-process transport | **後段依存** on future owner program |
| contracts/model robustness | typed falsifiers + bounded/runtime classifications | network faults/general proof | SYS-7 contract only |
| attach/detach/DAG evolution | leave/fresh, local cut, bounded patch | durable/general evolution | **後段依存** |
| `atomic_cut` / ordering | high-level edges, ST, selected OW1, bounded model | network mapping/general memory | contract wording only |
| executable samples | four-locus toy + `conform-i2` | public/product workflow | runnable regression |
| Typed-Effect seam | typed owner/designated request/result + no-mint checks | broader providers/network | **後段依存** |
| PrismCascade | separate performance kernel | no I2 integration required | deferred |
| View/browser/renderer | historical boundary only | product/API program | deferred |
| upper applications | toy + historical consumers | no domain Core promotion | product-specific |

## recent log

- 2026-08-28 18:58 JST: SYS-6 cut `5429712d...` closed the exact 22-row
  source-first finite I2 profile. SYS-6 25+8, SYS-2/3/4/5 28/28/104/62, M10
  67+4, workspace, format/Clippy/diff, and final independent ACCEPT support
  ADR-0032 official I2 entry then exit. Theory T1 and broad PHASE-I1 remain;
  SYS-7 is sole active and I3/OPEN-032 transport choice remain inactive/open.
- 2026-08-28 14:09 JST: SYS-5 cut `53a21e64...` closed the four-locus toy and
  joined typed devtools boundary (Report 2597 / OBL-062).
- 2026-08-27 21:06 JST: SYS-4 cut `22196f93...` closed generated-plan-only
  in-process dispatch, selected ST/OW1 evidence, ST cut, and bounded patch.
- 2026-08-27 07:07 JST: corrected SYS-3 cut `3013e7fe...` closed the source-
  derived designated-consumer projection after the earlier candidate reopen.
- 2026-08-27 01:09 JST: SYS-2 cut `920d3fe...` closed selected ST/OW1 and
  bounded ordering evidence.
- 2026-08-26 23:09 JST: SYS-1 cut `94e3707c...` closed the internal runtime
  kernel and carrier boundary.
- 2026-08-05 15:53 JST: M10 cut `23f5a813...` accepted the finite I1+ baseline.
