import sys
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import current_l2_guided_samples as guided  # noqa: E402


class CurrentL2GuidedSamplesTests(unittest.TestCase):
    def test_problem_specs_cover_two_big_problems(self) -> None:
        specs = guided.problem_specs()

        self.assertEqual(set(specs.keys()), {"problem1", "problem2"})

    def test_problem1_show_text_mentions_primary_and_supporting_samples(self) -> None:
        spec = guided.problem_specs()["problem1"]

        text = guided.render_problem_guide(spec)

        self.assertIn("Problem 1", text)
        self.assertIn("p06-typed-proof-owner-handoff", text)
        self.assertIn("p10-typed-authorized-fingerprint-declassification", text)
        self.assertIn("p12-typed-classified-fingerprint-publication-block", text)
        self.assertIn("verification_preview", text)
        self.assertIn("artifact_preview", text)

    def test_problem2_primary_run_commands_use_representative_pair(self) -> None:
        spec = guided.problem_specs()["problem2"]

        commands = guided.build_run_commands(spec, output_format="pretty", include_all=False)
        command_texts = [" ".join(command) for command in commands]

        self.assertEqual(len(commands), 2)
        self.assertTrue(
            any("p07-dice-late-join-visible-history.txt" in text for text in command_texts)
        )
        self.assertTrue(
            any("p08-dice-stale-reconnect-refresh.txt" in text for text in command_texts)
        )
        self.assertFalse(
            any("p09-dice-delegated-rng-provider-placement.txt" in text for text in command_texts)
        )

    def test_problem2_all_run_commands_include_reserve_and_negative_samples(self) -> None:
        spec = guided.problem_specs()["problem2"]

        commands = guided.build_run_commands(spec, output_format="json", include_all=True)
        command_texts = [" ".join(command) for command in commands]

        self.assertTrue(
            any("p09-dice-delegated-rng-provider-placement.txt" in text for text in command_texts)
        )
        self.assertTrue(
            any(
                "p13-dice-late-join-missing-publication-witness.txt" in text
                for text in command_texts
            )
        )
        self.assertTrue(
            any(
                "p14-dice-late-join-handoff-before-publication.txt" in text
                for text in command_texts
            )
        )


if __name__ == "__main__":
    unittest.main()
