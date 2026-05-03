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
- `samples/practical-alpha1/` は current repo state で存在するが、
  current cut は limited `package.mir.json` front-door fixture family に留まる
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

### PA1-6 — devtools / viewer

- JSON/export schema
- viewer/devtools surface
- redaction / retention checks

### PA1-7 — local save/load command

- practical save/load command
- local roundtrip
- stale membership non-resurrection
- invalid distributed cut reject

### PA1-8 — practical product prototype

- one small product-like world package
- local + Docker
- package/layer attach
- avatar fallback
- local save/load
- devtools export
- hands-on docs

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
9. `P-A1-08` product prototype

current reading after `P-A1-04b`:

- `P-A1-00`、`P-A1-01`、`P-A1-02`、`P-A1-03`、`P-A1-04a`、`P-A1-04b` are closed
- `P-A1-03` fixed the distinct carrier split:
  `checked package -> runtime plan -> local runtime report`
- `P-A1-04a` fixed a second distinct carrier split:
  `checked package -> hotplug plan -> non-final hot-plug report`
- `P-A1-04b` actualized `HP-A1-04B1` attach-time stale-membership reject、
  `HP-A1-04B2` attach-time missing-witness reject、
  and `HP-A1-06` narrow object package attach preview seam
- current actualized hot-plug rows are `HP-A1-01..05`、`HP-A1-04B1`、`HP-A1-04B2`、`HP-A1-06`
- `P-A1-04c` is the promoted next package inside `PA1-4`

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
`P-A1-04b` 後は initial practical sample root、library-first front-door、distinct lowered IR、first checker floor、distinct runtime-plan carrier、first practical local-runtime floor、distinct hotplug-plan carrier、attach-time stale-membership/missing-witness reject、narrow object package attach preview seam は存在するが、detach minimal contract、transport、save/load、devtools、product prototype が未完成であり、typed checking も first floor に留まるため overall readiness は still moderate に留める。

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
  docker/
```

- current first cut uses `packages/` and `expected/` for `SRC-01..05`、`CHK-*`、`RUN-01/02`、`HP-A1-01..05`、`HP-A1-04B1`、`HP-A1-04B2`、`HP-A1-06`
- `source/` and `docker/` are reserved for later textual-source / Docker packages

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

`P-A1-04b` keeps the practical hot-plug validation path and widens it with freshness/object-preview rows:

- `cargo test -p mir-ast --test practical_alpha1_front_door -- --nocapture`
- `cargo test -p mir-ast --test practical_alpha1_hotplug_plan -- --nocapture`
- `cargo test -p mir-runtime --test hotplug_runtime_skeleton -- --nocapture`
- `cargo test -p mir-runtime --test practical_alpha1_hotplug -- --nocapture`
- `cargo test -p mir-runtime --test alpha_layer_insertion_runtime`
- `python3 scripts/practical_alpha1_attach.py check-all --format json`
- `python3 scripts/practical_alpha1_attach.py closeout --format json`

## non-claims carried forward

- current alpha-0 evidence closeout is not public alpha / `U1`
- only a narrow non-final `package.mir.json` practical front-door exists so far
- no final public parser/runtime/checker ABI is fixed
- no production WAN/federation claim is made
- no durable distributed save/load claim is made
- no signed-native-is-safe claim is made

## next reopen point

- after `P-A1-04b`, the next safe package is expected to be `P-A1-04c`
  if the non-final hot-plug floor stays distinct from final object package attach,
  detach contract completion, transport execution, save/load command,
  and final public runtime/devtools ABI
- current recommendation is:
  - keep the current `package.mir.json` cut explicit and non-final
  - keep `P-A1-02` as the first checker floor rather than force full typed-checking completion
  - keep `P-A1-03` as the first local-runtime floor over `RUN-01/02`
  - keep `P-A1-04a` / `P-A1-04b` as the first practical hot-plug floor over `HP-A1-01..05`、`HP-A1-04B1`、`HP-A1-04B2`、`HP-A1-06`
  - carry capability / auth / witness lanes without claiming full runtime enforcement yet
  - keep `samples/alpha/` unchanged while practical root grows separately
- queue authority remains `progress.md` / `tasks.md`
