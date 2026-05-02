import unittest

from scripts import alpha_avatar_runtime_samples as runner


class AlphaAvatarRuntimeSamplesTest(unittest.TestCase):
    def test_list_returns_only_implemented_rows(self) -> None:
        rows = runner.list_samples()
        self.assertEqual(
            [row["sample_id"] for row in rows],
            ["AV-01", "AV-02", "AV-06", "AV-08", "AV-09", "HP-11", "HP-12", "HP-15"],
        )

    def test_closeout_keeps_planned_rows_explicit(self) -> None:
        payload = runner.closeout()
        self.assertIn("AV-03", payload["planned_only_rows"])
        self.assertIn("HP-01", payload["planned_only_rows"])
        self.assertIn("HP-10", payload["planned_only_rows"])
        self.assertIn("HP-02", payload["mirrored_elsewhere_rows"])

    def test_validate_expected_fields_accepts_matching_positive_payload(self) -> None:
        row = runner._implemented_row("AV-08")
        report = {
            "sample_id": "AV-08",
            "terminal_outcome": "accepted",
            "reason_family": None,
            "hotplug_skeleton": {"verdict": {"verdict_kind": "rejected"}},
            "representation_state": {
                "fallback_applied": True,
                "native_execution_performed": False,
            },
        }
        runner._validate_expected_fields("AV-08", row, report)

    def test_validate_expected_fields_rejects_native_execution_claim(self) -> None:
        row = runner._implemented_row("HP-11")
        report = {
            "sample_id": "HP-11",
            "terminal_outcome": "rejected",
            "reason_family": "provenance_policy",
            "hotplug_skeleton": {"verdict": {"verdict_kind": "rejected"}},
            "representation_state": {
                "fallback_applied": False,
                "native_execution_performed": True,
            },
        }
        with self.assertRaisesRegex(RuntimeError, "native execution unexpectedly performed"):
            runner._validate_expected_fields("HP-11", row, report)

    def test_validate_expected_fields_rejects_reason_family_drift(self) -> None:
        row = runner._implemented_row("AV-09")
        report = {
            "sample_id": "AV-09",
            "terminal_outcome": "rejected",
            "reason_family": "capability_policy",
            "hotplug_skeleton": {"verdict": {"verdict_kind": "rejected"}},
            "representation_state": {
                "fallback_applied": False,
                "native_execution_performed": False,
            },
        }
        with self.assertRaisesRegex(RuntimeError, "expected reason_family"):
            runner._validate_expected_fields("AV-09", row, report)

    def test_validate_sidecar_parity_rejects_drift(self) -> None:
        report = {"sample_id": "AV-01", "terminal_outcome": "accepted"}
        expected_sidecar = {"sample_id": "AV-01", "terminal_outcome": "rejected"}

        with self.assertRaisesRegex(RuntimeError, "sidecar drift detected"):
            runner._validate_sidecar_parity("AV-01", report, expected_sidecar)


if __name__ == "__main__":
    unittest.main()
