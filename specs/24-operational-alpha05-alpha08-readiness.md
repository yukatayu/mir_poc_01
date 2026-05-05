# 24 — Operational Alpha-0.5 / Alpha-0.8 Readiness

## role

この文書は、α-0.5 / α-0.8 / α-0.9 を
**artifact closeout ではなく operational readiness**
として読む bounded theory freeze を置く。

- evidence / first-floor evidence / operational readiness / product/public readiness を分ける
- current repo の first floors を operational-ready と混同しない
- final public product completion を claim しない

## decision level

- `L1`
  - `100%` は外部開発者が layer を実際に使える operational workflow または product/public layer にだけ使う
  - helper / sidecar / report / expected JSON / first-floor runner は completion ではなく evidence として分類する
  - evidence と first-floor evidence は operational completion と混同しない
  - α-0.5 / α-0.8 / α-0.9 operational readiness は session-bound workflow を要する
- `L2`
  - current readiness reading
  - rough percentage reading
  - next reopen boundary

## four completion categories

### 1. evidence

repo-local evidence、validation、report、snapshot docs が同期している状態。

examples:

- expected JSON matches
- helper runner passes
- report exists
- docs/status are synced

これは practical usability を意味しない。

### 2. first-floor evidence

non-final だが、limited carrier / runner / sample family が動く状態。

examples:

- limited `package.mir.json` loader
- selected checker rows
- selected local runtime rows
- selected hot-plug / transport / save-load / export rows

これは useful だが、
operational-ready や product-ready とは別 category である。

### 3. operational-layer-ready

開発者が、その layer の目的に沿って
workflow を再現し、動作と状態推移を観測できる状態。

minimum pattern:

```text
input
-> check
-> run / evaluate
-> observe / debug
-> negative case
-> inspectable state transition or rejection
-> README / hands-on procedure
```

### 4. product/public-ready

public API / ABI / packaging / versioning / support boundary が fixed され、
outside developer が依存できる状態。

α-0.5 / α-0.8 / α-0.9 ではここまでは要求しない。

## α-0.5 operational readiness

α-0.5 は local observable runtime である。

definition:

> local runtime 上で、typed package input から checked runtime plan を作り、
> `Place` / `MessageEnvelope` / membership / effect / publish / witness / handoff
> を実行し、event DAG と debug export を見られ、
> local save/load の最小 roundtrip と stale-membership rejection を確認できる状態。

minimum conditions:

- typed package input exists
- checker exists
- runtime plan exists
- local runtime session or equivalent state carrier exists
- event DAG export exists
- observer-safe debug export exists
- negative membership / capability / witness behavior is inspectable
- local save/load resume works
- stale membership non-resurrection is visible
- at least one typed host-I/O minimal demo exists
- README / hands-on procedure reproduces the workflow

## α-0.8 operational readiness

α-0.8 は same-session hot-plug runtime である。

definition:

> α-0.5 session に debug / auth / rate-limit / object / avatar-placeholder package を
> typed contract に基づいて hot-plug し、
> accepted / rejected / deferred / activation cut / trace を
> 同じ runtime session 上で観測できる状態。

minimum conditions:

- attach command/API against a live session
- accepted debug attach
- rejected non-admin attach
- auth explicit contract update
- declared-failure rate-limit path
- incompatible patch reject
- stale membership attach reject
- missing witness attach reject
- object/avatar placeholder visibility in session
- unsupported runtime fallback visibility
- explicit detach accepted/rejected/deferred boundary
- hot-plug lifecycle visible in session-bound devtools
- attach changes later observation or behavior in the same session

## α-0.9 operational readiness

α-0.9 は session-bound devtools である。

definition:

> runtime session の event DAG / route trace / membership timeline / witness relation /
> hot-plug lifecycle / fallback degradation / save-load timeline / redacted observer view を、
> 実行と対応した形で export / viewer 表示できる状態。

minimum conditions:

- session-bound export source
- JSON export
- non-final viewer surface
- observer-safe redacted view
- admin/debug full view or explicit kept-later marker
- redaction / retention semantics
- on-demand trace boundary

## current repo reading

current repo has:

- alpha-0 evidence closeout for Stage B / D / E
- practical first-floor package/checker/runtime/hot-plug/transport/save-load/devtools/product-preview carriers
- bounded α-0.5 same-session runtime carrier
- minimal typed external host-I/O direct semantic execution lane
- bounded α-0.8 same-session hot-plug runtime over the same carrier
- bounded α-0.9 session-bound devtools export over the same carrier
- bounded practical α-1 integrated workflow carrier over exact first-floor evidence and bounded operational carriers
- exact report bundles and companion preview bundles

current repo still lacks at least:

- final public viewer / telemetry ABI
- durable audit backend
- distributed durable save/load
- product/public-ready alpha-1 boundary

therefore:

- Stage B = current-scope evidence
- Stage D = current-scope evidence
- Stage E = current-scope evidence
- practical `RUN-*` / `HP-A1-*` / `TR-A1-*` / `SL-A1-*` / `VIS-A1-*` / `PE2E-*`
  = first-floor evidence
- `OA05-*` / `OA08-*` / `OA09-*` = evidence rows for bounded operational α-0.5 / α-0.8 / α-0.9 workflows
- none of the above imply product/public-ready completion

## workflow-readiness reading

future snapshot docs should use workflow status as the primary reading.

recommended style:

```text
alpha-0.5 evidence rows: evidence-closed
alpha-0.5 local session workflow: workflow-ready only after session carrier + host-I/O lane exist
alpha-0.5 product/public readiness: not ready
```

same reading for α-0.8 / α-0.9:

- helper / sidecar / report / expected JSON / first-floor runner rows are evidence
- operational readiness is claimed only when the corresponding workflow is reproducible
- product/public readiness remains not ready until public boundary is fixed
- `100%` is reserved for externally usable operational or product/public layers, not evidence rows

## required sample matrix

α-0.5 required operational rows:

- accepted local dispatch
- stale membership reject
- missing capability reject
- missing witness reject
- fallback degradation visible event
- local save/load resume
- local save/load stale-membership non-resurrection
- event DAG export
- observer-safe debug export
- typed host-I/O minimal demo

α-0.8 required operational rows:

- debug layer attach accepted
- non-admin debug attach rejected
- auth explicit contract update accepted
- rate-limit declared failure accepted + runtime reject preview
- incompatible patch rejected
- stale membership attach rejected
- missing witness attach rejected
- object package attach preview or visible session representation
- unsupported runtime fallback
- deferred detach minimal contract
- hot-plug lifecycle export
- same-session attach changes devtools/runtime behavior

α-0.9 required operational rows:

- event DAG live/session export
- route trace
- membership timeline
- witness relation
- hot-plug lifecycle
- fallback degradation
- save/load timeline
- observer-safe redacted view
- retention / on-demand trace

## current recommended reopen point

after this theory freeze,
the safest promoted next package is the smallest package that creates a session-bound carrier
without overclaiming runtime breadth.

current recommendation:

- `P-A1-19`、`P-A1-20`、`P-A1-21`、`P-A1-22` are now the bounded actualized operational sequence
- `P-A1-23` is the bounded practical α-1 integrated workflow carrier over that sequence and exact first-floor evidence
- keep final public viewer / telemetry / product runtime completion behind a separate product/public boundary decision

## stop line

- practical alpha-1 complete と書かない
- α-0.5 / α-0.8 / α-0.9 complete と書かない unless the operational conditions are met
- current first-floor evidence を operational-ready と書かない
- helper-local preview / exact report bundle を same-session runtime と書かない
- local save/load を distributed durable save/load と書かない
