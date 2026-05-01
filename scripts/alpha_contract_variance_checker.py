#!/usr/bin/env python3

import sys

from current_l2_family_checker_support import run_family_checker


CONTRACT_VARIANCE_KINDS = {
    "precondition_strengthening",
    "postcondition_weakening",
    "failure_row_widening",
    "effect_row_widening",
    "cost_degradation",
    "hidden_shadowing",
}


def main(argv: list[str] | None = None) -> int:
    return run_family_checker(
        argv=argv,
        cluster_name="alpha_contract_variance_static_floor",
        description=(
            "Alpha-0 contract/layer compatibility familyの first checker cut として、"
            "sidecar-side `expected_static.checked_reason_codes` と synthetic "
            "artifact の narrow static rows だけを照合する non-production helper。"
        ),
        kinds=CONTRACT_VARIANCE_KINDS,
        missing_status="sample_expected_reason_rows_missing",
    )


if __name__ == "__main__":
    sys.exit(main())
