# plan/120 - Repo triage recut matrix

## Purpose

This file is LAB repository memory.

It classifies existing Product Alpha, Full System V1, and Surface evidence for
the next theory / management recut. The classification is intentionally about
how to read and reuse evidence. It is not a file-move plan, archive operation,
canon edit, gate exit, or implementation-state promotion.

Use this file when a future package needs to decide whether an old alpha row is:

- a core idea to preserve in the recut;
- a useful floor / regression anchor to keep without promoting to final status;
- exploration / inventory evidence to retain but not drive the next line; or
- a later-gate / dropped-from-current-recut claim that should not be carried
  into the immediate theory target.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- LAB claim-family ledger: `plan/70-lab-to-canon-reconciliation-ledger.md`
- Remaining claim-family priority map:
  `plan/119-g0-remaining-claim-family-drilldown-priority.md`
- Management synthesis:
  `plan/69-consultation-synthesis-and-management-roadmap.md`
- Snapshot task / progress maps: `tasks.md`, `progress.md`
- Runnable dashboard: `samples_progress.md`

If this file conflicts with canon, canon wins. If it conflicts with
`samples_progress.md`, use `samples_progress.md` for current runnable workflow
status and this file only for triage reading.

## Triage vocabulary

These labels are a management overlay. They do not replace existing repo
vocabulary such as `workflow-ready`, `product-release-candidate`,
`evidence-closed`, `boundary-fixed`, `planned`, `LAB-evidence-only`, or
`later-gate`.

| Triage label | Meaning |
|---|---|
| `keep-core-idea` | Preserve the semantic principle or architectural boundary as a pressure-test seed for the recut. This does not imply canon promotion or immediate gate work. |
| `useful-floor` | Keep as reproducible compatibility, regression, adoption, or evidence anchor. This does not imply final API/runtime/product completion. |
| `archive-exploration` | Retain as useful exploration, scaffold, inventory, compatibility evidence, or historical pressure. Do not use as the next default theory target. |
| `postpone/drop-from-current-recut` | Do not carry the claim into the immediate recut. Reopen only through a future gate, user decision, or explicit package. This is not file deletion. |

## Classification axes

Use all axes together. Do not classify by root name alone.

| Axis | Source | Use in this matrix |
|---|---|---|
| Source hierarchy | `mirrorea_canon/meta/source-hierarchy.md`, `plan/70` | Canon controls normative meaning; LAB rows are evidence only. |
| Workflow status | `samples_progress.md` | Preserve current workflow/evidence status labels; do not invent completion. |
| Semantic stratum | `plan/69` S0..S5 | Separate ordinary source intent, Core elaboration, trace, verification, projection, and domain/library pressure. |
| Gate / phase | `mirrorea_canon/plan/01-phases.md`, `plan/119` | Current canon phase remains T0/G0; later G2..G7 rows are not immediate defaults. |
| Validation anchor | sample READMEs, helper commands, reports | Cite exact commands / files when reusing evidence; validation is evidence, not proof. |
| Non-claim | `README.md`, `Documentation.md`, sample READMEs | Carry stop lines forward so evidence does not become final API/runtime/product status. |

## Product Alpha matrix

| Root / evidence family | Current status reading | Primary triage | Preserve for | Do not infer |
|---|---|---|---|---|
| `samples/product-alpha1/demo/`, release-check, installed-binary probe, native host bundle | product alpha release-candidate / bounded public-ish adoption evidence | `useful-floor` | product workflow ergonomics, clean command family, release validation shape, local observer-safe bundle pressure | final public product, final CLI/API/ABI, installer/archive/hosted distribution, final semantic source authority |
| `samples/product-alpha1/operational/{world-core,membership-chat,sugoroku-world,portal-worldlink,two-shard-hard-boundary,two-shard-gradient-observation}/` | workflow-ready canonical operational alpha suite, not final product | `useful-floor` plus domain-name `archive-exploration` | domain/application pressure cases for locus, membership, handoff, fallback, observation, and bounded catalog shape | `World` / room / game vocabulary as Mir core primitives, production shared-space catalog completion |
| `samples/product-alpha1/operational/templates/` | template-only authoring starters | `archive-exploration` | authoring ergonomics and examples for later docs / starter design | active operational roots, complete scaffold generator, final user-facing project template system |
| `samples/product-alpha1/operational/future/` | retained blueprint/profile inventory | `archive-exploration` | future boundary notes and possible profile vocabulary | executable roots, active roadmap commitment, final catalog breadth |
| `samples/product-alpha1/docker/` and Docker release-check evidence | controlled transport fixture for alpha validation | `postpone/drop-from-current-recut` | environment-gated product validation evidence | transport semantics, WAN/federation, deployment model, Docker as required final runtime |
| `samples/product-alpha1/computational/` | bounded Mir-owned computation and host-boundary evidence | `keep-core-idea` / `useful-floor` | first-floor computation, explicit effect / failure / capability boundary pressure | final grammar, broad effect semantics, direct LLVM/native backend |
| `samples/product-alpha1/posegraph/` | bounded PoseGraph helper evidence with planned residual rows | `keep-core-idea` / `useful-floor` | no-split-frame and transform/anchor pressure for later pose/fallback/cut gates | full PoseGraph runtime completion, global simultaneity, renderer-owned semantics |
| `samples/product-alpha1/projection/` | planned-only projection boundary inventory scaffold | `archive-exploration` | target manifest / packet / FFI inventory vocabulary for later G6 | server/client code generation, backend execution, final projection ABI |
| `samples/product-alpha1/engine-adapter/` | planned-only engine / WASM / FFI adapter inventory scaffold | `archive-exploration` | provider contract inventory and disabled/inventory-only execution policy | provider admission, arbitrary native/WASM execution, final engine adapter ABI |

## Full System V1 matrix

| Root / evidence family | Current status reading | Primary triage | Preserve for | Do not infer |
|---|---|---|---|---|
| `samples/full-system-v1/computational/` | source-first parser, checker, bounded runtime evidence | `keep-core-idea` / `useful-floor` | source authority, typed checker, explicit failure/effect rows, bounded interpreter pressure | final public grammar, final typed IR/runtime API |
| `samples/full-system-v1/{world-core,membership-chat,sugoroku-world,portal-worldlink,two-shard-hard-boundary,gradient-observation}/` | bounded source-first operational evidence | `useful-floor` plus domain-name `archive-exploration` | source-first domain pressure separate from Product Alpha package artifacts | final operational runtime/transport, domain vocabulary as core primitives |
| `samples/full-system-v1/avatar-pose/` | bounded PoseGraph runtime / save-load / devtools evidence | `keep-core-idea` / `useful-floor` | no-split-frame, anchor-switch frontier, fallback-only reacquire, observer-safe pose/devtools pressure | distributed durable pose save/load, final devtools family completion |
| `samples/full-system-v1/projection/` | bounded projection IR + boundary-schema evidence | `keep-core-idea` / `useful-floor` | preservation targets, packet/FFI schema inventory, source-owned capability/failure rows | final packet/FFI transport semantics, optimal placement, server/client compiler completion |
| `samples/full-system-v1/server-client/` | bounded same-binary local role-split evidence | `useful-floor` | role-run split pressure and undeclared-entry rejection | final executable server/client binary split or deployment planner |
| `samples/full-system-v1/provider-adapter/` and renderer-pose rows | bounded provider-admission and renderer-pose evidence | `keep-core-idea` / `useful-floor` | provider non-ownership, disabled-native default, inventory-only WASM, renderer delivery constraints | arbitrary native/WASM execution, provider as semantic owner, final provider ABI |
| Full System V1 release-check / report / viewer bundle | bounded line-level release-check lane, audit-closed | `useful-floor` | validation aggregation and claim/non-claim audit pattern | final product status, final grammar/API/ABI, canon implementation-state completion |

## Surface matrix

| Root / evidence family | Current status reading | Primary triage | Preserve for | Do not infer |
|---|---|---|---|---|
| `samples/full-system-v1-surface/syntax/` | parser evidence floor | `keep-core-idea` / `useful-floor` | ordinary source surface, `S { ... }` syntax, parser diagnostics | final public grammar/API freeze |
| `samples/full-system-v1-surface/indexed-state/` | semantic checker evidence floor | `keep-core-idea` / `useful-floor` | S-owned indexed state, key-not-authority, stale-key and ambient-authority fences | runtime membership lifecycle or final distributed table semantics |
| `samples/full-system-v1-surface/elaboration/` | Surface-to-Core elaboration and generated communication evidence | `keep-core-idea` / `useful-floor` | immediate G1 ordinary-assignment pressure, owner-directed writes, dependency rows, generated failure rows, source spans | runtime MessageEnvelope dispatch, C-static conformance, theorem discharge |
| post-`P-SURF-99` G1 E-ROW / OBL addenda | LAB diagnostic / repair / statement-shape evidence | `keep-core-idea` / `useful-floor` | OBL-001/020/021/024/025 statement pressure, diagnostic projection, non-final repair-shape guards | proof discharge, OBL status movement, final diagnostic/repair ABI, G1 exit |
| `samples/full-system-v1-surface/role-admission/` | report-level admission / capability grant evidence | `keep-core-idea` / `useful-floor` | G3 authority/admission pressure; role claim is not authority | production identity provider, hardware attestation, WAN admission, transport-as-authority |
| `samples/full-system-v1-surface/source-patch/` | source patch hot-plug pipeline evidence | `keep-core-idea` / later `postpone/drop-from-current-recut` | later G7 patch pipeline, no-direct-eval, activation cut, rejection without mutation | hot-plug as first theory target, final source patch ABI, migration engine completion |
| `samples/full-system-v1-surface/devtools/` | static observer-safe diagnostics evidence | `useful-floor` with G4 `keep-core-idea` pressure | G4 observation pressure, source/Core mapping, redaction, retention, source spans | final viewer/telemetry ABI or untyped debug leak |
| `samples/full-system-v1-surface/{world-core,membership-chat,sugoroku-world,portal-worldlink,two-shard-hard-boundary,gradient-observation}/` | source operational evidence roots | `useful-floor` plus domain-name `archive-exploration` | source-first domain/application pressure that stays below core semantics | final Surface runtime/transport or final shared-space catalog |

## Dropped from the immediate recut

The following claims should be dropped from the current theory recut unless a
future package explicitly reopens them:

| Claim shape | Triage | Reopen condition |
|---|---|---|
| `package.mir.json` as semantic source authority | `postpone/drop-from-current-recut` | only as alpha compatibility artifact unless canon changes source authority |
| `World`, `Room`, `Avatar`, `Game`, or `Event` as Mir core primitives | `postpone/drop-from-current-recut` | only as domain/library vocabulary or future upper-layer work |
| hot-plug-first semantics | `postpone/drop-from-current-recut` | after G1/G2/G3/G4/G5/G6 prerequisites make G7 precise |
| final viewer / telemetry ABI | `postpone/drop-from-current-recut` | future G4 observation package with authority/redaction/retention requirements |
| final transport, WAN/federation, Docker-as-final-runtime | `postpone/drop-from-current-recut` | explicit runtime/network gate or user decision |
| final public grammar/API/ABI/SDK | `postpone/drop-from-current-recut` | explicit public-boundary freeze package and human decision |
| arbitrary native/WASM execution or provider-owned semantics | `postpone/drop-from-current-recut` | future provider/admission package that preserves authority and non-ownership |
| distributed durable save/load R3/R4 and migration replay | `postpone/drop-from-current-recut` | future G5/G7 package after earlier gate context is stable |

## Immediate scheduling result

The safest next theoretical line remains narrow G1 ordinary-assignment support
and proof-boundary refinement, not a broad runtime/product rewrite.

Use the keep-core rows above as semantic pressure tests and examples. Use
useful-floor rows as reproducible regression / compatibility anchors. Do not
promote either category to canon implementation-state completion. Use
archive-exploration rows as retained inventory or compatibility evidence. Use
postpone/drop rows as explicit stop lines for the current recut.

## Required non-claims

- No canon edit.
- No G0 exit.
- No T0 -> T1 transition.
- No G1..G7 exit.
- No proof-obligation status movement.
- No proof discharge.
- No C-static, C-runtime, or C-distributed conformance claim.
- No implementation-state completion.
- No sample workflow/evidence status relabel.
- No archive move, file deletion, or root rename.
- No final grammar/API/ABI/runtime/transport/projection/provider/viewer freeze.

## Open questions

- Should a later package create a machine-readable version of this triage matrix,
  or is the human-readable `plan/` memory sufficient for T0/G0?
- Which keep-core rows should become the first minimal source-first vertical
  slice after G1 ordinary-assignment boundaries are stable?
- Should Product Alpha release-check evidence remain only a compatibility /
  adoption anchor, or should a later public-boundary package split its reusable
  workflow ideas into a new non-alpha demonstration root?
- When broader distribution / final shared-space catalog breadth is reopened,
  which Product Alpha operational roots should be kept as examples and which
  should be replaced by a new canon-aligned catalog?
