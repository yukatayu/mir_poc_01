import json
import sys
import tempfile
import unittest
from contextlib import redirect_stdout
from io import StringIO
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import current_l2_diff_detached_aggregates as aggregate_diff  # noqa: E402


def write_json(path: Path, payload: dict) -> None:
    path.write_text(json.dumps(payload, ensure_ascii=False), encoding="utf-8")


class DetachedAggregateDiffTests(unittest.TestCase):
    def test_compare_summary_core_detects_typed_count_difference(self) -> None:
        left = {
            "summary_core": {
                "total_bundles": 9,
                "runtime_bundles": 7,
                "static_only_bundles": 2,
                "passed": 8,
                "failed": 1,
                "bundle_failure_kind_counts_scope": "migrated-kinds-only",
                "bundle_failure_kind_counts": [
                    {"failure_kind": "host-plan-coverage-failure", "count": 1}
                ],
            }
        }
        right = {
            "summary_core": {
                "total_bundles": 9,
                "runtime_bundles": 7,
                "static_only_bundles": 2,
                "passed": 9,
                "failed": 0,
                "bundle_failure_kind_counts_scope": "migrated-kinds-only",
                "bundle_failure_kind_counts": [],
            }
        }

        differences = aggregate_diff.compare_summary_core(
            aggregate_diff.read_summary_core(left),
            aggregate_diff.read_summary_core(right),
        )

        self.assertIn(
            '- summary_core.passed: left=8 right=9',
            differences,
        )
        self.assertIn(
            '- summary_core.bundle_failure_kind_counts: left=[{"failure_kind": "host-plan-coverage-failure", "count": 1}] right=[]',
            differences,
        )

    def test_compare_summary_core_ignores_reference_only_sections(self) -> None:
        left = {
            "summary_core": {
                "total_bundles": 9,
                "runtime_bundles": 7,
                "static_only_bundles": 2,
                "passed": 9,
                "failed": 0,
                "bundle_failure_kind_counts_scope": "migrated-kinds-only",
                "bundle_failure_kind_counts": [],
            },
            "aggregate_context": {"directory_path": "left", "aggregate_scope": "directory-all"},
            "detached_noncore": {"host_plan_coverage_failures": [{"fixture_id": "e3"}]},
        }
        right = {
            "summary_core": {
                "total_bundles": 9,
                "runtime_bundles": 7,
                "static_only_bundles": 2,
                "passed": 9,
                "failed": 0,
                "bundle_failure_kind_counts_scope": "migrated-kinds-only",
                "bundle_failure_kind_counts": [],
            },
            "aggregate_context": {"directory_path": "right", "aggregate_scope": "directory-all"},
            "detached_noncore": {"host_plan_coverage_failures": []},
        }

        differences = aggregate_diff.compare_summary_core(
            aggregate_diff.read_summary_core(left),
            aggregate_diff.read_summary_core(right),
        )

        self.assertEqual(differences, [])

    def test_read_summary_core_requires_object_field(self) -> None:
        with self.assertRaises(ValueError):
            aggregate_diff.read_summary_core({"payload_core": {}})

    def test_main_returns_zero_when_summary_core_matches(self) -> None:
        payload = {
            "summary_core": {
                "total_bundles": 9,
                "runtime_bundles": 7,
                "static_only_bundles": 2,
                "passed": 9,
                "failed": 0,
                "bundle_failure_kind_counts_scope": "migrated-kinds-only",
                "bundle_failure_kind_counts": [],
            },
            "aggregate_context": {"directory_path": "left", "aggregate_scope": "directory-all"},
            "detached_noncore": {"host_plan_coverage_failures": []},
        }

        with tempfile.TemporaryDirectory() as temp_dir:
            left_path = Path(temp_dir) / "left.json"
            right_path = Path(temp_dir) / "right.json"
            write_json(left_path, payload)
            write_json(right_path, payload)

            exit_code = aggregate_diff.main([str(left_path), str(right_path)])

        self.assertEqual(exit_code, 0)

    def test_main_prints_reference_only_differences_per_field(self) -> None:
        left = {
            "summary_core": {
                "total_bundles": 1,
                "runtime_bundles": 1,
                "static_only_bundles": 0,
                "passed": 1,
                "failed": 0,
                "bundle_failure_kind_counts_scope": "migrated-kinds-only",
                "bundle_failure_kind_counts": [],
            },
            "aggregate_context": {
                "directory_path": "/tmp/left",
                "aggregate_scope": "directory-all",
            },
            "detached_noncore": {
                "host_plan_coverage_failures": [{"fixture_id": "e3"}],
            },
        }
        right = {
            "summary_core": {
                "total_bundles": 1,
                "runtime_bundles": 1,
                "static_only_bundles": 0,
                "passed": 1,
                "failed": 0,
                "bundle_failure_kind_counts_scope": "migrated-kinds-only",
                "bundle_failure_kind_counts": [],
            },
            "aggregate_context": {
                "directory_path": "/tmp/right",
                "aggregate_scope": "directory-all",
            },
            "detached_noncore": {
                "host_plan_coverage_failures": [],
            },
        }

        with tempfile.TemporaryDirectory() as temp_dir:
            left_path = Path(temp_dir) / "left.json"
            right_path = Path(temp_dir) / "right.json"
            write_json(left_path, left)
            write_json(right_path, right)

            buffer = StringIO()
            with redirect_stdout(buffer):
                exit_code = aggregate_diff.main([str(left_path), str(right_path)])
            output = buffer.getvalue()

        self.assertEqual(exit_code, 0)
        self.assertIn("reference-only differences:", output)
        self.assertIn(
            '- aggregate_context.directory_path: left="/tmp/left" right="/tmp/right"',
            output,
        )
        self.assertIn(
            '- detached_noncore.host_plan_coverage_failures: left=[{"fixture_id": "e3"}] right=[]',
            output,
        )
        self.assertNotIn("- aggregate_context: left=", output)


if __name__ == "__main__":
    unittest.main()
