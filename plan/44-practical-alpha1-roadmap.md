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

### PA1-2 — typed IR / checker

- IR definitions
- checker diagnostics
- LIF / VAR / CUT obligations の reusable checker integration
- `check` front-door

### PA1-3 — runtime plan execution

- checked plan から local world 実行
- reusable runtime API
- event DAG export
- local world runnable sample

### PA1-4 — package / hot-plug practical API

- package manifest schema
- package admission checker
- debug/auth/rate-limit/object attach
- negative hot-plug cases

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

current reading after `P-A1-01`:

- `P-A1-00` and `P-A1-01` are closed
- `P-A1-02` is the promoted next package

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
`P-A1-01` 後は initial practical sample root と library-first front-door は存在するが、
checker/runtime/product surfaces が未完成なため overall readiness は still low に保つ。

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

- current first cut uses `packages/` and `expected/` for `SRC-01..05`
- `source/` and `docker/` are reserved for later textual-source / Docker packages

## validator roadmap

`P-A1-00` extends required scaffold to:

- `specs/18-practical-alpha1-scope.md`
- `plan/44-practical-alpha1-roadmap.md`
- `sub-agent-pro/alpha-1/`

`P-A1-01` may require `samples/practical-alpha1/README.md` and the root directory,
but should not require practical runner scripts before they are added.

## non-claims carried forward

- current alpha-0 evidence closeout is not public alpha / `U1`
- only a narrow non-final `package.mir.json` practical front-door exists so far
- no final public parser/runtime/checker ABI is fixed
- no production WAN/federation claim is made
- no durable distributed save/load claim is made
- no signed-native-is-safe claim is made

## next reopen point

- after `P-A1-01`, the next safe package is expected to be `P-A1-02`
  if the current front-door output can be connected to reusable typed-checker
  diagnostics without freezing a final public checker API
- current recommendation is:
  - keep the current `package.mir.json` cut explicit and non-final
  - add IR/checker integration on top of the existing front-door output
  - keep `samples/alpha/` unchanged while practical root grows separately
- queue authority remains `progress.md` / `tasks.md`
