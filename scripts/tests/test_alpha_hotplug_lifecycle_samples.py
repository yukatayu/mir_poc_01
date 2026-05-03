import sys
import unittest
from pathlib import Path
from unittest import mock


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import alpha_hotplug_lifecycle_samples as runner  # noqa: E402


class AlphaHotplugLifecycleSamplesTests(unittest.TestCase):
    def valid_layer_reports(self) -> list[dict]:
        reports: list[dict] = []
        for row in runner.LAYER_IMPLEMENTED_ROWS:
            sample_id = row["sample_id"]
            expected_runtime = runner._load_layer_expected_sidecar(row)["expected_runtime"]
            report = {
                "sample_id": sample_id,
                "terminal_outcome": expected_runtime["terminal_outcome"],
                "retained_later_refs": [
                    "completed_hotplug_lifecycle",
                    "detach_runtime",
                    "durable_migration",
                    "distributed_activation_ordering",
                    "runtime_package_avatar_admission",
                    "final_public_layer_attachment_abi",
                ],
            }
            if sample_id == "LI-01":
                report.update(
                    {
                        "active_layers_after": [expected_runtime["activated_layer"]],
                        "pre_attach_trace_rows": [],
                        "post_attach_trace_rows": [
                            {"redaction_level": expected_runtime["redaction_level"]},
                            {"redaction_level": expected_runtime["redaction_level"]},
                        ],
                        "source_runtime_sample_ref": expected_runtime["source_runtime_sample_ref"],
                    }
                )
            elif sample_id == "LI-02":
                report.update(
                    {
                        "hotplug_runtime_report": {
                            "verdict": {
                                "authorization_reason_refs": [
                                    expected_runtime["required_authorization_reason_ref"]
                                ]
                            }
                        },
                        "active_layers_after": [],
                        "post_attach_trace_rows": [],
                    }
                )
            elif sample_id == "LI-03":
                report.update(
                    {
                        "compatibility": {"accepted_path": expected_runtime["accepted_path"]},
                        "attach_request": {
                            "contract_update_ref": expected_runtime["required_contract_update_ref"]
                        },
                    }
                )
            elif sample_id == "LI-04":
                report.update(
                    {
                        "runtime_preview": {
                            "terminal_outcome": expected_runtime["preview_terminal_outcome"],
                            "reason_refs": list(
                                expected_runtime["required_preview_reason_refs"]
                            ),
                        }
                    }
                )
            elif sample_id == "LI-05":
                report.update(
                    {
                        "compatibility": {
                            "failed_reason_refs": list(
                                expected_runtime["required_failure_refs"]
                            )
                        },
                        "active_layers_after": [],
                    }
                )
            reports.append(report)
        return reports

    def test_list_samples_exposes_stage_d_rows(self) -> None:
        rows = runner.list_samples()
        self.assertEqual(
            [row["sample_id"] for row in rows],
            runner.LAYER_REQUIRED_ROWS + runner.AVATAR_PACKAGE_REQUIRED_ROWS,
        )

    def test_closeout_reports_stage_d_required_rows(self) -> None:
        payload = runner.closeout()
        self.assertEqual(payload["layer_rows"], runner.LAYER_REQUIRED_ROWS)
        self.assertEqual(
            payload["runtime_package_avatar_rows"],
            runner.AVATAR_PACKAGE_REQUIRED_ROWS,
        )
        self.assertIn(
            "python3 scripts/alpha_hotplug_lifecycle_samples.py stage-d-closeout --format json",
            payload["validation_floor"],
        )
        self.assertFalse(payload["active_root_promoted"])

    def test_validate_layer_closeout_reports_accepts_valid_subset(self) -> None:
        runner._validate_layer_closeout_reports(self.valid_layer_reports())

    def test_validate_layer_closeout_reports_rejects_missing_required_failure_ref(self) -> None:
        reports = self.valid_layer_reports()
        for report in reports:
            if report["sample_id"] == "LI-05":
                report["compatibility"]["failed_reason_refs"] = ["provided_interface_shrunk"]
                break
        with self.assertRaisesRegex(RuntimeError, "missing compatibility failure ref"):
            runner._validate_layer_closeout_reports(reports)

    def test_stage_d_closeout_requires_layer_and_avatar_subset(self) -> None:
        with (
            mock.patch.object(
                runner,
                "check_layer_closeout",
                return_value={
                    "sample_count": 5,
                    "passed": runner.LAYER_REQUIRED_ROWS,
                    "failed": [],
                },
            ),
            mock.patch.object(
                runner,
                "check_avatar_package_floor",
                return_value={
                    "sample_count": 8,
                    "passed": runner.AVATAR_PACKAGE_REQUIRED_ROWS,
                    "failed": [],
                    "inventory": runner.avatar_runtime_samples.closeout(),
                },
            ),
        ):
            payload = runner.stage_d_closeout()
        self.assertTrue(payload["stage_d_complete"])
        self.assertFalse(payload["detach_runtime_complete"])
        self.assertFalse(payload["final_public_runtime_package_abi_claimed"])

    def test_stage_d_closeout_surfaces_failures(self) -> None:
        with (
            mock.patch.object(
                runner,
                "check_layer_closeout",
                return_value={
                    "sample_count": 5,
                    "passed": runner.LAYER_REQUIRED_ROWS,
                    "failed": [],
                },
            ),
            mock.patch.object(
                runner,
                "check_avatar_package_floor",
                return_value={
                    "sample_count": 8,
                    "passed": [],
                    "failed": [{"floor": "avatar-package", "error": "boom"}],
                    "inventory": runner.avatar_runtime_samples.closeout(),
                },
            ),
        ):
            payload = runner.stage_d_closeout()
        self.assertFalse(payload["stage_d_complete"])
        self.assertEqual(
            payload["runtime_package_avatar_floor"]["failed"][0]["floor"],
            "avatar-package",
        )


if __name__ == "__main__":
    unittest.main()
