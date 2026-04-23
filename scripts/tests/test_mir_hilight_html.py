import re
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
HTML_PATH = REPO_ROOT / "mir_hilight.html"


class MirHilightHtmlTests(unittest.TestCase):
    def test_single_file_viewer_exists_without_external_dependencies(self):
        html = HTML_PATH.read_text(encoding="utf-8")

        self.assertIn('<html lang="ja" data-theme="solarized-dark">', html)
        self.assertNotRegex(html, r"<script[^>]+src=")
        self.assertNotRegex(html, r"<link[^>]+href=")
        self.assertNotIn("https://", html)
        self.assertNotIn("http://", html)

    def test_embeds_every_active_clean_near_end_mir_sample(self):
        html = HTML_PATH.read_text(encoding="utf-8")
        sample_paths = sorted(
            path.relative_to(REPO_ROOT).as_posix()
            for path in (REPO_ROOT / "samples" / "clean-near-end").rglob("*.mir")
        )

        self.assertGreaterEqual(len(sample_paths), 20)
        for sample_path in sample_paths:
            self.assertIn(f'"path": "{sample_path}"', html)

    def test_supports_required_highlight_features_and_themes(self):
        html = HTML_PATH.read_text(encoding="utf-8")

        self.assertIn("tok-keyword", html)
        self.assertIn("tok-definition", html)
        self.assertIn("line-number", html)
        self.assertIn("code-line", html)
        self.assertIn("collectDefinedSymbols", html)
        self.assertIn("highlightMir", html)

        themes = re.findall(r'<option value="([^"]+)">', html)
        self.assertIn("solarized-dark", themes)
        self.assertIn("vscode-dark", themes)
        self.assertIn("github-light", themes)
        self.assertIn("github-dark", themes)
        self.assertIn("dracula", themes)
        self.assertIn("monokai", themes)

    def test_supports_custom_code_input(self):
        html = HTML_PATH.read_text(encoding="utf-8")

        self.assertIn('id="custom-source"', html)
        self.assertIn('id="render-custom"', html)
        self.assertIn('id="load-current"', html)
        self.assertIn('id="clear-custom"', html)
        self.assertIn("renderCustomSource", html)
        self.assertIn("custom source", html)
        self.assertIn("highlightMir(customSource.value)", html)

    def test_documents_update_timing_for_grammar_changes(self):
        html = HTML_PATH.read_text(encoding="utf-8")

        self.assertIn("文法や active sample path を変更したら", html)
        self.assertIn("syntax token list", html)


if __name__ == "__main__":
    unittest.main()
