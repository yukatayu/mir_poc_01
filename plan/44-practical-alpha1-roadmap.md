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
- practical alpha-1 toolchain の front-door、reusable checker/runtime API、
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

## sample-root roadmap

- keep `samples/alpha/` as evidence root
- do not silently promote it
- practical root is planned as:

```text
samples/practical-alpha1/
  README.md
  packages/
  source/
  expected/
  docker/
```

- this root should not be required by source-hierarchy validators until it exists

## validator roadmap

`P-A1-00` should extend required scaffold to:

- `specs/18-practical-alpha1-scope.md`
- `plan/44-practical-alpha1-roadmap.md`
- `sub-agent-pro/alpha-1/`

but should not require practical runner scripts or `samples/practical-alpha1/`
before they are added.

## non-claims carried forward

- current alpha-0 evidence closeout is not public alpha / `U1`
- no practical source front-door exists yet
- no final public parser/runtime/checker ABI is fixed
- no production WAN/federation claim is made
- no durable distributed save/load claim is made
- no signed-native-is-safe claim is made

## next reopen point

- after `P-A1-00`, the next safe package is expected to be `P-A1-01`
  if a narrow alpha source / manifest front-door can be defined without
  freezing final public grammar
- current recommendation is:
  - alpha source may start as a limited textual or JSON/package manifest cut
  - document it explicitly as alpha-local and non-final
  - keep `samples/alpha/` unchanged while growing a separate practical root
- queue authority remains `progress.md` / `tasks.md`
