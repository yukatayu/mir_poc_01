import sys
from current_l2_family_checker_support import run_family_checker


CAPABILITY_KINDS = {
    "capability_strengthens",
}


def main(argv: list[str] | None = None) -> int:
    return run_family_checker(
        argv=argv,
        cluster_name="capability_strengthening_floor",
        description=(
            "current L2 の third checker spike として、fixture-side "
            "`checked_reason_codes` と static gate detached artifact の "
            "capability-strengthening reason rows だけを narrow に照合する "
            "non-production helper。"
        ),
        kinds=CAPABILITY_KINDS,
        missing_status="fixture_capability_rows_missing",
    )


if __name__ == "__main__":
    sys.exit(main())
