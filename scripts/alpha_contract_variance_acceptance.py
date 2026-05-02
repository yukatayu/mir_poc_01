#!/usr/bin/env python3

import sys

from current_l2_family_acceptance_support import run_family_acceptance_checker


CONTRACT_VARIANCE_ACCEPTANCE_KINDS = {
    "transparent_observe_only_layer",
    "output_covariance_checked",
    "readonly_covariance_checked",
}


def main(argv: list[str] | None = None) -> int:
    return run_family_acceptance_checker(
        argv=argv,
        cluster_name="alpha_contract_variance_acceptance_floor",
        description=(
            "Alpha-0 contract/layer compatibility familyの helper-local acceptance floor "
            "として、sidecar-side `expected_acceptance.checked_acceptance_rows` と "
            "synthetic artifact の narrow positive rows だけを照合する non-production helper。"
        ),
        kinds=CONTRACT_VARIANCE_ACCEPTANCE_KINDS,
        missing_status="sample_expected_acceptance_rows_missing",
        expected_scope="alpha-acceptance-floor",
    )


if __name__ == "__main__":
    sys.exit(main())
