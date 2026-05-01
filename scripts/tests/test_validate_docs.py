from __future__ import annotations

import io
import sys
import tempfile
import unittest
from contextlib import redirect_stdout
from pathlib import Path
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

import validate_docs


class ValidateDocsTests(unittest.TestCase):
    def _write_required_scaffold(self, root: Path, template_text: str) -> None:
        for relative in validate_docs.REQUIRED:
            path = root / relative
            path.parent.mkdir(parents=True, exist_ok=True)
            if relative == "docs/reports/TEMPLATE.md":
                path.write_text(template_text, encoding="utf-8")
            else:
                path.write_text(f"# {relative}\n", encoding="utf-8")
        (root / "docs" / "reports" / "0001-smoke.md").write_text(
            "# Report 0001\n", encoding="utf-8"
        )

    def test_report_template_requires_commands_run_section(self) -> None:
        heading = "## Commands run"
        template_text = (
            Path(__file__).resolve().parents[2]
            / "docs"
            / "reports"
            / "TEMPLATE.md"
        ).read_text(encoding="utf-8")

        self.assertIn(heading, validate_docs.REQUIRED_TEMPLATE_HEADINGS)
        self.assertIn(heading, template_text)

    def test_main_rejects_template_missing_commands_run_section(self) -> None:
        heading = "## Commands run"
        template_text = "\n".join(
            h for h in validate_docs.REQUIRED_TEMPLATE_HEADINGS if h != heading
        )

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, template_text)

            stdout = io.StringIO()
            with mock.patch.object(validate_docs, "ROOT", root):
                with redirect_stdout(stdout):
                    exit_code = validate_docs.main()

        self.assertEqual(exit_code, 1)
        self.assertIn("Report template is missing required sections", stdout.getvalue())
        self.assertIn(heading, stdout.getvalue())

    def test_main_rejects_latest_report_missing_commands_run_section(self) -> None:
        heading = "## Commands run"
        template_text = "\n".join(validate_docs.REQUIRED_TEMPLATE_HEADINGS)
        latest_report_text = "\n".join(
            h for h in validate_docs.REQUIRED_TEMPLATE_HEADINGS if h != heading
        )

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, template_text)
            (root / "docs" / "reports" / "0002-latest.md").write_text(
                latest_report_text, encoding="utf-8"
            )

            stdout = io.StringIO()
            with mock.patch.object(validate_docs, "ROOT", root):
                with redirect_stdout(stdout):
                    exit_code = validate_docs.main()

        self.assertEqual(exit_code, 1)
        self.assertIn("Latest report is missing required sections", stdout.getvalue())
        self.assertIn("0002-latest.md", stdout.getvalue())
        self.assertIn(heading, stdout.getvalue())

    def test_main_allows_historical_report_missing_heading_when_latest_is_valid(self) -> None:
        heading = "## Commands run"
        template_text = "\n".join(validate_docs.REQUIRED_TEMPLATE_HEADINGS)
        historical_report_text = "\n".join(
            h for h in validate_docs.REQUIRED_TEMPLATE_HEADINGS if h != heading
        )
        latest_report_text = "\n".join(validate_docs.REQUIRED_TEMPLATE_HEADINGS)

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, template_text)
            (root / "docs" / "reports" / "0001-smoke.md").write_text(
                historical_report_text, encoding="utf-8"
            )
            (root / "docs" / "reports" / "0002-latest.md").write_text(
                latest_report_text, encoding="utf-8"
            )

            stdout = io.StringIO()
            with mock.patch.object(validate_docs, "ROOT", root):
                with redirect_stdout(stdout):
                    exit_code = validate_docs.main()

        self.assertEqual(exit_code, 0)
        self.assertIn("Documentation scaffold looks complete", stdout.getvalue())


if __name__ == "__main__":
    unittest.main()
