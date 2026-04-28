# Report 0957 — P10 mirrorea-core first-tranche extraction candidates

- Date: 2026-04-28 13:47:02 JST
- Author / agent: Codex
- Scope: local repository state only; read-only analysis for `P10` `mirrorea-core` first real implementation tranche
- Decision levels touched: none changed; analysis only across existing L1/L2 boundaries

## 1. Objective

Identify the smallest concrete extraction candidates that can move from the current helper/runtime layer into `crates/mirrorea-core` without collapsing Mir / Mirrorea / adapter / visualization boundaries.

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/04-mir-core.md`
- `specs/05-mirrorea-fabric.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/19-repository-map-and-taxonomy.md`
- `crates/mirrorea-core/Cargo.toml`
- `crates/mirrorea-core/src/lib.rs`
- `crates/mir-runtime/src/lib.rs`
- `crates/mir-runtime/src/clean_near_end.rs`
- `scripts/sugoroku_world_samples.py`
- `scripts/avatar_follow_samples.py`
- `scripts/tests/test_sugoroku_world_samples.py`
- `scripts/tests/test_avatar_follow_samples.py`

## 3. Actions taken

1. Read the required front-door docs, specs, and plan files in repository order.
2. Confirmed the current `P10` / `P11` queue reading and current stop lines.
3. Inspected the `mirrorea-core` placeholder crate state.
4. Compared duplicated carriers and helper-local pure logic across:
   - `crates/mir-runtime/src/clean_near_end.rs`
   - `scripts/sugoroku_world_samples.py`
   - `scripts/avatar_follow_samples.py`
5. Identified extraction candidates that are:
   - pure data carriers or pure helper logic,
   - already mirrored across Rust/Python or clearly Mirrorea-scoped,
   - not transport/auth/viewer/public-API freeze by stealth.

## 4. Files changed

- Added `docs/reports/0957-p10-mirrorea-core-first-tranche-extraction-candidates.md`
- `plan/` 更新不要
- `progress.md` 更新不要
- `tasks.md` 更新不要
- `samples_progress.md` 更新不要

## 5. Commands run and exact outputs

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
  - Output: `Task baseline recorded.`
- `date '+%Y-%m-%d %H:%M:%S %Z'`
  - Output: `2026-04-28 13:47:02 JST`
- `find crates/mirrorea-core -maxdepth 3 -type f | sort`
  - Output:
    - `crates/mirrorea-core/Cargo.toml`
    - `crates/mirrorea-core/src/lib.rs`
- `find crates/mir-runtime/src -maxdepth 2 -type f | sort`
  - Output:
    - `crates/mir-runtime/src/bin/mir-clean-near-end.rs`
    - `crates/mir-runtime/src/bin/mir-current-l2.rs`
    - `crates/mir-runtime/src/clean_near_end.rs`
    - `crates/mir-runtime/src/current_l2.rs`
    - `crates/mir-runtime/src/current_l2_cli.rs`
    - `crates/mir-runtime/src/lib.rs`
- `rg -n 'LayerSignature|MessageEnvelope|AuthEvidence|MembershipRegistry|hotplug|visualization|telemetry|witness' crates/mir-runtime/src/clean_near_end.rs scripts/sugoroku_world_samples.py scripts/avatar_follow_samples.py`
  - Output: matched duplicate carrier definitions and helper/runtime ownership sites used in this report.
- Multiple `nl -ba ... | sed -n ...` reads on the inputs listed in section 2.
  - Output: line-numbered excerpts used as evidence anchors in section 6.

## 6. Evidence / findings

### 6.1 Current `mirrorea-core` state

- `crates/mirrorea-core` is still a placeholder with no dependencies and no real exported logic.
- `Cargo.toml` only declares the package and lib path.
- `src/lib.rs` documents intended responsibility as logical naming, route structure, overlay model, patch abstractions, and audit-facing types, but explicitly says it contains no production logic yet.

### 6.2 Minimal realistic extraction candidates

1. Shared fabric evidence carriers:
   - `LayerSignature`
   - `PrincipalClaim`
   - `AuthEvidence`
   - `MessageEnvelope`
   Current evidence:
   - duplicated in Rust runtime at `crates/mir-runtime/src/clean_near_end.rs`
   - duplicated in Sugoroku helper at `scripts/sugoroku_world_samples.py`
   Why acceptable for `P10`:
   - pure carriers,
   - already aligned by `P5` / `P6`,
   - Mirrorea-scoped enough to leave Mir core alone,
   - can move without forcing real network, public transport ABI, or viewer API.

2. Shared lane/schema constants plus duplicate-name merge rule:
   - `layer_signature_lanes`
   - `message_envelope_lanes`
   - `auth_evidence_lanes`
   - closeout duplicate-name rejection for `LayerSignature`
   Current evidence:
   - Rust lanes and merge rule in `crates/mir-runtime/src/clean_near_end.rs`
   - Python closeout merge rule in `scripts/sugoroku_world_samples.py`
   Why acceptable for `P10`:
   - pure logic,
   - reinforces inventory shape without freezing public protocol,
   - directly supports the `report-local invariants` deliverable named for `P10`.

3. Membership epoch/incarnation kernel:
   - `MemberRecord`
   - `MembershipRegistry`
   - possibly the minimal `active` / `add_member` / `mark_inactive` / snapshot logic
   Current evidence:
   - only in `scripts/sugoroku_world_samples.py`
   - already called out in docs as `membership_model.source_of_truth = MembershipRegistry`
   Why acceptable for `P10`:
   - pure state machine with monotone epoch/incarnation behavior,
   - aligned with `P8`,
   - small enough to become a core carrier before `P11` runtime widening.
   Constraint:
   - keep it as fabric-local membership state, not session/auth transport logic.

4. Helper-pure envelope finalization helpers:
   - transport seam mapping by envelope kind
   - emitter principal derivation
   - freshness-check derivation
   Current evidence:
   - `_helper_transport_seam`
   - `_helper_emitter_principal`
   - `_helper_freshness_checks`
   - `_finalize_message_envelope`
   all in `scripts/sugoroku_world_samples.py`
   Why acceptable for `P10`:
   - pure logic over already-stabilized envelope fields,
   - can move as non-network Mirrorea route/audit normalization helpers.
   Constraint:
   - keep sample-specific envelope IDs and game-specific capability strings out of the core API; only the normalization shape should move.

## 7. Changes in understanding

- The strongest `P10` path is not “build runtime first”; it is “move already-stabilized shared carriers and pure normalization rules out of helper-local duplication first.”
- `LayerSignature` and `MessageEnvelope` are more mature than attach/detach runtime behavior itself: the row schema and seam lanes are already mirrored across helper/runtime, while actual multi-place runtime behavior is explicitly deferred to `P11`.
- `avatar_follow_samples.py` is useful mainly as a negative boundary check: it shows representative application-slice state and reopen gates that should not be mistaken for first `mirrorea-core` substrate.

## 8. Open questions

- Whether `TermSignature` belongs in `mirrorea-core` or should stay runtime/helper-local until a stronger Mirrorea-facing use emerges.
- Whether `VisualizationView` / `TelemetryRow` should wait until after `P10`, since docs keep viewer/public telemetry policy explicitly unresolved.
- Whether `MembershipRegistry` should enter `mirrorea-core` in the first cut, or only a smaller membership freshness/epoch carrier should move first.
- Whether the first `mirrorea-core` cut should expose JSON/serde-facing types immediately, or start internal-only and let helper/runtime adapters serialize them separately.

## 9. Suggested next prompt

Implement `P10` as a narrow code+docs cut: create first real `mirrorea-core` types for `LayerSignature`, `PrincipalClaim`, `AuthEvidence`, and `MessageEnvelope`, move duplicate lane constants and duplicate-name merge logic into the crate, wire `mir-runtime` to use them, leave Sugoroku/avatar Python helpers unchanged except for doc references, and validate with `cargo test -p mirrorea-core`, `cargo test -p mir-runtime`, `python3 -m unittest scripts.tests.test_sugoroku_world_samples scripts.tests.test_avatar_follow_samples`, `python3 scripts/validate_docs.py`, and `git diff --check`.
