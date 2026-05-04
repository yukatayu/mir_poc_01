# plan/44 — practical alpha-1 roadmap

## purpose

この文書は、Mirrorea Spaces practical alpha-1 line の
repository-memory roadmap を置く。

ここで保持するのは、

- practical alpha-1 readiness の段階
- alpha-0 evidence closeout との関係
- safe next package
- non-claim / blocker inventory

であり、final public product roadmap ではない。

## current repo state

- `P-A0-01..28` により、`specs/13..17` / `plan/39..43` / `samples/alpha/` に基づく
  current-scope evidence closeout line は一巡している
- Stage A..F の existing 100% は current-scope evidence closeout であり、
  practical alpha-1 readiness 100% ではない
- active runnable roots は依然として:
  - `samples/clean-near-end/`
  - `samples/current-l2/`
  - `samples/lean/`
- `samples/alpha/` は mixed evidence root であり、
  practical front-door sample root ではない
- `samples/practical-alpha1/` は current repo state で存在し、
  current cut は limited `package.mir.json` front-door fixture family に加え、
  checker / local-runtime / hot-plug / transport の current practical fixture family まで actualize 済みである
- practical alpha-1 toolchain の reusable checker/runtime API、
  product-like package root、practical CLI/viewer surface はまだ未完成である

## decisions mirrored from specs/18

- future `100%` は practical alpha-1 readiness を指す
- current-scope evidence closeout は separate metric として保持する
- alpha-1 は library-first + CLI + viewer/devtools の toolchain である
- parser/front-door は required だが final grammar ではなくてよい
- typed IR / checker / runtime plan / reusable runtime を経由する
- transport は Docker/local TCP まで
- save/load は local save/load + invalid distributed cut reject まで
- avatar/package は non-core runtime package / adapter として扱う
- Reversed Library と PrismCascade は scope 外

## practical alpha-1 package map

### PA1-0 — rebaseline / honesty repair

- current-scope evidence closeout と practical readiness を分離する
- `specs/18` と `plan/44` を追加する
- snapshot docs / dashboards / validators を practical semantics に同期する

### PA1-1 — alpha source front-door

- limited alpha source / manifest format
- parser か JSON/package loader
- positive / negative parse tests
- separate practical sample root initial cut
- current actualized cut:
  - `samples/practical-alpha1/`
  - `crates/mir-ast/src/practical_alpha1.rs`
  - `SRC-01..05` parse/load tests over `package.mir.json`
- non-claim:
  - final textual grammar ではない
  - checker/runtime execution ではない
  - practical CLI command completion ではない

### PA1-2 — typed IR / checker first floor

- IR definitions
- checker diagnostics
- LIF / VAR / CUT obligations の reusable checker integration
- `check` front-door
- current actualized cut:
  - `crates/mir-ast/src/practical_alpha1_checker.rs`
  - `crates/mir-ast/examples/mir_practical_alpha1_check.rs`
  - `scripts/practical_alpha1_check.py`
  - `samples/practical-alpha1/packages/chk-*/`
  - `samples/practical-alpha1/expected/chk-*.expected.json`
- current non-claim:
  - full `specs/18` typed-checking completion ではない
  - runtime plan emission ではない
  - local run / Docker run ではない
  - final public checker / CLI API ではない

### PA1-3 — runtime plan execution

- checked plan から local world 実行
- reusable runtime API
- event DAG export
- local world runnable sample
- current actualized cut:
  - `crates/mir-ast/src/practical_alpha1_runtime_plan.rs`
  - `crates/mir-runtime/src/practical_alpha1_local_runtime.rs`
  - example `crates/mir-runtime/examples/mir_practical_alpha1_run_local.rs`
  - `scripts/practical_alpha1_run_local.py`
  - `samples/practical-alpha1/packages/run-01-local-sugoroku/`
  - `samples/practical-alpha1/packages/run-02-stale-membership-rejected/`
  - `samples/practical-alpha1/expected/run-01-local-sugoroku.expected.json`
  - `samples/practical-alpha1/expected/run-02-stale-membership-rejected.expected.json`
- current non-claim:
  - package/hot-plug API completion ではない
  - Docker/local TCP execution ではない
  - local save/load command ではない
  - final public runtime / devtools ABI ではない

### PA1-4 — package / hot-plug practical API

- package manifest schema
- package admission checker
- debug/auth/rate-limit/object attach
- negative hot-plug cases
- current internal recut:
  - `P-A1-04a`: layer-only practical hot-plug first floor
  - `P-A1-04b`: freshness/missing-witness negatives plus object-attach seam
  - `P-A1-04c`: detach minimal contract

### PA1-5 — transport practical E2E

- practical package inputから local TCP / Docker E2E
- stale membership / missing capability / missing witness negatives
- route trace export
- auth lane separation
- current actualized cut:
  - `crates/mir-ast/src/practical_alpha1_transport_plan.rs`
  - `crates/mir-runtime/src/practical_alpha1_transport.rs`
  - example `crates/mir-runtime/examples/mir_practical_alpha1_transport.rs`
  - `scripts/practical_alpha1_transport.py`
  - `samples/practical-alpha1/packages/tr-a1-01-local-tcp-accepted/`
  - `samples/practical-alpha1/packages/tr-a1-02-docker-two-node-accepted/`
  - `samples/practical-alpha1/packages/tr-a1-03-stale-membership-rejected/`
  - `samples/practical-alpha1/packages/tr-a1-04-missing-capability-rejected/`
  - `samples/practical-alpha1/packages/tr-a1-05-missing-witness-rejected/`
  - `samples/practical-alpha1/packages/tr-a1-06-observer-safe-route-trace/`
  - `samples/practical-alpha1/packages/tr-a1-07-auth-lane-preserved/`
  - `samples/practical-alpha1/expected/tr-a1-01-local-tcp-accepted.expected.json`
  - `samples/practical-alpha1/expected/tr-a1-02-docker-two-node-accepted.expected.json`
  - `samples/practical-alpha1/expected/tr-a1-03-stale-membership-rejected.expected.json`
  - `samples/practical-alpha1/expected/tr-a1-04-missing-capability-rejected.expected.json`
  - `samples/practical-alpha1/expected/tr-a1-05-missing-witness-rejected.expected.json`
  - `samples/practical-alpha1/expected/tr-a1-06-observer-safe-route-trace.expected.json`
  - `samples/practical-alpha1/expected/tr-a1-07-auth-lane-preserved.expected.json`
  - `samples/practical-alpha1/docker/docker-compose.practical-alpha1.yml`
- current carrier split:
  `checked package -> transport plan -> non-final transport report`
- current non-claim:
  - WAN/federation ではない
  - local save/load command ではない
  - devtools / viewer completion ではない
  - product prototype completion ではない
  - final public transport ABI ではない

### PA1-6 — devtools / viewer

- current widened floor after `P-A1-06` and `P-A1-09`:
  - `scripts/practical_alpha1_export_devtools.py`
  - exact expected bundles `samples/practical-alpha1/expected/vis-a1-01-event-dag-export.expected.json`
  - `samples/practical-alpha1/expected/vis-a1-02-route-trace-export.expected.json`
  - `samples/practical-alpha1/expected/vis-a1-04-hotplug-lifecycle.expected.json`
  - `samples/practical-alpha1/expected/vis-a1-06-redacted-observer-view.expected.json`
- current carrier split:
  `exact practical reports -> distinct devtools export bundle -> non-final viewer`
- current actualized observables:
  - `VIS-A1-01` event DAG + publication / witness / handoff relation export
  - `VIS-A1-02` observer-safe route trace export
  - `VIS-A1-04` hot-plug lifecycle export over exact attach accepted and deferred detach boundary reports
  - `VIS-A1-06` redacted observer view with auth-lane separation
- current deferred observables:
  - `VIS-A1-03` membership timeline
  - `VIS-A1-05` fallback degradation
  - `VIS-A1-07` retention/on-demand trace
- current non-claim:
  - full devtools / viewer completion ではない
  - `VIS-A1-04` は detach runtime lifecycle execution ではない
  - local save/load command ではない
  - product prototype completion ではない
  - final public viewer / telemetry / runtime-devtools ABI ではない

### PA1-7 — local save/load command

- practical save/load command
- local roundtrip
- stale membership non-resurrection
- invalid distributed cut reject
- current actualized cut after `P-A1-07`:
  - `crates/mir-ast/src/practical_alpha1_save_load_plan.rs`
  - `crates/mir-runtime/src/practical_alpha1_save_load.rs`
  - example `crates/mir-runtime/examples/mir_practical_alpha1_save_load.rs`
  - `scripts/practical_alpha1_save_load.py`
  - `samples/practical-alpha1/packages/sl-a1-01-local-save-load-resume/`
  - `samples/practical-alpha1/packages/sl-a1-02-local-load-stale-membership-rejected/`
  - `samples/practical-alpha1/expected/sl-a1-01-local-save-load-resume.expected.json`
  - `samples/practical-alpha1/expected/sl-a1-02-local-load-stale-membership-rejected.expected.json`
- current actualized rows:
  - `SL-A1-01`
  - `SL-A1-02`
- current carrier split:
  - `checked package -> runtime plan`
  - `checked package + one exact local-runtime frontier + distinct save-load plan -> saved local frontier -> non-final save-load report`
- current guard reuse:
  - `CHK-CUT-01` is reused only as an existing orphan-receive checker guard
- current non-claim:
  - full save/load completion ではない
  - full consistent-cut / `Z-cycle` completion ではない
  - stale witness / stale lease non-resurrection completion ではない
  - distributed durable save/load ではない
  - Docker/local TCP save/load ではない
  - hot-plug lifecycle persistence ではない
  - final public save-load ABI ではない

### PA1-8 — first practical product-preview floor

- one small product-like world preview bundle
- local + Docker evidence composition
- companion layer/object preview evidence
- local save/load preview
- devtools viewer preview
- hands-on docs
- current actualized cut after `P-A1-08`:
  - `samples/practical-alpha1/previews/`
  - `scripts/practical_alpha1_product_preview.py`
  - exact expected bundles `samples/practical-alpha1/expected/pe2e-a1-*.expected.json`
- current actualized rows:
  - `PE2E-01` local full-toolchain preview
  - `PE2E-02` Docker full-toolchain preview
  - `PE2E-03` debug-layer companion preview
  - `PE2E-04` placeholder object companion preview
  - `PE2E-05` local save/load continue preview
  - `PE2E-06` invalid distributed save rejected preview
  - `PE2E-07` devtools viewer preview
- current carrier split:
  `preview manifest -> exact practical reports / exact practical devtools bundles -> non-final product-preview bundle`
- current non-claim:
  - full practical product prototype completion ではない
  - `PE2E-04` は `HP-A1-06` placeholder object preview companion evidence に narrow される
  - custom Mir avatar runtime ではない
  - unsupported runtime fallback ではない
  - same-session runtime attach / detach lifecycle execution ではない
  - final public CLI / viewer / package-avatar / save-load / transport ABI ではない

## current recommendation on implementation order

recommended current promoted line:

1. `P-A1-00` practical-alpha-rebaseline
2. `P-A1-01` alpha-source front-door design / initial implementation cut
3. `P-A1-02` typed IR / checker integration
4. `P-A1-03` runtime plan execution
5. `P-A1-04` package / hot-plug API
6. `P-A1-05` transport practical E2E
7. `P-A1-06` devtools / viewer
8. `P-A1-07` local save/load command
9. `P-A1-09` exact-report hot-plug lifecycle export widening
10. `P-A1-08` first practical product-preview floor

current reading after `P-A1-08`:

- `P-A1-00`、`P-A1-01`、`P-A1-02`、`P-A1-03`、`P-A1-04a`、`P-A1-04b`、`P-A1-04c`、`P-A1-05`、`P-A1-06`、`P-A1-07`、`P-A1-09`、`P-A1-08` are closed
- `P-A1-03` fixed the distinct carrier split:
  `checked package -> runtime plan -> local runtime report`
- `P-A1-04a` fixed a second distinct carrier split:
  `checked package -> hotplug plan -> non-final hot-plug report`
- `P-A1-04b` actualized `HP-A1-04B1` attach-time stale-membership reject、
  `HP-A1-04B2` attach-time missing-witness reject、
  and `HP-A1-06` narrow object package attach preview seam
- `P-A1-04c` actualized `HP-A1-07` as an explicit deferred detach minimal contract with `operation_kind = detach` and `detach_boundary_ref`
- current actualized hot-plug rows are `HP-A1-01..05`、`HP-A1-04B1`、`HP-A1-04B2`、`HP-A1-06`、`HP-A1-07`
- `P-A1-05` fixed a third distinct carrier split:
  `checked package -> transport plan -> non-final transport report`
- `P-A1-05` actualized `TR-A1-01..07` as local TCP accept, Docker Compose TCP accept, stale-membership reject, missing-capability reject, missing-witness reject, observer-safe route trace, and auth-lane separation
- `P-A1-06` fixed a fourth distinct carrier split:
  `exact practical reports -> distinct devtools export bundle -> non-final viewer`
- `P-A1-06` actualized `VIS-A1-01/02/06` as event DAG + publication/witness/handoff export, observer-safe route trace export, and redacted observer view with auth-lane separation
- `P-A1-09` safely reopened `PA1-6` before `P-A1-08` and actualized `VIS-A1-04` as exact-report hot-plug lifecycle export over `HP-A1-01` accepted attach and `HP-A1-07` deferred detach boundary, without claiming detach runtime lifecycle execution
- `P-A1-07` fixed a fifth distinct carrier split:
  `checked package -> runtime plan`, plus `one exact practical local-runtime frontier + distinct save-load plan -> saved local frontier -> non-final save-load report`
- `P-A1-07` actualized `SL-A1-01/02` as local-only roundtrip resume and stale-membership non-resurrection first-floor rows
- `CHK-CUT-01` is reused in `P-A1-07` only as an orphan-receive checker guard, not as full consistent-cut or `Z-cycle` completion
- `P-A1-08` fixed a sixth distinct carrier split:
  `preview manifest -> exact practical reports / exact practical devtools bundles -> non-final product-preview bundle`
- `P-A1-08` actualized `PE2E-01..07` as a first practical product-preview floor over exact existing practical carriers
- `PE2E-04` is explicitly narrowed to `HP-A1-06` placeholder object preview companion evidence only
- `P-A1-08` does not actualize custom Mir avatar runtime or unsupported runtime fallback, and `PA1-8` remains a first floor rather than full practical product completion

## readiness reading

rough initial reading after `P-A0-28` and before `P-A1-00` close:

- theory / sample evidence foundation:
  strong
- practical parser/front-door:
  weak
- practical checker/runtime reuse:
  weak to moderate
- practical transport/devtools/save-load/product surface:
  partial but fragmented

practical alpha-1 overall readiness should therefore stay low until
front-door, reusable checker/runtime, and practical sample root exist.
`P-A1-04c` 後は initial practical sample root、library-first front-door、distinct lowered IR、first checker floor、distinct runtime-plan carrier、first practical local-runtime floor、distinct hotplug-plan carrier、attach-time stale-membership/missing-witness reject、narrow object package attach preview seam、explicit deferred detach minimal contract は存在するが、transport、save/load、devtools、product prototype が未完成であり、typed checking も first floor に留まるため overall readiness は still moderate に留める。

## sample-root roadmap

- keep `samples/alpha/` as evidence root
- do not silently promote it
- practical root is:

```text
samples/practical-alpha1/
  README.md
  packages/
  source/
  expected/
  previews/
  docker/
```

- current first cut uses `packages/` and `expected/` for `SRC-01..05`、`CHK-*`、`RUN-01/02`、`HP-A1-01..05`、`HP-A1-04B1`、`HP-A1-04B2`、`HP-A1-06`、`HP-A1-07`、`TR-A1-01..07`、`SL-A1-01/02`
- `previews/` now holds `PE2E-01..07` preview manifests and `expected/` also carries the exact expected `pe2e-a1-*.expected.json` product-preview bundles
- `docker/` now contains the Compose fixture used by `TR-A1-02`
- `source/` is reserved for later textual-source packages

## validator roadmap

`P-A1-00` extends required scaffold to:

- `specs/18-practical-alpha1-scope.md`
- `plan/44-practical-alpha1-roadmap.md`
- `sub-agent-pro/alpha-1/`

`P-A1-01` may require `samples/practical-alpha1/README.md` and the root directory,
but should not require practical runner scripts before they are added.

`P-A1-03` adds the first practical local-runtime validation path:

- `cargo test -p mir-ast practical_alpha1_runtime_plan -- --nocapture`
- `cargo test -p mir-runtime --test practical_alpha1_local_runtime -- --nocapture`
- `python3 scripts/practical_alpha1_run_local.py check-all --format json`

`P-A1-04c` keeps the practical hot-plug validation path and widens it with the detach row:

- `cargo test -p mir-ast --test practical_alpha1_front_door -- --nocapture`
- `cargo test -p mir-ast --test practical_alpha1_hotplug_plan -- --nocapture`
- `cargo test -p mir-runtime --test hotplug_runtime_skeleton -- --nocapture`
- `cargo test -p mir-runtime --test practical_alpha1_hotplug -- --nocapture`
- `cargo test -p mir-runtime --test alpha_layer_insertion_runtime`
- `python3 scripts/practical_alpha1_attach.py check-all --format json`
- `python3 scripts/practical_alpha1_attach.py closeout --format json`

`P-A1-05` adds the practical transport validation path:

- `docker --version`
- `docker compose version`
- `cargo test -p mir-ast --test practical_alpha1_transport_plan -- --nocapture`
- `cargo test -p mir-runtime --test practical_alpha1_transport -- --nocapture`
- `python3 scripts/practical_alpha1_transport.py check-all --format json`
- `python3 scripts/practical_alpha1_transport.py closeout --format json`

`P-A1-08` adds the first practical product-preview validation path:

- `cargo test -p mir-ast --test practical_alpha1_front_door -- --nocapture`
- `cargo test -p mir-ast practical_alpha1_checker -- --nocapture`
- `cargo test -p mir-ast practical_alpha1_runtime_plan -- --nocapture`
- `cargo test -p mir-ast --test practical_alpha1_hotplug_plan -- --nocapture`
- `cargo test -p mir-ast --test practical_alpha1_save_load_plan -- --nocapture`
- `cargo test -p mir-runtime --test practical_alpha1_local_runtime -- --nocapture`
- `cargo test -p mir-runtime --test practical_alpha1_hotplug -- --nocapture`
- `cargo test -p mir-runtime --test practical_alpha1_transport -- --nocapture`
- `python3 scripts/practical_alpha1_check.py check-all --format json`
- `python3 scripts/practical_alpha1_run_local.py check-all --format json`
- `python3 scripts/practical_alpha1_attach.py check-all --format json`
- `python3 scripts/practical_alpha1_transport.py check-all --format json`
- `python3 scripts/practical_alpha1_export_devtools.py check-all --format json`
- `python3 scripts/practical_alpha1_save_load.py check-all --format json`
- `python3 scripts/practical_alpha1_product_preview.py check-all --format json`
- `python3 scripts/practical_alpha1_product_preview.py render-html PE2E-07 --format json`
- `python3 scripts/practical_alpha1_product_preview.py closeout --format json`

## non-claims carried forward

- current alpha-0 evidence closeout is not public alpha / `U1`
- only a narrow non-final `package.mir.json` practical front-door exists so far
- no final public parser/runtime/checker ABI is fixed
- no production WAN/federation claim is made
- no durable distributed save/load claim is made
- no signed-native-is-safe claim is made

## next reopen point

- no safe promoted package exists at the current snapshot after `P-A1-08`
  because the first practical product-preview floor is closed but `AV-A1-02/03` custom-avatar / unsupported-fallback semantics remain unactualized, and the remaining `VIS-A1-03/05/07` observables plus broader save/load widening are not yet backed by equivalent exact practical evidence
- current recommendation is:
  - keep the current `package.mir.json` cut explicit and non-final
  - keep `P-A1-02` as the first checker floor rather than force full typed-checking completion
  - keep `P-A1-03` as the first local-runtime floor over `RUN-01/02`
  - keep `P-A1-04a` / `P-A1-04b` / `P-A1-04c` as the first practical hot-plug floor over `HP-A1-01..05`、`HP-A1-04B1`、`HP-A1-04B2`、`HP-A1-06`、`HP-A1-07`
  - keep `P-A1-05` as the first practical transport floor over `TR-A1-01..07` without collapsing it into WAN/federation, save/load, devtools, or product prototype claims
  - keep `P-A1-06` + `P-A1-09` as the widened practical devtools export floor over `VIS-A1-01/02/04/06` without collapsing it into full devtools completion or membership/fallback/retention completion, and without reading `VIS-A1-04` as detach runtime lifecycle completion
  - keep `P-A1-08` as the first practical product-preview floor over `PE2E-01..07` without collapsing it into custom Mir avatar runtime, unsupported runtime fallback, or final product prototype completion
  - keep `HP-A1-07` as explicit deferred detach boundary only; do not upgrade it into accepted detach runtime execution
  - carry capability / auth / witness lanes without claiming full runtime enforcement yet
  - keep `samples/alpha/` unchanged while practical root grows separately
  - reopen only after a safe next package is identified for either practical `AV-A1-02/03` carriers or an equally narrow remaining exact-evidence widening
- queue authority remains `progress.md` / `tasks.md`
