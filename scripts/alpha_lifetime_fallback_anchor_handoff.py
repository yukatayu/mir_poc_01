#!/usr/bin/env python3

from current_l2_family_anchor_handoff_support import run_family_anchor_handoff_checker


ALPHA_LIFETIME_FALLBACK_ANCHOR_HANDOFF_KINDS = {
    "bird_sparkle_anchor_chain_inherited_after_object_delete",
}


def main(argv: list[str] | None = None) -> int:
    return run_family_anchor_handoff_checker(
        argv=argv,
        cluster_name="alpha_lifetime_fallback_anchor_handoff_floor",
        description=(
            "Validate helper-local synthetic anchor-handoff rows for the "
            "Alpha lifetime/fallback family."
        ),
        kinds=ALPHA_LIFETIME_FALLBACK_ANCHOR_HANDOFF_KINDS,
        missing_status="sample_expected_anchor_handoff_rows_missing",
        expected_scope="alpha-anchor-handoff-floor",
    )


if __name__ == "__main__":
    raise SystemExit(main())
