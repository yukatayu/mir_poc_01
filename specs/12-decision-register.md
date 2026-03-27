# 12 — Decision Register

This document records major architectural decisions and their current strength level.

| ID | Topic | Current decision | Level | Notes |
|---|---|---|---|---|
| D-001 | Overall architecture | Keep Mir, Mirrorea, PrismCascade, and Typed-Effect Wiring Platform separate but interoperable | L1 | Core architectural stance |
| D-002 | Core implementation language | Prefer Rust for core implementations | L2 | Recommendation, not a law |
| D-003 | Graph evolution | Prefer downstream / leaf-style patching as the default evolution rule | L0/L1 | Strong foundational constraint |
| D-004 | API evolution | Forbid silent shadowing; allow compatibility-preserving overlays only | L0/L1 | Central to safe evolution |
| D-005 | Failure handling | Keep failure explicit and structured | L0/L1 | Exact formalization still being refined |
| D-006 | Prism runtime placement | Keep PrismCascade runtime separate from Mir runtime | L1 | Integration should happen at explicit boundaries |
| D-007 | Legacy integration | Permit tunnel/proxy-based wrapping of legacy systems with explicit boundary semantics | L2 | Exact guarantees vary by case |
| D-008 | Coroutine direction | Structured, restricted coroutine model appears plausible; unrestricted coroutine model is not accepted | L2 | Requires further semantics work |
| D-009 | Typed-effects wiring platform | Treat as separate but closely related subsystem | L2 | Exact placement relative to Mirrorea still open |
| D-010 | Tooling workflow | Every substantive Codex task must write a new report | L1 | Process discipline |

## Important note

A decision listed here is not immutable. It means only that the project currently prefers this direction at the stated level.
