from __future__ import annotations

import json
import subprocess
import tempfile
import unittest
from pathlib import Path

import sys

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

import surface_mir_release_check as runner  # noqa: E402


REPO_ROOT = Path(__file__).resolve().parents[2]


class SurfaceMirReleaseCheckTests(unittest.TestCase):
    def test_plan_includes_p_surf_99_audit_commands(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            plan = runner.plan_check_all(Path(tmp))

        names = [command.name for command in plan.commands]

        self.assertIn("test:surface-parser", names)
        self.assertIn("test:indexed-state-semantics", names)
        self.assertIn("test:surface-to-core-elaboration", names)
        self.assertIn("test:role-admission-capability-grant", names)
        self.assertIn("test:source-patch-hotplug", names)
        self.assertIn("test:surface-mir-cli", names)
        self.assertIn("test:surface-samples", names)
        self.assertIn("test:surface-release-check", names)
        self.assertIn("helper:surface-samples", names)
        self.assertIn("helper:surface-authoring", names)
        self.assertIn("anchor:product-alpha1-release", names)
        self.assertIn("anchor:operational-product-samples", names)
        self.assertIn("anchor:minimal-alpha1-patterns", names)

    def test_run_check_all_reports_p_surf_99_scope(self) -> None:
        self.assertEqual(
            runner.RELEASE_CHECK_SCOPE,
            "p_surf_99_final_surface_alpha_audit",
        )

    def test_helper_semantic_check_keeps_devtools_floor_non_workflow_ready(self) -> None:
        command = runner.PlannedCommand(
            name="helper:surface-samples",
            argv=["python3", "scripts/surface_mir_samples.py", "check-all", "--format", "json"],
        )
        payload = {
            "sample_count": 46,
            "failed": [],
            "workflow_ready": False,
            "results": [
                {
                    "sample_id": "DEV-01",
                    "accepted": True,
                    "actual": {
                        "accepted": True,
                        "panel_ids": [
                            "surface_source",
                            "generated_core_ir",
                            "indexed_state_map",
                            "generated_communication",
                            "role_admission",
                            "patch_lifecycle",
                            "source_spans",
                        ],
                        "all_required_panels_present": True,
                        "observer_safe": True,
                        "raw_private_payload_exposed": False,
                        "source_authority": ".mir",
                        "final_public_viewer_frozen": False,
                        "indexed_state_semantic_backing": True,
                        "diagnostic_codes": [],
                    },
                    "verification_report": {
                        "redacted": True,
                        "contains_sensitive_devtools_material": False,
                    },
                },
                {
                    "sample_id": "DEV-02",
                    "accepted": True,
                    "actual": {
                        "accepted": False,
                        "panel_ids": [
                            "surface_source",
                            "generated_core_ir",
                            "indexed_state_map",
                            "generated_communication",
                            "role_admission",
                            "patch_lifecycle",
                            "source_spans",
                        ],
                        "all_required_panels_present": True,
                        "observer_safe": True,
                        "raw_private_payload_exposed": False,
                        "source_authority": ".mir",
                        "final_public_viewer_frozen": False,
                        "indexed_state_semantic_backing": True,
                        "diagnostic_codes": ["private_field_auto_publish_rejected"],
                    },
                    "verification_report": {
                        "redacted": True,
                        "contains_sensitive_devtools_material": False,
                    },
                },
            ],
        }

        self.assertEqual(runner.semantic_errors_for_result(command, payload), [])

    def test_helper_semantic_check_rejects_unredacted_devtools_payload(self) -> None:
        command = runner.PlannedCommand(
            name="helper:surface-samples",
            argv=["python3", "scripts/surface_mir_samples.py", "check-all", "--format", "json"],
        )
        payload = {
            "sample_count": 46,
            "failed": [],
            "workflow_ready": False,
            "results": [
                {
                    "sample_id": "DEV-01",
                    "accepted": True,
                    "raw_parse_report": {},
                    "actual": {
                        "accepted": True,
                        "panel_ids": list(runner.REQUIRED_DEVTOOLS_PANELS),
                        "all_required_panels_present": True,
                        "observer_safe": True,
                        "raw_private_payload_exposed": False,
                        "source_authority": ".mir",
                        "final_public_viewer_frozen": False,
                        "indexed_state_semantic_backing": True,
                        "diagnostic_codes": [],
                    },
                    "verification_report": {
                        "redacted": True,
                        "contains_sensitive_devtools_material": False,
                        "capability_refs": ["capability-frontier-0001"],
                    },
                },
                {
                    "sample_id": "DEV-02",
                    "accepted": True,
                    "actual": {
                        "accepted": False,
                        "panel_ids": list(runner.REQUIRED_DEVTOOLS_PANELS),
                        "all_required_panels_present": True,
                        "observer_safe": True,
                        "raw_private_payload_exposed": False,
                        "source_authority": ".mir",
                        "final_public_viewer_frozen": False,
                        "indexed_state_semantic_backing": True,
                        "diagnostic_codes": ["private_field_auto_publish_rejected"],
                    },
                    "verification_report": {
                        "redacted": True,
                        "contains_sensitive_devtools_material": False,
                    },
                },
            ],
        }

        errors = runner.semantic_errors_for_result(command, payload)

        self.assertTrue(any("raw_parse_report" in error for error in errors))
        self.assertTrue(any("sensitive material" in error for error in errors))

    def test_product_alpha_anchor_semantic_check_rejects_failed_anchor(self) -> None:
        command = runner.PlannedCommand(
            name="anchor:product-alpha1-release",
            argv=[],
        )

        errors = runner.semantic_errors_for_result(
            command,
            {
                "failed_commands": [],
                "product_alpha1_release_candidate_ready": False,
                "product_alpha1_ready": True,
            },
        )

        self.assertIn(
            "Product Alpha release anchor is not release-candidate ready",
            errors,
        )

    def test_anchor_payload_summaries_are_redacted(self) -> None:
        result = runner.CommandResult(
            name="anchor:product-alpha1-release",
            argv=[],
            returncode=0,
            stdout="{large-json}",
            stderr="",
            payload={
                "surface_kind": "product_alpha1_release_check_report",
                "status": "accepted",
                "product_alpha1_release_candidate_ready": True,
                "product_alpha1_ready": True,
                "failed_commands": [],
                "command_results": [{}, {}],
                "final_product_claimed": False,
                "final_public_api_frozen": False,
            },
            semantic_errors=[],
        )

        payload = runner.result_payload(result)

        self.assertEqual(
            payload["stdout"],
            "<json stdout summarized; see payload summary>",
        )
        self.assertTrue(payload["payload"]["redacted"])
        self.assertEqual(payload["payload"]["command_result_count"], 2)

    def test_plan_command_accepts_global_format_before_subcommand(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            completed = subprocess.run(
                [
                    "python3",
                    "scripts/surface_mir_release_check.py",
                    "--format",
                    "json",
                    "plan",
                    "--out",
                    tmp,
                ],
                cwd=REPO_ROOT,
                text=True,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                check=False,
            )

        self.assertEqual(completed.returncode, 0, completed.stderr)
        payload = json.loads(completed.stdout)
        self.assertEqual(payload["surface_kind"], "surface_mir_release_check_plan")
        self.assertFalse(payload["final_public_grammar_frozen"])


if __name__ == "__main__":
    unittest.main()
