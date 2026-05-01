#!/usr/bin/env python3

import sys

from current_l2_family_checker_support import run_family_checker


LIFETIME_FALLBACK_KINDS = {
    "missing_lineage_evidence",
    "incompatible_access_target",
    "capability_promotion",
    "write_through_readonly_fallback",
}


def main(argv: list[str] | None = None) -> int:
    return run_family_checker(
        argv=argv,
        cluster_name="alpha_lifetime_fallback_static_floor",
        description=(
            "Alpha-0 lifetime/fallback familyの first checker cut として、"
            "sidecar-side `expected_static.checked_reason_codes` と synthetic "
            "artifact の narrow static rows だけを照合する non-production helper。"
        ),
        kinds=LIFETIME_FALLBACK_KINDS,
        missing_status="sample_expected_reason_rows_missing",
    )


if __name__ == "__main__":
    sys.exit(main())
