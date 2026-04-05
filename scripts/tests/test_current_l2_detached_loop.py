import argparse
import tempfile
import sys
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import current_l2_detached_loop as loop  # noqa: E402


class DetachedLoopPathTests(unittest.TestCase):
    def test_aggregate_artifact_path_uses_run_label_and_batch_summary_name(self) -> None:
        path = loop.aggregate_artifact_path(
            REPO_ROOT / "target" / "current-l2-detached",
            "smoke-run",
        )
        self.assertEqual(
            path,
            REPO_ROOT
            / "target"
            / "current-l2-detached"
            / "aggregates"
            / "smoke-run"
            / "batch-summary.detached.json",
        )

    def test_aggregate_artifact_path_reuses_run_label_validation(self) -> None:
        with self.assertRaises(ValueError):
            loop.aggregate_artifact_path(
                REPO_ROOT / "target" / "current-l2-detached",
                "bad/label",
            )

    def test_compare_aggregates_uses_run_label_paths(self) -> None:
        captured: list[tuple[Path, Path]] = []
        original_compare = loop.compare_aggregates

        def fake_compare(left: Path, right: Path) -> int:
            captured.append((left, right))
            return 0

        loop.compare_aggregates = fake_compare
        try:
            exit_code = loop.command_compare_aggregates(
                argparse.Namespace(
                    artifact_root=str(REPO_ROOT / "target" / "current-l2-detached"),
                    left_run_label="run-left",
                    right_run_label="run-right",
                )
            )
        finally:
            loop.compare_aggregates = original_compare

        self.assertEqual(exit_code, 0)
        self.assertEqual(
            captured,
            [
                (
                    REPO_ROOT
                    / "target"
                    / "current-l2-detached"
                    / "aggregates"
                    / "run-left"
                    / "batch-summary.detached.json",
                    REPO_ROOT
                    / "target"
                    / "current-l2-detached"
                    / "aggregates"
                    / "run-right"
                    / "batch-summary.detached.json",
                )
            ],
        )

    def test_smoke_fixture_emits_bundle_and_aggregate_and_tolerates_diff_status_one(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_dir = temp_root / "fixtures"
            fixture_dir.mkdir()
            fixture_path = fixture_dir / "sample.json"
            fixture_path.write_text("{}", encoding="utf-8")
            sidecar_path = fixture_dir / "sample.host-plan.json"
            sidecar_path.write_text("{}", encoding="utf-8")

            emitted_fixtures: list[tuple[Path, Path, bool]] = []
            emitted_aggregates: list[tuple[Path, Path, bool]] = []
            compared_artifacts: list[tuple[Path, Path]] = []
            compared_aggregates: list[tuple[Path, Path]] = []

            original_emit_fixture = loop.emit_fixture
            original_emit_aggregate = loop.emit_aggregate
            original_compare_artifacts = loop.compare_artifacts
            original_compare_aggregates = loop.compare_aggregates

            def fake_emit_fixture(
                fixture: Path,
                output: Path,
                overwrite: bool,
            ) -> int:
                emitted_fixtures.append((fixture, output, overwrite))
                return 0

            def fake_emit_aggregate(
                fixture_directory: Path,
                output: Path,
                overwrite: bool,
            ) -> int:
                emitted_aggregates.append((fixture_directory, output, overwrite))
                return 0

            def fake_compare_artifacts(left: Path, right: Path) -> int:
                compared_artifacts.append((left, right))
                return 1

            def fake_compare_aggregates(left: Path, right: Path) -> int:
                compared_aggregates.append((left, right))
                return 1

            loop.emit_fixture = fake_emit_fixture
            loop.emit_aggregate = fake_emit_aggregate
            loop.compare_artifacts = fake_compare_artifacts
            loop.compare_aggregates = fake_compare_aggregates
            try:
                exit_code = loop.command_smoke_fixture(
                    argparse.Namespace(
                        fixture_path=str(fixture_path),
                        reference_fixture=str(fixture_path),
                        artifact_root=str(temp_root / "artifacts"),
                        run_label="sample-smoke",
                        reference_label="sample-reference",
                        overwrite=True,
                    )
                )
            finally:
                loop.emit_fixture = original_emit_fixture
                loop.emit_aggregate = original_emit_aggregate
                loop.compare_artifacts = original_compare_artifacts
                loop.compare_aggregates = original_compare_aggregates

        self.assertEqual(exit_code, 0)
        self.assertEqual(len(emitted_fixtures), 2)
        self.assertEqual(
            emitted_fixtures[0][1],
            temp_root
            / "artifacts"
            / "bundles"
            / "sample-smoke"
            / "sample.detached.json",
        )
        self.assertEqual(
            emitted_fixtures[1][1],
            temp_root
            / "artifacts"
            / "bundles"
            / "sample-reference"
            / "sample.detached.json",
        )
        self.assertEqual(
            compared_artifacts,
            [
                (
                    temp_root
                    / "artifacts"
                    / "bundles"
                    / "sample-smoke"
                    / "sample.detached.json",
                    temp_root
                    / "artifacts"
                    / "bundles"
                    / "sample-reference"
                    / "sample.detached.json",
                )
            ],
        )
        self.assertEqual(len(emitted_aggregates), 2)
        self.assertEqual(
            emitted_aggregates[0][1],
            temp_root
            / "artifacts"
            / "aggregates"
            / "sample-smoke-full"
            / "batch-summary.detached.json",
        )
        self.assertEqual(
            emitted_aggregates[1][1],
            temp_root
            / "artifacts"
            / "aggregates"
            / "sample-smoke-single"
            / "batch-summary.detached.json",
        )
        self.assertEqual(
            compared_aggregates,
            [
                (
                    temp_root
                    / "artifacts"
                    / "aggregates"
                    / "sample-smoke-full"
                    / "batch-summary.detached.json",
                    temp_root
                    / "artifacts"
                    / "aggregates"
                    / "sample-smoke-single"
                    / "batch-summary.detached.json",
                )
            ],
        )

    def test_smoke_fixture_propagates_helper_failures_above_diff_status_one(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_dir = temp_root / "fixtures"
            fixture_dir.mkdir()
            fixture_path = fixture_dir / "sample.json"
            fixture_path.write_text("{}", encoding="utf-8")

            original_emit_fixture = loop.emit_fixture
            original_emit_aggregate = loop.emit_aggregate
            original_compare_aggregates = loop.compare_aggregates

            loop.emit_fixture = lambda fixture, output, overwrite: 0
            loop.emit_aggregate = lambda fixture_directory, output, overwrite: 0
            loop.compare_aggregates = lambda left, right: 2
            try:
                exit_code = loop.command_smoke_fixture(
                    argparse.Namespace(
                        fixture_path=str(fixture_path),
                        reference_fixture=None,
                        artifact_root=str(temp_root / "artifacts"),
                        run_label="sample-smoke",
                        reference_label="reference",
                        overwrite=True,
                    )
                )
            finally:
                loop.emit_fixture = original_emit_fixture
                loop.emit_aggregate = original_emit_aggregate
                loop.compare_aggregates = original_compare_aggregates

        self.assertEqual(exit_code, 2)

    def test_smoke_try_rollback_locality_delegates_to_smoke_fixture_defaults(self) -> None:
        captured: list[argparse.Namespace] = []
        original_command = loop.command_smoke_fixture

        def fake_command(args: argparse.Namespace) -> int:
            captured.append(args)
            return 0

        loop.command_smoke_fixture = fake_command
        try:
            exit_code = loop.command_smoke_try_rollback_locality(
                argparse.Namespace(
                    artifact_root=str(REPO_ROOT / "target" / "current-l2-detached"),
                    run_label="try-rollback-mismatch",
                    reference_label="try-rollback-frontier",
                    overwrite=True,
                )
            )
        finally:
            loop.command_smoke_fixture = original_command

        self.assertEqual(exit_code, 0)
        self.assertEqual(len(captured), 1)
        delegated = captured[0]
        self.assertEqual(
            Path(delegated.fixture_path),
            REPO_ROOT
            / "crates/mir-ast/tests/fixtures/current-l2/e22-try-atomic-cut-place-mismatch.json",
        )
        self.assertEqual(
            Path(delegated.reference_fixture),
            REPO_ROOT
            / "crates/mir-ast/tests/fixtures/current-l2/e21-try-atomic-cut-frontier.json",
        )
        self.assertEqual(delegated.run_label, "try-rollback-mismatch")
        self.assertEqual(delegated.reference_label, "try-rollback-frontier")
        self.assertTrue(delegated.overwrite)


if __name__ == "__main__":
    unittest.main()
