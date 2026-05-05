# Product α-1 devtools and visualization design

## Design principle

Devtools are not polish. They are part of the product's theory:

> If a transition, message, witness, fallback, cut, or hot-plug decision matters semantically, the developer must be able to observe it.

Product α-1 must present the running system in a way that explains:

- what happened;
- why it happened;
- who/what authorized it;
- which effect/failure path applied;
- what is visible to observer vs admin/debug view;
- what can be saved/restored.

## Viewer surfaces

Implement static HTML or local viewer. Static HTML is enough if it loads JSON bundles.

### 1. Product overview panel

Show:

- package id/version;
- runtime session id;
- place count;
- participant count;
- active layers/packages;
- current membership epoch;
- current config epoch;
- last savepoint;
- warnings/non-claims.

### 2. Place graph

Graph nodes:

- Place
- ParticipantPlace
- Game/Object/Layer place or runtime package
- Adapter/Host boundary

Edges:

- owns/hosts
- sends to
- observes
- attaches to

Use labels, not raw ids only.

### 3. Event DAG

Nodes:

- local transition;
- send/receive;
- publish/observe;
- witness create/use;
- fallback degrade;
- attach request/verdict/activation;
- save/load/quiescent seal/drain.

Edges:

- program order;
- send -> receive;
- publish -> observe;
- witness create -> use;
- request -> verdict -> activation cut;
- save protocol order.

Must be filterable by place, sample, session, event kind.

### 4. Message/envelope route graph

Show:

- from_place -> to_place;
- transport medium;
- auth lane;
- membership epoch/incarnation;
- capability requirements;
- witness refs;
- dispatch outcome.

Observer-safe mode must redact raw witness/auth evidence.

### 5. Membership/config frontier timeline

Timeline:

- join;
- leave;
- membership epoch advance;
- incarnation change;
- config epoch advance;
- stale message reject.

This panel explains why stale membership was rejected.

### 6. Witness relation timeline

Show:

- witness created;
- used by handoff/save/hot-plug;
- missing witness rejection.

### 7. Hot-plug lifecycle panel

Rows:

- request;
- compatibility check;
- auth/capability/witness check;
- verdict;
- activation cut;
- deferred detach boundary;
- rejected attach.

Show whether the operation mutated runtime state.

### 8. Save/load and quiescent save panel

Show:

- savepoint class R0/R1/R2/etc.;
- all_places_sealed;
- no_inflight;
- no_post_cut_send;
- drained/failed messages;
- restore result;
- stale membership/witness/lease non-resurrection status.

### 9. Message failure/recovery panel

Show:

- message id;
- state progression;
- transport contract;
- failure kind;
- recovery policy;
- retry/fallback/reject result.

### 10. Fallback degradation panel

Show:

- logical access path;
- selected option;
- degraded from/to;
- reason;
- witness/lease/membership frontier;
- whether fallback is visible or hidden (must be visible).

### 11. Auth/capability decision panel

Show:

- required capabilities;
- granted capabilities;
- auth stack;
- contract update yes/no;
- failure row;
- rejection reason.

### 12. Observer vs admin/debug

Two modes:

- observer-safe:
  - no raw witness;
  - no auth secrets;
  - redacted payloads;
  - public labels.
- admin/debug:
  - richer fields;
  - still no secrets unless explicitly allowed;
  - clear "debug view" banner.

### 13. Retention/on-demand trace panel

Show:

- retained artifacts;
- retention scope;
- fetch query;
- hit/miss;
- expired/leased-out only if implemented.

## Visual encoding

Use consistent shapes:

- Place: rounded rectangle
- Message: arrow
- Witness: diamond
- Cut/savepoint: vertical barrier
- Rejection: red outline or warning icon
- Fallback degradation: dashed arrow from old to new target
- Hot-plug activation: thick boundary line
- Quiescent save: seal/drain barrier

Use simple accessible colors and include text labels. Do not rely on color alone.

## Required outputs

```text
devtools/
  viewer.html
  bundle.json
  event-dag.json
  place-graph.json
  route-trace.json
  membership-timeline.json
  witness-timeline.json
  hotplug-lifecycle.json
  save-load.json
  message-recovery.json
  redaction-summary.json
```

## Validation

- JSON schema-ish validation.
- Viewer openability command.
- Observer-safe leak test.
- Admin/debug view availability or explicit kept-later marker.
- Product demo screenshots are optional; generated HTML is required.

## Non-claims

- no final public viewer API;
- no telemetry service backend;
- no durable audit backend unless actually implemented.
