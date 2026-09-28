# W4-C shared local authority use — bounded LAB repair

This is a local implementation dependency of W4-C, not whole-C acceptance, a
network authority protocol, or a change to Canon THM/OBL/Plan250 state.
Direct consumer: the contextual owner branch of `LocalFabric::step_locus_inner`,
including ordinary inbound carriers and the existing I3 owner handoff.
REQ AU01/04/05/08, VF04/05, DS01/02/03/04/08; PT03/11/14, SC04/07, Q18.

## Meaning and alternatives

Two fabrics created from the same sealed admission share an authority floor but
have separate cached authority and M8 backends. Before this repair, one fabric
could publish a genuine capability revocation while its sibling subsequently
executed a checked-source write using its old cache. Both ST and OW1 reproduced
this counterexample, including a generated carrier retained before revocation.

The repair acquires the existing shared floor, compares all runtime authority
facts, and retains the guard through actual backend use. Validation counters do
not participate in equality. Existing M9/M8 admission checks still decide
permission; facts equality itself grants none. Poison/staleness uses the existing
typed refusal. The smallest alternative, checking then releasing before use,
permits a publication between validation and effect. A forced actual execution
schedule rejects this alternative on both backends.

## Definitions and checked propositions

The Lean module quantifies over arbitrary facts, requests and a finite family of
caches. `Allowed` is declarative; `check`/`execute` are executable. Actions are
acquire, use, release, publish and refresh. The invariant says a held cache equals
the head. Fourteen general lemmas cover checker exactness, relative completeness,
initialization and transition/history preservation, current facts at accepted use,
conditional authorization meaning, stale acquisition and exclusion of publication.
Seven finite guards are examples/counterexamples, not general proofs.

`checked_history_use_facts_current` needs no assumption about the semantic
correctness of local authorization. `accepted_use_current` additionally assumes
local checker exactness against the independent authorization predicate. The latter
is not a proof of all Rust authorization checks. There are no Mir-specific axioms,
`sorry` or `admit`; fresh module-owned declaration audit allows only standard Lean
logic axioms. The inspected proof dependencies use `propext` or no axioms.

## Physical correspondence and TCB

- Facts contains the complete compared program/generation/inventory/lineage/
  tombstone data, excluding mutable validation observations. A number alone is
  insufficient. A sibling with different observation counts remains usable.
- Model `held` denotes the owner use interval. A publisher's internal mutex
  acquisition is an unobservable step; publication/refresh cannot interleave with
  a protected owner use. Head monotonicity is a separate existing M9 obligation.
- ST calls M8 synchronously. OW1 sends to one worker and blocks on its synchronous
  reply; the reply follows M8 execution. No production timeout, detached owner
  write or user callback exists in this path. Worker panic can occur after a write
  and before reply: an error must not be interpreted as evidence of no write.
- The guard ends before coordinator mirror/trace/reply processing. The mirror
  update is a real store write representing the earlier protected backend effect.
  It neither authorizes a fresh owner use nor grants observation disclosure.
- Actual lifecycle/patch/restore installers use the same floor. Detached candidate
  execution is separately rechecked at canonical installation. The three inspected
  non-test lifecycle accessor callers drop the temporary accessor before another
  fabric entry. Arbitrary internal nested same-floor access remains non-reentrant;
  ordinary Mir source cannot obtain that private accessor.
- Mutex exclusion, RAII/poison, Rust compiler/stdlib, honest facts construction and
  synchronous worker ordering remain TCB/implementation obligations. There is no
  general Rust memory-model or fairness/availability theorem here.

The repair leaves source/Core correspondence, all non-owner entries, custody,
resources, authenticated multiprocess peers, persistence, secret observation and
G1 staging/G2 resolution versus immutable W3 tickets open for their own consumers.
Q18 is not silently resolved by this local lock.

## Reproduce and evidence

Use Lean4.29.1, `--trust=0 -j1`, and an external fresh copy; see the manual command
in `samples/lean/README.md`. This module imports only `Std` and prints axioms for
all fourteen lemmas. It is outside the frozen W4-B 206-module manifest.

From the repository root, with serial builds and a suitable existing work area:

```sh
cargo test --locked --offline -j 1 -p mir-runtime --lib proof_first_shared_authority_floor -- --test-threads=1
```

The tests use normal source parsing/checking/projection, sealed M9 admission, actual
M8 ST/OW1 execution and actual partition observations. Test-only rendezvous hooks
force publication order or a worker panic; they do not manufacture trace events.
Checks cover unchanged-head positives, stale/queued rejection before M9 use,
current-but-revoked rejection, counter skew, lock poison, actual protected intervals,
historical continuation and worker panic before/after write.

Report2614 and `docs/proof-first/W4_CHECK.json` retain command/hash/results and
Oracle dispositions. Oracle is advisory, not a signature, proof or acceptance.
The original pre-repair failures, unsuccessful resource-limited compilation and
weak-implementation failures remain historical evidence.
