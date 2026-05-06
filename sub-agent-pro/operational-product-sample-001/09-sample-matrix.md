# 09 — sample matrix

## OPS-01 — WorldCore

Status target:

- executable package if current schema supports it
- representative `.mir` always included

Files:

```text
samples/product-alpha1/operational/world-core/README.md
samples/product-alpha1/operational/world-core/world-core.mir
samples/product-alpha1/operational/world-core/package.mir.json
samples/product-alpha1/operational/expected/ops-01-world-core.expected.json
```

Required evidence:

- check accepted
- package schema valid
- world identity declared
- membership/capability policy declared
- observation/redaction/retention policy declared

## OPS-02 — MembershipChat

Files:

```text
samples/product-alpha1/operational/membership-chat/README.md
samples/product-alpha1/operational/membership-chat/membership-chat.mir
samples/product-alpha1/operational/membership-chat/package.mir.json
samples/product-alpha1/operational/expected/ops-02-membership-chat.expected.json
```

Required evidence:

- dependency on WorldCore represented
- join / leave / send message declared
- observer-safe chat view declared
- stale membership failure row declared
- rate-limit failure row declared

If string chat cannot execute in current runtime:

- keep chat as declared package surface
- do not claim direct string host-I/O execution
- use AddOne as the executed host-I/O lane only if it is wired through the package

## OPS-03 — SugorokuWorld

Files:

```text
samples/product-alpha1/operational/sugoroku-world/README.md
samples/product-alpha1/operational/sugoroku-world/sugoroku-world.mir
samples/product-alpha1/operational/sugoroku-world/package.mir.json
samples/product-alpha1/operational/expected/ops-03-sugoroku-world.expected.json
```

Required evidence:

- dependency on MembershipChat represented
- roll / publish / witness / handoff declared
- missing witness failure row declared
- stale membership failure row declared
- run-local produces event DAG
- save/load and quiescent-save commands work if package can be run

## OPS-04 — HotPlug Layers

Files:

```text
samples/product-alpha1/operational/packages/debug-layer/package.mir.json
samples/product-alpha1/operational/packages/auth-layer/package.mir.json
samples/product-alpha1/operational/packages/rate-limit-layer/package.mir.json
samples/product-alpha1/operational/packages/placeholder-object/package.mir.json
samples/product-alpha1/operational/packages/custom-avatar-preview/package.mir.json
```

Required evidence:

- debug/auth/rate-limit attach accepted where proper authority is declared
- object/avatar preview may be deferred
- lifecycle visible in devtools
- no hidden native execution

## OPS-05 — Deployment and projection profile

Files:

```text
samples/product-alpha1/operational/deployments/local/deployment.profile.json
samples/product-alpha1/operational/deployments/docker/docker-compose.operational.yml
samples/product-alpha1/operational/deployments/projection/projection.profile.json
```

Required evidence:

- local workflow described
- Docker workflow described if runnable
- projection profile separates server/client intent
- no emitted server/client native binary claim
- packet/FFI boundary inventories present

## OPS-06 — Devtools diagrams

Files:

```text
samples/product-alpha1/operational/expected/ops-devtools-*.expected.json
```

Required panels:

- import graph
- package graph
- projection graph
- Place graph
- route graph
- event DAG
- membership/config frontier
- witness timeline
- hot-plug lifecycle
- save/load timeline
- contract/effect/failure summary

## OPS-07 — Portal / WorldLink future

Files:

```text
samples/product-alpha1/operational/future/portal-worldlink/README.md
samples/product-alpha1/operational/future/portal-worldlink.package.mir.json
```

Required evidence:

- planned / future status if not runnable
- portal contract fields documented
- no WAN/federation claim

## OPS-08 — Spatial Shard future

Files:

```text
samples/product-alpha1/operational/future/two-shard-hard-boundary/README.md
samples/product-alpha1/operational/future/spatial-shard-future.profile.json
```

Required evidence:

- authority region / observation region / boundary region distinction
- single-owner first
- no global vector-clock default
- optional replication profiles listed
- no continuous federation completion claim
