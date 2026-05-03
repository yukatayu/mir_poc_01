# 15 — Cut / Save / Load / Checkpoint（Alpha-0）

## role

この文書は、Mirrorea Spaces alpha line における
**`atomic_cut` / save-load / consistent cut / rollback / checkpoint dependency**
の alpha-local 規範を置く。

- `atomic_cut` の place-local boundary を保持する
- distributed save/load without consistent cut を禁止する
- local save/load と distributed durable save/load を混同しない

## decision level

- `L1`
  - `atomic_cut` は place-local rollback frontier のみを確定する
  - distributed save/load には consistent cut predicate が必要である
  - load は stale fact を resurrect しない
- `L2`
  - current alpha-local causal closure set
  - save-state content inventory
  - Z-cycle / useless-checkpoint boundary

## `atomic_cut` retained boundary

`atomic_cut` は current `Place` の rollback frontier だけを final にする。

それは次ではない。

- global synchronization point
- process-wide transaction commit
- distributed consensus point
- durable persistence guarantee
- distributed save/load checkpoint

rollback は same `Place` 内でも `atomic_cut` を跨いで hidden に進まない。

## event DAG and causal order

alpha-local event graph は acyclic でなければならない。
minimum causal family は少なくとも次を含む。

- `program_order`
- `state_dependency_order`
- `send_envelope -> receive_envelope`
- `publish -> observe`
- `witness_create -> witness_use`
- `handoff_predecessor -> handoff_successor`
- `hotplug_request -> hotplug_verdict -> activation_cut`
- `membership_update -> dependent dispatch`
- `capability_grant -> capability_use`
- `auth_evidence_create -> auth_evidence_use`

## consistent cut predicate

cut `K` は causal order に関して prefix-closed であるとき consistent である。

```text
if e ∈ K and e' ≺ e, then e' ∈ K
```

minimum operational consequences:

- receive を cut に入れるなら send も入れるか channel state として represent する
- observe を入れるなら publish を入れる
- witness use を入れるなら witness create を入れる
- capability use を入れるなら capability grant / admission frontier を入れる
- auth evidence use を入れるなら auth evidence create / provenance frontier を入れる
- hot-plug activation を入れるなら request / verdict を入れる
- membership-dependent dispatch を入れるなら relevant membership frontier を入れる

## save-state contents

multi-Place save は local memory dump では足りない。
current alpha-local inventory は少なくとも次を含む。

- per-Place local state frontier
- queues and in-flight `MessageEnvelope`
- channel / transport state or equivalent
- `MembershipRegistry` frontier
- `PlaceCatalog` frontier
- capability / authority state
- capability grant / admission provenance frontier
- auth evidence frontier
- witness store
- lease / freshness frontier
- fallback degradation position
- hot-plug lifecycle state
- runtime package version/admission state
- adapter state
- external irreversible effect obligations
- randomness / provider evidence when relevant

## load / rollback constraints

load or rollback は次を満たさなければならない。

1. saved cut is consistent
2. rollback does not cross `atomic_cut`
3. durable/finalized prefix is not reverted unless future explicit durable protocol says so
4. expired lease is not resurrected
5. stale witness / membership epoch is not resurrected
6. capability grant / auth evidence frontier is not resurrected or severed from dependent use
7. in-flight message treatment is explicit
8. irreversible external effects are isolated or compensated
9. runtime package activation state is cut-consistent

## local save/load alpha scope

current alpha-local implementation claim として admissible なのは次である。

- local Place save/load
- consistent-cut checker for proposed multi-Place snapshots
- negative samples for inconsistent distributed cut
- current Stage B closeout may reuse only the local-only subset
  `CUT-04` and `CUT-17`
  together with the integrated local-runtime floor

これだけで distributed durable save/load completion を claim しない。
また、`CUT-04/17` を使って Stage B を閉じても、
`CUT-10/12/16` や family 全体の completion は意味しない。

## distributed save/load deferred scope

alpha-local current line で deferred に残すもの:

- production distributed snapshot algorithm
- durable replay service
- consensus mechanism
- WAN federation persistence contract
- durable migration / distributed activation ordering protocol

## Z-cycle / useless checkpoint boundary

checkpoint dependency graph に Z-cycle がある場合、
その checkpoint は consistent global checkpoint に属せない可能性がある。

current alpha-local rule:

- consistent-cut story without Z-cycle story is insufficient
- distributed save/load claim requires either avoidance, detection, or handling
- if only local save/load is implemented, distributed claim must remain absent

## `durable_cut` deferred boundary

`durable_cut` は Mir-0 に含めない。
current alpha-local sample / checker line では implemented item として扱わない。

- use of `durable_cut` in Mir-0 sample is rejected or deferred
- durable persistence meaning and storage realization remain later work

## irreversible external effects and compensation

effect は rollback behavior で少なくとも次に分ける。

- pure / local reversible
- local state reversible before `atomic_cut`
- external reversible with explicit compensation
- external irreversible
- external isolated/provider-owned

rollback region に external irreversible effect を含める場合、
isolation か explicit compensation obligation が必要である。

## proof / checker obligations

future checker / proof line が preserve すべき obligation は次である。

1. `atomic_cut` rollback boundary
2. consistent-cut prefix closure
3. no orphan receive
4. no orphan observe
5. no orphan witness use
6. hot-plug activation closure
7. no orphan capability use
8. no orphan auth-evidence use
9. load preserves consistency
10. no lease / witness / membership / capability provenance resurrection
11. irreversible effect compensation or isolation
12. Z-cycle checkpoint inadmissibility

## required sample references

current alpha-local required sample family は少なくとも次を含む。

- `CUT-01` local try rollback before cut
- `CUT-02` rollback across `atomic_cut` rejected
- `CUT-03` irreversible effect in rollback region rejected
- `CUT-04` local save/load valid
- `CUT-05` inconsistent distributed snapshot rejected
- `CUT-06` in-flight message channel state valid
- `CUT-07` observe without publish rejected
- `CUT-08` witness use without create rejected
- `CUT-09` hot-plug activation without request rejected
- `CUT-10` load does not resurrect expired lease
- `CUT-11` Z-cycle checkpoint invalid
- `CUT-14` capability use without grant rejected
- `CUT-15` auth evidence use without create rejected
- `CUT-16` load does not resurrect stale witness
- `CUT-17` load does not resurrect stale membership
- `CUT-13` `durable_cut` deferred in Mir-0

sample inventory の current repository memory は
`plan/41-save-load-checkpoint-roadmap.md` と
`samples/alpha/cut-save-load/` を参照する。

## deferred

この spec で intentionally deferred に残すのは次である。

- production distributed snapshot algorithm
- durable cut implementation
- durable migration / reattach protocol
- distributed activation ordering
- final storage backend choice
- replay service
- final compensation algebra

## stop line

- `atomic_cut` を global / durable / distributed commit と書かない
- local save/load or cut checker を distributed save/load completion と書かない
- in-flight message / witness / publish / hot-plug closure を省略しない
- Z-cycle risk を ignore しない
- this alpha-local spec を durable distributed persistence completion と混同しない
