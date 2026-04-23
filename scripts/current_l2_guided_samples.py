#!/usr/bin/env python3

from __future__ import annotations

import sys

import clean_near_end_samples


MIGRATION_NOTE = (
    "current_l2_guided_samples.py now forwards to the clean near-end active suite. "
    "Legacy problem1/problem2 bundle commands were retired with the 2026-04-22 clean-sample migration."
)


def main(argv: list[str] | None = None) -> int:
    args = list(sys.argv[1:] if argv is None else argv)
    if not args:
        print(MIGRATION_NOTE)
        return 0
    if args[0] not in {"list", "smoke-all", "closeout"}:
        print(MIGRATION_NOTE, file=sys.stderr)
        print(
            "supported compatibility commands: list, smoke-all, closeout",
            file=sys.stderr,
        )
        return 2
    return clean_near_end_samples.main(args)


if __name__ == "__main__":
    raise SystemExit(main())
