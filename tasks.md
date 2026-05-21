# tasks

最終更新: 2026-05-21 18:56 JST

## この文書について

- この文書は repo 全体の **current task map** です。
- 規範判断の正本は `specs/`、長期比較と source trace は `plan/`、runnable sample 状態は `samples_progress.md`、実行証跡は `docs/reports/` に置きます。
- append-only 履歴ではありません。current checkpoint、次に詰める gate、blocker を読める snapshot として保ちます。
- 進捗率は primary metric ではありません。helper / sidecar / report / expected JSON / first-floor runner は completion ではなく evidence として書きます。

## current task-level status

- active executable floor is maintained:
  `samples/clean-near-end/`, `samples/current-l2/`, and `samples/lean/` remain the current-L2 runnable / source / mechanization roots.
- operational alpha readiness is separated and actualized where scoped:
  α-0.5 local observable runtime, α-0.8 same-session hot-plug runtime, and α-0.9 session-bound devtools have bounded workflow anchors under `specs/19..24` and `plan/45..49`.
- product alpha-1 release candidate is reproducible:
  `samples/product-alpha1/demo`, `mirrorea-alpha`, and `scripts/product_alpha1_release_check.py check-all` cover check / run-local / session / attach / save / load / quiescent-save / local-Docker transport / export-devtools / view / build-native-bundle / demo.
- installed-binary adoption probe is reproducible:
  `scripts/product_alpha1_installed_binary_check.py check-all` builds `target/debug/mirrorea-alpha`, generates a native host launch bundle, and verifies bundle `run.sh check` / `run.sh view`.
- canonical operational product sample suite is reproducible:
  `samples/product-alpha1/operational/` covers `WorldCore -> MembershipChat -> SugorokuWorld -> PortalWorldLink -> TwoShardHardBoundary -> TwoShardGradientObservation`, shared attach packages, projection inventory, local/Docker transport, observer-safe devtools, R0/R2 save evidence, native host bundle, and helper-reported scope blocks.
- Product Alpha-1 operational runtime widening queue is exhausted:
  `widening_queue_scope` keeps room-chat, portal/shard starter, and broader Sugoroku reopenings non-promoted; `user_final_decision_scope` marks broader distribution / final catalog breadth as a user-spec-required gate.
- current docs/spec self-driven line is reopened:
  `P-COMP-00` rebaselined the target from distribution-only to Mir-owned computation, while preserving the existing alpha runtime/product floor.
- autonomous execution default is reviewer-integrated and fixed:
  `specs/32` and `plan/57` define a docs/scaffold front half, an implementation half, safe defaults, sub-agent review cadence, validation floor, report / commit / push policy, and user-spec-required gate isolation. `plan/57` is repository memory; live queue authority remains this document plus `progress.md`.

## ordered self-driven packages

| Order | Package | Macro / stage | Objective | Close condition | Rough estimate |
|---:|---|---|---|---|---|
| 1 | `P-COMP-01` Mir computational sample scaffold actualization | `Macro 1`, docs/scaffold front half | create `samples/product-alpha1/computational/` planned-only roots, helper, matrix, and tests without runtime overclaim | planned rows are machine-readable; `run comp-02-pure-add-one` rejects as `planned_only`; current AddOne remains `not_mir_owned` for computation completion | small |
| 2 | `P-POSE-01` Transform / PoseGraph scaffold | `Macro 8`, docs/scaffold front half | define and scaffold `Transform`, `PoseSnapshot`, `Anchor`, `AnchorBinding`, `AnchorSwitch`, fallback admissibility, and no-split-frame rows | planned sample matrix and non-claims are indexed; no runtime PoseGraph evidence is claimed | small |
| 3 | `P-PROJ-01` projection target / packet / FFI inventory | `Macro 7`, docs/scaffold front half | represent server/client/adapter target manifests, packet/FFI schemas, and manifest-provider compatibility as inventory | inventory surface has accepted/rejected compatibility rows and is explicitly non-codegen | small-medium |
| 4 | `P-ENG-01` engine/WASM/FFI adapter boundary | `Macro 7/8`, docs/scaffold front half | fix provider contract fields, rollback/replay/cut policy, and native/WASM gated defaults while keeping semantics in Mir/Mirrorea | provider classes and stop lines are indexed; native/WASM admission remains disabled or user-spec-required | small |
| 5 | front-half closeout audit | `Macro 0/1/7/8`, closeout | confirm docs/scaffold rows, validators, reports, and non-claims are synchronized before runtime claims begin | all four front-half helpers/tests pass once created; `P-COMP-02` is named as next implementation package | small-medium |
| 6 | `P-COMP-02` pure AddOne in Mir | `Macro 1/7`, implementation half | introduce narrow `mir-semantics` computational AST/evaluator and make `x + 1` Mir-owned while host input/output remain typed external adapters | host input / Mir compute / host output are separate observable events; adapter-owned AddOne path rejects as `not_mir_owned` | medium |
| 7 | `P-COMP-03` variables / arrays / records / control-flow first floor | `Macro 1`, implementation half | add positive and negative rows for first C-like computation floor | each family has machine-readable accept/reject evidence | medium |
| 8 | `P-COMP-04` effect boundary around internal computation | `Macro 1/5`, implementation half | separate pure computation from declared host read/write effect wrappers and explicit failure/capability rows | undeclared effect / failure / capability rows are rejectable; broader publish/observe/witness/handoff remains later if needed | medium |
| 9 | `P-POSE-02` no-split-frame runtime evidence | `Macro 8`, implementation half | prove same-client same-observation-snapshot pose coherence and a negative mismatch row | positive and negative machine-readable evidence exists | medium |
| 10 | autonomous all-up closeout audit | `Macro 0/1/7/8`, closeout | confirm all package lines, docs, samples, reports, validators, and non-claims are synchronized | all focused helpers and common validation floor pass; final report lists remaining non-claims | medium |

## current recommendation

- recommended reopen point:
  `P-COMP-01` Mir computational core spec and sample scaffold
- recommendation reason:
  current runnable floors remain valid, but the handoff identified a more basic architecture drift: host-boundary `AddOne` must not stand in for Mir-owned computation. The least risky next self-driven step is docs/spec scaffold and sample matrix, not runtime widening or distribution hardening.
- stop line:
  do not claim runtime completion in `P-COMP-01`, final grammar, final public parser / viewer / telemetry ABI, backend realization, server/client binary split, distributed durable save-load, WAN/federation, arbitrary native/WASM execution, final product completion, or current AddOne as Mir computational-core completion.

## research-discovery items

| Item | Impact | Main options | Current recommendation |
|---|---|---|---|
| proof side discharge granularity | `plan/48` and external prover bridge | coarse obligation family / static-row granularity | keep current residual obligation carrier and refine when proof target work reopens |
| distributed durable save/load line | `specs/20` later family | keep local/Docker R0/R2 / reopen R3/R4 | keep out of current alpha-1 gate; reopen only after explicit durability decision |
| auth policy catalog breadth | `specs/21` and attach package line | minimal initial set / broader policy catalog | keep minimal current attach evidence; widen policy breadth only with concrete runtime evidence |
| product checker finite fragment breadth | `specs/25` and product CLI | existing rows / broader product fragment | keep the bounded package schema, effect/failure, capability/witness, message recovery, and savepoint policy rows |
| projection inventory breadth | future backend / server-client split | current summary / richer projection IR / planner-adjacent IR | keep current schema-backed summary until actual split/planner work is chosen |
| post-gradient operational widening | `specs/26..27` | room-chat / Sugoroku / portal-shard starters / user-final gate | keep current runtime widenings non-promoted until the user-final gate is decided |
| computational-core proof shape | `specs/28` / `plan/53` | pure fragment first / combined effectful judgment first | start with pure fragment, add explicit effect and failure rows at effectful layer |
| PoseGraph carrier shape | `specs/29` / `plan/54` | renderer frame / observation snapshot / save carrier | use same-client observation snapshot, explicit `Anchor` / `AnchorSwitch`, and later save/devtools carrier hooks |
| computational implementation home | `specs/28` / `plan/57` | extend adapter lane / add `mir-semantics` computational module | add narrow `mir-semantics` AST/evaluator; do not repurpose adapter-owned AddOne as Mir compute |

## user decision items

| Item | Impact | Main options | Current recommendation |
|---|---|---|---|
| `U1` beyond alpha packaging / host target / shipped surface | final public product line | repo-local alpha / archive or installer / hosted service / other | current alpha keeps developer-built binary + generated host launch bundle only; wait for explicit user choice before widening |
| final shared-space operational catalog breadth | product/public scope | bounded product alpha-1 narrow showcase / broader final product line | keep narrow showcase as current alpha; decide broader catalog separately |
| final public grammar / ABI | final public product line | evolve `package.mir.json` / freeze textual grammar | do not freeze in product alpha-1 |
| hosted service / production WAN | final public product line | local/Docker alpha / hosted service / WAN federation | keep out of current alpha-1 completion claim |
| backend realization beyond inventory | projection/backend line | inventory-only / server-client split / direct backend | keep inventory-only until explicit implementation package |
| bounded native / WASM provider admission | engine adapter line | disabled / inventory-only / bounded admitted provider | keep disabled or inventory-only until a package proves schema/effect/failure/capability/observation/sandbox/rollback policy |
| final engine adapter ABI | final product engine line | internal inventory / public SDK / engine-specific ABI | keep deferred; no Unity/Unreal/VRM compatibility claim |

## self-driven maintenance tasks

| Task | Objective | Validation | Report requirement | Stop line |
|---|---|---|---|---|
| docs freshness audit | keep `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, and index docs aligned with current queue | `python3 scripts/check_source_hierarchy.py`, `python3 scripts/validate_docs.py`, `git diff --check` | new report | snapshot docs must not create new normative decisions |
| runnable dashboard refresh | keep sample status, validation command, and blocker columns evidence-backed | relevant helper closeout commands | report + `samples_progress.md` | conceptual-only rows must not be marked workflow-ready |
| operational suite guide refresh | keep `samples/product-alpha1/operational/`, hands-on guide, research summary, and helper CLI surface synchronized | `python3 -m unittest scripts.tests.test_operational_product_samples`, `python3 scripts/operational_product_samples.py check-all --format json` | report if touched | do not promote future inventory into runnable claim |
| regression repair | repair docs / tests / formatting failures without unrelated feature work | affected tests, `cargo fmt --check`, `git diff --check` | report if non-trivial | do not mix feature widening into maintenance |
| computational docs freshness | keep `specs/28..31`, `plan/53..56`, snapshot docs, and planned sample rows aligned | docs validation and source hierarchy checks | new report | planned samples/scripts must not be marked runnable |
| autonomous plan freshness | keep `specs/32`, `plan/57`, snapshot docs, and package cadence aligned with actual execution | docs validation and source hierarchy checks | new report | autonomy policy must not erase user-spec-required final-product gates |

## non-promoted references

- `P-A0-*` Stage B/D/E/F rows remain current-scope evidence, not operational α-0.5 / α-0.8 / α-0.9 completion.
- practical alpha-1 first-floor rows remain evidence, not product/public-ready alpha-1 completion.
- operational helper scope blocks are evidence-backed queue state, not final public product decisions.
