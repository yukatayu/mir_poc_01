import os
import subprocess
import tempfile
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]


class StorageWorkdirGuardTests(unittest.TestCase):
    def storage_env(self, workdir: Path) -> dict[str, str]:
        env = os.environ.copy()
        env.update(
            {
                "MIRROREA_WORKDIR": str(workdir),
                "CARGO_TARGET_DIR": str(workdir / "cargo-target"),
                "MIRROREA_GENERATED_ARTIFACT_DIR": str(
                    workdir / "generated-artifacts"
                ),
                "MIRROREA_CARGO_REGISTRY_CACHE": str(
                    workdir / "cargo-registry-cache"
                ),
                "CARGO_HOME": str(workdir / "cargo-registry-cache"),
                "MIRROREA_LLVM_SRC_DIR": str(workdir / "llvm" / "src"),
                "MIRROREA_LLVM_BUILD_DIR": str(workdir / "llvm" / "build"),
                "MIRROREA_LLVM_INSTALL_DIR": str(workdir / "llvm" / "install"),
                "MIRROREA_LEAN_CACHE_DIR": str(workdir / "lean-cache"),
                "MIRROREA_TEMP_DIR": str(workdir / "temp"),
                "MIRROREA_LOG_DIR": str(workdir / "logs"),
            }
        )
        return env

    def test_env_refuses_ensure_dirs_for_existing_unmounted_workdir(self) -> None:
        with tempfile.TemporaryDirectory(prefix="mirrorea-unmounted-workdir-") as tmp:
            workdir = Path(tmp) / "mirrorea-work"
            workdir.mkdir()

            completed = subprocess.run(
                [
                    "bash",
                    str(REPO_ROOT / "scripts/env/mirrorea_storage_env.sh"),
                    "--ensure-dirs",
                ],
                cwd=REPO_ROOT,
                env=self.storage_env(workdir),
                text=True,
                capture_output=True,
                check=False,
            )

            self.assertEqual(completed.returncode, 2, completed.stdout)
            self.assertIn("unmounted default root", completed.stderr)
            self.assertFalse((workdir / "cargo-target").exists())

    def test_cleanup_confirm_refuses_existing_unmounted_workdir(self) -> None:
        with tempfile.TemporaryDirectory(prefix="mirrorea-unmounted-workdir-") as tmp:
            workdir = Path(tmp) / "mirrorea-work"
            target_dir = workdir / "cargo-target"
            target_dir.mkdir(parents=True)
            marker = target_dir / "keep.txt"
            marker.write_text("do not delete\n")

            completed = subprocess.run(
                [
                    "bash",
                    str(REPO_ROOT / "scripts/storage/cleanup_disposable_artifacts.sh"),
                    "--confirm",
                ],
                cwd=REPO_ROOT,
                env=self.storage_env(workdir),
                text=True,
                capture_output=True,
                check=False,
            )

            self.assertEqual(completed.returncode, 2, completed.stdout)
            self.assertIn("unmounted workdir", completed.stderr)
            self.assertTrue(marker.exists())

    def test_cleanup_allow_unmounted_override_survives_env_source(self) -> None:
        with tempfile.TemporaryDirectory(prefix="mirrorea-unmounted-workdir-") as tmp:
            workdir = Path(tmp) / "mirrorea-work"
            target_dir = workdir / "cargo-target"
            target_dir.mkdir(parents=True)
            marker = target_dir / "delete-me.txt"
            marker.write_text("explicit temp cleanup\n")

            completed = subprocess.run(
                [
                    "bash",
                    str(REPO_ROOT / "scripts/storage/cleanup_disposable_artifacts.sh"),
                    "--confirm",
                    "--allow-unmounted",
                ],
                cwd=REPO_ROOT,
                env=self.storage_env(workdir),
                text=True,
                capture_output=True,
                check=False,
            )

            self.assertEqual(completed.returncode, 0, completed.stderr)
            self.assertIn("[cleanup] mounted: no", completed.stdout)
            self.assertIn("[cleanup] removed:", completed.stdout)
            self.assertFalse(target_dir.exists())


if __name__ == "__main__":
    unittest.main()
