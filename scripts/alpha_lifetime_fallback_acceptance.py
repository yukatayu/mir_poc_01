#!/usr/bin/env python3

import sys

from current_l2_family_acceptance_support import run_family_acceptance_checker


LIFETIME_FALLBACK_ACCEPTANCE_KINDS = {
    "fallback_chain_canonicalized",
    "inherited_chain_spliced_with_lineage",
    "plain_ref_boundary_preserved",
}


def main(argv: list[str] | None = None) -> int:
    return run_family_acceptance_checker(
        argv=argv,
        cluster_name="alpha_lifetime_fallback_acceptance_floor",
        description=(
            "Alpha-0 lifetime/fallback familyの helper-local acceptance floor として、"
            "sidecar-side `expected_acceptance.checked_acceptance_rows` と synthetic "
            "artifact の narrow positive rows だけを照合する non-production helper。"
        ),
        kinds=LIFETIME_FALLBACK_ACCEPTANCE_KINDS,
        missing_status="sample_expected_acceptance_rows_missing",
        expected_scope="alpha-acceptance-floor",
    )


if __name__ == "__main__":
    sys.exit(main())
