# AGENTS.md

This repository is intended for repeated work by agents that may start with **no retained context**.
The repository therefore treats documentation structure as part of the project's correctness.

## Non-negotiable rules

1. **Read in order**
   - Start with `README.md`, then `Documentation.md`, then the ordered specs in `specs/00...03`, then `specs/09`, then the subsystem-specific document you need.

2. **Do not invent requirements**
   - If something is not decided, write **UNRESOLVED** or **OPEN QUESTION**.
   - Do not silently turn a hypothesis into a fact.

3. **Respect decision levels**
   - `L0` = foundational / changing it affects the whole system.
   - `L1` = strong directional decision.
   - `L2` = design proposal under active refinement.
   - `L3` = exploratory / unresolved.
   These labels appear throughout the specs and must be preserved.

4. **Always write a new report**
   - Every non-trivial task must create a **new** markdown file under `docs/reports/`.
   - Never overwrite a previous report.
   - Use the report template and include: objective, documents consulted, actions taken, files changed, commands run, evidence, unresolved questions, suggested next prompt.

5. **Keep the architecture separable**
   - Mir, Mirrorea, PrismCascade, and the Typed-Effect Wiring Platform are related but intentionally separable.
   - Do not collapse them into a single implementation without an explicit design decision.

6. **Preserve core invariants**
   - Directed acyclic graph discipline for patch evolution.
   - No silent API shadowing. Only compatibility-preserving overlays are allowed.
   - Contracts and failure behavior must stay explicit.
   - Lifetimes and ownership must remain monotone / non-duplicating.
   - Distinguish settled semantics from implementation convenience.

7. **Prefer clarification in writing over silent assumption**
   - If a task needs a choice between two unresolved options, document both and state the reason you chose a temporary working assumption.

## Strong project-specific constraints

- The system is **specification-first**.
- The current stage is **still architecture and semantics**, not broad implementation.
- Performance-sensitive kernels (for example PrismCascade runtime) must not be casually folded into Mir runtime semantics.
- Dynamic evolution must respect the project's design principle of **safe downstream addition** unless an explicit subsystem spec says otherwise.

## Reporting policy

Every report should contain, in this order:

1. Title and identifier
2. Objective
3. Scope and assumptions
4. Documents consulted
5. Actions taken
6. Evidence / outputs / test results
7. What changed in understanding
8. Open questions
9. Suggested next prompt

## Editing policy

- `specs/` are normative documents. Edit carefully.
- If you change a normative statement, add an explicit note to the report.
- `Documentation.md` should stay concise and current.
- Keep diagrams in Mermaid source (`docs/diagrams/*.mmd`).

## Preferred style

- Use precise language.
- Expand unfamiliar abbreviations on first use.
- Separate **what is decided** from **what is proposed**.
- Avoid metaphor when the technical statement can be written directly.
