import sys
from current_l2_family_checker_support import run_family_checker


MISSING_OPTION_KINDS = {
    "missing_chain_head_option",
    "missing_predecessor_option",
    "missing_successor_option",
}


def main(argv: list[str] | None = None) -> int:
    return run_family_checker(
        argv=argv,
        cluster_name="missing_option_structure_floor",
        description=(
            "current L2 の second checker spike として、fixture-side "
            "`checked_reason_codes` と static gate detached artifact の "
            "missing-option structure reason rows だけを narrow に照合する "
            "non-production helper。"
        ),
        kinds=MISSING_OPTION_KINDS,
        missing_status="fixture_missing_option_rows_missing",
    )


if __name__ == "__main__":
    sys.exit(main())
