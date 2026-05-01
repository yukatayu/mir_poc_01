# alpha sample family — Lifetime / Fallback

- Status: planned skeleton only
- Phase: Phase 1
- Stage: Stage A -> B bridge
- Current runners do not execute this family yet.
- Current package adds a non-public checker floor for selected negative-static rows via sidecar-declared `expected_static.checked_reason_codes`.
- Validation for this package is synthetic-artifact checker tests plus filesystem/docs integrity.

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
| `LIF-11` | `lif-11-bird_sparkle_anchor_inheritance.mir` | positive | bird deletion keeps sparkle on FriendHead |
| `LIF-12` | `lif-12-bird_sparkle_no_inheritance.mir` | negative/contrast | bird deletion => WorldOrigin fallback |
| `LIF-13` | `lif-13-snapshot_selected_anchor.mir` | positive | FriendHead>A, not FriendHead>Shoulder>A |
| `LIF-14` | `lif-14-remote_observed_ref_stale_membership.mir` | negative runtime | Reject(StaleMembership) |
| `LIF-15` | `lif-15-remote_read_only_covariant.mir` | positive | valid |

## Policy

- `.mir` files here are source-ish planned skeletons, not active runnable samples.
- `.expected.json` sidecars record the intended verdict or runtime outcome for future runners/checkers.
- `LIF-05..08` currently carry checker-floor seed rows for the first static diagnostic cut.
- Promotion to active/runnable status requires dedicated validation commands, report evidence, and snapshot updates.

## Validation anchor for this package

```bash
find samples/alpha/lifetime-fallback -maxdepth 1 -type f | sort
python3 -m unittest scripts.tests.test_alpha_lifetime_fallback_checker
```
