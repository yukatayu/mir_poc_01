#!/usr/bin/env python3

import sys

from current_l2_family_runtime_mirror_support import (
    run_family_runtime_mirror_checker,
)


CONTRACT_VARIANCE_RUNTIME_MIRROR_KINDS = {
    "declared_failure_runtime_preview",
    "redacted_debug_layer_runtime_preview",
    "auth_contract_update_runtime_preview",
}


def main(argv: list[str] | None = None) -> int:
    return run_family_runtime_mirror_checker(
        argv=argv,
        cluster_name="alpha_contract_variance_runtime_mirror_floor",
        description=(
            "Alpha-0 contract/layer compatibility familyの runtime-sensitive positive rows "
            "について、sidecar-side `runtime_mirror` と existing layer-insertion "
            "runtime-floor sidecars を照合する non-public mirror helper。"
        ),
        kinds=CONTRACT_VARIANCE_RUNTIME_MIRROR_KINDS,
        missing_status="sample_expected_runtime_mirror_missing",
        expected_scope="alpha-runtime-mirror-floor",
    )


if __name__ == "__main__":
    sys.exit(main())
