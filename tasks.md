# tasks

最終更新: 2026-05-07 13:25 JST

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
- current self-driven queue is exhausted:
  `widening_queue_scope` keeps room-chat, portal/shard starter, and broader Sugoroku reopenings non-promoted; `user_final_decision_scope` marks broader distribution / final catalog breadth as a user-spec-required gate.

## ordered self-driven packages

| Order | Package | Macro / stage | Objective | Close condition | Rough estimate |
|---:|---|---|---|---|---|
| 1 | no active self-driven package | `Macro 7/8`, `U-spec gate` | keep current bounded alpha-1 line stable while waiting for broader distribution / final catalog breadth decision | user chooses the next shipped-surface / catalog direction, then a concrete reopen package can be scoped | hold |

## current recommendation

- recommended reopen point:
  user-spec-required later user-final distribution decision
- recommendation reason:
  current runnable floors and helper-reported queue state agree that no further operational runtime widening is promoted without user input. The least risky next step is to decide whether alpha-1 remains a developer-built binary + generated host launch bundle with a bounded narrow showcase, or broadens toward archive / installer / hosted-service / wider final catalog.
- stop line:
  do not claim final public parser / viewer / telemetry ABI, distributed durable save-load, WAN/federation, arbitrary native execution, or final product completion.

## research-discovery items

| Item | Impact | Main options | Current recommendation |
|---|---|---|---|
| proof side discharge granularity | `plan/48` and external prover bridge | coarse obligation family / static-row granularity | keep current residual obligation carrier and refine when proof target work reopens |
| distributed durable save/load line | `specs/20` later family | keep local/Docker R0/R2 / reopen R3/R4 | keep out of current alpha-1 gate; reopen only after explicit durability decision |
| auth policy catalog breadth | `specs/21` and attach package line | minimal initial set / broader policy catalog | keep minimal current attach evidence; widen policy breadth only with concrete runtime evidence |
| product checker finite fragment breadth | `specs/25` and product CLI | existing rows / broader product fragment | keep the bounded package schema, effect/failure, capability/witness, message recovery, and savepoint policy rows |
| projection inventory breadth | future backend / server-client split | current summary / richer projection IR / planner-adjacent IR | keep current schema-backed summary until actual split/planner work is chosen |
| post-gradient operational widening | `specs/26..27` | room-chat / Sugoroku / portal-shard starters / user-final gate | keep current runtime widenings non-promoted until the user-final gate is decided |

## user decision items

| Item | Impact | Main options | Current recommendation |
|---|---|---|---|
| `U1` beyond alpha packaging / host target / shipped surface | final public product line | repo-local alpha / archive or installer / hosted service / other | current alpha keeps developer-built binary + generated host launch bundle only; wait for explicit user choice before widening |
| final shared-space operational catalog breadth | product/public scope | bounded product alpha-1 narrow showcase / broader final product line | keep narrow showcase as current alpha; decide broader catalog separately |
| final public grammar / ABI | final public product line | evolve `package.mir.json` / freeze textual grammar | do not freeze in product alpha-1 |
| hosted service / production WAN | final public product line | local/Docker alpha / hosted service / WAN federation | keep out of current alpha-1 completion claim |

## self-driven maintenance tasks

| Task | Objective | Validation | Report requirement | Stop line |
|---|---|---|---|---|
| docs freshness audit | keep `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, and index docs aligned with current queue | `python3 scripts/check_source_hierarchy.py`, `python3 scripts/validate_docs.py`, `git diff --check` | new report | snapshot docs must not create new normative decisions |
| runnable dashboard refresh | keep sample status, validation command, and blocker columns evidence-backed | relevant helper closeout commands | report + `samples_progress.md` | conceptual-only rows must not be marked workflow-ready |
| operational suite guide refresh | keep `samples/product-alpha1/operational/`, hands-on guide, research summary, and helper CLI surface synchronized | `python3 -m unittest scripts.tests.test_operational_product_samples`, `python3 scripts/operational_product_samples.py check-all --format json` | report if touched | do not promote future inventory into runnable claim |
| regression repair | repair docs / tests / formatting failures without unrelated feature work | affected tests, `cargo fmt --check`, `git diff --check` | report if non-trivial | do not mix feature widening into maintenance |

## non-promoted references

- `P-A0-*` Stage B/D/E/F rows remain current-scope evidence, not operational α-0.5 / α-0.8 / α-0.9 completion.
- practical alpha-1 first-floor rows remain evidence, not product/public-ready alpha-1 completion.
- operational helper scope blocks are evidence-backed queue state, not final public product decisions.
