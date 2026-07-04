import subprocess
import sys
import unittest
from pathlib import Path
from unittest import mock


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import practical_alpha1_attach  # noqa: E402
import practical_alpha1_avatar  # noqa: E402
import practical_alpha1_check  # noqa: E402
import practical_alpha1_run_local  # noqa: E402
import practical_alpha1_save_load  # noqa: E402


class PracticalAlphaFailureRedactionTests(unittest.TestCase):
    def assert_non_json_failure_redacts_repo_path(
        self,
        *,
        module,
        build_function_name: str,
        package_dir: str,
    ) -> None:
        package_path = module.REPO_ROOT / package_dir
        leaked_stdout = f"stdout leak {module.REPO_ROOT}/{package_dir}"
        completed = subprocess.CompletedProcess(
            ["cargo"],
            0,
            stdout=leaked_stdout,
            stderr="",
        )
        with mock.patch.object(module.subprocess, "run", return_value=completed):
            with self.assertRaisesRegex(RuntimeError, "did not return JSON") as caught:
                getattr(module, build_function_name)(package_path)
        message = str(caught.exception)
        self.assertIn(package_dir, message)
        self.assertNotIn(str(module.REPO_ROOT), message)

    def test_checker_direct_non_json_failure_redacts_repo_path(self) -> None:
        self.assert_non_json_failure_redacts_repo_path(
            module=practical_alpha1_check,
            build_function_name="_build_check_report",
            package_dir="samples/practical-alpha1/packages/chk-lif-02-fallback-access-valid",
        )

    def test_run_local_direct_non_json_failure_redacts_repo_path(self) -> None:
        self.assert_non_json_failure_redacts_repo_path(
            module=practical_alpha1_run_local,
            build_function_name="_build_runtime_report",
            package_dir="samples/practical-alpha1/packages/run-01-local-sugoroku",
        )

    def test_attach_direct_non_json_failure_redacts_repo_path(self) -> None:
        self.assert_non_json_failure_redacts_repo_path(
            module=practical_alpha1_attach,
            build_function_name="_build_hotplug_report",
            package_dir="samples/practical-alpha1/packages/hp-a1-01-debug-layer-attach",
        )

    def test_avatar_direct_non_json_failure_redacts_repo_path(self) -> None:
        self.assert_non_json_failure_redacts_repo_path(
            module=practical_alpha1_avatar,
            build_function_name="_build_avatar_report",
            package_dir="samples/practical-alpha1/packages/av-a1-01-placeholder-avatar-runtime",
        )

    def test_save_load_direct_non_json_failure_redacts_repo_path(self) -> None:
        self.assert_non_json_failure_redacts_repo_path(
            module=practical_alpha1_save_load,
            build_function_name="_build_runtime_save_load_report",
            package_dir="samples/practical-alpha1/packages/sl-a1-01-local-save-load-resume",
        )

    def test_package_style_imports_for_direct_helpers(self) -> None:
        completed = subprocess.run(
            [
                sys.executable,
                "-c",
                (
                    "from scripts import practical_alpha1_check, "
                    "practical_alpha1_run_local, practical_alpha1_attach, "
                    "practical_alpha1_avatar, practical_alpha1_transport, "
                    "practical_alpha1_save_load, practical_alpha05_session, "
                    "practical_alpha08_session_hotplug, practical_alpha09_devtools, "
                    "practical_alpha1_export_devtools, practical_alpha1_product_preview, "
                    "practical_alpha1_integrated_workflow"
                ),
            ],
            cwd=REPO_ROOT,
            check=True,
            capture_output=True,
            text=True,
        )
        self.assertEqual(completed.stdout, "")


if __name__ == "__main__":
    unittest.main()
