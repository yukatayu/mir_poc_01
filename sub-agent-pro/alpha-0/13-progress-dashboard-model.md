# 13 — Progress dashboard model

`progress.md` and `samples_progress.md` must make the repository's current stage obvious at all times.

## 1. Required progress.md fields

Add or maintain a section similar to:

```text
## Current Alpha-0 / Mirrorea Spaces stage

Large stage: Stage B — Alpha 0.5 Integrated local Mirrorea runtime
Concrete phase: Phase 1 — Typed IR and checker skeleton
Current package: P-A0-05 checker skeleton first cut
Current status: in progress / closed / blocked
Validation freshness: <timestamp and commands>
Current blockers: ...
Next autonomous package: ...
User-decision blockers: ...
```

Do not bury the current location in a long recent log.

## 2. Stage rows

Represent the large stages:

| Stage | Name | Current status | Main evidence | Not yet claimed |
|---|---|---|---|---|
| A | repo-local alpha-ready floor | mostly reached | clean-near-end, Sugoroku, helper evidence | final public product |
| B | alpha0.5 local runtime | planned/in progress | Rust Place runtime | real network |
| C | alpha0.7 transport | planned | Docker E2E | production WAN |
| D | alpha0.8 hot-plug lifecycle | planned | request/verdict/activation | durable migration |
| E | alpha0.9 devtools | planned | event DAG/viewer | final viewer API |
| F | alpha1 Spaces alpha | planned | integrated demo | full VRChat/Reversed Library |
| G | Spaces product expansion | future | avatar/world ecosystem | alpha scope |
| H | Atlas | future | multi-world graph | alpha scope |
| I | Reversed Library | future | knowledge space | alpha scope |

## 3. Three-axis progress

For each major layer, track:

- logical specification
- user-facing specification
- implementation/operation

Example:

| Layer | Logical spec | User-facing spec | Impl/op | Current note |
|---|---:|---:|---:|---|
| Lifetime/fallback | 70% | 35% | 20% | specs planned, checker skeleton pending |
| Layer compatibility | 55% | 25% | 10% | variance rules planned |
| Save/load/cut | 45% | 15% | 5% | atomic_cut fixed, distributed save deferred |
| Hot-plug runtime | 55% | 25% | 25% | carrier exists, lifecycle pending |
| Network transport | 35% | 20% | 20% | canaries exist, real transport pending |
| Devtools/viewer | 35% | 35% | 20% | prototype inventory exists |
| Runtime package/avatar | 25% | 20% | 5% | policy planned |
| Mirrorea Spaces alpha | 25% | 20% | 10% | demo pending |

Percentages are rough and evidence-backed. Do not use 100% unless current-scope implementation/docs/tests/report/commit/push are complete.

## 4. samples_progress.md rows

Add rows for sample families:

| Family | Stage/phase | Path | Status | Validation | Blocker | Notes |
|---|---|---|---|---|---|---|
| lifetime-fallback | Phase 1 | `samples/alpha/lifetime-fallback/` | planned/skeleton/runnable | runner command | checker pending | ... |
| contract-variance | Phase 1/4 | ... | ... | ... | ... | ... |
| cut-save-load | Phase 2/6 | ... | ... | ... | ... | ... |
| local-runtime | Phase 3 | ... | ... | ... | ... | ... |
| layer-insertion | Phase 4 | ... | ... | ... | ... | ... |
| network-docker | Phase 5 | ... | ... | ... | ... | ... |
| hotplug-runtime | Phase 4/8 | ... | ... | ... | ... | ... |
| avatar-runtime | Phase 7 | ... | ... | ... | ... | ... |
| visualization | Phase 4/5 | ... | ... | ... | ... | ... |
| e2e | Phase 8 | ... | ... | ... | ... | ... |

## 5. `tasks.md` structure

Keep `tasks.md` as current task map, not exhaustive history.

Required sections:

- current task-level status
- executable floor
- large stage map
- ordered current work
- self-driven maintenance tasks
- autonomous alpha packages
- user decision blockers
- research-discovery items
- validation floor
- reporting requirement

## 6. User-decision blockers to keep visible

- first public surface: core library / CLI / engine adapter / browser client
- first host target: native process / browser / engine / mixed
- first network scope: local / Docker / WAN federation
- avatar compatibility first target: placeholder/custom Mir/VRM/VRChat skeleton
- native binary policy: forbidden / sandboxed / signed trusted
- save/load initial scope: local only / consistent multi-Place / durable replay
- UI target: CLI / HTML viewer / VR viewer
- final shared-space catalog breadth

Use provisional recommendations, but do not silently decide user-facing product adoption.

## 7. Recent log format

Keep a short recent log with timestamps, e.g.:

```text
- 2026-05-XX HH:MM JST — Added Alpha-0 theory freeze specs 13..17; docs scaffold validation passed; no runtime implementation claimed.
```

Do not replicate report details in progress.
