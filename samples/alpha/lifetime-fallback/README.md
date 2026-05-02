# alpha sample family — Lifetime / Fallback

- Status: mixed scaffold with synthetic negative checker floor, selected helper-local acceptance floor, selected helper-local snapshot-selected floor, and selected helper-local anchor-handoff floor
- Phase: Phase 1
- Stage: Stage A -> B bridge
- Current runners still do not execute this family as a parser/runtime sample root.
- Current package adds a non-public checker floor for selected negative-static rows via sidecar-declared `expected_static.checked_reason_codes`.
- Current package also adds a helper-local synthetic acceptance floor for `LIF-02/03/04` via sidecar-declared `expected_acceptance.checked_acceptance_rows`.
- `P-A0-20` additionally actualizes `LIF-13` via sidecar-declared `expected_snapshot.checked_snapshot_rows`.
- `P-A0-21` additionally actualizes `LIF-11` via sidecar-declared `expected_anchor_handoff.checked_anchor_handoff_rows`.
- `reason_codes_scope = alpha-static-floor`、`acceptance_scope = alpha-acceptance-floor`、`snapshot_scope = alpha-snapshot-selected-floor`、`anchor_handoff_scope = alpha-anchor-handoff-floor` は distinct carrier boundaries.
- Validation for this package is synthetic helper-local checker / acceptance / snapshot-selected / anchor-handoff tests plus filesystem/docs integrity.

## Rows

| ID | File | Kind | Expected |
|---|---|---|---|
| `LIF-01` | `lif-01-raw_dangling_reference_rejected.mir` | negative static | static error |
| `LIF-02` | `lif-02-fallback_extends_access_path.mir` | positive | valid, canonical chain `c>a` |
| `LIF-03` | `lif-03-nested_inherit_chain_valid.mir` | positive | valid, canonical `e>b>a` |
| `LIF-04` | `lif-04-plain_ref_does_not_inherit.mir` | positive/negative distinction | valid, `d` expiry => `a` |
| `LIF-05` | `lif-05-underdeclared_fallback_static_error.mir` | negative static | static error |
| `LIF-06` | `lif-06-incompatible_access_target_rejected.mir` | negative static | static reject |
| `LIF-07` | `lif-07-capability_promotion_rejected.mir` | negative static | static reject |
| `LIF-08` | `lif-08-write_after_read_only_fallback_rejected.mir` | negative static | static error |
| `LIF-09` | `lif-09-rollback_cannot_resurrect_lease.mir` | negative semantic | lease remains expired |
| `LIF-10` | `lif-10-atomic_cut_no_repromotion.mir` | negative semantic | no earlier option resurrection |
| `LIF-11` | `lif-11-bird_sparkle_anchor_inheritance.mir` | positive | inherited FriendHead>SelfShoulder>WorldOrigin path remains after Bird disappearance |
| `LIF-12` | `lif-12-bird_sparkle_no_inheritance.mir` | negative/contrast | bird deletion => WorldOrigin fallback |
| `LIF-13` | `lif-13-snapshot_selected_anchor.mir` | positive | FriendHead>A, not FriendHead>Shoulder>A |
| `LIF-14` | `lif-14-remote_observed_ref_stale_membership.mir` | negative runtime | Reject(StaleMembership) |
| `LIF-15` | `lif-15-remote_read_only_covariant.mir` | positive | valid |

## Policy

- `.mir` files here are source-ish planned skeletons, not active runnable samples.
- `.expected.json` sidecars record the intended verdict or runtime outcome for future runners/checkers.
- `LIF-01` and `LIF-05..08` currently carry checker-floor seed rows for the first static diagnostic cut.
- `LIF-02/03/04` currently carry helper-local synthetic acceptance rows only.
- `LIF-13` currently carries helper-local synthetic snapshot-selected rows only.
- `LIF-11` currently carries helper-local synthetic anchor-handoff rows only.
- `LIF-15` remains outside the current acceptance / snapshot / anchor-handoff floors because it needs a remote freshness/membership/frontier carrier.
- `P-A0-20` actualizes only `LIF-13` as `snapshot_scope = alpha-snapshot-selected-floor`.
- `P-A0-21` actualizes only `LIF-11` as `anchor_handoff_scope = alpha-anchor-handoff-floor`.
- `snapshot_selected` is not an acceptance row and not a reason-code row. It proves selected-option capture plus exclusion of non-selected options only.
- `anchor_handoff` is not an acceptance row, not a snapshot row, and not a reason-code row. It proves explicit inherited anchor-chain outcome after object deletion only; it does not extend Bird lifetime or prove runtime deletion implementation.
- `P-A0-19` inventory still records only `LIF-15` in this family as a docs-first blocker after `P-A0-20` and `P-A0-21`.
- Promotion to active/runnable status requires dedicated validation commands, report evidence, and snapshot updates.

## Validation anchor for this package

```bash
find samples/alpha/lifetime-fallback -maxdepth 1 -type f | sort
python3 -m unittest \
  scripts.tests.test_alpha_lifetime_fallback_checker \
  scripts.tests.test_alpha_lifetime_fallback_acceptance \
  scripts.tests.test_alpha_lifetime_fallback_snapshot \
  scripts.tests.test_alpha_lifetime_fallback_anchor_handoff
```
