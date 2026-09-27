"""Observe terminal completion of the three private proof-first evidence stages.

The truthful local observer and stable filesystem are TCB. This is not a signed
attestation. A stage's own successful JSON is insufficient. The caller must also
observe this supervisor's terminal exit; its JSON does not prove its own exit.
V2 retains the executed producer/observer sources. A historical selection checks
those archives and consumed results/logs, not later edits at original locations.
The selected observer must still be trusted; archives do not confer authenticity.
"""
from pathlib import Path
import argparse
import datetime
import hashlib
import json
import os
import subprocess
import sys
import tempfile


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def identity(path):
    path = Path(path).resolve()
    return dict(path=str(path), sha256=sha(path))


def archive_source(path, target):
    """Preserve the exact source used; its original location is provenance."""
    before = identity(path)
    with Path(target).open('xb') as output:
        output.write(Path(path).read_bytes())
    archived = identity(target)
    if identity(path) != before or archived['sha256'] != before['sha256']:
        raise ValueError('source changed while archiving')
    return archived


def observation_inputs(record):
    """Files consumed later, excluding historical executable locations."""
    return {record[k]['path']: record[k]['sha256']
            for k in ['producer_source', 'observer_source', 'result', 'log']}


def consumed_bindings(bindings, provenance_only, retained=()):
    """Exclude only named original sources; any retained role takes priority.

    Directory ancestry never classifies a live artifact as historical source.
    Callers enumerate original source locations and inherited consumption roles.
    All other bindings, including newly produced artifacts, remain live.
    """
    omitted = set(map(str, provenance_only)) - set(map(str, retained))
    if not omitted <= bindings.keys() or not set(map(str, retained)) <= bindings.keys():
        raise ValueError('export role refers to an unbound path')
    return {p: digest for p, digest in bindings.items() if p not in omitted}


def observe(command, cwd, log, observation, producer, locate_result, env=None):
    """Wait for the real producer; preserve failures even if it saved success."""
    log, observation, producer = Path(log), Path(observation), Path(producer).resolve()
    if observation.exists() or log.exists():
        raise ValueError('observation would overwrite prior evidence')
    record = dict(schema='mir-proof-first-process-observation-v2', command=command,
                  cwd=str(Path(cwd).resolve()), producer=identity(producer),
                  interpreter=identity(command[0]), observer=identity(__file__),
                  started=datetime.datetime.now(datetime.timezone.utc).isoformat())
    record['producer_source'] = archive_source(producer, observation.with_suffix('.producer.py'))
    record['observer_source'] = archive_source(__file__, observation.with_suffix('.observer.py'))
    with log.open('xb') as output:
        completed = subprocess.run(command, cwd=cwd, env=env, stdout=output,
                                   stderr=subprocess.STDOUT)
    record.update(exit=completed.returncode, log=identity(log),
                  finished=datetime.datetime.now(datetime.timezone.utc).isoformat())
    try:
        record['result'] = identity(locate_result())
        if identity(producer) != record['producer']:
            raise ValueError('producer changed while executing')
        if identity(command[0]) != record['interpreter']:
            raise ValueError('interpreter changed while executing')
        if identity(__file__) != record['observer']:
            raise ValueError('observer changed while executing')
    except (OSError, ValueError) as error:
        record['error'] = str(error)
    with observation.open('x') as output:
        json.dump(record, output, indent=2)
        output.write('\n')
    return completed.returncode if completed.returncode else (1 if 'error' in record else 0)


def validate_observation(observation, result, producer_sha256):
    """Validate a truthful external exit observation, not a self-reported status."""
    record = json.loads(Path(observation).read_text())
    if (record.get('schema') != 'mir-proof-first-process-observation-v2'
            or type(record.get('exit')) is not int or record['exit'] != 0
            or 'error' in record):
        raise ValueError('producer terminal success not observed')
    if record.get('producer', {}).get('sha256') != producer_sha256:
        raise ValueError('observed producer differs from selected source')
    if record.get('result', {}).get('path') != str(Path(result).resolve()):
        raise ValueError('observation belongs to a different result')
    for key in ['producer_source', 'observer_source', 'result', 'log']:
        row = record[key]
        if identity(row['path']) != row:
            raise ValueError('observed ' + key + ' changed')
    for key in ['producer', 'observer']:
        if record[key]['sha256'] != record[key + '_source']['sha256']:
            raise ValueError('archived ' + key + ' differs from executed source')
    return record


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--stage', required=True, choices=['model', 'prepare', 'physical'])
    parser.add_argument('--work-root', required=True, type=Path)
    parser.add_argument('--model-run', type=Path)
    parser.add_argument('--model-observation', type=Path)
    parser.add_argument('--prepared', type=Path)
    parser.add_argument('--preparation-observation', type=Path)
    args = parser.parse_args()
    scripts = Path(__file__).resolve().parent
    root = args.work_root.resolve()
    if not root.is_dir() or root.is_relative_to(scripts.parent):
        parser.error('existing external work root required')
    if args.stage == 'model':
        producer = scripts / 'proof_first_reference_source_check.py'
        arguments = ['--work-root', str(root), '--with-host-model']
        result_name, result_root = 'RESULT.json', root
    elif args.stage == 'prepare':
        if not args.model_run or not args.model_observation:
            parser.error('preparation needs --model-run and --model-observation')
        producer = scripts / 'proof_first_host_prepare.py'
        arguments = ['--work-root', str(root), '--model-run', str(args.model_run.resolve()),
                     '--model-observation', str(args.model_observation.resolve())]
        result_name, result_root = 'RESULT.json', root
    else:
        if not args.prepared or not args.preparation_observation:
            parser.error('physical execution needs --prepared and --preparation-observation')
        producer = scripts / 'proof_first_host_run.py'
        arguments = ['--prepared', str(args.prepared.resolve()),
                     '--preparation-observation', str(args.preparation_observation.resolve())]
        result_name, result_root = 'HOST_EXECUTION.json', args.prepared.resolve()
    work = Path(tempfile.mkdtemp(prefix='observed-' + args.stage + '-', dir=root))
    print(work, flush=True)
    cache = work / 'empty-cache'
    cache.mkdir()
    command = [sys.executable, '-E', '-B', '-X', 'pycache_prefix=' + str(cache),
               str(producer), *arguments]
    env = {k: v for k, v in os.environ.items() if not k.startswith('PYTHON')}

    def locate_result():
        with (work / 'RUN.log').open() as log:
            selected = Path(log.readline().strip())
        if not selected.is_absolute() or selected.resolve().parent != result_root:
            raise ValueError('stage output did not identify a fresh immediate work directory')
        return selected / result_name

    status = observe(command, scripts.parent, work / 'RUN.log', work / 'OBSERVATION.json',
                     producer, locate_result, env)
    raise SystemExit(status)


if __name__ == '__main__':
    main()
