# 08 — theory and verification

## 1. Verification stratification

Keep the existing 3-line model:

```text
Line 1: decidable static checker
Line 2: model-check second line
Line 3: proof side line
```

Operational samples should identify which line discharges each property.

## 2. Type/checker obligations for operational sample

### WorldCore

- package schema valid
- world identity declared
- membership/capability policy declared
- observation/redaction/retention policy declared
- native policy declared

### MembershipChat

- dependency on WorldCore valid
- join/leave effects declared
- send-room-message effect declared
- failure row includes StaleMembership / MissingCapability / RateLimited
- observer-safe chat output has redaction policy

### SugorokuWorld

- dependency on MembershipChat valid
- RollDice / PublishRoll / HandoffTurn declared
- witness `draw_pub` declared
- missing witness failure row declared
- stale membership failure row declared
- handoff target active precondition declared

### HotPlug layers

- debug layer requires DebugAuthority
- auth layer is explicit contract update or base contract has declared failure
- rate-limit layer has RateLimited failure row
- incompatible patch rejected
- object/avatar preview deferred boundary explicit

### Save/load

- R0 local savepoint type
- R2 quiescent savepoint type
- NoInFlight / AllPlacesSealed / NoPostCutSend proof or preflight evidence
- no distributed durability claim

## 3. Model-check obligations

For current package, implement only if feasible. Otherwise document planned obligations.

Minimum planned properties:

- stale membership cannot mutate game state
- missing witness cannot handoff
- reset invalidates old action
- hot-plug rejected attach does not mutate active runtime
- quiescent save rejects if in-flight message remains
- portal handoff future: no double membership / no duplicate owner
- two-shard future: no double owner across boundary

## 4. Proof side obligations

Keep as residual if not mechanized.

Examples:

- projection preserves boundary contracts
- observer-safe redaction noninterference
- single-owner handoff invariant for arbitrary shard graph
- source import dependency graph is acyclic
- native host bundle does not imply semantic safety of package-native code

## 5. Cut/save correctness

Use these definitions:

```text
ConsistentCut(K):
  if event e is in K, all causal predecessors of e are in K.

R0 LocalSavePoint:
  local session state only.

R2 QuiescentSavePoint:
  ConsistentCut + NoInFlight + AllPlacesSealed + NoPostCutSend.
```

Do not call R2 distributed durable.

## 6. Auth / layer algebra

Transparent overlay conditions:

- no precondition strengthening
- no postcondition weakening
- no undeclared effect widening
- no undeclared failure widening
- no capability strengthening
- no observation widening
- no redaction weakening
- no retention widening

Auth and rate-limit are usually not transparent. They require explicit contract update or predeclared failure/effect rows.

## 7. Typed external boundary

Host I/O sample should not use stdio builtin.

Use typed adapter:

```text
AddOne: int -> int
EchoText: text -> text  // planned or implemented
ChatText: text -> room event  // operational sample target
```

If EchoText / ChatText is not implemented, document it as planned and keep AddOne as the current executed host-I/O lane.

## 8. Native boundary

Native host launch bundle must preserve:

- package schema
- boundary schemas
- devtools reports
- provenance metadata
- NativeExecutionPolicy = Disabled by default

Do not infer safety from signature.
