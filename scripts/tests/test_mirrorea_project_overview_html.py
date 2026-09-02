import re
import unittest
from html.parser import HTMLParser
from pathlib import Path
from urllib.parse import unquote, urlsplit


REPO_ROOT = Path(__file__).resolve().parents[2]
HTML_PATH = REPO_ROOT / "docs" / "mirrorea-project-overview.html"
DOCUMENTATION_PATH = REPO_ROOT / "Documentation.md"
CANON_MAP_PATH = REPO_ROOT / "mirrorea_canon" / "MAP.md"
CANON_ROOT_PATH = REPO_ROOT / "CANON.md"


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
            "M0–M10",
            "SYS-0--SYS-7",
            "有限 I1+ deterministic reference profile は受理済み",
            "広い PHASE-I1 exit は未受理",
            "ADR-0026",
            "SYS-0 completed",
            "ADR-0027 / SYS-1 runtime kernel / internal carrier",
            "ADR-0028 / SYS-2 ST/OW1 refinement",
            "SYS-0--SYS-7までclosedです",
            "official I2 entry→exit は受理済み",
            "official I2 entry後exitを受理",
            "ADR-0033 / SYS-7 I3 entry contract only",
            "PROPOSAL-037 / ADR-0034",
            "Mirrorea I3 Distributed Foundation bounded program",
            "Plan 250がsole current roadmap",
            "Mirrorea I3 Distributed Foundation / I3-2 accepted, owner-paused",
            "ALIGN-0 / ALIGN-1 / ALIGN-2 / I3-0 / I3-1 / I3-2 completed、no active semantic milestone",
            "official I3 lifecycle entryは未受理",
            "official I3 lifecycle entryとproductionは主張しません",
            "closed / SYS-3",
            "closed / SYS-4",
            "closed / SYS-5",
            "closed / SYS-6",
            "closed / SYS-7",
            "SYS-6 I2 assurance / lifecycle closeout",
            "受理済みSYS-3 source/evidence cut",
            "3013e7fe075a7605a1ffe01e0b14f4a0856eaeb9",
            "SYS-4 cut",
            "22196f93...",
            "partial regression history",
            "OBL-060 runtime-monitored (static finite only)",
            "OBL-061",
            "finite in-process dispatch",
            "SYS-5 cut",
            "53a21e64...",
            "OBL-062 runtime-monitored",
            "OBL-063 runtime-monitored",
            "5429712d...",
            "PROPOSAL-035",
            "ADR-0032",
            "PROPOSAL-036",
            "ADR-0033",
            "PROPOSAL-040",
            "ADR-0037",
            "QUIC reliable streamをprivate provisional adapterとして選択",
            "TLS-over-TCP framed reliable streamはdeferred baseline",
            "private provisional",
            "deferred baseline",
            "QUIC datagram",
            "QUIC datagramは除外",
            "OPEN-032はこのbounded programに限り解決済み",
            "final public API/ABI/wire ではありません",
            "runtime-monitored",
            "OPEN-030をI2-internalに限って固定した",
            "official I2 exit acceptance は本物ですが",
            "Theory T2、broad I1、general theorem、public contract、production への自動昇格ではありません",
            "Plan 247とPlan 249はclosed baselines",
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

    def test_requires_the_independent_three_axis_project_product_map(self) -> None:
        required_terms = (
            "独立した三軸",
            "semantic strata（S0–S6）",
            "project/product layers（PL-0–PL-6）",
            "lifecycle / implementation phases（T0–T2 / I1–I6）",
            "多対多",
            "PL-0 Host / physical substrate",
            "PL-1 Mir language and semantic kernel",
            "PL-2 Mirrorea distributed fabric",
            "PL-3 Mir Browser / Host safe participant runtime",
            "PL-4 Shared-Space / World-Web platform",
            "責任境界のみ",
            "PL-5 Domain Kits and applications",
            "PL-6 Reversed Library / knowledge-world project",
            "Reversed LibraryはMirroreaのcompletion条件へ混ぜない",
            "PrismCascadeとTyped-Effect Wiring Platformはsatelliteとして保持",
        )
        for term in required_terms:
            self.assert_contains_marker(self.html, term, "HTML three-axis project/product map")

        self.assertIn(
            "../mirrorea_canon/architecture/06-project-product-layers.md",
            self.parser.hrefs,
            "HTML reader must link the normative project/product layer map",
        )
        self.assertNotRegex(self.html, r">\s*S7(?:\s|<)")
        self.assert_omits_marker(
            self.html,
            "S0–S5 は意味層、S6 は host realization の境界です。",
            "obsolete semantic-strata split",
        )

    def test_exposes_align2_browser_host_contracts_without_claiming_i3_entry(self) -> None:
        required_terms = (
            "trust tier T0–T4",
            "Theory T0–T2 とは別",
            "BND-007",
            "Runtime/Projection→View",
            "presentation-local computation",
            "authoritative domain semantics",
            "direct store",
            "redaction",
            "BND-010",
            "BND-011",
            "BND-012",
            "BND-013",
            "BND-014",
            "BND-015",
            "BND-016",
            "raw FFI",
            "T1 checked untrusted Mir package",
            "package admission",
            "semantic grant",
            "I3-2 accepted, owner-paused",
            "ADR-0037",
            "QUIC reliable streamをprivate provisional adapterに選択",
            "TLS-over-TCP framed reliable streamはdeferred baseline",
            "OPEN-032はこのbounded programに限り解決済み",
            "official I3 lifecycle entryは未受理",
        )
        for term in required_terms:
            self.assert_contains_marker(self.html, term, "HTML ALIGN-2 Browser/Host boundary")

        self.assertIn(
            "../mirrorea_canon/architecture/07-browser-host-trust-boundaries.md",
            self.parser.hrefs,
            "HTML reader must link the normative ALIGN-2 boundary contract",
        )

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
            "新しいowner directionが必要",
            "owner direction 待ち",
            "future I3 owner decision",
            "次にowner directionが必要な地点: I3 program activation",
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
            "active goal は SYS-5",
            "SYS-5 active",
            "SYS-0--SYS-5 completed",
            "active goal は SYS-6",
            "next goal は SYS-6",
            "SYS-6 next",
            "SYS-6 active",
            "SYS-7 next",
            "next goal は SYS-7",
            "SYS-7 active",
            "active goal は SYS-7 only",
            "SYS-7 sole active",
            "Active Plan 249",
            "Plan 250 / ALIGN-0 active/closing",
            "Plan 250 / ALIGN-0がactiveで",
            "Plan 250 / ALIGN-0 active</span>",
            "ALIGN-1はnext/not active",
            "Plan 250 / ALIGN-1が現在地（activation only）",
            "Mirrorea I3 Distributed Foundation / ALIGN-1 sole active goal",
            "ALIGN-0 completed、ALIGN-2 next/not active",
            "ALIGN-1 activation-only",
            "ALIGN-0 completed / ALIGN-1 active",
            "Plan 250 / ALIGN-2が現在地",
            "Mirrorea I3 Distributed Foundation / ALIGN-2 sole active goal",
            "ALIGN-0 / ALIGN-1 completed、I3-0 next/not active",
            "Plan 250 / I3-0が現在地",
            "Mirrorea I3 Distributed Foundation / I3-0 sole active goal",
            "ALIGN-0 / ALIGN-1 / ALIGN-2 completed、I3-2 accepted",
            "Plan 250 / I3-0 active",
            "I3-0はtransport比較段階で両候補UNSELECTED、official lifecycle未entryです。",
            "TLS-over-TCP framed reliable streamをprivate provisional adapterに選択",
            "現在 / SYS-7",
            "次 / SYS-7",
            "現在 / SYS-3",
            "今 / SYS-3",
            "SYS-4 next",
            "次 / SYS-4",
            "現在 / SYS-4",
            "今 / SYS-4",
            "次 / SYS-5",
            "現在 / SYS-5",
            "今 / SYS-5",
            "次 / SYS-6",
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
        self.assert_contains_marker(documentation, "Plan 247とPlan 249はclosed execution", "Documentation roadmap")
        self.assert_contains_marker(documentation, "SYS-0--SYS-7 completed / closed", "Documentation SYS status")
        self.assert_contains_marker(documentation, "Plan 250 sole roadmap", "Documentation I3 program status")
        self.assert_contains_marker(
            documentation,
            "ALIGN-0--2, I3-0/I3-1 and I3-2 completed; no active semantic milestone.",
            "Documentation I3 program status",
        )
        self.assert_contains_marker(
            documentation, "PROPOSAL-040 / ADR-0037", "Documentation I3 selection boundary"
        )
        self.assert_contains_marker(documentation, "PROPOSAL-036 / ADR-0033 / Canon plan 05", "Documentation SYS-7 boundary")
        self.assert_contains_marker(
            documentation, "ともにUNSELECTED", "Documentation historical SYS-7 candidate boundary"
        )
        self.assert_contains_marker(documentation, "private QUIC reliable stream", "Documentation I3 selected adapter")
        self.assert_contains_marker(
            documentation, "TLS/TCPはdeferred replacement baseline", "Documentation I3 deferred candidate"
        )
        self.assertRegex(
            documentation,
            r"criteria 8/9に勝者はなく、criterion 10 future browser\s+relevance",
            "Documentation I3 selected-criterion boundary",
        )
        self.assert_contains_marker(documentation, "QUIC datagramは除外", "Documentation I3 datagram exclusion")
        self.assert_contains_marker(documentation, "OPEN-032はこのbounded programだけresolved", "Documentation I3 scoped resolution")
        self.assert_omits_marker(documentation, "active roadmap はありません", "Documentation current roadmap")
        self.assert_omits_marker(documentation, "active programがない現在", "Documentation current program")
        self.assert_omits_marker(
            documentation,
            "ALIGN-0がactive/closingである",
            "Documentation stale ALIGN-0 state",
        )
        self.assert_omits_marker(
            documentation,
            "ALIGN-0 completed, ALIGN-1 sole active goal, ALIGN-2 next (not active).",
            "Documentation stale ALIGN-1 state",
        )
        self.assert_omits_marker(
            documentation,
            "ALIGN-0, ALIGN-1, and ALIGN-2 completed; I3-0 sole active goal.",
            "Documentation stale I3-0 state",
        )
        self.assert_omits_marker(
            documentation,
            "both candidates unselected, OPEN-032 unresolved",
            "Documentation stale I3-0 selection state",
        )
        self.assert_omits_marker(
            documentation,
            "private TLS-over-TCP",
            "Documentation stale TLS selected adapter",
        )
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
        self.assert_contains_marker(documentation, "OBL-062", "Documentation SYS-5 proof boundary")
        self.assert_contains_marker(documentation, "53a21e64", "Documentation SYS-5 boundary")
        self.assert_contains_marker(documentation, "PROPOSAL-035 / ADR-0032 / Canon spec 15 / OBL-063", "Documentation SYS-6 boundary")
        self.assert_contains_marker(documentation, "cut `5429712d...`の22-row finite `conform-i2` profileを受理", "Documentation SYS-6 boundary")
        self.assert_contains_marker(documentation, "official I2 entry後", "Documentation I2 lifecycle")
        self.assert_contains_marker(documentation, "exitを適用した", "Documentation I2 lifecycle")
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
        self.assert_omits_marker(documentation, "active goal は SYS-5", "Documentation stale SYS status")
        self.assert_omits_marker(documentation, "SYS-5 active", "Documentation stale SYS status")
        self.assert_omits_marker(documentation, "SYS-0--SYS-5 completed", "Documentation stale SYS status")
        self.assert_omits_marker(documentation, "active goal は SYS-6", "Documentation stale SYS status")
        self.assert_omits_marker(documentation, "active goal はSYS-6", "Documentation stale SYS status")
        self.assert_omits_marker(documentation, "next goal は SYS-6", "Documentation stale SYS status")
        self.assert_omits_marker(documentation, "SYS-6 next", "Documentation stale SYS status")
        self.assert_omits_marker(documentation, "next goal は SYS-7", "Documentation stale SYS status")
        self.assert_omits_marker(documentation, "SYS-7 active", "Documentation stale SYS status")
        self.assert_omits_marker(self.html, "I3-1 active", "HTML stale I3-1 active")
        self.assert_omits_marker(self.html, "I3-1がactive", "HTML stale I3-1 Japanese active")
        self.assert_omits_marker(self.html, "I3-1だけをactive", "HTML stale I3-1-only active")
        self.assert_omits_marker(documentation, "active goal はSYS-7", "Documentation stale SYS status")

    def test_canon_map_uses_the_current_align_frontier(self) -> None:
        canon_map = CANON_MAP_PATH.read_text(encoding="utf-8")

        self.assert_contains_marker(canon_map, "PROPOSAL-041 / ADR-0038でI3-1、PROPOSAL-042 / ADR-0039でI3-2をcompleted", "Canon MAP I3-2 close state")
        self.assertRegex(
            canon_map,
            r"I3-1[\s\S]*I3-2.*completed",
            "Canon MAP must retain I3-1/I3-2 status across prose line wrapping",
        )
        self.assert_contains_marker(
            canon_map, "criterion 10 future browser relevanceによりQUIC reliable", "Canon MAP QUIC selection"
        )
        self.assert_contains_marker(
            canon_map, "TLS-over-TCP framed reliable streamはrejected/", "Canon MAP TLS deferral"
        )
        self.assert_omits_marker(canon_map, "ALIGN-1がactiveである", "Canon MAP stale ALIGN-1 state")
        self.assert_omits_marker(canon_map, "I3-0がsole active goal", "Canon MAP stale I3-0 state")

    def test_root_canon_uses_the_current_i3_frontier(self) -> None:
        canon_root = CANON_ROOT_PATH.read_text(encoding="utf-8")

        self.assert_contains_marker(canon_root, "two-process runtime is closed by ADR-0039", "root CANON I3-2 close state")
        self.assert_contains_marker(
            canon_root, "select QUIC reliable stream as the", "root CANON QUIC selection"
        )
        self.assertNotIn(
            "QUIC reliable stream remains the deferred comparison baseline",
            canon_root,
            "root CANON must not both select and defer QUIC",
        )
        self.assertNotRegex(canon_root, r"I3-0\s+is (?:the )?sole active goal")


if __name__ == "__main__":
    unittest.main()
