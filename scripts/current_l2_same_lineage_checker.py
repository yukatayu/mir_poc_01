import sys
from current_l2_family_checker_support import run_family_checker


SAME_LINEAGE_KINDS = {
    "missing_lineage_assertion",
    "lineage_assertion_edge_mismatch",
    "declared_target_missing",
    "declared_target_mismatch",
}


def main(argv: list[str] | None = None) -> int:
    return run_family_checker(
        argv=argv,
        cluster_name="same_lineage_evidence_floor",
        description=(
            "current L2 の first checker cut spike として、fixture-side "
            "`checked_reason_codes` と static gate detached artifact の same-lineage "
            "reason rows だけを narrow に照合する non-production helper。"
        ),
        kinds=SAME_LINEAGE_KINDS,
        missing_status="fixture_same_lineage_rows_missing",
        expected_scope="stable-clusters-only",
    )


if __name__ == "__main__":
    sys.exit(main())
