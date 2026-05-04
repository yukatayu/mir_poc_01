import json
import sys
import unittest
from pathlib import Path
from unittest import mock


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import practical_alpha1_export_devtools as runner  # noqa: E402


class PracticalAlpha1ExportDevtoolsTests(unittest.TestCase):
    def sidecar_path(self, relative: str) -> Path:
        return REPO_ROOT / relative

    def test_list_samples_matches_first_floor_rows(self) -> None:
        rows = runner.list_samples()
        self.assertEqual(
            [row["sample_id"] for row in rows],
            [row["sample_id"] for row in runner.IMPLEMENTED_ROWS],
        )
        self.assertTrue(
            all(row["family"] == "practical-alpha1-devtools-export" for row in rows)
        )

    def test_run_sample_rejects_bundle_drift(self) -> None:
        row = runner.IMPLEMENTED_ROWS[0]
        expected = json.loads(self.sidecar_path(row["expected_report"]).read_text())
        drifted = dict(expected)
        drifted["bundle_kind"] = "drifted_bundle_kind"
        with mock.patch.object(runner, "build_bundle", return_value=drifted):
            with self.assertRaisesRegex(RuntimeError, "expected devtools bundle drift"):
                runner.run_sample(row["sample_id"])

    def test_render_html_writes_sample_title_and_panel_ids(self) -> None:
        bundle = {
            "sample_id": "VIS-A1-01",
            "bundle_kind": "event_dag_export",
            "panel_ids": ["event_dag_graph", "publication_relation_summary"],
            "telemetry_ids": ["roll_commit#1"],
            "panels": [
                {
                    "panel_id": "event_dag_graph",
                    "panel_kind": "event_dag",
                    "label": "practical:event-dag",
                    "authority": "InspectEventDag(WorldPlace[AlphaRoom#1])",
                    "redaction": "event_structure_only",
                    "retention_scope": "report_local_inventory",
                    "source_report_refs": ["RUN-01"],
                    "focus_refs": ["roll_commit#1"],
                    "notes": [],
                }
            ],
            "telemetry_rows": [],
            "what_it_proves": ["event DAG export remains typed"],
            "what_it_does_not_prove": ["final public viewer API"],
        }
        with mock.patch.object(runner, "run_sample", return_value=bundle):
            with mock.patch.object(
                runner, "build_bundle", side_effect=AssertionError("build_bundle bypassed exact parity")
            ):
                rendered = runner.render_html("VIS-A1-01")
        self.assertEqual(rendered["sample_id"], "VIS-A1-01")
        self.assertIn("VIS-A1-01", rendered["html"])
        self.assertIn("event_dag_graph", rendered["html"])

    def test_hotplug_lifecycle_bundle_uses_exact_hotplug_reports(self) -> None:
        bundle = runner.run_sample("VIS-A1-04")
        self.assertEqual(bundle["sample_id"], "VIS-A1-04")
        self.assertTrue(
            all(
                ref["family"] == "practical-alpha1-hotplug"
                and ref["carrier_scope"] == "practical-alpha1-hotplug-floor"
                and ref["surface_kind"] == "practical_alpha1_nonfinal_hotplug_report"
                for ref in bundle["source_reports"]
            )
        )
        self.assertEqual(
            [ref["sample_id"] for ref in bundle["source_reports"]],
            ["HP-A1-01", "HP-A1-07"],
        )
        self.assertEqual(
            bundle["panel_ids"],
            ["attach_lifecycle", "membership_snapshot", "detach_lifecycle"],
        )
        self.assertEqual(
            bundle["export_sections"]["attach_boundary"]["activation_cut_ref"],
            "activation_cut#debug_trace_layer_attach",
        )
        self.assertEqual(
            bundle["export_sections"]["detach_boundary"]["detach_boundary_ref"],
            "detach_boundary#alpha_local_hotplug_minimal_contract",
        )
        self.assertEqual(
            bundle["export_sections"]["attach_boundary"]["operation_kind"], "attach"
        )
        self.assertEqual(
            bundle["export_sections"]["detach_boundary"]["operation_kind"], "detach"
        )
        self.assertIn(
            "detach runtime lifecycle execution",
            bundle["what_it_does_not_prove"],
        )

    def test_closeout_keeps_remaining_observables_deferred(self) -> None:
        with mock.patch.object(
            runner,
            "check_all",
            return_value={
                "sample_count": 4,
                "passed": ["VIS-A1-01", "VIS-A1-02", "VIS-A1-04", "VIS-A1-06"],
                "failed": [],
                "devtools_export_first_floor_complete": True,
                "actualized_observables": [
                    "VIS-A1-01",
                    "VIS-A1-02",
                    "VIS-A1-04",
                    "VIS-A1-06",
                ],
                "deferred_observables": ["VIS-A1-03", "VIS-A1-05", "VIS-A1-07"],
            },
        ):
            payload = runner.closeout()
        self.assertTrue(payload["devtools_export_first_floor_complete"])
        self.assertEqual(
            payload["deferred_observables"],
            ["VIS-A1-03", "VIS-A1-05", "VIS-A1-07"],
        )


if __name__ == "__main__":
    unittest.main()
