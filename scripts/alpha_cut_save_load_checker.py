#!/usr/bin/env python3

import sys

from current_l2_family_checker_support import run_family_checker


CUT_SAVE_LOAD_KINDS = {
    "orphan_receive",
    "orphan_observe",
    "orphan_witness_use",
    "orphan_hotplug_activation",
    "durable_cut_deferred",
    "orphan_capability_use",
    "orphan_auth_evidence_use",
}


def main(argv: list[str] | None = None) -> int:
    return run_family_checker(
        argv=argv,
        cluster_name="alpha_cut_save_load_static_floor",
        description=(
            "Alpha-0 cut/save/load familyの first checker cut として、"
            "sidecar-side `expected_static.checked_reason_codes` と synthetic "
            "artifact の narrow static/checker rows だけを照合する non-production helper。"
        ),
        kinds=CUT_SAVE_LOAD_KINDS,
        missing_status="sample_expected_reason_rows_missing",
    )


if __name__ == "__main__":
    sys.exit(main())
