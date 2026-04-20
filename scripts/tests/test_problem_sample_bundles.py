import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]


class ProblemSampleBundleDocsTests(unittest.TestCase):
    def test_problem_bundle_index_mentions_two_big_problem_guides(self) -> None:
        text = (REPO_ROOT / "samples" / "problem-bundles" / "README.md").read_text(encoding="utf-8")

        self.assertIn("二大問題", text)
        self.assertIn("problem1-typed-theorem-model-check.md", text)
        self.assertIn("problem2-order-handoff-shared-space.md", text)
        self.assertIn("python3 scripts/current_l2_guided_samples.py smoke-all", text)
        self.assertIn("python3 scripts/current_l2_guided_samples.py quickstart problem1", text)
        self.assertIn("python3 scripts/current_l2_guided_samples.py quickstart-parity", text)
        self.assertIn("最短 quickstart", text)
        self.assertIn("failed step", text)
        self.assertIn("非ゼロ", text)

    def test_problem1_bundle_doc_mentions_representative_paths_and_commands(self) -> None:
        text = (
            REPO_ROOT
            / "samples"
            / "problem-bundles"
            / "problem1-typed-theorem-model-check.md"
        ).read_text(encoding="utf-8")

        self.assertIn("p06-typed-proof-owner-handoff", text)
        self.assertIn("最短 quickstart", text)
        self.assertIn("python3 scripts/current_l2_guided_samples.py smoke problem1", text)
        self.assertIn("見るべき結果", text)
        self.assertIn("current_l2_inspect_request_head_clause_bundle", text)
        self.assertIn("python3 scripts/current_l2_guided_samples.py bundle problem1", text)
        self.assertIn(
            "samples/prototype/current-l2-parser-companion/p06-typed-proof-owner-handoff.request.txt",
            text,
        )
        self.assertIn(
            "samples/lean/current-l2/p06-typed-proof-owner-handoff/",
            text,
        )
        self.assertIn("final public verifier contract", text)

    def test_problem2_bundle_doc_mentions_representative_reserve_and_negative_paths(self) -> None:
        text = (
            REPO_ROOT
            / "samples"
            / "problem-bundles"
            / "problem2-order-handoff-shared-space.md"
        ).read_text(encoding="utf-8")

        self.assertIn("p07-dice-late-join-visible-history", text)
        self.assertIn("p08-dice-stale-reconnect-refresh", text)
        self.assertIn("p09-dice-delegated-rng-provider-placement", text)
        self.assertIn("p13-dice-late-join-missing-publication-witness", text)
        self.assertIn("最短 quickstart", text)
        self.assertIn("python3 scripts/current_l2_guided_samples.py smoke problem2", text)
        self.assertIn("見るべき結果", text)
        self.assertIn("current_l2_inspect_request_head_clause_bundle", text)
        self.assertIn("python3 scripts/current_l2_guided_samples.py bundle problem2", text)
        self.assertIn(
            "samples/prototype/current-l2-parser-companion/p07-dice-late-join-visible-history.request.txt",
            text,
        )
        self.assertIn(
            "samples/lean/current-l2/p08-dice-stale-reconnect-refresh/",
            text,
        )
        self.assertIn("exhaustive shared-space catalog", text)


if __name__ == "__main__":
    unittest.main()
