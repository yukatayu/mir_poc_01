import re
import unittest
from html.parser import HTMLParser
from pathlib import Path
from urllib.parse import unquote, urlsplit


REPO_ROOT = Path(__file__).resolve().parents[2]
HTML_PATH = REPO_ROOT / "docs" / "mirrorea-project-overview.html"
DOCUMENTATION_PATH = REPO_ROOT / "Documentation.md"


class OverviewParser(HTMLParser):
    def __init__(self) -> None:
        super().__init__()
        self.ids: list[str] = []
        self.hrefs: list[str] = []

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        attributes = dict(attrs)
        element_id = attributes.get("id")
        if element_id:
            self.ids.append(element_id)
        if tag == "a" and attributes.get("href"):
            self.hrefs.append(attributes["href"])


class MirroreaProjectOverviewHtmlTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.html = HTML_PATH.read_text(encoding="utf-8")
        cls.parser = OverviewParser()
        cls.parser.feed(cls.html)

    def assert_contains_marker(self, haystack: str, marker: str, label: str) -> None:
        if marker not in haystack:
            self.fail(f"missing {label} marker: {marker!r}")

    def assert_omits_marker(self, haystack: str, marker: str, label: str) -> None:
        if marker in haystack:
            self.fail(f"stale {label} marker is still present: {marker!r}")

    def test_is_a_self_contained_japanese_reader_view(self) -> None:
        self.assertIn('<html lang="ja">', self.html)
        self.assertNotRegex(self.html, r"<script[^>]+src=")
        self.assertNotRegex(self.html, r"<link[^>]+href=")
        self.assertNotIn("https://", self.html)
        self.assertNotIn("http://", self.html)
        self.assertIn("@media print", self.html)
        self.assertIn("prefers-reduced-motion", self.html)
        self.assertIn('class="skip-link"', self.html)

    def test_has_unique_sections_for_the_required_reading_path(self) -> None:
        required_ids = {
            "north-star",
            "now",
            "system",
            "timeline",
            "experience",
            "theory",
            "barriers",
            "decisions",
            "evidence",
            "sources",
        }

        self.assertEqual(len(self.parser.ids), len(set(self.parser.ids)))
        self.assertTrue(required_ids.issubset(self.parser.ids))

    def test_states_the_exact_current_lifecycle_and_acceptance_boundary(self) -> None:
        required_facts = (
            "official lifecycle は T1",
            "M0–M10 は完走・閉鎖",
            "有限 I1+ deterministic reference profile は受理済み",
            "広い PHASE-I1 exit は未受理",
            "ADR-0026",
            "active roadmap は Plan 249",
            "SYS-0 completed",
            "SYS-1 completed",
            "SYS-2 completed",
            "SYS-0--SYS-4 completed",
            "active goal は SYS-5",
            "next goal は SYS-6",
            "SYS-4 completed",
            "SYS-5 active",
            "SYS-6 next",
            "closed / SYS-3",
            "closed / SYS-4",
            "現在 / SYS-5",
            "今 / SYS-5",
            "次 / SYS-6",
            "受理済みSYS-3 source/evidence cut",
            "3013e7fe075a7605a1ffe01e0b14f4a0856eaeb9",
            "accepted SYS-4 cut",
            "22196f93...",
            "partial regression history",
            "OBL-060 runtime-monitored (static finite only)",
            "OBL-061",
            "finite in-process dispatch",
            "runtime-monitored",
            "OPEN-030 は I2 internal bounded contract として解決",
            "program activation は broad PHASE-I1 exit / I2 lifecycle acceptance ではない",
            "Plan 247 は closed",
            "OPEN-030",
            "26/26",
            "47/47",
            "ConformanceAccepted",
            "083523518fdae0a111522f49b148c818ca0d5c21b4b7cc4f34dd476f10d172e7",
        )
        for fact in required_facts:
            self.assert_contains_marker(self.html, fact, "HTML lifecycle/acceptance")

    def test_separates_end_user_theory_evidence_and_future_claims(self) -> None:
        required_terms = (
            "エンドユーザー視点",
            "言語・理論視点",
            "lean-proved",
            "model-checked-bounded",
            "runtime-monitored",
            "intentionally-deferred",
            "Mirrorea",
            "PrismCascade",
            "Typed-Effect Wiring Platform",
            "same-owner RMW",
            "designated evaluator",
            "maintained relation",
            "semantic fallback",
            "presentation fallback",
        )
        for term in required_terms:
            self.assert_contains_marker(self.html, term, "HTML evidence separation")

        for phase in ("T0", "T1", "T2", "I1", "I2", "I3", "I4", "I5", "I6"):
            self.assertRegex(self.html, rf">\s*{phase}(?:\s|<)")

    def test_removes_obsolete_pre_m0_current_state_claims(self) -> None:
        stale_claims = (
            "公式経路は T0 に留まる",
            "公式 implementation state は T0",
            "proof ledger の <code>OBL-001</code>〜<code>OBL-028</code> も全件 <code>open</code>",
            "integration S2-A の直後",
            "現在地は integration S2-A",
            "G0 exit と T1 entry は未成立",
            "active roadmap なし",
            "新 owner direction が必要",
            "owner direction 待ち",
            "successor を開くか pause するか",
            "I2 は未開始",
            "新 owner roadmap",
            "post-program scope を owner が選ぶ地点",
            "closed M10 baseline の次に、どの direct consumer と acceptance profile を開くかが未選択",
            "SYS-0 closing",
            "next goal は SYS-1",
            "active goal は SYS-1",
            "next goal は SYS-2",
            "active goal は SYS-2",
            "next goal は SYS-3",
            "SYS-3 reopened",
            "active goal は SYS-3",
            "next goal は SYS-4",
            "SYS-3 active / reopened",
            "SYS-4 reopened",
            "active goal は SYS-4",
            "SYS-4 active",
            "next goal は SYS-5",
            "SYS-5 next",
            "現在 / SYS-3",
            "今 / SYS-3",
            "SYS-4 next",
            "次 / SYS-4",
            "現在 / SYS-4",
            "今 / SYS-4",
            "次 / SYS-5",
            "OBL-060 intentionally-deferred",
            "OBL-061 intentionally-deferred",
        )
        for claim in stale_claims:
            self.assert_omits_marker(self.html, claim, "HTML current-state")

    def test_all_internal_fragments_and_local_links_resolve(self) -> None:
        known_ids = set(self.parser.ids)
        missing_fragments: list[str] = []
        missing_paths: list[str] = []

        for href in self.parser.hrefs:
            parsed = urlsplit(href)
            if parsed.scheme or parsed.netloc:
                continue
            if parsed.fragment and parsed.fragment not in known_ids:
                missing_fragments.append(href)
            if not parsed.path:
                continue
            target = (HTML_PATH.parent / unquote(parsed.path)).resolve()
            try:
                target.relative_to(REPO_ROOT.resolve())
            except ValueError:
                missing_paths.append(href)
                continue
            if not target.exists():
                missing_paths.append(href)

        self.assertEqual([], missing_fragments)
        self.assertEqual([], missing_paths)

    def test_uses_semantic_html_for_diagrams_and_tables(self) -> None:
        self.assertGreaterEqual(len(re.findall(r"<figure(?:\s|>)", self.html)), 3)
        self.assertGreaterEqual(len(re.findall(r"<table(?:\s|>)", self.html)), 4)
        self.assertEqual(self.html.count("<caption>"), self.html.count("</caption>"))
        self.assertNotIn("<svg", self.html)

    def test_linked_reader_entry_uses_the_current_proof_and_program_boundary(self) -> None:
        documentation = DOCUMENTATION_PATH.read_text(encoding="utf-8")

        self.assert_contains_marker(
            documentation,
            "General OBL-001..025 と OBL-027 は `intentionally-deferred`",
            "Documentation proof boundary",
        )
        self.assert_contains_marker(documentation, "OBL-026 は `lean-proved`", "Documentation proof boundary")
        self.assert_contains_marker(
            documentation, "OBL-028 は `model-checked-bounded`", "Documentation proof boundary"
        )
        self.assert_contains_marker(documentation, "M0--M10 program は closed", "Documentation program boundary")
        self.assert_contains_marker(documentation, "active roadmap は Plan 249", "Documentation roadmap")
        self.assert_contains_marker(documentation, "SYS-0--SYS-4 completed", "Documentation SYS status")
        self.assert_contains_marker(documentation, "active goal は SYS-5", "Documentation SYS status")
        self.assert_contains_marker(documentation, "next goal は SYS-6", "Documentation SYS status")
        self.assert_contains_marker(documentation, "`ded622fe...`を", "Documentation SYS-3 history")
        self.assert_contains_marker(documentation, "partial regression evidenceへ", "Documentation SYS-3 history")
        self.assert_contains_marker(
            documentation,
            "corrected source/evidence cut `3013e7fe...`でclosed",
            "Documentation SYS-3 history",
        )
        self.assert_contains_marker(
            documentation,
            "SYS-4 accepted cut `22196f93b0112b8fd2987ec078021c8865b71651`",
            "Documentation SYS-4 boundary",
        )
        self.assert_contains_marker(
            documentation,
            "SYS-3のOBL-060もstatic finite compiler/projector evidenceだけを",
            "Documentation SYS-3 proof boundary",
        )
        self.assert_contains_marker(documentation, "OBL-061", "Documentation SYS-4 proof boundary")
        self.assert_contains_marker(
            documentation, "finite in-process dispatch", "Documentation SYS-4 proof boundary"
        )
        self.assert_contains_marker(
            documentation,
            "`runtime-monitored`とし、Lean/general proof又はruntime dispatch evidenceではありません",
            "Documentation SYS-3 proof boundary",
        )
        self.assert_omits_marker(documentation, "goal-first integration は `plan/246", "Documentation stale queue")
        self.assert_omits_marker(documentation, "S2-A が提示する次の判断", "Documentation stale queue")
        self.assert_omits_marker(
            documentation, "この主線の停止条件は、I1 を開始できる状態", "Documentation stale queue"
        )
        self.assert_omits_marker(documentation, "active goal は SYS-4", "Documentation stale SYS status")
        self.assert_omits_marker(documentation, "next goal は SYS-5", "Documentation stale SYS status")
        self.assert_omits_marker(documentation, "SYS-4 active", "Documentation stale SYS status")
        self.assert_omits_marker(documentation, "SYS-5 next", "Documentation stale SYS status")


if __name__ == "__main__":
    unittest.main()
