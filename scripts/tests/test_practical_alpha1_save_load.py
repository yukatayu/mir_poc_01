import unittest
import sys
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import practical_alpha1_check  # noqa: E402
import practical_alpha1_save_load as runner  # noqa: E402


class PracticalAlpha1SaveLoadTests(unittest.TestCase):
    def test_list_samples_matches_save_load_rows(self) -> None:
        rows = runner.list_samples()
        self.assertEqual(
            [row["sample_id"] for row in rows],
            [row["sample_id"] for row in runner.IMPLEMENTED_ROWS],
        )
        self.assertTrue(
            all(row["family"] == "practical-alpha1-save-load" for row in rows)
        )

    def test_closeout_reuses_invalid_distributed_cut_checker_guard(self) -> None:
        payload = runner.closeout()
        self.assertEqual(payload["checker_guard_refs"], ["CHK-CUT-01"])

    def test_check_all_requires_checker_guard(self) -> None:
        original = practical_alpha1_check.run_sample

        def fake_run_sample(sample_id: str):
            if sample_id == "CHK-CUT-01":
                raise RuntimeError("checker guard missing")
            return original(sample_id)

        practical_alpha1_check.run_sample = fake_run_sample
        try:
            payload = runner.check_all()
        finally:
            practical_alpha1_check.run_sample = original

        self.assertFalse(payload["local_save_load_first_floor_complete"])
        self.assertFalse(payload["invalid_distributed_cut_guard_present"])


if __name__ == "__main__":
    unittest.main()
