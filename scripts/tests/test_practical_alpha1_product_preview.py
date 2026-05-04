import json
import sys
import unittest
from pathlib import Path
from unittest import mock


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import practical_alpha1_product_preview as runner  # noqa: E402


class PracticalAlpha1ProductPreviewTests(unittest.TestCase):
    def sidecar_path(self, relative: str) -> Path:
        return REPO_ROOT / relative

    def preview_path(self, relative: str) -> Path:
        return REPO_ROOT / relative

    def test_list_samples_matches_preview_rows(self) -> None:
        rows = runner.list_samples()
        self.assertEqual(
            [row["sample_id"] for row in rows],
            [row["sample_id"] for row in runner.IMPLEMENTED_ROWS],
        )
        self.assertTrue(
            all(row["family"] == "practical-alpha1-product-preview" for row in rows)
        )

    def test_run_sample_rejects_bundle_drift(self) -> None:
        row = runner.IMPLEMENTED_ROWS[0]
        expected = json.loads(self.sidecar_path(row["expected_report"]).read_text())
        drifted = dict(expected)
        drifted["bundle_kind"] = "drifted_product_bundle"
        with mock.patch.object(runner, "build_bundle", return_value=drifted):
            with self.assertRaisesRegex(RuntimeError, "expected product-preview bundle drift"):
                runner.run_sample(row["sample_id"])

    def test_check_preview_path_supports_manifest_directory(self) -> None:
        bundle = runner.check_path(
            self.preview_path(
                "samples/practical-alpha1/previews/pe2e-a1-03-hotplug-debug-layer-preview"
            )
        )
        self.assertEqual(bundle["sample_id"], "PE2E-03")
        self.assertEqual(
            [package["package_id"] for package in bundle["source_packages"]],
            ["run-01-local-sugoroku", "hp-a1-01-debug-layer-attach"],
        )
        self.assertIn(
            "same-session runtime attach execution",
            bundle["what_it_does_not_prove"],
        )

    def test_placeholder_object_preview_keeps_avatar_non_claims(self) -> None:
        bundle = runner.run_sample("PE2E-04")
        self.assertEqual(
            bundle["preview_sections"]["object_package"]["selected_representation"],
            "static_capsule_placeholder",
        )
        self.assertIn("custom Mir avatar runtime", bundle["what_it_does_not_prove"])
        self.assertIn("unsupported runtime fallback", bundle["what_it_does_not_prove"])

    def test_custom_avatar_companion_preview_uses_exact_avatar_report(self) -> None:
        bundle = runner.run_sample("PE2E-08")
        self.assertEqual(
            bundle["preview_sections"]["avatar_preview"]["selected_representation"],
            "mir_humanoid_runtime_preview",
        )
        self.assertFalse(
            bundle["preview_sections"]["avatar_preview"]["native_execution_performed"]
        )
        self.assertIn(
            "same-session runtime attach execution",
            bundle["what_it_does_not_prove"],
        )

    def test_unsupported_runtime_fallback_preview_retains_rejected_source(self) -> None:
        bundle = runner.run_sample("PE2E-09")
        self.assertEqual(
            bundle["preview_sections"]["avatar_preview"]["source_hotplug_terminal_outcome"],
            "rejected",
        )
        self.assertTrue(bundle["preview_sections"]["avatar_preview"]["fallback_applied"])
        self.assertIn(
            "successful unsupported-runtime execution",
            bundle["what_it_does_not_prove"],
        )

    def test_invalid_distributed_save_preview_uses_save_load_preflight_reject(self) -> None:
        bundle = runner.run_sample("PE2E-06")
        self.assertEqual(bundle["source_reports"][0]["family"], "practical-alpha1-save-load")
        self.assertEqual(bundle["source_reports"][0]["sample_id"], "SL-A1-03")
        self.assertEqual(
            bundle["preview_sections"]["save_load_preflight_reject"]["terminal_outcome"],
            "rejected_invalid_distributed_cut_preflight",
        )
        self.assertEqual(
            bundle["preview_sections"]["save_load_preflight_reject"]["source_rejected_kind"],
            "orphan_receive",
        )
        self.assertFalse(
            bundle["preview_sections"]["save_load_preflight_reject"][
                "saved_local_frontier_emitted"
            ]
        )

    def test_render_html_uses_run_sample_exact_parity(self) -> None:
        bundle = {
            "sample_id": "PE2E-07",
            "bundle_kind": "devtools_viewer_preview",
            "source_packages": [
                {
                    "package_id": "run-01-local-sugoroku",
                    "role": "world_package",
                    "package_kind": "world",
                    "front_door_entry": "samples/practical-alpha1/packages/run-01-local-sugoroku/package.mir.json",
                }
            ],
            "source_reports": [{"sample_id": "VIS-A1-01"}],
            "preview_sections": {
                "viewer_exports": [
                    {
                        "bundle_ref": "VIS-A1-01",
                        "panel_ids": ["event_dag_graph"],
                    }
                ]
            },
            "what_it_proves": ["viewer preview remains grounded in exact bundles"],
            "what_it_does_not_prove": ["final public viewer API"],
        }
        with mock.patch.object(runner, "run_sample", return_value=bundle):
            with mock.patch.object(
                runner,
                "build_bundle",
                side_effect=AssertionError("build_bundle bypassed exact parity"),
            ):
                rendered = runner.render_html("PE2E-07")
        self.assertEqual(rendered["sample_id"], "PE2E-07")
        self.assertIn("PE2E-07", rendered["html"])
        self.assertIn("VIS-A1-01", rendered["html"])

    def test_closeout_keeps_stage_pa1_8_incomplete(self) -> None:
        with mock.patch.object(
            runner,
            "check_all",
            return_value={
                "sample_count": 9,
                "passed": [
                    "PE2E-01",
                    "PE2E-02",
                    "PE2E-03",
                    "PE2E-04",
                    "PE2E-05",
                    "PE2E-06",
                    "PE2E-07",
                    "PE2E-08",
                    "PE2E-09",
                ],
                "failed": [],
                "product_preview_first_floor_complete": True,
                "stage_pa1_8_complete": False,
                "actualized_rows": [
                    "PE2E-01",
                    "PE2E-02",
                    "PE2E-03",
                    "PE2E-04",
                    "PE2E-05",
                    "PE2E-06",
                    "PE2E-07",
                    "PE2E-08",
                    "PE2E-09",
                ],
                "deferred_avatar_semantics": [],
            },
        ):
            payload = runner.closeout()
        self.assertTrue(payload["product_preview_first_floor_complete"])
        self.assertFalse(payload["stage_pa1_8_complete"])
        self.assertEqual(payload["deferred_avatar_semantics"], [])


if __name__ == "__main__":
    unittest.main()
