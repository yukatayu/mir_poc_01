# tasks

最終更新: 2026-05-21 23:37 JST

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
- current docs/spec self-driven line is in the implementation-half queue:
  `P-COMP-00` rebaselined the target from distribution-only to Mir-owned computation, while preserving the existing alpha runtime/product floor, and the front-half scaffolds are now closed.
- `P-COMP-01` is closed:
  `samples/product-alpha1/computational/`, `matrix.json`, `scripts/mir_computational_samples.py`, and the planned-only rejection surface are now actualized as scaffold baseline.
- `P-COMP-02` is closed:
  `samples/product-alpha1/computational/add-one-pure-mir/`, `package.mir.json`, `crates/mir-semantics::computational_core`, product-alpha schema/runtime/session/savepoint support, and helper/runtime tests now prove one bounded Mir-owned row. Legacy adapter-owned `typed_host_io.add_one` remains unchanged.
- `P-COMP-03` is closed:
  `crates/mir-semantics::computational_core` now covers variables / arrays / records / control-flow / imports, product-alpha schema/runtime tests cover the widened module registry, and `samples/product-alpha1/computational/` now has 5 accepted helper rows and 5 expected runtime rejection rows.
- `P-COMP-04` is closed:
  `samples/product-alpha1/computational/host-io-internal-transform/` now has one direct accepted host read/write boundary row and three expected `check` rejections, while product-alpha schema/runtime tests cover `required_capabilities` and `failure_tag` as declared admission-boundary evidence without claiming broad effectful runtime semantics.
- `P-POSE-01` is closed:
  `samples/product-alpha1/posegraph/`, `matrix.json`, `scripts/posegraph_samples.py`, and the planned-only rejection surface are now actualized. This is scaffold closeout, not runtime proof.
- `P-POSE-02` is closed:
  `samples/product-alpha1/posegraph/` now has helper-only `package.mir.json` inputs for `pose-04` and `pose-05`, one accepted same-snapshot row, one `violation_export` mismatch row, and seven residual planned rows. This is bounded helper evidence, not full PoseGraph runtime completion.
- `P-PROJ-01` is closed:
  `samples/product-alpha1/projection/`, `matrix.json`, `scripts/projection_boundary_samples.py`, and compatibility-row inventory are now actualized. This is scaffold closeout, not code generation or binary split.
- `P-ENG-01` is closed:
  `samples/product-alpha1/engine-adapter/`, `matrix.json`, `scripts/engine_adapter_boundary_samples.py`, and provider contract inventory are now actualized. This is scaffold closeout, not provider admission.
- front-half docs/scaffold closeout is complete:
  the computational / PoseGraph / projection / engine-adapter roots, helpers, tests, validators, reports, and snapshot docs are synchronized, so the promoted queue now moves to the implementation half.
- all-up closeout audit is closed:
  focused helpers, Cargo regressions, product alpha release check, installed-binary probe, operational suite, docs validator, and source hierarchy checks have been rerun after `P-POSE-02`, so the current self-driven package chain is closed through its planned audit package.
- `P-PAT-01` minimal alpha-1 pattern verification is closed:
  `scripts/minimal_alpha1_patterns.py` and `docs/hands_on/minimal_alpha1_patterns_01.md` now provide a compact strict verifier over existing computational / PoseGraph / projection / engine-adapter rows, while keeping product release-candidate and operational Sugoroku as workflow anchors rather than new final-product claims.
- autonomous execution default is reviewer-integrated and fixed:
  `specs/32` and `plan/57` define a docs/scaffold front half, an implementation half, safe defaults, sub-agent review cadence, validation floor, report / commit / push policy, and user-spec-required gate isolation. `plan/57` is repository memory; live queue authority remains this document plus `progress.md`.

## ordered self-driven packages

No promoted self-driven package remains in the current chain. The line defined in `plan/57` is closed through all-up closeout.

## current recommendation

- recommended reopen point:
  none in the current self-driven chain
- recommendation reason:
  the pure computational core now has one direct `add_one` row, widened first-floor helper rows, one bounded host read/write boundary closeout, and PoseGraph now has bounded accepted/violation helper evidence without overloading renderer/global-simultaneity claims. The planned all-up audit has also passed, and `P-PAT-01` adds a compact verifier for the minimal alpha-1 pattern set. Future reopenings should be chosen explicitly from the research-discovery or user-decision items below rather than pretending another package is already promoted.
- stop line:
  do not claim runtime completion in the front-half scaffolds, final grammar, final public parser / viewer / telemetry ABI, backend realization, server/client binary split, distributed durable save-load, WAN/federation, arbitrary native/WASM execution, final product completion, or current AddOne / helper-backed no-split-frame evidence as implementation completion.

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
| computational docs freshness | keep `specs/28..31`, `plan/53..56`, snapshot docs, and executable/planned sample rows aligned | docs validation and source hierarchy checks | new report | only evidence-backed rows may be marked runnable |
| minimal alpha-1 pattern verifier refresh | keep `scripts/minimal_alpha1_patterns.py`, its unit test, and hands-on/research summary aligned with the exact current sample rows | `python3 -m unittest scripts.tests.test_minimal_alpha1_patterns`, `python3 scripts/minimal_alpha1_patterns.py check-all --format json` | new report if touched | verifier drift must not create a new runtime claim |
| autonomous plan freshness | keep `specs/32`, `plan/57`, snapshot docs, and package cadence aligned with actual execution | docs validation and source hierarchy checks | new report | autonomy policy must not erase user-spec-required final-product gates |

## non-promoted references

- `P-A0-*` Stage B/D/E/F rows remain current-scope evidence, not operational α-0.5 / α-0.8 / α-0.9 completion.
- practical alpha-1 first-floor rows remain evidence, not product/public-ready alpha-1 completion.
- operational helper scope blocks are evidence-backed queue state, not final public product decisions.
