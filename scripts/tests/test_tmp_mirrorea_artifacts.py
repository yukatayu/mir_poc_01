import subprocess
import tempfile
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
SCRIPT = REPO_ROOT / "scripts/storage/tmp_mirrorea_artifacts.sh"


class TmpMirroreaArtifactsTests(unittest.TestCase):
    def test_list_reports_mirrorea_dirs_without_deleting(self) -> None:
        with tempfile.TemporaryDirectory(prefix="mirrorea-tmp-artifacts-test-") as tmp:
            tmp_root = Path(tmp)
            candidate = tmp_root / "mirrorea-alpha1-release"
            candidate.mkdir()
            (candidate / "payload.txt").write_text("release artifact\n")
            non_candidate = tmp_root / "not-mirrorea"
            non_candidate.mkdir()
            (tmp_root / "mirrorea-file").write_text("not a directory\n")

            completed = subprocess.run(
                ["bash", str(SCRIPT), "--tmp-root", str(tmp_root), "--list"],
                cwd=REPO_ROOT,
                text=True,
                capture_output=True,
                check=False,
            )

            self.assertEqual(completed.returncode, 0, completed.stderr)
            self.assertIn("candidate_count=1", completed.stdout)
            self.assertIn("total_kib=", completed.stdout)
            self.assertIn(str(candidate), completed.stdout)
            self.assertNotIn(str(non_candidate), completed.stdout)
            self.assertTrue(candidate.exists())
            self.assertTrue(non_candidate.exists())

    def test_cleanup_refuses_without_confirm(self) -> None:
        with tempfile.TemporaryDirectory(prefix="mirrorea-tmp-artifacts-test-") as tmp:
            tmp_root = Path(tmp)
            candidate = tmp_root / "mirrorea-full-v1-release"
            candidate.mkdir()
            marker = candidate / "keep-until-confirmed.txt"
            marker.write_text("safe until explicit confirm\n")

            completed = subprocess.run(
                ["bash", str(SCRIPT), "--tmp-root", str(tmp_root), "--cleanup"],
                cwd=REPO_ROOT,
                text=True,
                capture_output=True,
                check=False,
            )

            self.assertEqual(completed.returncode, 2, completed.stdout)
            self.assertIn("--confirm", completed.stderr)
            self.assertTrue(marker.exists())

    def test_cleanup_with_confirm_deletes_only_mirrorea_dirs(self) -> None:
        with tempfile.TemporaryDirectory(prefix="mirrorea-tmp-artifacts-test-") as tmp:
            tmp_root = Path(tmp)
            candidate = tmp_root / "mirrorea-surface-release"
            candidate.mkdir()
            (candidate / "payload.txt").write_text("surface artifact\n")
            non_candidate = tmp_root / "other-tool-output"
            non_candidate.mkdir()
            (non_candidate / "keep.txt").write_text("unrelated output\n")
            mirrorea_file = tmp_root / "mirrorea-file"
            mirrorea_file.write_text("not a directory\n")

            completed = subprocess.run(
                [
                    "bash",
                    str(SCRIPT),
                    "--tmp-root",
                    str(tmp_root),
                    "--cleanup",
                    "--confirm",
                ],
                cwd=REPO_ROOT,
                text=True,
                capture_output=True,
                check=False,
            )

            self.assertEqual(completed.returncode, 0, completed.stderr)
            self.assertIn("removed=", completed.stdout)
            self.assertFalse(candidate.exists())
            self.assertTrue(non_candidate.exists())
            self.assertTrue(mirrorea_file.exists())


if __name__ == "__main__":
    unittest.main()
