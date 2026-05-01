# 14 — Contract Subtyping と Layer Compatibility（Alpha-0）

## role

この文書は、Mirrorea Spaces alpha line における
**contract subtyping / layer insertion / variance / observation safety** の
alpha-local 規範を置く。

- final public layer-attachment API を freeze しない
- production auth schema / telemetry backend / final cost algebra を freeze しない
- transparent overlay と explicit contract update を混同しない

## decision level

- `L1`
  - compatibility-preserving overlay は hidden contract narrowing をしない
  - no silent API shadowing
  - auth / rate-limit / debug / telemetry は typed layer として扱う
- `L2`
  - current alpha contract shape
  - layer kind / attach point inventory
  - current decidable containment checks

## contract shape

current alpha-local contract comparison は少なくとも次の row を持つ。

```text
Contract = {
  input_type,
  output_type,
  precondition,
  postcondition,
  effect_row,
  failure_row,
  required_capabilities,
  provided_capabilities,
  provided_interface,
  cost_bound,
  observation_labels,
  redaction_policy,
  observation_policy,
  retention_policy
}
```

exact serialization / public API field naming は deferred とする。

## transparent overlay substitutability

layer `C'` が base contract `C` を transparent に wrap / replace できるのは、
existing client safety を保つ場合に限る。

### input contravariance

```text
I_base <: I_layer
```

layer は base が受け入れた input を少なくとも受け入れなければならない。

### output covariance

```text
O_layer <: O_base
```

layer は existing client が読める output を返さなければならない。

### precondition weakening

```text
P_base => P_layer
```

layer は caller に stronger requirement を課してはならない。

### postcondition strengthening

```text
Q_layer => Q_base
```

layer は base guarantee を弱めてはならない。

## effect / failure containment

### effect row containment

```text
E_layer ⊆ E_base_allowed
```

- undeclared external telemetry write
- undeclared adapter side effect
- undeclared persistence side effect

のような widening は invalid である。

### failure row containment

```text
F_layer ⊆ F_base_allowed
```

- undeclared `AuthFailed`
- undeclared `MissingCapability`
- undeclared `RateLimited`
- undeclared `Deferred`

のような outcome を silent に add してはならない。

## capability monotonicity

transparent layer は same contract call に対して
caller capability requirement を strengthen してはならない。

allowed:

- attach-time admin capability を layer installation に要求する

not allowed:

- ordinary call path を attach 後に admin-only に変える
- fallback / subtyping / wrapper insertion を通して stronger capability を要求する

## provided surface preservation

transparent overlay は provided/interface surface を hidden に shrink してはならない。

- required client-visible entry / field / capability を silent に消さない
- rename / rebinding があるなら explicit contract update path に送る
- `provided_capabilities` / `provided_interface` は base-visible surface を preserve する

hidden shadowing reject は provider side にも適用する。

## cost / latency / resource bound

layer は declared cost / latency / resource bound を worsen してはならない。
例外は、contract が `Approximate` / `Deferred` / `RateLimited` のような
explicit degraded outcome を already declared している場合だけである。

## observation / redaction / retention

layer は observation noninterference を守らなければならない。

- higher-label data を unauthorized observer へ leak しない
- debug / telemetry output は redaction rule を持つ
- retention は declared retention policy を超えない
- debug / visualization は untyped debug leak ではなく
  typed information-bearing effect として扱う

transparent overlay comparison では少なくとも次を守る。

```text
observation_labels_layer ⊆ observation_labels_base_allowed
redaction_policy_layer ⪰ redaction_policy_base
retention_policy_layer ⊆ retention_policy_base_allowed
```

- observer set を widen しない
- redaction を weaken しない
- retention を widen しない

## current variance rules

current alpha-local variance floor は次である。

```text
Read<T>        covariant
Observe<T>     covariant
Write<T>       non-covariant
ReadWrite<T>   invariant
MutableRef<T>  invariant
Linear<T>      invariant
```

read-only covariance は label / contract / observation policy の下でのみ admissible である。
mutable / read-write / linear carrier を Java-like に covariant にしない。

## layer kinds

current alpha-local layer inventory は少なくとも次を含む。

- `Debug`
- `Telemetry`
- `Auth`
- `Authorization`
- `RateLimit`
- `Redaction`
- `Adapter`
- `Policy`
- `Projection`

## attach points

current alpha-local attach point inventory は少なくとも次を含む。

- `MessageDispatch`
- `EffectCall`
- `PlaceBoundary`
- `StateTransition`
- `PublishObserve`
- `WitnessUse`
- `Handoff`
- `HotPlugLifecycle`
- `ExternalAdapter`
- `VisualizationStream`

## auth / rate-limit special cases

auth / rate-limit layer は often non-transparent である。

transparent として admissible なのは次の場合に限る。

1. base contract が `AuthFailed` / `RateLimited` を already declared している
2. layer が observe/log only で admission/outcome を変えない

explicit contract version update + activation cut を通る path は
transparent overlay ではなく **explicit contract update path** として扱う。

base precondition を
`member(room)` から `member(room) AND admin`
へ変えるような insertion は transparent ではない。

## debug layer authority

debug layer は security-sensitive layer である。

最低限次を明示しなければならない。

- who may attach
- who may read
- observation labels
- redaction rule
- retention scope
- whether it can affect runtime behavior
- whether tracing is on-demand or always-on

observer-safe mode では raw witness ref / raw auth evidence / secret / high-label value を出してはならない。

## proof / checker obligations

future checker / proof line が preserve すべき obligation は次である。

1. overlay substitutability
2. precondition weakening
3. postcondition strengthening
4. effect-row containment
5. failure-row containment
6. mutable invariance
7. read-only covariance soundness
8. provider-side surface preservation / no silent API shadowing
9. observation-label containment
10. redaction monotonicity
11. retention compliance

## required sample references

current alpha-local required sample family は少なくとも次を含む。

- `VAR-01` logging layer valid
- `VAR-02` precondition strengthening rejected
- `VAR-03` postcondition weakening rejected
- `VAR-04` output covariance valid
- `VAR-05` mutable covariance rejected
- `VAR-06` read-only covariance valid
- `VAR-07` failure row widening rejected
- `VAR-08` rate-limit declared failure valid
- `VAR-09` effect row widening rejected
- `VAR-10` cost degradation rejected
- `VAR-11` redaction layer valid
- `VAR-12` debug layer requires authority
- `VAR-13` auth layer contract update valid
- `VAR-14` adapter transform preserves contract
- `VAR-15` hidden shadowing rejected

sample inventory の current repository memory は
`plan/40-layer-compatibility-freeze-roadmap.md` と
`samples/alpha/contract-variance/` を参照する。

## deferred

この spec で intentionally deferred に残すのは次である。

- full dependent refinement subtyping
- theorem-prover-backed implication discharge
- final cost/latency algebra
- final label lattice
- final public layer attachment API
- production auth schema
- production telemetry service
- final viewer / verifier public contract

## stop line

- layer insertion を untyped middleware と書かない
- auth / rate-limit outcome widening を transparent overlay と書かない
- debug / telemetry を label-less leak と書かない
- mutable/read-write carrier を covariant と書かない
- this alpha-local spec を final public attachment ABI freeze と混同しない
