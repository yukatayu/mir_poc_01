import sys
import unittest
from pathlib import Path
from unittest import mock


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import practical_alpha09_devtools as runner  # noqa: E402


class PracticalAlpha09DevtoolsTests(unittest.TestCase):
    def setUp(self) -> None:
        runner.build_session_devtools_payload.cache_clear()

    def tearDown(self) -> None:
        runner.build_session_devtools_payload.cache_clear()

    def test_list_samples_covers_operational_alpha09_matrix(self) -> None:
        rows = runner.list_samples()
        self.assertEqual(
            [row["sample_id"] for row in rows],
            [
                "OA09-01",
                "OA09-02",
                "OA09-03",
                "OA09-04",
                "OA09-05",
                "OA09-06",
                "OA09-07",
                "OA09-08",
                "OA09-09",
            ],
        )

    def test_closeout_claims_session_bound_devtools_operational_readiness(self) -> None:
        payload = runner.closeout()
        self.assertTrue(payload["session_bound_devtools_ready"])
        self.assertTrue(payload["event_dag_live_session_present"])
        self.assertTrue(payload["route_trace_present"])
        self.assertTrue(payload["membership_timeline_present"])
        self.assertTrue(payload["witness_relation_present"])
        self.assertTrue(payload["hotplug_lifecycle_present"])
        self.assertTrue(payload["fallback_degradation_present"])
        self.assertTrue(payload["save_load_timeline_present"])
        self.assertTrue(payload["observer_safe_redacted_view_present"])
        self.assertTrue(payload["retention_on_demand_trace_present"])
        self.assertTrue(payload["operational_alpha09_ready"])

    def test_run_sample_oa09_05_includes_accepted_rejected_and_deferred_hotplug(self) -> None:
        payload = runner.run_sample("OA09-05")
        section = payload["devtools_export"]["export_sections"]["hotplug_lifecycle"]
        outcomes = {
            (entry["sample_id"], entry["terminal_outcome"], entry["session_mutated"])
            for entry in section
        }
        self.assertIn(("HP-A1-01", "accepted", True), outcomes)
        self.assertIn(("HP-A1-02", "rejected", False), outcomes)
        self.assertIn(("HP-A1-07", "deferred_detach_minimal_contract", True), outcomes)

    def test_render_html_uses_same_session_export_payload(self) -> None:
        rendered = runner.render_html()
        self.assertEqual(rendered["sample_id"], "OA09-09")
        self.assertIn("event_dag_live_session", rendered["html"])
        self.assertIn("retention_on_demand_trace", rendered["html"])

    def test_repo_cli_arg_relativizes_repo_owned_package_dir(self) -> None:
        package_dir = REPO_ROOT / runner.BASE_SESSION_PACKAGE
        self.assertEqual(runner.repo_cli_arg(package_dir), runner.BASE_SESSION_PACKAGE)

    def test_repo_cli_arg_keeps_external_path_absolute(self) -> None:
        external = Path("/tmp/mirrorea-external-alpha09-package")
        self.assertEqual(runner.repo_cli_arg(external), str(external))

    def test_build_session_devtools_payload_uses_repo_relative_package_args(self) -> None:
        calls: list[tuple[str, ...]] = []

        def fake_cargo_session(*args: str) -> dict:
            calls.append(args)
            return {"command": args[0]}

        with mock.patch.object(runner, "_cargo_session", side_effect=fake_cargo_session):
            payload = runner.build_session_devtools_payload()

        self.assertEqual(payload["session_report_started"], {"command": "start"})
        self.assertEqual(len(calls), 15)
        self.assertEqual(calls[0][0:2], ("start", runner.BASE_SESSION_PACKAGE))
        self.assertEqual(calls[1][0], "host-io")
        self.assertEqual(calls[1][2], runner.HOST_IO_PACKAGE)

        attach_calls = [call for call in calls if call[0] == "attach"]
        self.assertEqual(len(attach_calls), len(runner.ATTACH_SEQUENCE))
        self.assertEqual(
            [call[2] for call in attach_calls],
            runner.ATTACH_SEQUENCE,
        )

        for call in calls:
            for arg in call:
                self.assertFalse(str(arg).startswith(f"{runner.REPO_ROOT}/"))

        session_args = [
            arg
            for call in calls
            for arg in call
            if str(arg).endswith("session.json")
        ]
        self.assertTrue(session_args)
        self.assertTrue(all(Path(arg).is_absolute() for arg in session_args))


if __name__ == "__main__":
    unittest.main()
