# 21 — Auth Layer Algebra

## role

この文書は、auth / authorization / rate-limit / debug layer を
**typed contract transformer** として扱う algebra を固定する。

- layer を untyped middleware にしない
- transparent overlay と explicit contract update を分ける
- signature/provenance と semantic safety を分ける

## decision level

- `L1`
  - layer は `Contract -> Contract` transformer として扱う
  - transparent overlay は substitutability を満たす場合に限る
  - auth / rate-limit は often non-transparent である
  - signature is provenance, not semantic safety
- `L2`
  - auth provider interface
  - stack composition law
  - initial standard-library-like auth/policy set

## layer as contract transformer

```text
Layer : Contract -> Contract
```

minimum contract shape:

```text
Contract = {
  input_type,
  output_type,
  precondition,
  postcondition,
  effect_row,
  failure_row,
  required_capabilities,
  provided_surface,
  observation_policy,
  redaction_policy,
  retention_policy,
  cost_bound
}
```

## transparent overlay condition

transparent overlay is admissible only if substitutability holds.

```text
I_base <: I_layer
O_layer <: O_base
P_base => P_layer
Q_layer => Q_base
E_layer ⊆ E_base_allowed
F_layer ⊆ F_base_allowed
R_layer <= R_base
S_base ⊆ S_layer
Obs_layer ⊆ Obs_base_allowed
Redact_layer >= Redact_base
Retain_layer ⊆ Retain_base
```

meaning:

- no input narrowing
- no output weakening
- no precondition strengthening
- no postcondition weakening
- no undeclared effect widening
- no undeclared failure widening
- no required capability strengthening on the ordinary call path
- no provided surface shrink
- no observation widening
- no redaction weakening
- no retention widening

## explicit contract update

if the above transparency law fails,
the layer must go through explicit contract update.

minimum carrier:

```text
ContractUpdate = {
  old_contract_ref,
  new_contract_ref,
  reason_kind,
  activation_cut_ref,
  admitted_by,
  observation_delta
}
```

examples that are usually explicit update, not transparent overlay:

- `member(room)` -> `member(room) AND admin`
- adding `AuthFailed`
- adding `RateLimited`
- requiring stronger debug authority on ordinary calls

## why auth / rate-limit are often non-transparent

auth layer / rate-limit layer often changes:

- preconditions
- failure rows
- required capability / authority
- observation / audit surface

therefore they frequently violate transparency unless:

1. the base contract already declares the failure kinds, and
2. the layer is observe/log only, not admission-changing

## auth provider interface

```text
AuthProvider = {
  provider_id,
  input_evidence_schema,
  output_claim_schema,
  granted_capabilities,
  failure_kinds,
  audit_event_shape,
  revocation_semantics,
  observation_policy
}
```

minimum checker obligations:

- granted capabilities are declared
- failure kinds are within declared failure row
- audit events are explicit
- observation policy is not widened silently
- revocation is monotone unless new epoch/evidence is issued

## auth stack composition

### `all_of`

```text
all_of(A, B)
```

admission requires both `A` and `B`.
This is admissible if failure rows, audit events, and granted-capability scope are declared.

### restricted `any_of`

```text
any_of(A, B)
```

`any_of` is dangerous because a weaker provider may bypass a stronger one.
current rule:

- allowed only if the contract explicitly declares `any_of`
- allowed only if each branch's failure/audit surface is declared
- allowed only if capability grants remain within the declared union
- do not use `any_of` as hidden policy weakening

## containment laws

layer stack composition must preserve explicit containment for:

- effect row
- failure row
- required capability
- provided surface
- observation policy
- redaction policy
- retention policy

ordinary call path strengthening is not allowed to hide inside composition.

## standard-library-like initial auth / policy set

current initial set should remain small and explicit.

```text
AuthNoneDev
MembershipAuth
CapabilityAuth
WitnessAuth
PackageProvenanceAuth
LocalTokenAuth
DebugAuthorityAuth
RateLimitPolicy
RedactionPolicy
RetentionPolicy
PackageAdmissionPolicy
NativeExecutionPolicy
```

notes:

- `AuthNoneDev` is development-only and must be labeled non-production
- `MembershipAuth` checks membership epoch / participant incarnation freshness
- `WitnessAuth` checks witness refs and scope
- `PackageProvenanceAuth` checks signature/provenance only
- `NativeExecutionPolicy` requires sandbox/trust policy beyond signature

## debug layer authority

debug layer must declare:

- who may attach
- who may read
- labels / redaction
- retention
- whether runtime behavior can change
- on-demand vs always-on trace

observer-safe mode must not expose raw witness payload, raw auth evidence, high-label state, or secrets.

## signature / provenance boundary

signature proves provenance.
it does not prove:

- capability safety
- semantic safety
- resource safety
- isolation safety
- fallback correctness

native package admission still needs:

- explicit capability manifest
- effect/failure containment
- resource limits
- sandbox / crash containment
- revocation story
- audit + redaction boundary

## required anchors

current freeze で visible であるべき row family:

- `VAR-08`
- `VAR-11`
- `VAR-13`
- `LI-01..05`
- `HP-A1-01..05`
- `HP-A1-04B1`
- `HP-A1-04B2`
- `HP-A1-07`
- `TR-A1-07`

`VAR-14` remains separate as adapter-transform preservation line.

## deferred

- final public auth schema
- final public attachment ABI
- final cost algebra
- final label lattice
- production telemetry service

## stop line

- auth / rate-limit layer を無条件 transparent overlay と書かない
- failure row widening を hidden にしない
- signature を semantic safety proof と書かない
- debug layer を label-less leak と書かない
- adapter transform preservation を already discharged と書かない
