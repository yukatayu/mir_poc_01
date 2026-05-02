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
- `P-A0-05` first checker-floor helper actualization, widened by `P-A0-16`, and complemented by `P-A0-17` helper-local acceptance floor now exists via:
  - `scripts/alpha_contract_variance_checker.py`
  - `scripts/alpha_contract_variance_acceptance.py`
  - `samples/alpha/contract-variance/var-02` / `03` / `05` / `07` / `09` / `10` / `15` sidecars
  - `samples/alpha/contract-variance/var-01` / `04` / `06` sidecars
  synthetic detached artifact comparison only; negative rows are confined to `reason_codes_scope = alpha-static-floor`, positive rows are confined to `acceptance_scope = alpha-acceptance-floor`, and there is still no layer runtime or parser bridge claim
- `P-A0-18` runtime-mirror closeout now exists via:
  - `scripts/current_l2_family_runtime_mirror_support.py`
  - `scripts/alpha_contract_variance_runtime_mirror.py`
  - `samples/alpha/contract-variance/var-08` / `11` / `13` sidecars
  - `samples/alpha/layer-insertion/li-04` / `01` / `03` runtime-floor sidecars as mirror source authority
  mirror evidence only; `runtime_mirror.scope = alpha-runtime-mirror-floor` stays separate from both `reason_codes_scope = alpha-static-floor` and `acceptance_scope = alpha-acceptance-floor`, and there is still no parser/runtime bridge claim for `contract-variance/`

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
  - mutable covariance reject
  - failure-row widening reject
  - effect-row widening reject
  - cost degradation reject
  - hidden shadowing reject

remaining later in this family:

- current helper-local acceptance floor actualizes `VAR-01/04/06` only
- current runtime-mirror floor actualizes `VAR-08/11/13` only, using already-actualized
  `LI-04/01/03` runtime-floor sidecars as source authority
- broader positive covariance / contract-preserving transform anchors stay later
- runtime/layer-backed compatibility evidence stays later

### later runtime cut

- `P-A0-08` first actualized subset now exists as a non-public Rust layer-insertion floor over the `P-A0-07` local-runtime cut:
  accepted debug attach with authority, rejected non-admin debug attach, explicit auth contract-update path, declared-failure rate-limit path, incompatible patch reject
- `P-A0-18` mirrors that already-actualized subset back into `contract-variance/` only for:
  - `VAR-08 <- LI-04`
  - `VAR-11 <- LI-01`
  - `VAR-13 <- LI-03`
  without widening the runtime semantics, the acceptance floor, or the parser/runtime bridge boundary
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
- redaction layer valid
- cost degradation reject
- auth layer contract update valid
- adapter transform preserves contract
- debug attach requires authority
- hidden shadowing reject

## deferred questions

- final public attachment API
- exact theorem-backed implication discharge
- final cost algebra
- final label lattice
- production auth schema / telemetry backend

## next reopen point

- after `P-A0-18`, the current contract-variance widening is intentionally limited to:
  - negative checker-floor rows `VAR-02/03/05/07/09/10/15`
  - helper-local synthetic acceptance rows `VAR-01/04/06`
  - runtime-mirror rows `VAR-08/11/13` mirrored from `LI-04/01/03`
- current line still keeps out:
  - `VAR-14` because adapter transform preservation semantics are not yet actualized
  - any parser/runtime bridge claim for `contract-variance/`
- next safe reopen requires either:
  - a narrower carrier for `VAR-14`
  - or a separate remaining-row inventory/package for `LIF-11/13/15`
- queue authority remains `progress.md` / `tasks.md`
