"""Bounded privileged cohort-observer experiment: 60s wall, original CPU/memory limits.
Copied selected native context; only wall bound/explicit Peer module lookup and
receipt metadata differ. It is not a production timeout change or liveness claim.
"""
from pathlib import Path
from contextlib import contextmanager
import os,sys,time,tempfile,subprocess
import publication_process_check as h
digest=h.digest
install_limits=h.install_limits
effective_limits=h.effective_limits
@contextmanager
def native(build, args, name, root, receipt):
    binary = Path(build['binary'])
    if digest(binary.read_bytes()) != build['binary_sha256']:
        raise RuntimeError('native executable identity changed')
    capture = root/name; capture.mkdir()
    process = None
    err = tempfile.TemporaryFile()
    try:
        process = subprocess.Popen([str(binary),*args], stdin=subprocess.PIPE, stdout=subprocess.PIPE,
            stderr=err, preexec_fn=install_limits)
        deadline = time.monotonic()+60
        receipt[name] = dict(binary=str(binary), binary_sha256=build['binary_sha256'], pid=process.pid,
            limits=effective_limits(process.pid), capture=str(capture), state='running',
            numeric_arguments=list(args) if all(str(arg).isdigit() for arg in args) else None)
        receipt[name]['observer_wall_limit_seconds']=60
        yield h.Peer(process, deadline, capture, receipt.setdefault('stream_sequence',[]), name)
        process.stdin.close()
        process.wait(timeout=max(0.001,deadline-time.monotonic()))
        if process.returncode != 0: raise RuntimeError(f'{name} exit {process.returncode}')
        # A successful transcript must account for all child stdout, not just
        # the listed response payloads. Nonblocking read also refuses an open
        # inherited writer instead of assuming EOF after the direct child exits.
        tail = os.read(process.stdout.fileno(), 65537)
        if tail: raise RuntimeError(f'{name} emitted uncaptured trailing stdout')
        receipt[name]['stdout_eof'] = True
    finally:
        primary = sys.exception(); failures = []
        if process is not None:
            # Optional exact-child CPU evidence before wait/reap, including an
            # EOF-producing zombie. Missing proc data is not inferred as zero.
            try:
                if process.returncode is None:
                    fields=Path(f'/proc/{process.pid}/stat').read_text().rsplit(')',1)[1].split()
                    receipt[name]['pre_reap_cpu_seconds']=(int(fields[11])+int(fields[12]))/os.sysconf('SC_CLK_TCK')
                    receipt[name]['pre_reap_process_state']=fields[0]
            except (OSError,ValueError,IndexError,KeyError):pass
            try:
                if process.poll() is None: process.kill()
                process.wait()
            except BaseException as error: failures.append(error)
            for stream in (process.stdin, process.stdout):
                if stream is not None:
                    try: stream.close()
                    except BaseException as error: failures.append(error)
            try:
                if name in receipt:
                    err.seek(0); stderr = err.read(); (capture/'stderr.log').write_bytes(stderr)
                    receipt[name].update(exit=process.returncode, state='reaped' if process.returncode is not None else 'not_reaped', stderr_sha256=digest(stderr))
            except BaseException as error: failures.append(error)
        try: err.close()
        except BaseException as error: failures.append(error)
        if failures:
            prior_cleanup = getattr(primary, 'cleanup_failure', None)
            if prior_cleanup is not None: failures.insert(0, prior_cleanup)
            group = BaseExceptionGroup('native test cleanup/evidence failed', failures)
            group.child_process = process
            if primary is not None:
                primary.cleanup_failure = group
                primary.add_note('Native cleanup/evidence also failed; exact child handle retained in cleanup_failure.')
                raise primary
            raise group
