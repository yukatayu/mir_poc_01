import json
import sys
import tempfile
import unittest
from contextlib import redirect_stdout
from io import StringIO
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import current_l2_diff_detached_artifacts as bundle_diff  # noqa: E402


def write_json(path: Path, payload: dict) -> None:
    path.write_text(json.dumps(payload, ensure_ascii=False), encoding="utf-8")


class DetachedArtifactDiffTests(unittest.TestCase):
    def test_main_prints_reference_only_differences_per_field(self) -> None:
        left = {
            "payload_core": {
                "static_verdict": "accepted",
                "entered_evaluation": True,
                "terminal_outcome": "success",
                "event_kinds": ["perform_on"],
                "non_admissible_metadata": [],
                "narrative_explanations": [],
            },
            "bundle_context": {
                "fixture_id": "e3_option_admit_chain",
                "fixture_path": "fixtures/e3-option-admit-chain.json",
            },
            "detached_noncore": {
                "steps_executed": 3,
            },
        }
        right = {
            "payload_core": {
                "static_verdict": "accepted",
                "entered_evaluation": True,
                "terminal_outcome": "success",
                "event_kinds": ["perform_on"],
                "non_admissible_metadata": [],
                "narrative_explanations": [],
            },
            "bundle_context": {
                "fixture_id": "e6_write_after_expiry",
                "fixture_path": "fixtures/e6-write-after-expiry.json",
            },
            "detached_noncore": {
                "steps_executed": 5,
            },
        }

        with tempfile.TemporaryDirectory() as temp_dir:
            left_path = Path(temp_dir) / "left.json"
            right_path = Path(temp_dir) / "right.json"
            write_json(left_path, left)
            write_json(right_path, right)

            buffer = StringIO()
            with redirect_stdout(buffer):
                exit_code = bundle_diff.main([str(left_path), str(right_path)])
            output = buffer.getvalue()

        self.assertEqual(exit_code, 0)
        self.assertIn("reference-only differences:", output)
        self.assertIn(
            '- bundle_context.fixture_id: left="e3_option_admit_chain" right="e6_write_after_expiry"',
            output,
        )
        self.assertIn(
            '- detached_noncore.steps_executed: left=3 right=5',
            output,
        )
        self.assertNotIn("- bundle_context: left=", output)


if __name__ == "__main__":
    unittest.main()
