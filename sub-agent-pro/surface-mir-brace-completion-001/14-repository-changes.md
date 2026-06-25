# 14 — Repository Changes

## Specs to add

```text
specs/39-surface-mir-placement-elaboration.md
specs/40-indexed-state-semantics.md
specs/41-role-admission-and-capability-grant.md
specs/42-source-patch-hotplug-semantics.md
specs/43-surface-mir-v1-alpha-scope.md
```

## Plans to add

```text
plan/64-surface-mir-placement-roadmap.md
plan/65-indexed-state-roadmap.md
plan/66-role-admission-roadmap.md
plan/67-source-patch-hotplug-roadmap.md
plan/68-surface-full-system-v1-roadmap.md
```

## Sample roots

```text
samples/full-system-v1-surface/
  world-core/
  membership-chat/
  sugoroku-world/
  role-admission/
  patch-hotplug/
  posegraph/
  projection/
  provider/
```

## Crate changes likely required

```text
crates/mir-ast
  surface parser / AST

crates/mir-semantics
  surface-to-core elaboration
  indexed-state semantics
  computational + effect rows

crates/mir-runtime
  source patch hot-plug
  indexed state runtime
  role admission runtime
  devtools source/core linkage

crates/mirrorea-cli
  check-source
  parse-source
  elaborate-source
  patch-source
  export-core-ir
```

## Scripts

```text
scripts/surface_mir_samples.py
scripts/surface_mir_release_check.py
scripts/surface_mir_authoring_check.py
```

## Docs

```text
README.md
Documentation.md
progress.md
tasks.md
samples_progress.md
samples/README.md
scripts/README.md
docs/hands_on/surface_mir_alpha_01.md
docs/hands_on/source_patch_hotplug_01.md
docs/research_abstract/surface_mir_alpha_01.md
```

## Report prefix

```text
p-surf-*
```
