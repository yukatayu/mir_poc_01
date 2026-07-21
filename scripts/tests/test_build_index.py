from __future__ import annotations

import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path


SCRIPT = (
    Path(__file__).resolve().parents[2]
    / "mirrorea_canon"
    / "meta"
    / "build-index.py"
)


class BuildIndexTests(unittest.TestCase):
    def test_make_check_runs_index_freshness_check(self) -> None:
        root = Path(__file__).resolve().parents[2]
        result = subprocess.run(
            ["make", "-n", "check"],
            cwd=root,
            capture_output=True,
            text=True,
            check=False,
        )

        self.assertEqual(0, result.returncode)
        self.assertIn(
            "cd mirrorea_canon && python3 meta/build-index.py --check", result.stdout
        )

    def test_check_rejects_stale_index(self) -> None:
        front_matter = (
            "---\n"
            "id: root/north-star\n"
            "status: L0-frozen\n"
            "maturity: draft\n"
            "depends_on: []\n"
            "summary: test root\n"
            "open_items: []\n"
            "---\n\n"
            "# Test root\n"
        )
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / "NORTH-STAR.md").write_text(front_matter, encoding="utf-8")
            (root / "INDEX.json").write_text(
                json.dumps({"files": 0}), encoding="utf-8"
            )

            result = subprocess.run(
                [sys.executable, str(SCRIPT), "--check"],
                cwd=root,
                capture_output=True,
                text=True,
                check=False,
            )

        self.assertNotEqual(0, result.returncode)
        self.assertIn("INDEX.json is stale", result.stdout)

    def test_check_rejects_duplicate_front_matter_fields(self) -> None:
        front_matter = (
            "---\n"
            "id: root/north-star\n"
            "status: L1-fixed\n"
            "status: L0-frozen\n"
            "maturity: draft\n"
            "depends_on: []\n"
            "summary: test root\n"
            "open_items: []\n"
            "---\n\n"
            "# Test root\n"
        )
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / "NORTH-STAR.md").write_text(front_matter, encoding="utf-8")
            (root / "INDEX.json").write_text(
                json.dumps({"files": 0}), encoding="utf-8"
            )

            result = subprocess.run(
                [sys.executable, str(SCRIPT), "--check"],
                cwd=root,
                capture_output=True,
                text=True,
                check=False,
            )

        self.assertNotEqual(0, result.returncode)
        self.assertIn("duplicate front matter field", result.stdout)

    def test_regeneration_does_not_overwrite_index_when_front_matter_is_invalid(self) -> None:
        front_matter = (
            "---\n"
            "id: root/north-star\n"
            "status: L1-fixed\n"
            "status: L0-frozen\n"
            "maturity: draft\n"
            "depends_on: []\n"
            "summary: test root\n"
            "open_items: []\n"
            "---\n\n"
            "# Test root\n"
        )
        original_index = json.dumps({"files": 0})
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / "NORTH-STAR.md").write_text(front_matter, encoding="utf-8")
            (root / "INDEX.json").write_text(original_index, encoding="utf-8")

            result = subprocess.run(
                [sys.executable, str(SCRIPT)],
                cwd=root,
                capture_output=True,
                text=True,
                check=False,
            )

            self.assertNotEqual(0, result.returncode)
            self.assertEqual(
                original_index, (root / "INDEX.json").read_text(encoding="utf-8")
            )

    def test_regeneration_rejects_malformed_front_matter_lines_without_writing(self) -> None:
        front_matter = (
            "---\n"
            "id: root/north-star\n"
            "status: L0-frozen\n"
            "maturity: draft\n"
            "this is not front matter\n"
            "depends_on: []\n"
            "summary: test root\n"
            "open_items: []\n"
            "---\n\n"
            "# Test root\n"
        )
        original_index = json.dumps({"files": 0})
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / "NORTH-STAR.md").write_text(front_matter, encoding="utf-8")
            (root / "INDEX.json").write_text(original_index, encoding="utf-8")

            result = subprocess.run(
                [sys.executable, str(SCRIPT)],
                cwd=root,
                capture_output=True,
                text=True,
                check=False,
            )

            self.assertNotEqual(0, result.returncode)
            self.assertIn("malformed front matter line", result.stdout)
            self.assertEqual(
                original_index, (root / "INDEX.json").read_text(encoding="utf-8")
            )


if __name__ == "__main__":
    unittest.main()
