"""Private one-request experiment's shared launcher contract, not Oracle limits."""
import hashlib
from pathlib import Path
import resource
import subprocess

MEMORY_BYTES = 4 * 1024**3
CPU_SECONDS = 10
WALL_SECONDS = 15


def install_limits():
    resource.setrlimit(resource.RLIMIT_AS, (MEMORY_BYTES, MEMORY_BYTES))
    resource.setrlimit(resource.RLIMIT_CPU, (CPU_SECONDS, CPU_SECONDS))
    resource.setrlimit(resource.RLIMIT_CORE, (0, 0))


def effective_limits(pid):
    selected = ('Max cpu time', 'Max address space', 'Max core file size')
    return [line for line in Path(f'/proc/{pid}/limits').read_text().splitlines()
            if line.startswith(selected)]


def run_worker(binary, assigned, data, expected_hash, wall_seconds=WALL_SECONDS, hold_open=False):
    binary = Path(binary)
    if hashlib.sha256(binary.read_bytes()).hexdigest() != expected_hash:
        raise RuntimeError('Native executable identity changed')
    process = subprocess.Popen([str(binary), *map(str, assigned)], stdin=subprocess.PIPE,
                               stdout=subprocess.PIPE, stderr=subprocess.PIPE, preexec_fn=install_limits)
    limits = effective_limits(process.pid)
    classification = 'exited'
    try:
        if hold_open:
            process.stdin.write(data)
            process.stdin.flush()
            # Deliberately keep the writer open to test the reclamation path.
            process.wait(timeout=wall_seconds)
            stdout, stderr = process.communicate()
        else:
            stdout, stderr = process.communicate(data, timeout=wall_seconds)
    except subprocess.TimeoutExpired:
        classification = 'wall_resource_failure'
        process.kill()
        stdout, stderr = process.communicate()
    return {'pid': process.pid, 'exit': process.returncode, 'classification': classification,
            'effective_limits': limits, 'wall_seconds': wall_seconds,
            'stdout': stdout, 'stderr': stderr}
