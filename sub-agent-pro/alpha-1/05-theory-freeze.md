# 05 — Theory Requirements for Practical Alpha-1

## 1. Lifetime / fallback

Maintain current decisions:

- fallback is access-path availability extension, not target lifetime extension
- raw ref / guarded ref / inherit_chain / snapshot_selected are distinct
- implicit chain propagation is forbidden
- write capability must not strengthen through fallback
- expired lease / stale witness / stale membership are not resurrected by rollback/load

Practical alpha-1 checker must cover:

- raw dangling reject
- fallback chain canonicalization
- inherited chain with lineage
- plain ref boundary preservation
- snapshot selected distinction
- capability promotion reject
- read-only fallback write reject
- remote observed ref remains planned/later unless carrier is implemented

## 2. Contract subtyping / layer compatibility

Maintain current decisions:

- input contravariant
- output covariant
- precondition weakening only
- postcondition strengthening only
- effect row containment
- failure row containment
- capability requirement non-strengthening
- mutable/read-write invariant
- observation label / redaction / retention monotonicity

Practical alpha-1 checker/runtime must cover:

- logging/debug observe-only valid
- output covariance valid
- readonly covariance valid
- mutable covariance reject
- precondition strengthening reject
- postcondition weakening reject
- failure row widening reject unless declared
- declared-failure rate-limit path
- auth explicit contract-update path

## 3. Cut / save / load

Maintain current decisions:

- atomic_cut is place-local rollback frontier
- not distributed commit
- distributed save/load requires consistent cut
- invalid distributed cut is rejected
- local save/load may exist but must not claim durable distributed persistence

Practical alpha-1 must cover:

- local save/load roundtrip
- local stale membership non-resurrection
- invalid distributed cut rejection
- explicit non-claim for durable distributed save/load

Later:

- expired lease store non-resurrection
- stale witness store non-resurrection
- communication-induced checkpoint repair
- durable cut

## 4. Hot-plug / runtime package

Maintain current decisions:

- package/layer/object attach goes through request/verdict/activation cut
- rejected attach has no state mutation
- accepted attach is visible in hot-plug lifecycle trace
- native package signature is not semantic safety
- avatar compatibility is adapter/package, not core

Practical alpha-1 must cover:

- debug layer attach/reject
- auth contract-update path
- rate-limit declared-failure path
- incompatible patch reject
- placeholder/custom avatar package
- runtime unavailable fallback
- unsigned native reject
- over-capability reject

## 5. Proof obligations

Alpha-1 does not require full theorem prover completion. It does require proof obligations to be explicit and sample-checked.

Core obligations:

- no successful dangling read
- no implicit re-promotion
- no mutable covariance
- overlay substitutability
- invalid cut rejection
- no hidden resurrection on load
- no hidden API shadowing
- redaction non-leak for observer-safe traces
