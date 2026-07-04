from __future__ import annotations

import importlib.util
import io
import json
import os
import sys
import tempfile
import unittest
from contextlib import redirect_stdout
from pathlib import Path
from unittest import mock


REPO_ROOT = Path(__file__).resolve().parents[2]
MODULE_PATH = REPO_ROOT / ".agents" / "skills" / "discord-report" / "scripts" / "discord_notify.py"

spec = importlib.util.spec_from_file_location("discord_notify_skill", MODULE_PATH)
assert spec is not None
discord_notify = importlib.util.module_from_spec(spec)
assert spec.loader is not None
sys.modules[spec.name] = discord_notify
spec.loader.exec_module(discord_notify)


class DiscordNotifySkillTests(unittest.TestCase):
    def test_progress_accepts_summary_and_next_step_files(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            summary_path = root / "summary.txt"
            next_step_path = root / "next-step.txt"
            summary_path.write_text("P105: commit `abc1234` pushed", encoding="utf-8")
            next_step_path.write_text("Continue with `validate_docs`", encoding="utf-8")

            argv = [
                "discord_notify.py",
                "progress",
                "--cwd",
                str(root),
                "--summary-file",
                str(summary_path),
                "--next-step-file",
                str(next_step_path),
                "--force",
                "--print-payload",
            ]

            stdout = io.StringIO()
            sent_payloads: list[dict[str, object]] = []
            with mock.patch.object(sys, "argv", argv):
                with mock.patch.dict(
                    os.environ,
                    {"CODEX_DISCORD_WEBHOOK_URL": "https://example.invalid/webhook"},
                    clear=False,
                ):
                    with mock.patch.object(discord_notify, "post_webhook", lambda _url, payload: sent_payloads.append(payload)):
                        with redirect_stdout(stdout):
                            exit_code = discord_notify.main()

            self.assertEqual(exit_code, 0)
            self.assertEqual(len(sent_payloads), 1)
            payload = json.loads(stdout.getvalue().split("\nProgress notification sent.", 1)[0])
            embed = payload["embeds"][0]
            self.assertEqual(embed["description"], "P105: commit `abc1234` pushed")
            self.assertEqual(embed["fields"][0]["name"], "Next")
            self.assertEqual(embed["fields"][0]["value"], "Continue with `validate_docs`")

    def test_complete_accepts_summary_file(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            summary_path = root / "summary.txt"
            summary_path.write_text("Done: `940d1c62` stayed literal", encoding="utf-8")

            argv = [
                "discord_notify.py",
                "complete",
                "--cwd",
                str(root),
                "--summary-file",
                str(summary_path),
                "--print-payload",
            ]

            stdout = io.StringIO()
            sent_payloads: list[dict[str, object]] = []
            with mock.patch.object(sys, "argv", argv):
                with mock.patch.dict(
                    os.environ,
                    {"CODEX_DISCORD_WEBHOOK_URL": "https://example.invalid/webhook"},
                    clear=False,
                ):
                    with mock.patch.object(discord_notify, "post_webhook", lambda _url, payload: sent_payloads.append(payload)):
                        with redirect_stdout(stdout):
                            exit_code = discord_notify.main()

            self.assertEqual(exit_code, 0)
            self.assertEqual(len(sent_payloads), 1)
            payload = json.loads(stdout.getvalue().split("\nCompletion notification sent.", 1)[0])
            embed = payload["embeds"][0]
            self.assertEqual(embed["description"], "Done: `940d1c62` stayed literal")


if __name__ == "__main__":
    unittest.main()
