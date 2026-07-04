import sys
import unittest
from pathlib import Path
from unittest import mock


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import practical_alpha08_session_hotplug as runner  # noqa: E402


class PracticalAlpha08SessionHotPlugTests(unittest.TestCase):
    def test_list_samples_covers_operational_alpha08_matrix(self) -> None:
        rows = runner.list_samples()
        self.assertEqual(
            [row["sample_id"] for row in rows],
            [
                "OA08-01",
                "OA08-02",
                "OA08-03",
                "OA08-04",
                "OA08-05",
                "OA08-06",
                "OA08-07",
                "OA08-08",
                "OA08-09",
                "OA08-10",
            ],
        )

    def test_closeout_claims_same_session_hotplug_operational_readiness(self) -> None:
        payload = runner.closeout()
        self.assertTrue(payload["same_session_hotplug_ready"])
        self.assertTrue(payload["accepted_debug_attach_present"])
        self.assertTrue(payload["auth_contract_update_present"])
        self.assertTrue(payload["rate_limit_preview_present"])
        self.assertTrue(payload["rejected_attach_no_mutation_present"])
        self.assertTrue(payload["object_preview_present"])
        self.assertTrue(payload["unsupported_runtime_fallback_present"])
        self.assertTrue(payload["deferred_detach_boundary_present"])
        self.assertTrue(payload["hotplug_lifecycle_export_present"])
        self.assertTrue(payload["same_session_behavior_change_present"])
        self.assertTrue(payload["operational_alpha08_ready"])

    def test_run_sample_oa08_03_records_auth_activation_cut(self) -> None:
        payload = runner.run_sample("OA08-03")
        report = payload["attach_report"]
        observer = payload["observer_safe_export_after_attach"]

        self.assertEqual(report["terminal_outcome"], "accepted_contract_update")
        self.assertEqual(
            report["activation_cut_ref"], "activation_cut#auth_contract_update"
        )
        self.assertIn("auth_gate_layer", report["active_layers_after"])
        self.assertIn("auth_contract_update_active", observer["runtime_behavior_markers"])

    def test_repo_cli_arg_relativizes_repo_owned_package_dir(self) -> None:
        package_dir = REPO_ROOT / runner.BASE_SESSION_PACKAGE
        self.assertEqual(runner.repo_cli_arg(package_dir), runner.BASE_SESSION_PACKAGE)

    def test_repo_cli_arg_keeps_external_path_absolute(self) -> None:
        external = Path("/tmp/mirrorea-external-alpha08-package")
        self.assertEqual(runner.repo_cli_arg(external), str(external))

    def test_run_session_start_uses_repo_relative_package_arg(self) -> None:
        session_path = Path("/tmp/mirrorea-alpha08-test-session/session.json")
        package_dir = REPO_ROOT / runner.BASE_SESSION_PACKAGE
        with mock.patch.object(
            runner,
            "_cargo_session",
            return_value={"command": "start"},
        ) as mocked_session:
            payload = runner._run_session_start(package_dir, session_path)

        self.assertEqual(payload, {"command": "start"})
        self.assertEqual(
            mocked_session.call_args.args,
            ("start", runner.BASE_SESSION_PACKAGE, str(session_path)),
        )
        self.assertTrue(session_path.is_absolute())

    def test_run_session_attach_uses_repo_relative_package_arg(self) -> None:
        session_path = Path("/tmp/mirrorea-alpha08-test-session/session.json")
        package = "samples/practical-alpha1/packages/hp-a1-01-debug-layer-attach"
        package_dir = REPO_ROOT / package
        with mock.patch.object(
            runner,
            "_cargo_session",
            return_value={"command": "attach"},
        ) as mocked_session:
            payload = runner._run_session_attach(session_path, package_dir)

        self.assertEqual(payload, {"command": "attach"})
        self.assertEqual(
            mocked_session.call_args.args,
            ("attach", str(session_path), package, str(session_path)),
        )
        self.assertFalse(
            any(
                str(arg).startswith(f"{runner.REPO_ROOT}/")
                for arg in mocked_session.call_args.args
            )
        )


if __name__ == "__main__":
    unittest.main()
