#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from pathlib import Path


SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parent
RUNTIME_PREFIX = [
    "cargo",
    "run",
    "-q",
    "-p",
    "mir-runtime",
    "--bin",
    "mir-clean-near-end",
    "--",
]


def runtime_command(*parts: str, fmt: str = "pretty") -> list[str]:
    return [*RUNTIME_PREFIX, *parts, "--format", fmt]


def run_runtime(*parts: str, fmt: str = "pretty") -> str:
    completed = subprocess.run(
        runtime_command(*parts, fmt=fmt),
        cwd=REPO_ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    return completed.stdout.rstrip()


def smoke_all(fmt: str) -> str:
    payload = {
        "list": json.loads(run_runtime("list", fmt="json")),
        "typing": json.loads(run_runtime("run-family", "typing", fmt="json")),
        "order_handoff": json.loads(run_runtime("run-family", "order-handoff", fmt="json")),
        "model_check": json.loads(run_runtime("run-family", "model-check", fmt="json")),
        "modal": json.loads(run_runtime("run-family", "modal", fmt="json")),
        "matrix": json.loads(run_runtime("matrix", fmt="json")),
        "closeout": json.loads(run_runtime("closeout", fmt="json")),
    }
    if fmt == "json":
        return json.dumps(payload, indent=2, ensure_ascii=False)
    return json.dumps(payload, indent=2, ensure_ascii=False)


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser()
    sub = parser.add_subparsers(dest="command", required=True)

    list_parser = sub.add_parser("list")
    list_parser.add_argument("--format", choices=["pretty", "json"], default="pretty")

    run = sub.add_parser("run")
    run.add_argument("family", choices=["typing", "order-handoff", "model-check", "modal"])
    run.add_argument("--format", choices=["pretty", "json"], default="pretty")

    matrix = sub.add_parser("matrix")
    matrix.add_argument("--format", choices=["pretty", "json"], default="pretty")

    closeout = sub.add_parser("closeout")
    closeout.add_argument("--format", choices=["pretty", "json"], default="pretty")

    smoke = sub.add_parser("smoke-all")
    smoke.add_argument("--format", choices=["pretty", "json"], default="pretty")
    return parser


def main(argv: list[str] | None = None) -> int:
    args = build_parser().parse_args(argv)
    if args.command == "list":
        print(run_runtime("list", fmt=args.format))
    elif args.command == "run":
        print(run_runtime("run-family", args.family, fmt=args.format))
    elif args.command == "matrix":
        print(run_runtime("matrix", fmt=args.format))
    elif args.command == "closeout":
        print(run_runtime("closeout", fmt=args.format))
    elif args.command == "smoke-all":
        print(smoke_all(args.format))
    else:
        raise AssertionError(f"unsupported command {args.command}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
