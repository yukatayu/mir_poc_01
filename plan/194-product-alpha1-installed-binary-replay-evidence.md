# Plan 194 - Product Alpha installed-binary replay evidence boundary

## Role and authority

This is LAB operational evidence at source cut `f90c2c29`. `mirrorea_canon/`
remains normative. It records one bounded replay of existing Product Alpha
surfaces; it does not select semantics, a transport contract, an ABI, a grammar,
an authorization model, or a Canon/Gate/OBL/Phase outcome.

## Question

What did the existing installed-binary validation run actually execute, and
which claims remain justified when its outer aggregate helper result is not
available to the caller after console-session detachment?

## Scope and fixed inputs

- Source cut: clean `f90c2c29` before this documentation-only package.
- Existing command: `python3 scripts/product_alpha1_installed_binary_check.py
  --format json check-all --out /tmp/mirrorea-alpha1-installed-binary-check-20260724`.
- Output directory: `/tmp/mirrorea-alpha1-installed-binary-check-20260724`
  (123 MiB when inspected). It is uncommitted, disposable evidence. It was not
  removed because no explicit cleanup confirmation was requested.
- One original command session was allowed to proceed. Two later accidental
  duplicate monitoring launches were stopped and excluded; they made no
  repository-source change. This plan's evidence is only JSON observed in the
  retained directory, not output individually attributed to one launcher. The
  original session's final aggregate stdout and exit status were not recoverable
  through the detached caller console.

## Directly inspected observed evidence

| Artifact | Direct observation | Bounded meaning |
| --- | --- | --- |
| `demo/reports/demo.json` | `status: accepted`, `product_alpha1_ready: true`, `product_alpha1_release_candidate_ready: true`, Docker included/accepted, same-session reopen checked, attach matrix verified | The scripted Product Alpha demo reached its own bounded success condition. |
| `native-bundle/reports/verification-report.json` | `status: accepted`; native package execution and signature-as-safety are both false | The documented native host launch bundle verified; it is not a general native compiler/distribution guarantee. |
| `demo/reports/transport-docker.json` | Docker Compose executed, TCP wire roundtrip executed, participant/world terminal outcomes accepted | The existing two-process Docker Compose TCP alpha path ran. |
| direct `cargo check` / `cargo fmt --check` | both exited successfully | Current Rust compile and formatting floor passed. |

## Explicit non-claims

- The missing outer aggregate JSON and exit status mean this plan does **not**
  assert that the installed-binary helper as a whole reported `accepted`, exited
  zero, or completed every `check-all` stage.
- The output does not establish WAN/federation, distributed durable save/load,
  production orchestration, arbitrary native package execution,
  signature-is-safety, final public CLI/API/ABI, final textual `.mir` grammar,
  final viewer/telemetry service, C-distributed conformance, or public product
  completion.
- The replay makes no statement about Canon semantics, transport authority,
  authentication/authorization, proof obligations, conformance, Gate/Phase, or
  sample workflow readiness beyond the pre-existing bounded Product Alpha line.

## Operational interpretation

This replay strengthens confidence that the existing narrow workflow is
executable on this host. It must not be used as a tie-breaker for a theory
decision or be silently promoted into a release, public interface, or
distributed-system claim. A future release claim requires a run whose aggregate
result is captured, reproducible in its documented environment, and evaluated
against the relevant Canon conformance criteria.

## Follow-up boundary

No new implementation task follows from this replay. Re-run the existing helper
only when a source change affects its command list or a release-specific task
needs a captured aggregate report. Any new transport, persistence, distribution,
ABI, or public-product work needs its own scoped plan and must not inherit this
evidence.
