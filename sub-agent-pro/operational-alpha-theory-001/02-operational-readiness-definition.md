# 02 — operational readiness definition

## four completion categories

### 1. evidence closeout

repo-local evidence が存在し、validation / report / docs / snapshot が同期している状態。

例:

- expected JSON が一致する
- sidecar がある
- helper runner が pass する
- report がある

これは実用可能性を意味しない。

### 2. first-floor closeout

non-final だが、限定された carrier / runner / sample family が動く状態。

例:

- limited `package.mir.json` loader
- selected checker rows
- selected local runtime rows
- selected save/load rows

これは operational readiness の一部だが、完成ではない。

### 3. operational-layer-ready

開発者が、当該 layer の目的に沿って実際に workflow を再現し、動作と状態推移を観測できる状態。

最低条件:

```text
input exists
check exists
run/evaluate exists
observe/debug exists
negative case exists
state transition or rejection is inspectable
README / hands-on procedure exists
```

### 4. product-ready / public-ready

public API / ABI / docs / packaging / versioning / support boundary が固定され、外部開発者が依存できる状態。

α-0.5 / α-0.8 ではここまでは要求しない。

## revised alpha numbering

### α-0.5 — local observable runtime

local runtime 上で、typed package input から checked runtime plan を作り、Place / MessageEnvelope / membership / effect / publish / witness / handoff を実行し、event DAG と debug export を見られ、local save/load の最小 roundtrip と stale-membership rejection を確認できる状態。

必須:

- package input
- checker
- runtime plan
- local runtime execution
- session or equivalent runtime state carrier
- event DAG export
- observer-safe debug export
- local save/load resume
- stale membership non-resurrection
- at least one typed host-I/O minimal sample

### α-0.8 — same-session hot-plug runtime

α-0.5 session に debug / auth / rate-limit / object / avatar-placeholder package を typed contract に基づいて hot-plug し、accepted / rejected / deferred / activation cut / trace を同じ runtime session 上で観測できる状態。

必須:

- attach command or API against a session
- accepted debug attach
- rejected non-admin attach
- auth explicit contract update
- declared-failure rate-limit
- incompatible patch reject
- missing witness / stale membership reject
- object/avatar placeholder attach or preview with session visibility
- deferred detach minimal contract
- hot-plug lifecycle devtools export

### α-0.9 — session-bound devtools

runtime session の event DAG / route trace / membership timeline / witness relation / hot-plug lifecycle / fallback degradation / save-load timeline / redacted observer view を、実行と対応した形で export / viewer 表示できる状態。

必須:

- session-bound export source
- JSON export
- non-final HTML or viewer surface
- observer-safe view
- admin/debug view or explicit kept-later marker
- redaction / retention semantics
- on-demand trace boundary

## rule for percentages

今後、裸の `100%` は operational-layer-ready または product-ready のいずれかを明示して使う。

推奨表記:

```text
α-0.5 first-floor: 100%
α-0.5 operational readiness: 65%
α-0.5 product readiness: 0%
```

