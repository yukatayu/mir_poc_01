# Mir / Mirrorea / PrismCascade / Typed-Effect Wiring Platform — Starter Workspace

This repository is a **specification-first starter workspace** for a large research-and-engineering program.
It is intentionally organized so that an agent with **no retained context** can recover the project's structure by reading a small, ordered set of documents.

## What this workspace is for

This repository is the starting point for work on four tightly related but intentionally separable systems:

1. **Mir** — a language and semantic core for causality, contracts, effects, ownership, lifetimes, and safe evolution.
2. **Mirrorea** — a fabric/runtime layer for logical naming, safe overlay insertion, patching, routing, audit, and dynamic reconfiguration.
3. **PrismCascade** — an independent media-processing kernel for video/audio editing and live pipelines.
4. **Typed-Effect Wiring Platform** — a routable, inspectable effect layer for containers/services and legacy system integration.

## Current stage

This repository is **not** implementation-complete and is **not** an MVP codebase.
It is a structured starting point for:

- refining specifications,
- proving or falsifying invariants,
- building small proofs of concept,
- writing detailed reports after each task.

## Required reading order for humans and agents

1. `AGENTS.md`
2. `Documentation.md`
3. `specs/00-document-map.md`
4. `specs/01-charter-and-decision-levels.md`
5. `specs/02-system-overview.md`
6. `specs/03-layer-model.md`
7. `specs/09-invariants-and-constraints.md`
8. Then enter the relevant subsystem spec:
   - `specs/04-mir-core.md`
   - `specs/05-mirrorea-fabric.md`
   - `specs/06-prismcascade-positioning.md`
   - `specs/07-typed-effects-wiring-platform.md`
   - `specs/08-cross-system-relations.md`
   - `specs/10-open-questions.md`
   - `specs/11-roadmap-and-workstreams.md`

## Working style

Every non-trivial task should produce a **new markdown report** under `docs/reports/`.
Use `python scripts/new_report.py --slug <short-name>` to create a report from the template.

## Repository layout

- `specs/` — authoritative specifications and design documents.
- `docs/reports/` — chronological work reports.
- `docs/diagrams/` — Mermaid source diagrams.
- `agents/` — project-local agent guidance/configuration files.
- `scripts/` — helper scripts for reporting and validation.
- `crates/` — intentionally minimal Rust workspace skeletons.

## Status of implementation language choice

The current recommendation is:

- **Rust** for core implementations and runtime kernels.
- **Web-based visualization** (for example TypeScript/HTML/SVG/WebGL) later, if and when needed.
- **C ABI / engine adapters** for future integration with game engines.

This is a **recommended implementation direction**, not an architectural law.
See `specs/11-roadmap-and-workstreams.md` for implementation guidance.


## Current environment note

This scaffold was created in an environment where Python was available but `cargo` was not.
The Rust workspace skeleton is present, but compilation still needs to be validated on a Rust-enabled machine.
