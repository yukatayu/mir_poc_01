import unittest


from scripts import practical_alpha_error_display as display


class PracticalAlphaErrorDisplayTests(unittest.TestCase):
    def test_repo_path_is_displayed_relative_to_repo_root(self) -> None:
        value = display.REPO_ROOT / "samples/practical-alpha1/packages/run-01-local-sugoroku"
        self.assertEqual(
            display.repo_display_text(value),
            "samples/practical-alpha1/packages/run-01-local-sugoroku",
        )

    def test_exact_repo_root_is_displayed_as_dot(self) -> None:
        self.assertEqual(display.repo_display_text(display.REPO_ROOT), ".")

    def test_exact_repo_root_with_trailing_slash_is_displayed_as_dot(self) -> None:
        self.assertEqual(display.repo_display_text(f"{display.REPO_ROOT}/"), ".")

    def test_external_path_stays_absolute(self) -> None:
        self.assertEqual(
            display.repo_display_text("/tmp/mirrorea-practical-alpha1-output"),
            "/tmp/mirrorea-practical-alpha1-output",
        )

    def test_repo_root_prefix_lookalike_stays_absolute(self) -> None:
        lookalike = f"{display.REPO_ROOT}_backup/samples/practical-alpha1"
        self.assertEqual(display.repo_display_text(lookalike), lookalike)

    def test_external_path_containing_repo_path_stays_absolute(self) -> None:
        external = f"/tmp/mirror{display.REPO_ROOT}/samples/practical-alpha1"
        self.assertEqual(display.repo_display_text(external), external)

    def test_free_text_repo_paths_are_redacted_without_touching_external_paths(self) -> None:
        text = (
            f"repo={display.REPO_ROOT}; "
            f"repo_slash={display.REPO_ROOT}/; "
            f"pkg={display.REPO_ROOT}/samples/practical-alpha1/packages/run-01-local-sugoroku; "
            "tmp=/tmp/mirrorea-practical-alpha1-output"
        )
        observed = display.repo_display_text(text)
        self.assertNotIn(str(display.REPO_ROOT), observed)
        self.assertIn("repo=.", observed)
        self.assertIn("repo_slash=.;", observed)
        self.assertIn(
            "pkg=samples/practical-alpha1/packages/run-01-local-sugoroku",
            observed,
        )
        self.assertIn("tmp=/tmp/mirrorea-practical-alpha1-output", observed)


if __name__ == "__main__":
    unittest.main()
