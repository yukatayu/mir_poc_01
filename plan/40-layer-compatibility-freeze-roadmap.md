# plan/40 — layer compatibility freeze roadmap

## purpose

この文書は、Alpha-0 line における
`specs/14-contract-subtyping-layer-compatibility.md`
の repository-memory roadmap を置く。

ここで保持するのは current alpha-local compatibility law と sample/checker next line であり、
final public layer-attachment ABI や production auth/telemetry service ではない。

## current repo state

- existing repo already fixes:
  - no silent API shadowing
  - compatibility-preserving overlay direction
  - typed visualization / telemetry boundary
  - transport/auth/membership/capability/witness non-collapse
- existing runtime/sample evidence is strongest around:
  - `LayerSignature`
  - visualization security envelope
  - hot-plug request/verdict carrier
- `P-A0-05` first checker-floor helper actualization now exists via:
  - `scripts/alpha_contract_variance_checker.py`
  - `samples/alpha/contract-variance/var-02` / `03` / `07` / `09` / `10` / `15` sidecars
  synthetic detached artifact comparison only; no layer runtime or parser bridge claim

## decisions mirrored from specs/14

- transparent overlay cannot strengthen preconditions
- transparent overlay cannot weaken guarantees
- effect/failure rows cannot silently widen
- auth/rate-limit often require explicit contract update
- debug/telemetry require authority/redaction/retention
- mutable/read-write carrier remains invariant

## layer insertion implementation roadmap

### first safe cut

- represent Alpha-0 contract variance sample family as planned skeleton + expected verdict
- seed selected negative-static rows with `expected_static.checked_reason_codes`
- map existing runtime/private carriers to attach-point inventory without claiming public ABI
- keep debug/auth/rate-limit/redaction as typed layer categories only

### checker-first cut

- reuse current narrow checker/report floor where possible
- actualize Alpha-0 checker helper coverage for:
  - precondition strengthening reject
  - postcondition weakening reject
  - failure-row widening reject
  - effect-row widening reject
  - cost degradation reject
  - hidden shadowing reject

remaining later in this family:

- mutable covariance reject
- positive covariance / contract-preserving transform anchors

### later runtime cut

- `P-A0-08` first actualized subset now exists as a non-public Rust layer-insertion floor over the `P-A0-07` local-runtime cut:
  accepted debug attach with authority, rejected non-admin debug attach, explicit auth contract-update path, declared-failure rate-limit path, incompatible patch reject
- attachpoint-local contract comparison is current finite-set / exact-type-name floor only
- redaction / retention / trace-mode safety are checked at attach time in the current cut
- broader detach / migration / distributed ordering / public ABI remain later

## devtools / telemetry roadmap

- observer-safe redacted view remains first-class requirement
- retention monotonicity and authority must stay explicit
- do not widen helper-local viewer inventory into final viewer API prematurely

## sample roadmap

sample family root:

- `samples/alpha/contract-variance/`

must keep visible:

- logging layer valid
- precondition strengthening reject
- postcondition weakening reject
- output covariance valid
- mutable covariance reject
- read-only covariance valid
- rate-limit declared failure valid
- cost degradation reject
- adapter transform preserves contract
- debug attach requires authority
- hidden shadowing reject

## deferred questions

- final public attachment API
- exact theorem-backed implication discharge
- final cost algebra
- final label lattice
- production auth schema / telemetry backend

## next package

- after `P-A0-08` layer-insertion first cut:
  `P-A0-09` network / Docker E2E, while fuller contract algebra / theorem discharge remains later
