# 08 — typed external host boundary

## principle

Mir core does not have privileged standard I/O.

Host interaction is through typed external effect / adapter boundary.

## adapter model

```text
ExternalAdapter<A, B> = {
  adapter_id,
  input_schema: A,
  output_schema: B,
  effect_row,
  failure_row,
  required_capability,
  authority_policy,
  observation_policy,
  host_boundary_kind
}
```

## minimal host-I/O demos

α-0.5 operational readiness should require one of:

### EchoText

```text
input:  text
output: text
behavior: "Hello, " + input
```

### AddOne

```text
input:  int64
output: int64
behavior: input + 1
```

These are not stdio builtins.
They are typed adapter examples.

## required route

```text
host payload
  -> typed external request
  -> adapter admission
  -> runtime effect / transform
  -> typed receipt / output
  -> devtools observation
```

## required evidence

- input schema checked
- output schema checked
- effect row declared
- failure row declared
- adapter authority checked
- receipt emitted
- observer-safe view exported

## why early

FAQ 015 identifies generic host input -> transform -> host output as missing. Without this, α-0.5 remains a repo-internal runtime sample, not a practical developer workflow.

## stop line

- do not introduce stdio as Mir core primitive
- do not treat synthetic typed external preview as active semantic execution
- do not conflate host boundary with transport layer

