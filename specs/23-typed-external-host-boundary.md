# 23 — Typed External Host Boundary

## role

この文書は、host input / host output を
**typed external adapter boundary** として扱う current freeze を置く。

- stdio builtin を Mir core に入れない decision を維持する
- host boundary と transport layer を混同しない
- α-0.5 operational readiness に必要な最小 host-I/O demo family を定義する

## decision level

- `L1`
  - standard I/O is not Mir core primitive
  - host interaction is through typed external adapter boundary
  - synthetic preview and active semantic execution are distinct
- `L2`
  - external adapter carrier
  - minimal `EchoText` / `AddOne` sample family
  - current missing execution lane inventory

## adapter model

```text
ExternalAdapter<A, B> = {
  adapter_id,
  input_schema,
  output_schema,
  effect_row,
  failure_row,
  required_capability,
  authority_policy,
  observation_policy,
  host_boundary_kind
}
```

minimum declared fields that must stay explicit:

- input payload schema
- output payload schema
- effect row
- failure row
- authority policy
- observation / redaction policy
- host boundary kind

## no stdio builtin decision

Mir core keeps:

- causality
- effects
- contracts
- ownership / lifetime
- safe evolution

Mir core does not gain privileged builtin:

- `stdin`
- `stdout`
- `stderr`
- ad-hoc console primitive

host I/O remains at the typed external / adapter boundary.

## required route

```text
host payload
  -> typed external request
  -> adapter admission
  -> runtime effect / transform
  -> typed receipt / output
  -> devtools observation
```

host boundary is not transport delivery.
transport may carry the request,
but the host boundary keeps its own typed contract.

## minimal host-I/O sample family

α-0.5 operational readiness requires at least one of:

### `EchoText`

```text
input:  text
output: text
behavior: "Hello, " + input
```

### `AddOne`

```text
input:  int64
output: int64
behavior: input + 1
```

These are typed adapter examples.
They are not stdio builtins.

## required evidence

- input schema checked
- output schema checked
- effect row declared
- failure row declared
- adapter authority checked
- typed receipt emitted
- observer-safe export available

## current repo reading

current repo has useful partial evidence:

- typed external preview lane
- named request / receipt / local-queue preview
- observer-safe viewer inventory
- active runtime samples with internal publish/output behavior

current repo still lacks:

- typed external direct semantic execution lane
- generic host input ingestion -> transform -> host output runtime path
- minimal reusable `EchoText` / `AddOne` runnable family
- session-bound host-I/O observation path

therefore current typed external floor is not yet operational α-0.5 host-I/O completion.

## relationship to transport

- transport concerns delivery medium / route / membership / witness lane separation
- host boundary concerns payload schema / authority / effect / receipt semantics

do not collapse auth / capability / witness / observation policy into transport metadata.

## future required package boundary

next executable host-I/O line should remain narrow:

- one typed request
- one typed response
- one explicit authority gate
- one observer-safe export path
- no new Mir core primitive

repository-memory sequencing is kept in `plan/49`.

## deferred

- final public host adapter ABI
- final host target choice
- final packaging / installation surface
- general interactive shell surface

## stop line

- stdio builtin を Mir core に入れない
- synthetic typed external preview を active semantic execution と書かない
- host boundary を transport layer と混同しない
- host-I/O demo 不在で α-0.5 operational-ready と書かない
