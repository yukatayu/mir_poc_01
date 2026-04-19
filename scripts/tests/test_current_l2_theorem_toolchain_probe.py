import sys
import tempfile
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import current_l2_theorem_toolchain_probe as probe  # noqa: E402


def write_fake_tool(tool_dir: Path, name: str, version_line: str) -> None:
    tool_path = tool_dir / name
    tool_path.write_text(
        "#!/usr/bin/env bash\n"
        f"echo '{version_line}'\n",
        encoding="utf-8",
    )
    tool_path.chmod(0o755)


class TheoremToolchainProbeTests(unittest.TestCase):
    def test_probe_reports_unavailable_when_required_tools_are_missing(self) -> None:
        result = probe.probe_theorem_toolchain(search_path="")

        self.assertEqual(result.status, "unavailable")
        self.assertFalse(result.reopen_condition_met)
        self.assertEqual(result.missing_tools, ("lean", "lake", "elan"))
        self.assertEqual(result.available_tools, ())

    def test_probe_reports_ready_when_required_tools_are_present(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            tool_dir = Path(temp_dir)
            write_fake_tool(tool_dir, "lean", "Lean (version 4.12.0)")
            write_fake_tool(tool_dir, "lake", "Lake version 5.0.0")
            write_fake_tool(tool_dir, "elan", "elan 4.0.0")

            result = probe.probe_theorem_toolchain(search_path=str(tool_dir))

        self.assertEqual(result.status, "ready")
        self.assertTrue(result.reopen_condition_met)
        self.assertEqual(result.missing_tools, ())
        self.assertEqual(result.available_tools, ("elan", "lake", "lean"))
        self.assertEqual(result.tool_versions["lean"], "Lean (version 4.12.0)")
        self.assertEqual(result.tool_versions["lake"], "Lake version 5.0.0")
        self.assertEqual(result.tool_versions["elan"], "elan 4.0.0")

    def test_reopen_manifest_carries_pipeline_plan_for_sample(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            tool_dir = Path(temp_dir) / "tools"
            tool_dir.mkdir()
            write_fake_tool(tool_dir, "lean", "Lean (version 4.12.0)")
            write_fake_tool(tool_dir, "lake", "Lake version 5.0.0")
            write_fake_tool(tool_dir, "elan", "elan 4.0.0")

            manifest = probe.build_theorem_actual_lean_reopen_manifest(
                sample_stem="e5-underdeclared-lineage",
                artifact_root=Path(temp_dir) / "artifacts",
                run_label="toolchain-probe",
                python_executable="/usr/bin/python3",
                search_path=str(tool_dir),
            )

        self.assertTrue(manifest.reopen_condition_met)
        self.assertEqual(manifest.sample_stem, "e5-underdeclared-lineage")
        self.assertEqual(manifest.smoke_mode, "static")
        self.assertEqual(
            manifest.pipeline_plan["formal_hook_command"][2:4],
            ["smoke-formal-hook-static", "e5-underdeclared-lineage"],
        )
        self.assertIn(
            "environment_probe:current_l2.theorem_actual_lean_execution_toolchain_probe",
            manifest.reference_refs,
        )
        self.assertIn(
            "next_step:actual_lean_execution_narrow_probe_reopen",
            manifest.next_step_refs,
        )


if __name__ == "__main__":
    unittest.main()
