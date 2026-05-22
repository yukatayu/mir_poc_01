# Gradient Write Reject Negative

Negative `P-FSV1-02` row for GradientObservation.

- emits one observer-visible write-reject event
- emits one observer-visible stale-view-drop event
- fails as freshness `contract_require_failed`, not as a runtime-enforced write-authority gate
