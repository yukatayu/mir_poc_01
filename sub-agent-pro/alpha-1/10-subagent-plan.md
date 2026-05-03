# 10 — Sub-agent Plan

## 1. Required review perspectives

Use sub-agents or explicit focused reviews for:

- theory / type-system
- runtime / transport
- package / hot-plug / native policy
- devtools / visualization / redaction
- docs / progress / dashboard consistency
- sample / validation

## 2. When to use sub-agents

Use them for every non-trivial package, especially when:

- updating specs
- changing progress percentages
- adding checker rules
- adding runtime package / transport logic
- changing validation floor
- claiming stage completion

## 3. Reviewer prompts

### Theory reviewer

Ask:

- Does this overclaim semantics?
- Does it preserve fallback/lifetime invariants?
- Does it respect contract variance?
- Does it keep local/distributed cut boundary honest?

### Runtime reviewer

Ask:

- Is this reusable runtime/toolchain, or only sample-ID bridge?
- Are transport/auth/membership/capability/witness lanes separated?
- Are hot-plug and package claims narrow enough?

### Docs reviewer

Ask:

- Do `progress.md`, `tasks.md`, `samples_progress.md` use practical alpha-1 percentages correctly?
- Is current-scope evidence separated from practical readiness?
- Are non-claims explicit?

### Validation reviewer

Ask:

- Are positive and negative samples both covered?
- Are skipped validations justified?
- Are commands reproducible?

## 4. If sub-agent does not return

- wait and retry once if reasonable
- do focused local review
- report missing review as not returned
- do not treat silence as approval
