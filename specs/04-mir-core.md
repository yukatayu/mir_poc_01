# 04 — Mir Core

## Status

Mir Core is the most foundational part of the project.
Its architecture should be treated as decision level L0/L1 unless explicitly marked otherwise.

## The four pillars

### 1. Causality and Time
- Computation is represented as a directed acyclic graph of events.
- Boundaries such as `barrier`, `atomic_cut`, and `durable_cut` matter semantically.
- The system must be able to explain "what happened before what" and "what became final when".

### 2. Effects and Contracts
- External actions are first-class effects, not hidden implementation details.
- Contracts (`require`, `ensure`, `invariant`) are part of executable meaning.
- Failure is not a generic exception bucket; it is structured and contract-aware.

### 3. Ownership and Lifetime
- Linear or ownership-like resources matter.
- Lifetimes and fallback-style degradation must remain monotone.
- Preference chains and guarded references are currently an active design topic.

### 4. Safe Evolution
- Patches should preserve meaning and respect graph discipline.
- Default rule: evolve by downstream addition rather than arbitrary rewiring.
- Overlays must preserve compatibility and must not silently shadow existing APIs.

## Core language concerns already discussed

### Effects and failure
The project has discussed a failure lattice with at least the following categories:
- Reject,
- Approximate,
- Compensate.
The exact final formalization remains under refinement, but the project treats failure as semantically structured.

### `try` / rollback
A context-aware `try` that rolls back local state on a thrown effect is considered natural if:
- rollback boundaries are explicit,
- non-rollbackable effects are prohibited or compensable,
- cuts remain non-crossable by rollback.

### `emit`
`emit` is intended as an algebraic-effect-style operation rather than an ad-hoc event primitive.
Important unresolved details include:
- resumable vs abortive vs notify-style handlers,
- handler routing axes,
- interaction with patch hierarchy and authority.

### Fallback / preference chains
Original fallback examples involved lifetime-ordered references such as `A > B > C > ...` and nested fallback choices.
The current leading cleanup direction is to normalize these into monotone **preference chains**.
This remains an important unresolved semantic area.

### Coroutine support
Current understanding:
- one-shot, structured, same-place coroutines fit best,
- multi-shot is only plausible under strong restrictions,
- `await` inside cut/rollback boundaries is likely forbidden,
- coroutine support should be treated as a structured convenience layer over the core semantics, not as the core itself.

## What Mir is not
- It is not the same thing as the deployment fabric.
- It is not a domain-specific media runtime.
- It is not a virtual-reality engine.
- It is not only a type system; it also includes operational boundaries and evolution rules.
