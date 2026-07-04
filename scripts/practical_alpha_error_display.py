#!/usr/bin/env python3

from __future__ import annotations

import re
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parent.parent
REPO_ROOT_TEXT = REPO_ROOT.as_posix()
REPO_ROOT_WITH_SLASH_TOKEN_PATTERN = re.compile(
    rf"(?<![\w./-]){re.escape(REPO_ROOT_TEXT)}/(?=$|[^\w./-])"
)
REPO_ROOT_PREFIX_PATTERN = re.compile(
    rf"(?<![\w./-]){re.escape(REPO_ROOT_TEXT)}/"
)
REPO_ROOT_BOUNDARY_PATTERN = re.compile(
    rf"(?<![\w./-]){re.escape(REPO_ROOT_TEXT)}(?=$|[^\w./-])"
)


def repo_display_text(value: Any) -> str:
    text = str(value)
    if text == REPO_ROOT_TEXT or text == f"{REPO_ROOT_TEXT}/":
        return "."
    text = REPO_ROOT_WITH_SLASH_TOKEN_PATTERN.sub(".", text)
    text = REPO_ROOT_PREFIX_PATTERN.sub("", text)
    return REPO_ROOT_BOUNDARY_PATTERN.sub(".", text)


def failure_error_text(error: BaseException | Any) -> str:
    return repo_display_text(error)
