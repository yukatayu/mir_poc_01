# 10 — Sub-agent operating model

Use sub-agents actively. This work crosses type theory, distributed systems, runtime architecture, docs taxonomy, and product scope. A single pass is likely to miss issues.

## 1. Required sub-agent roles

### 1.1 Type-system reviewer

Scope:

- lifetime / region / lease
- guarded reference model
- fallback canonicalization
- chain inheritance
- read/write variance
- no re-promotion
- checker obligations

Must answer:

- Does any sample allow target lifetime extension instead of access path extension?
- Are underdeclared cases static errors?
- Are mutable references invariant?
- Are write capabilities prevented from strengthening by fallback?

### 1.2 Contract-variance reviewer

Scope:

- layer insertion
- contract subtyping
- effect row / failure row
- precondition/postcondition variance
- cost/latency
- auth/rate-limit layers

Must answer:

- Can a layer strengthen preconditions silently?
- Can a layer add undeclared failure outcomes?
- Can debug telemetry leak labels?
- Are rate-limit/auth layers treated as contract updates when necessary?

### 1.3 Cut/checkpoint reviewer

Scope:

- `atomic_cut`
- local rollback
- save/load
- consistent cuts
- in-flight messages
- Z-cycle/useless checkpoints
- durable cut deferred boundary

Must answer:

- Is `atomic_cut` still place-local?
- Is distributed save/load forbidden without consistent cut?
- Are witness/publish/hot-plug dependencies included in cut validity?
- Are Z-cycle samples included?

### 1.4 Runtime-package/avatar reviewer

Scope:

- runtime package manifest
- avatar non-core role
- VRM/VRChat/Unity adapter skeletons
- native binary trust
- unsupported runtime fallback

Must answer:

- Did any avatar format become core primitive?
- Does signature get confused with safety?
- Are capability manifests required?
- Is fallback representation monotone and explicit?

### 1.5 Repo taxonomy/docs reviewer

Scope:

- source hierarchy
- specs vs plan vs docs/reports vs progress/tasks
- sample active/planned/generated distinction
- Documentation/progress/tasks/samples_progress sync

Must answer:

- Are normative claims placed in specs?
- Are repository memory claims placed in plan?
- Are planned samples marked planned?
- Are helper-local previews not claimed as final APIs?

### 1.6 Sample coverage reviewer

Scope:

- sample matrix completeness
- positive/negative pairs
- expected verdict sidecars
- runner plan

Must answer:

- Does each theory rule have at least one positive and one negative sample?
- Are save/load/Z-cycle samples present?
- Are avatar/runtime package samples present?
- Are layer insertion samples present?

### 1.7 Product-scope guardian

Scope:

- Mirrorea Spaces vs Reversed Library vs PrismCascade
- VRChat-class baseline vs clone
- browser-like virtual world substrate
- deferred public/product decisions

Must answer:

- Is Reversed Library kept as upper application?
- Is PrismCascade kept separate?
- Is Mirrorea Spaces alpha neither too small nor overclaimed?

### 1.8 Validation runner

Scope:

- commands run
- failures
- git diff
- report evidence
- commit/push

Must answer:

- Were required validations run?
- Were skipped validations justified?
- Is git diff clean of whitespace errors?
- Was a report created?
- Was commit/push done?

## 2. Sub-agent workflow

For a substantial package:

1. Main agent reads required docs and creates implementation plan.
2. Spawn focused sub-agents before finalizing normative text.
3. Integrate findings.
4. Run local validation.
5. Spawn review sub-agents or focused reviewers for changed docs/samples.
6. Resolve all serious findings.
7. Update progress/tasks/samples_progress/report.
8. Commit and push.

## 3. Report requirements for sub-agent findings

The package report must include:

- sub-agents used
- scope of each sub-agent
- key findings
- changes made in response
- unresolved findings, if any
- reason unresolved findings are acceptable or blocker status

## 4. Do not overuse sub-agents for trivial edits

Small typo/formatting changes do not require all sub-agents. But theory freeze, checker rules, save/load, runtime package, and repository reorganization do.

## 5. Stop conditions

Stop and ask user only if:

- a user-facing decision is required and no provisional safe path exists
- two normative directions conflict and cannot be reconciled
- validation reveals a fundamental design contradiction
- repository tool access or permissions make commit/push impossible

Otherwise continue to the next autonomous package.
