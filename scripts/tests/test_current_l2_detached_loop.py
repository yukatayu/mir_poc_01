import argparse
import io
import json
import tempfile
import sys
import unittest
from contextlib import redirect_stderr, redirect_stdout
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import current_l2_detached_loop as loop  # noqa: E402


class DetachedLoopPathTests(unittest.TestCase):
    def test_resolve_fixture_argument_accepts_stem_without_extension(self) -> None:
        resolved = loop.resolve_fixture_argument("e3-option-admit-chain")
        self.assertEqual(
            resolved,
            REPO_ROOT
            / "crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json",
        )

    def test_resolve_fixture_argument_accepts_absolute_existing_path(self) -> None:
        fixture_path = (
            REPO_ROOT
            / "crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json"
        )
        resolved = loop.resolve_fixture_argument(str(fixture_path))
        self.assertEqual(resolved, fixture_path)

    def test_resolve_fixture_argument_rejects_missing_stem_with_helpful_error(self) -> None:
        with self.assertRaises(ValueError) as raised:
            loop.resolve_fixture_argument("e999-missing")

        self.assertIn("fixture not found", str(raised.exception))
        self.assertIn("current-l2", str(raised.exception))

    def test_emit_fixture_rejects_missing_fixture_before_spawning_subprocess(self) -> None:
        captured_cmds: list[list[str]] = []
        original_runner = loop.run_subprocess

        def fake_runner(cmd: list[str]) -> int:
            captured_cmds.append(cmd)
            return 0

        loop.run_subprocess = fake_runner
        try:
            stderr = io.StringIO()
            with redirect_stderr(stderr):
                exit_code = loop.emit_fixture(
                    REPO_ROOT
                    / "crates/mir-ast/tests/fixtures/current-l2/does-not-exist.json",
                    REPO_ROOT
                    / "target/current-l2-detached/bundles/test/missing.detached.json",
                    overwrite=True,
                )
        finally:
            loop.run_subprocess = original_runner

        self.assertEqual(exit_code, 2)
        self.assertEqual(captured_cmds, [])
        self.assertIn("fixture does not exist", stderr.getvalue())

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

    def test_compare_fixtures_derives_labels_from_fixture_stems_when_omitted(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_dir = temp_root / "fixtures"
            fixture_dir.mkdir()
            left_fixture = fixture_dir / "left-case.json"
            right_fixture = fixture_dir / "right-case.json"
            left_fixture.write_text("{}", encoding="utf-8")
            right_fixture.write_text("{}", encoding="utf-8")

            emitted_fixtures: list[tuple[Path, Path, bool]] = []
            original_emit_fixture = loop.emit_fixture
            original_compare_artifacts = loop.compare_artifacts

            def fake_emit_fixture(fixture: Path, output: Path, overwrite: bool) -> int:
                emitted_fixtures.append((fixture, output, overwrite))
                return 0

            loop.emit_fixture = fake_emit_fixture
            loop.compare_artifacts = lambda left, right: 0
            try:
                exit_code = loop.command_compare_fixtures(
                    argparse.Namespace(
                        left_fixture=str(left_fixture),
                        right_fixture=str(right_fixture),
                        artifact_root=str(temp_root / "artifacts"),
                        left_label=None,
                        right_label=None,
                        overwrite=True,
                    )
                )
            finally:
                loop.emit_fixture = original_emit_fixture
                loop.compare_artifacts = original_compare_artifacts

        self.assertEqual(exit_code, 0)
        self.assertEqual(
            emitted_fixtures[0][1],
            temp_root / "artifacts" / "bundles" / "left-case" / "left-case.detached.json",
        )
        self.assertEqual(
            emitted_fixtures[1][1],
            temp_root / "artifacts" / "bundles" / "right-case" / "right-case.detached.json",
        )

    def test_compare_fixture_aggregates_derives_single_labels_and_copies_sidecars(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_dir = temp_root / "fixtures"
            fixture_dir.mkdir()

            left_fixture = fixture_dir / "left-case.json"
            left_fixture.write_text("{}", encoding="utf-8")
            (fixture_dir / "left-case.host-plan.json").write_text("{}", encoding="utf-8")

            right_fixture = fixture_dir / "right-case.json"
            right_fixture.write_text("{}", encoding="utf-8")

            emitted_aggregates: list[tuple[Path, Path, bool, list[str]]] = []
            compared_aggregates: list[tuple[Path, Path]] = []
            original_emit_aggregate = loop.emit_aggregate
            original_compare_aggregates = loop.compare_aggregates

            def fake_emit_aggregate(
                fixture_directory: Path,
                output: Path,
                overwrite: bool,
            ) -> int:
                emitted_aggregates.append(
                    (
                        fixture_directory,
                        output,
                        overwrite,
                        sorted(path.name for path in fixture_directory.iterdir()),
                    )
                )
                return 0

            def fake_compare_aggregates(left: Path, right: Path) -> int:
                compared_aggregates.append((left, right))
                return 0

            loop.emit_aggregate = fake_emit_aggregate
            loop.compare_aggregates = fake_compare_aggregates
            try:
                exit_code = loop.command_compare_fixture_aggregates(
                    argparse.Namespace(
                        left_fixture=str(left_fixture),
                        right_fixture=str(right_fixture),
                        artifact_root=str(temp_root / "artifacts"),
                        left_label=None,
                        right_label=None,
                        overwrite=True,
                    )
                )
            finally:
                loop.emit_aggregate = original_emit_aggregate
                loop.compare_aggregates = original_compare_aggregates

        self.assertEqual(exit_code, 0)
        self.assertEqual(
            emitted_aggregates[0][1],
            temp_root
            / "artifacts"
            / "aggregates"
            / "left-case-single"
            / "batch-summary.detached.json",
        )
        self.assertEqual(
            emitted_aggregates[1][1],
            temp_root
            / "artifacts"
            / "aggregates"
            / "right-case-single"
            / "batch-summary.detached.json",
        )
        self.assertEqual(
            emitted_aggregates[0][3],
            ["left-case.host-plan.json", "left-case.json"],
        )
        self.assertEqual(
            emitted_aggregates[1][3],
            ["right-case.json"],
        )
        self.assertEqual(
            compared_aggregates,
            [
                (
                    temp_root
                    / "artifacts"
                    / "aggregates"
                    / "left-case-single"
                    / "batch-summary.detached.json",
                    temp_root
                    / "artifacts"
                    / "aggregates"
                    / "right-case-single"
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

    def test_smoke_formal_hook_runtime_emits_bundle_then_delegates(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_dir = temp_root / "fixtures"
            fixture_dir.mkdir()
            fixture_path = fixture_dir / "sample.json"
            fixture_path.write_text("{}", encoding="utf-8")

            emitted_fixtures: list[tuple[Path, Path, bool]] = []
            delegated: list[tuple[str, Path, Path, bool]] = []

            original_emit_fixture = loop.emit_fixture
            original_emit_formal_hook = loop.emit_formal_hook

            def fake_emit_fixture(
                fixture: Path,
                output: Path,
                overwrite: bool,
            ) -> int:
                emitted_fixtures.append((fixture, output, overwrite))
                output.parent.mkdir(parents=True, exist_ok=True)
                output.write_text("{}", encoding="utf-8")
                return 0

            def fake_emit_formal_hook(
                source_kind: str,
                source_artifact: Path,
                output: Path,
                overwrite: bool,
            ) -> int:
                delegated.append((source_kind, source_artifact, output, overwrite))
                return 0

            loop.emit_fixture = fake_emit_fixture
            loop.emit_formal_hook = fake_emit_formal_hook
            try:
                exit_code = loop.command_smoke_formal_hook_runtime(
                    argparse.Namespace(
                        fixture_path=str(fixture_path),
                        artifact_root=str(temp_root / "artifacts"),
                        run_label="formal-runtime",
                        overwrite=True,
                    )
                )
            finally:
                loop.emit_fixture = original_emit_fixture
                loop.emit_formal_hook = original_emit_formal_hook

        self.assertEqual(exit_code, 0)
        bundle_artifact = (
            temp_root
            / "artifacts"
            / "bundles"
            / "formal-runtime"
            / "sample.detached.json"
        )
        formal_hook_artifact = (
            temp_root
            / "artifacts"
            / "formal-hooks"
            / "formal-runtime"
            / "sample.formal-hook.json"
        )
        self.assertEqual(emitted_fixtures, [(fixture_path, bundle_artifact, True)])
        self.assertEqual(
            delegated,
            [("detached-bundle", bundle_artifact, formal_hook_artifact, True)],
        )

    def test_emit_formal_hook_runtime_writes_expected_contract_rows(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            source_artifact = temp_root / "sample.detached.json"
            output_artifact = temp_root / "sample.formal-hook.json"
            source_artifact.write_text(
                json.dumps(
                    {
                        "schema_version": "draft-current-l2-detached-bundle-v0",
                        "artifact_kind": "current-l2-bundle-detached-sketch",
                        "bundle_context": {
                            "fixture_id": "e2_try_fallback",
                            "fixture_path": "/tmp/e2-try-fallback.json",
                            "host_plan_path": None,
                            "runtime_requirement": "runtime-with-host-plan",
                        },
                        "payload_core": {
                            "static_verdict": "valid",
                            "entered_evaluation": True,
                            "terminal_outcome": "success",
                            "event_kinds": ["rollback", "perform-success"],
                            "non_admissible_metadata": [],
                            "narrative_explanations": [],
                        },
                        "detached_noncore": {"steps_executed": 3},
                    }
                ),
                encoding="utf-8",
            )

            exit_code = loop.emit_formal_hook(
                "detached-bundle",
                source_artifact,
                output_artifact,
                overwrite=True,
            )
            payload = json.loads(output_artifact.read_text(encoding="utf-8"))

        self.assertEqual(exit_code, 0)
        self.assertEqual(payload["subject_kind"], "runtime_try_cut_cluster")
        self.assertEqual(payload["subject_ref"], "e2_try_fallback")
        self.assertEqual(
            payload["contract_rows"],
            [
                {
                    "obligation_kind": "rollback_cut_non_interference",
                    "evidence_refs": [
                        {
                            "ref_kind": "fixture",
                            "ref_id": "e2_try_fallback",
                        },
                        {
                            "ref_kind": "runtime_cluster",
                            "ref_id": "e2_try_fallback",
                        },
                    ],
                }
            ],
        )

    def test_smoke_fixture_derives_labels_from_fixture_stems_when_omitted(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_dir = temp_root / "fixtures"
            fixture_dir.mkdir()
            fixture_path = fixture_dir / "sample-case.json"
            fixture_path.write_text("{}", encoding="utf-8")
            reference_fixture = fixture_dir / "reference-case.json"
            reference_fixture.write_text("{}", encoding="utf-8")

            emitted_fixtures: list[tuple[Path, Path, bool]] = []
            emitted_aggregates: list[tuple[Path, Path, bool]] = []
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

            loop.emit_fixture = fake_emit_fixture
            loop.emit_aggregate = fake_emit_aggregate
            loop.compare_artifacts = lambda left, right: 0
            loop.compare_aggregates = lambda left, right: 0
            try:
                exit_code = loop.command_smoke_fixture(
                    argparse.Namespace(
                        fixture_path=str(fixture_path),
                        reference_fixture=str(reference_fixture),
                        artifact_root=str(temp_root / "artifacts"),
                        run_label=None,
                        reference_label=None,
                        overwrite=True,
                    )
                )
            finally:
                loop.emit_fixture = original_emit_fixture
                loop.emit_aggregate = original_emit_aggregate
                loop.compare_artifacts = original_compare_artifacts
                loop.compare_aggregates = original_compare_aggregates

        self.assertEqual(exit_code, 0)
        self.assertEqual(
            emitted_fixtures[0][1],
            temp_root
            / "artifacts"
            / "bundles"
            / "sample-case"
            / "sample-case.detached.json",
        )
        self.assertEqual(
            emitted_fixtures[1][1],
            temp_root
            / "artifacts"
            / "bundles"
            / "reference-case"
            / "reference-case.detached.json",
        )
        self.assertEqual(
            emitted_aggregates[0][1],
            temp_root
            / "artifacts"
            / "aggregates"
            / "sample-case-full"
            / "batch-summary.detached.json",
        )
        self.assertEqual(
            emitted_aggregates[1][1],
            temp_root
            / "artifacts"
            / "aggregates"
            / "sample-case-single"
            / "batch-summary.detached.json",
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

    def test_smoke_fixture_prints_informational_notes_for_diff_status_one(self) -> None:
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
            loop.compare_aggregates = lambda left, right: 1
            capture = io.StringIO()
            try:
                with redirect_stdout(capture):
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

        self.assertEqual(exit_code, 0)
        output = capture.getvalue()
        self.assertIn("aggregate compare: differences found (informational)", output)
        self.assertIn("full directory aggregate と single-fixture aggregate", output)

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
