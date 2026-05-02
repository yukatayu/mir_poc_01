#!/usr/bin/env python3

import sys

from current_l2_family_snapshot_support import run_family_snapshot_checker


LIFETIME_FALLBACK_SNAPSHOT_KINDS = {
    "snapshot_selected_anchor",
}


def main(argv: list[str] | None = None) -> int:
    return run_family_snapshot_checker(
        argv=argv,
        cluster_name="alpha_lifetime_fallback_snapshot_floor",
        description=(
            "Alpha-0 lifetime/fallback familyの helper-local snapshot-selected floor として、"
            "sidecar-side `expected_snapshot.checked_snapshot_rows` と synthetic "
            "artifact の narrow snapshot rows だけを照合する non-production helper。"
        ),
        kinds=LIFETIME_FALLBACK_SNAPSHOT_KINDS,
        missing_status="sample_expected_snapshot_rows_missing",
        expected_scope="alpha-snapshot-selected-floor",
    )


if __name__ == "__main__":
    sys.exit(main())
