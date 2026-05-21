from __future__ import annotations

import sys
import unittest
from pathlib import Path
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

import minimal_alpha1_patterns


COMPUTATIONAL_ACCEPTED = [
    "comp-02-pure-add-one",
    "comp-03-variables-scope-positive",
    "comp-03-arrays-bounds-positive",
    "comp-03-records-vec3-positive",
    "comp-03-control-flow-positive",
    "comp-03-imports-functions-positive",
    "comp-04-host-io-internal-transform-positive",
]
COMPUTATIONAL_RUNTIME_REJECTIONS = [
    "comp-03-variables-scope-negative",
    "comp-03-arrays-bounds-negative",
    "comp-03-records-vec3-negative",
    "comp-03-control-flow-negative",
    "comp-03-imports-functions-negative",
]
COMPUTATIONAL_CHECK_REJECTIONS = [
    "comp-04-host-io-internal-transform-negative-undeclared-effect",
    "comp-04-host-io-internal-transform-negative-undeclared-failure",
    "comp-04-host-io-internal-transform-negative-missing-capability",
]
POSEGRAPH_PLANNED = [
    "pose-01-avatar-head-transform",
    "pose-02-anchored-object",
    "pose-03-sparkle-fallback-anchor",
    "pose-06-save-load-roundtrip",
    "pose-07-stale-anchor-after-membership-advance",
    "pose-08-anchor-switch-frontier-negative",
    "pose-09-stale-anchor-reacquire-required",
]


def fake_computational_check_all() -> dict[str, object]:
    return {
        "command": "check-all",
        "family": "mir-computational-core",
        "sample_root": "samples/product-alpha1/computational",
        "sample_count": 15,
        "planned": [],
        "accepted": list(COMPUTATIONAL_ACCEPTED),
        "expected_runtime_rejections": list(COMPUTATIONAL_RUNTIME_REJECTIONS),
        "expected_check_rejections": list(COMPUTATIONAL_CHECK_REJECTIONS),
        "passed": (
            list(COMPUTATIONAL_ACCEPTED)
            + list(COMPUTATIONAL_RUNTIME_REJECTIONS)
            + list(COMPUTATIONAL_CHECK_REJECTIONS)
        ),
        "failed": [],
        "matrix_status": "mixed",
        "workflow_ready": False,
    }


def fake_posegraph_check_all() -> dict[str, object]:
    return {
        "command": "check-all",
        "family": "transform-posegraph",
        "sample_root": "samples/product-alpha1/posegraph",
        "sample_count": 9,
        "planned": list(POSEGRAPH_PLANNED),
        "accepted": ["pose-04-no-split-frame-positive"],
        "violations": ["pose-05-split-frame-negative"],
        "passed": [
            "pose-04-no-split-frame-positive",
            "pose-05-split-frame-negative",
        ],
        "failed": [],
        "matrix_status": "mixed",
        "workflow_ready": False,
    }


def fake_projection_check_all() -> dict[str, object]:
    return {
        "command": "check-all",
        "family": "projection-boundary",
        "sample_root": "samples/product-alpha1/projection",
        "sample_count": 4,
        "planned": [
            "proj-01-server-client-target-manifest",
            "proj-01-packet-boundary-schema",
            "proj-01-ffi-boundary-schema",
            "proj-01-manifest-provider-compatibility",
        ],
        "passed": [],
        "failed": [],
        "accepted_rows": ["compat-accepted-renderer-view"],
        "rejected_rows": ["compat-rejected-missing-capability"],
        "workflow_ready": False,
    }


def fake_engine_check_all() -> dict[str, object]:
    return {
        "command": "check-all",
        "family": "engine-adapter-boundary",
        "provider_root": "samples/product-alpha1/engine-adapter",
        "provider_count": 8,
        "planned": [
            "renderer",
            "input-device",
            "asset-loader",
            "physics-spatial-query",
            "host-runtime-bridge",
            "wasm-sandbox",
            "native-library-bridge",
            "viewer-diagnostic-exporter",
        ],
        "passed": [],
        "failed": [],
        "world_semantics_owner": "mir_mirrorea",
        "default_native_execution_policy": "Disabled",
        "default_wasm_execution_policy": "InventoryOnly",
        "workflow_ready": False,
    }


class MinimalAlpha1PatternsTests(unittest.TestCase):
    def _patch_strict_helpers(self) -> object:
        return mock.patch.multiple(
            minimal_alpha1_patterns,
            _computational_check_all=fake_computational_check_all,
            _posegraph_check_all=fake_posegraph_check_all,
            _projection_check_all=fake_projection_check_all,
            _engine_check_all=fake_engine_check_all,
        )

    def test_list_patterns_exposes_minimal_practical_patterns(self) -> None:
        patterns = minimal_alpha1_patterns.list_patterns()
        ids = [pattern["pattern_id"] for pattern in patterns]

        self.assertIn("product-alpha1-release-candidate", ids)
        self.assertIn("operational-sugoroku-workflow", ids)
        self.assertIn("mir-compute-host-io-transform", ids)
        self.assertIn("mir-compute-missing-effect-reject", ids)
        self.assertIn("posegraph-split-frame-violation", ids)
        self.assertIn("projection-inventory-boundary", ids)
        self.assertIn("engine-adapter-wasm-inventory", ids)
        self.assertTrue(
            all(pattern["theory_anchor"] for pattern in patterns)
        )

    def test_matrix_reports_strict_and_workflow_anchor_counts(self) -> None:
        matrix = minimal_alpha1_patterns.matrix()

        self.assertEqual(matrix["package_id"], "P-PAT-01")
        self.assertGreaterEqual(matrix["pattern_count"], 9)
        self.assertEqual(matrix["default_strict_family_count"], 4)
        self.assertFalse(matrix["final_public_product_claimed"])
        self.assertIn("no final grammar", matrix["non_claims"][0])

    def test_check_all_accepts_exact_strict_floor(self) -> None:
        with self._patch_strict_helpers():
            result = minimal_alpha1_patterns.check_all()

        self.assertEqual(result["status"], "accepted")
        self.assertEqual(result["failed"], [])
        self.assertEqual(result["strict_family_count"], 4)
        self.assertEqual(
            result["strict_families"]["computational"]["accepted"],
            COMPUTATIONAL_ACCEPTED,
        )
        self.assertEqual(
            result["strict_families"]["computational"]["expected_check_rejections"],
            COMPUTATIONAL_CHECK_REJECTIONS,
        )
        self.assertEqual(
            result["strict_families"]["posegraph"]["violations"],
            ["pose-05-split-frame-negative"],
        )
        self.assertEqual(
            result["strict_families"]["engine_adapter"][
                "default_native_execution_policy"
            ],
            "Disabled",
        )

    def test_check_all_rejects_when_expected_row_drifts(self) -> None:
        drifted = fake_computational_check_all()
        drifted["accepted"] = COMPUTATIONAL_ACCEPTED[:-1]

        with mock.patch.multiple(
            minimal_alpha1_patterns,
            _computational_check_all=lambda: drifted,
            _posegraph_check_all=fake_posegraph_check_all,
            _projection_check_all=fake_projection_check_all,
            _engine_check_all=fake_engine_check_all,
        ):
            result = minimal_alpha1_patterns.check_all()

        self.assertEqual(result["status"], "rejected")
        self.assertIn("computational", result["failed"])
        self.assertIn("accepted rows drifted", result["failures"][0]["detail"])

    def test_run_named_pattern_forwards_to_underlying_helper(self) -> None:
        payload = {
            "command": "run",
            "sample_id": "comp-04-host-io-internal-transform-negative-undeclared-effect",
            "terminal_outcome": "check_rejection",
            "actual_diagnostic_code": "SchemaDecode",
            "outcome_matches_expected": True,
        }
        with mock.patch.object(
            minimal_alpha1_patterns.mir_computational_samples,
            "run_sample",
            return_value=payload,
        ) as run_sample:
            result = minimal_alpha1_patterns.run_pattern(
                "mir-compute-missing-effect-reject"
            )

        run_sample.assert_called_once_with(
            "comp-04-host-io-internal-transform-negative-undeclared-effect"
        )
        self.assertEqual(result["terminal_outcome"], "check_rejection")
        self.assertEqual(result["actual_diagnostic_code"], "SchemaDecode")

    def test_closeout_records_validation_floor_and_non_claims(self) -> None:
        closeout = minimal_alpha1_patterns.closeout()

        self.assertIn(
            "python3 scripts/minimal_alpha1_patterns.py check-all --format json",
            closeout["validation_floor"],
        )
        self.assertIn("no direct LLVM/native backend", closeout["non_claims"])
        self.assertFalse(closeout["final_public_product_claimed"])

    def test_pretty_formats_check_all_summary(self) -> None:
        with self._patch_strict_helpers():
            pretty = minimal_alpha1_patterns.format_pretty(
                minimal_alpha1_patterns.check_all()
            )

        self.assertIn("MINIMAL ALPHA-1 PATTERN CHECK", pretty)
        self.assertIn("status: accepted", pretty)
        self.assertIn("strict families: 4", pretty)

    def test_normalize_argv_promotes_bare_pattern_id_to_run(self) -> None:
        args = minimal_alpha1_patterns.normalize_argv(
            ["mir-compute-host-io-transform", "--format", "json"]
        )

        self.assertEqual(
            args,
            ["--format", "json", "run", "mir-compute-host-io-transform"],
        )


if __name__ == "__main__":
    unittest.main()
