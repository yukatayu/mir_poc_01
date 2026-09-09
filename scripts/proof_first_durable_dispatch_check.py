#!/usr/bin/env python3
"""Nonproduction Linux process-crash experiment, NOT a Mir runtime or E2E.

Fixed stable private files, cooperative flock users, honest local storage, no
rollback/replacement and no external child outliving its owner are assumptions.
SIGKILL tests do not simulate power loss, prove fsync, or authenticate recovery.
The effect sink records actual callback starts; it is never consulted for retry.
"""
import argparse
import fcntl
import json
import os
from pathlib import Path
import select
import signal
import subprocess
import sys
import tempfile

MAX_BYTES = 4096
MAX_RECORDS = 16


def write_all(fd, data):
    while data:
        n = os.write(fd, data)
        if n <= 0:
            raise OSError("zero write")
        data = data[n:]


def encoded(record):
    return (json.dumps(record, sort_keys=True, separators=(",", ":")) + "\n").encode()


def read_bounded(fd):
    data = bytearray()
    while len(data) <= MAX_BYTES:
        chunk = os.read(fd, MAX_BYTES + 1 - len(data))
        if not chunk:
            break
        data.extend(chunk)
    return bytes(data)


def unique_fields(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise ValueError("duplicate object field")
        result[key] = value
    return result


def checkpoint(wanted, stage):
    if wanted == stage:
        print(json.dumps({"checkpoint": stage}), flush=True)
        os.kill(os.getpid(), signal.SIGSTOP)


def decode(data):
    if len(data) > MAX_BYTES or (data and not data.endswith(b"\n")):
        raise ValueError("invalid journal extent")
    records = [json.loads(line, object_pairs_hook=unique_fields) for line in data.splitlines()]
    if len(records) > MAX_RECORDS:
        raise ValueError("journal record limit")
    keys = set()
    for r in records:
        if not isinstance(r, dict) or set(r) != {"key", "argument"}:
            raise ValueError("invalid record schema")
        if (not isinstance(r["key"], list) or len(r["key"]) != 3
                or any(type(v) is not int or not 0 <= v < 2**64 for v in r["key"])
                or type(r["argument"]) is not int or not 0 <= r["argument"] < 2**32):
            raise ValueError("invalid record value")
        key = tuple(r["key"])
        if key in keys:
            raise ValueError("duplicate journal key")
        keys.add(key)
    return records


def child(directory, request, argument, stop, unsafe):
    record = {"key": [1, 2, request], "argument": argument}
    blob = encoded(record)
    decode(blob)  # Input bounds before any mutation.
    # No O_CREAT on recovery, no replacement, no recovery from an arbitrary image.
    try:
        fd = os.open(directory / "journal", os.O_RDWR | os.O_APPEND | os.O_NOFOLLOW)
    except FileNotFoundError:
        return "missing-history"
    try:
        if unsafe != "no-lock":
            fcntl.flock(fd, fcntl.LOCK_EX)
        records = decode(read_bounded(fd))
        prior = next((r for r in records if r["key"] == record["key"]), None)
        if prior is not None:
            return "retained" if prior == record else "binding-conflict"
        if len(records) == MAX_RECORDS:
            return "capacity"
        checkpoint(stop, "before-write")
        if unsafe != "effect-first":
            cut = len(blob) // 2
            write_all(fd, blob[:cut])
            checkpoint(stop, "partial-write")
            write_all(fd, blob[cut:])
            checkpoint(stop, "written")
            os.fsync(fd)
            checkpoint(stop, "synced")
        # Actual local effect; no expected result or model supplies this record.
        effect_fd = os.open(directory / "effects", os.O_WRONLY | os.O_APPEND | os.O_NOFOLLOW)
        try:
            write_all(effect_fd, encoded({**record, "computed": argument * argument + 1}))
            os.fsync(effect_fd)
        finally:
            os.close(effect_fd)
        checkpoint(stop, "effect")
        if unsafe == "effect-first":
            write_all(fd, blob)
            os.fsync(fd)
        return "started"
    finally:
        os.close(fd)


def initialize(path):
    path.mkdir(mode=0o700)
    for name in ("journal", "effects"):
        fd = os.open(path / name, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
        try:
            os.fsync(fd)
        finally:
            os.close(fd)
    fd = os.open(path, os.O_RDONLY | os.O_DIRECTORY)
    try:
        os.fsync(fd)
    finally:
        os.close(fd)


def spawn(path, request=3, argument=4, stop="none", unsafe="none"):
    return subprocess.Popen(
        [sys.executable, __file__, "child", str(path), "--request", str(request),
         "--argument", str(argument), "--stop", stop, "--unsafe", unsafe],
        stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)


def collect(p):
    try:
        out, err = p.communicate(timeout=15)
        assert p.returncode == 0, (p.returncode, err)
        return json.loads(out)["outcome"]
    finally:
        if p.poll() is None:
            p.kill()
            p.wait()


def stopped(p, stage):
    assert select.select([p.stdout], [], [], 15)[0], "checkpoint timeout"
    assert json.loads(p.stdout.readline()) == {"checkpoint": stage}
    pid, status = os.waitpid(p.pid, os.WUNTRACED)
    assert pid == p.pid and os.WIFSTOPPED(status)


def effects(path):
    return [json.loads(line) for line in (path / "effects").read_text().splitlines()]


def test(work):
    root = Path(tempfile.mkdtemp(prefix="physical-cases-", dir=work))
    results = []
    def case(name):
        p = root / name
        initialize(p)
        return p
    def note(name, path, **details):
        results.append({"case": name, "effects": effects(path), **details})

    p = case("normal")
    assert collect(spawn(p)) == "started"
    assert collect(spawn(p)) == "retained"
    assert collect(spawn(p, argument=5)) == "binding-conflict"
    assert collect(spawn(p, request=4)) == "started"
    assert [x["computed"] for x in effects(p)] == [17, 17]
    note("normal", p)

    for stage, expect, count in [
        ("before-write", "started", 1), ("partial-write", "invalid", 0),
        ("written", "retained", 0), ("synced", "retained", 0),
        ("effect", "retained", 1),
    ]:
        p = case(stage)
        proc = spawn(p, stop=stage)
        try:
            stopped(proc, stage)
        finally:
            proc.kill()
            proc.wait()
        assert proc.returncode == -signal.SIGKILL
        assert collect(spawn(p)) == expect
        assert len(effects(p)) == count
        note(stage, p, retry=expect, child_returncode=proc.returncode)

    p = case("concurrent")
    first, second = spawn(p), spawn(p)
    outcomes = sorted([collect(first), collect(second)])
    assert outcomes == ["retained", "started"] and len(effects(p)) == 1
    note("concurrent", p, outcomes=outcomes)

    p = case("effect-first-counterexample")
    proc = spawn(p, stop="effect", unsafe="effect-first")
    try:
        stopped(proc, "effect")
    finally:
        proc.kill()
        proc.wait()
    assert collect(spawn(p)) == "started" and len(effects(p)) == 2
    note("effect-first-counterexample", p)

    p = case("rollback-counterexample")
    assert collect(spawn(p)) == "started"
    # Deliberately replace only this test-owned journal with its old empty bytes.
    with (p / "journal").open("wb") as f:
        f.flush()
        os.fsync(f.fileno())
    assert collect(spawn(p)) == "started" and len(effects(p)) == 2
    note("rollback-counterexample", p)

    p = case("no-lock-counterexample")
    pair = [spawn(p, stop="before-write", unsafe="no-lock") for _ in range(2)]
    try:
        for proc in pair:
            stopped(proc, "before-write")
        for proc in pair:
            os.kill(proc.pid, signal.SIGCONT)
        assert [collect(proc) for proc in pair] == ["started", "started"]
    finally:
        for proc in pair:
            if proc.poll() is None:
                proc.kill()
                proc.wait()
    assert len(effects(p)) == 2
    note("no-lock-counterexample", p)

    p = case("capacity")
    for key in range(MAX_RECORDS):
        assert collect(spawn(p, request=key)) == "started"
    assert collect(spawn(p, request=MAX_RECORDS)) == "capacity"
    assert len(effects(p)) == MAX_RECORDS
    note("capacity", p)

    p = case("missing-journal")
    # Test-owned artifact only: recovery must not silently recreate lost history.
    (p / "journal").unlink()
    assert collect(spawn(p)) == "missing-history"
    assert not (p / "journal").exists() and not effects(p)
    note("missing-journal", p)

    bad_images = {
        "duplicate-field": b'{"key":[1,2,3],"key":[1,2,4],"argument":4}\n',
        "boolean-id": b'{"key":[1,2,true],"argument":4}\n',
        "unknown-schema": b'{"key":[1,2,3],"argument":4,"extra":0}\n',
        "oversized": b" " * (MAX_BYTES + 1),
        "deep-json": b'{"key":' + b"[" * 1200 + b"0" + b"]" * 1200 + b',"argument":4}\n',
    }
    for name, data in bad_images.items():
        p = case(name)
        (p / "journal").write_bytes(data)
        assert collect(spawn(p)) == "invalid" and not effects(p)
        note(name, p)

    p = case("effect-sync-error")
    assert collect(spawn(p, unsafe="effect-sync-error")) == "failed-unknown"
    assert len(effects(p)) == 1 and collect(spawn(p)) == "retained"
    note("effect-sync-error", p)

    report = {"scope": "local cooperative Linux process SIGKILL; NOT power loss, auth or Mir E2E",
              "case_root": str(root), "cases": results}
    (root / "RESULT.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report, indent=2))


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("mode", choices=["child", "test"])
    parser.add_argument("directory", type=Path)
    parser.add_argument("--request", type=int, default=3)
    parser.add_argument("--argument", type=int, default=4)
    parser.add_argument("--stop", default="none")
    parser.add_argument("--unsafe", choices=["none", "effect-first", "no-lock", "effect-sync-error"], default="none")
    args = parser.parse_args()
    if args.mode == "test":
        test(args.directory)
    else:
        if args.unsafe == "effect-sync-error":
            # Fault injection in this test process, after actual effect write.
            real_sync = os.fsync
            sync_calls = 0
            def fail_second_sync(fd):
                global sync_calls
                sync_calls += 1
                if sync_calls == 2:
                    raise OSError("injected effect synchronization failure")
                return real_sync(fd)
            os.fsync = fail_second_sync
        try:
            result = child(args.directory, args.request, args.argument, args.stop, args.unsafe)
        except (ValueError, UnicodeError):
            result = "invalid"
        except OSError:
            # A write/fsync failure may occur after the callback has started.
            result = "failed-unknown"
        print(json.dumps({"outcome": result}), flush=True)
