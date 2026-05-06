# samples/product-alpha1/operational

This root holds the operational product sample suite introduced by `P-OPS-01` and widened by `P-OPS-03`, `P-OPS-04`, `P-OPS-05`, `P-OPS-06`, and `P-OPS-07`.

It stays separate from `samples/product-alpha1/demo/`.

- `demo/` remains the product alpha release-candidate workflow root.
- `operational/` is the next line: a more realistic development/process suite built from `WorldCore -> MembershipChat -> SugorokuWorld -> PortalWorldLink -> TwoShardHardBoundary`.
- Representative `.mir` files are explanatory only. Current executable input is versioned `package.mir.json`.
- `portal-worldlink/` is the active bounded portal runtime root.
- `two-shard-hard-boundary/` is the active bounded shard runtime root.
- `templates/` holds validated `template_only` authoring starters such as `templates/world-core-starter/`, `templates/membership-chat-starter/`, and `templates/sugoroku-world-starter/`; these roots are allowed to pass `check` and `run-local` but are not counted as active operational sample roots.
- `future/gradient-observation.profile.json` is a docs-first observer-only widening profile, not an executable runtime root.
- backend feasibility is docs-first: `native host launch bundle` is the only actualized backend-adjacent path; WASM/LLVM remain inventory-only
- Portal / shard files under `future/` are blueprint evidence only.

Current runnable floor:

- `world-core/`: `check`, `run-local`
- `membership-chat/`: `check`, `run-local`, bounded `EchoText("Taro") -> "Hello, Taro!"` direct host boundary, and session-bound `export-devtools` / `view --check` via `session#operational-membership-chat`
- `sugoroku-world/`: `check`, `run-local`, bounded same-session roll / publish / witness / handoff / stale membership reject evidence, `session`, `attach`, `save`, `quiescent-save`, `transport`, `export-devtools`, `view`, `build-native-bundle`
  deferred `placeholder-object` / `custom-avatar-preview` attach rows are part of the visible workflow, not hidden inventory
- `portal-worldlink/`: `check`, `run-local`, observer-safe `export-devtools`, and bounded same-session portal resolve / handoff offer / witness emit / destination admit evidence
- `two-shard-hard-boundary/`: `check`, `run-local`, observer-safe `export-devtools`, and bounded same-session shard offer / prepare / commit / old-owner reject / missing-witness reject / stale-config reject evidence
- `deployments/projection/projection.profile.json`: schema-backed non-final projection inventory reflected by `check`, runtime plan, and the observer-safe devtools projection panel for `sugoroku-world`
- `future/portal-worldlink/`: retained blueprint root for future portal manifest wording; it is not the executable root
- `future/two-shard-hard-boundary/`: retained blueprint root for future shard manifest wording; it is not the executable root
- `future/gradient-observation.profile.json`: retained observer-only profile for planned shard overlap reading; it is not an executable root
- `templates/world-core-starter/`: validated starter for bounded `world_core` authoring
- `templates/membership-chat-starter/`: validated starter for bounded `membership_chat` authoring; retarget its `../world-core-starter` dependency after copying it
- `templates/sugoroku-world-starter/`: validated starter for bounded `sugoroku_world` authoring; retarget its `../membership-chat-starter` dependency after copying it
- `docs/hands_on/operational_package_authoring_01.md`: starter selection / rename / dependency-retarget guide for the template catalog
- `docs/hands_on/operational_backend_inventory_01.md`: current host launch bundle / WASM / LLVM comparison inventory for this suite
- `docs/hands_on/operational_gradient_observation_profile_01.md`: docs-first guide for the observer-only gradient future profile
- `scripts/operational_product_samples.py`: orchestration helper around the `mirrorea-alpha` command family

Current non-claims:

- final textual `.mir` grammar
- final server/client binary split
- direct Mir-to-machine-code / LLVM backend
- WAN / federation / continuous infinite shard sync
- distributed durable save/load R3/R4
- final public ABI / SDK
- external issuer-backed auth / membership proof pipeline
- durable/distributed proof-grade `R2` quiescent-save witness

Shortest guide:

```bash
python3 scripts/operational_product_samples.py list --format json
python3 scripts/operational_product_samples.py run-membership-chat --format json
python3 scripts/operational_product_samples.py run-sugoroku --format json
python3 scripts/operational_product_samples.py run-portal-worldlink --format json
python3 scripts/operational_product_samples.py run-two-shard-hard-boundary --format json
python3 scripts/operational_product_samples.py check-all --format json
```
