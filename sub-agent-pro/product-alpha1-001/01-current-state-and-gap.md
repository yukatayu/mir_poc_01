# Current state and product/public α-1 gap

## Current repo reading

The current repo has moved beyond alpha-0 evidence. It has:

- practical alpha-1 first-floor package/check/runtime/hotplug/transport/save-load/devtools/product-preview floors;
- bounded operational α-0.5 session runtime;
- bounded operational α-0.8 same-session hot-plug workflow;
- bounded operational α-0.9 session-bound devtools workflow;
- bounded practical α-1 integrated workflow carrier.

This is meaningful. It is not docs-only.

## Remaining gap

It is not yet product/public-ready α-1 because it lacks:

- product-facing CLI / library surface that outside developers can treat as the α-1 entrypoint;
- versioned package format and developer docs;
- product demo that is not only exact-report composition;
- native launch bundle;
- product-ready devtools viewer UX;
- explicit message failure/recovery contract;
- quiescent savepoint implementation for controlled local/Docker scope;
- public-ish validation/release process;
- U1-level decisions or explicit alpha choices around packaging, host target, shipped surface.

## Why earlier goal recognition drifted

### 1. “alpha” had too many meanings

Historically `alpha` referred to:

- repo-local alpha-ready current layer;
- current-scope evidence closeout;
- first-floor evidence;
- bounded operational workflow;
- practical integrated workflow;
- product/public alpha.

This must stop. Always qualify which kind.

### 2. `100%` was artifact-driven

Reports / expected JSON / helpers / first-floor runners used to be counted as completion. The new rule is:

> `100%` only for an externally reproducible operational workflow or product/public layer.

### 3. Codex naturally closed narrow packages

Codex is good at exact sidecars and expected JSON. That was useful, but it over-emphasized evidence artifacts.

### 4. Observability was not strong enough as a per-stage completion condition

The user's intent is to implement while seeing the system run. Therefore every product α-1 stage must include:

- runtime state transition;
- observer-safe export;
- admin/debug export or explicit later marker;
- negative trace;
- viewer story.

### 5. Product/public boundary remained separate but visually blurred

`U1` remained open, but stage names looked product-like. The product α-1 line must explicitly close or defer U1 choices.

## Current product α-1 gap statement

Before product α-1 can be announced, the repo must offer:

```text
clone repo
build alpha CLI
run product demo
inspect viewer
modify small package
run again
save/load or quiescent-save
build native launch bundle
understand all non-goals
```

This is the operational bar.
