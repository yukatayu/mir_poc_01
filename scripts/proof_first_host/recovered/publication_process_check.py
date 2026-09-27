"""Four native processes with source-generated publication and actual endpoint acks.

This bounded component forwards only actual source-generated image/ticket and
actual owner result. The Python tree reader is test plumbing, not a proved
production decoder or an authenticated QUIC bridge. Captures are privileged
local data; this is not a public/passive observer or recovery demonstration.
"""
from contextlib import contextmanager, ExitStack
from pathlib import Path
import datetime, hashlib, json, os, selectors, subprocess, sys, tempfile, time
from owner_process_limits import install_limits, effective_limits

work = Path(__file__).resolve().parent

def require(condition, message='native experiment check failed'):
    if not condition: raise AssertionError(message)

def digest(data): return hashlib.sha256(data).hexdigest()
def nat(n): return ('nat', n)
def integer(n): return ('int', n)
def node(tag, *fields): return ('node', tag, tuple(fields))
def left(value): return node(0, value)
def right(value): return node(1, value)
def pair(a, b): return node(0, a, b)
unit = node(0)

def write_nat(n):
    out = bytearray()
    while n:
        out.append(1 + n % 2)
        n //= 2
    out.append(0)
    return bytes(out)

def encode(tree):
    kind = tree[0]
    if kind == 'nat': return b'\x03' + write_nat(tree[1])
    if kind == 'int': return bytes([4 if tree[1] >= 0 else 5]) + write_nat(tree[1] if tree[1] >= 0 else -tree[1]-1)
    if kind == 'text': return b'\x06' + write_nat(len(tree[1])) + b''.join(write_nat(ord(c)) for c in tree[1])
    if kind == 'node': return b'\x07' + write_nat(tree[1]) + write_nat(len(tree[2])) + b''.join(map(encode, tree[2]))
    raise ValueError('bad tree kind')

def decode(data):
    pos = 0
    def read_nat():
        nonlocal pos
        value, shift = 0, 0
        while True:
            if pos >= len(data): raise ValueError('short number')
            digit = data[pos]; pos += 1
            if digit == 0: break
            if digit not in (1, 2) or shift >= 128: raise ValueError('bad number')
            value += (digit - 1) << shift; shift += 1
        if shift and value < 1 << (shift-1): raise ValueError('noncanonical number')
        return value
    def read_tree(depth):
        nonlocal pos
        if depth > 256 or pos >= len(data): raise ValueError('bad tree')
        tag = data[pos]; pos += 1
        if tag == 3: return nat(read_nat())
        if tag in (4, 5):
            n = read_nat(); return integer(n if tag == 4 else -n-1)
        if tag == 6:
            count = read_nat()
            if count > 4096: raise ValueError('text bound')
            return ('text', ''.join(chr(read_nat()) for _ in range(count)))
        if tag == 7:
            kind, count = read_nat(), read_nat()
            if count > 256: raise ValueError('field bound')
            return node(kind, *(read_tree(depth+1) for _ in range(count)))
        raise ValueError('unknown tree tag')
    result = read_tree(0)
    if pos != len(data) or encode(result) != data: raise ValueError('tree tail/canonicality')
    return result

def product_fields(tree, count):
    fields = []
    for _ in range(count-1):
        require(tree[0:2] == ('node', 0) and len(tree[2]) == 2, 'native experiment check failed')
        first, tree = tree[2]; fields.append(first)
    return [*fields, tree]

def un_sum(tree):
    require(tree[0] == 'node' and tree[1] in (0, 1) and len(tree[2]) == 1, 'native experiment check failed')
    return tree[1], tree[2][0]

def un_bool(tree):
    kind, value = un_sum(tree); require(value == unit, 'native experiment check failed')
    return bool(kind)

def un_option(tree):
    kind, value = un_sum(tree)
    if kind == 0:
        require(value == unit, 'native experiment check failed')
        return None
    return value

def source_reply(data):
    accepted, output = product_fields(decode(data), 2)
    output = un_option(output)
    if output is None: return un_bool(accepted), None
    status, completed, remaining, writes, replies, values, pending, image = product_fields(output, 8)
    status_tag, status_value = un_sum(status)
    phase = 'failed' if status_tag else ('waiting' if un_bool(status_value) else 'ready')
    for value in (completed, remaining, writes, replies): require(value[0] == 'nat', 'native experiment check failed')
    return un_bool(accepted), dict(phase=phase, raw_status=status, completed=completed[1], remaining=remaining[1],
        writes=writes[1], replies=replies[1], values=values, pending=un_option(pending), image=image)

class Peer:
    def __init__(self, process, deadline, capture, sequence, endpoint):
        self.process, self.deadline, self.capture = process, deadline, capture
        self.sequence, self.endpoint = sequence, endpoint
        self.ordinal = 0
        os.set_blocking(process.stdin.fileno(), False)
        os.set_blocking(process.stdout.fileno(), False)
    def wait_fd(self, fd, events):
        remaining = self.deadline-time.monotonic()
        if remaining <= 0: raise TimeoutError('native component wall limit')
        with selectors.DefaultSelector() as selector:
            selector.register(fd, events)
            if not selector.select(remaining): raise TimeoutError('native IO wall limit')
    def read(self, count):
        data = bytearray()
        while len(data) < count:
            self.wait_fd(self.process.stdout, selectors.EVENT_READ)
            try: chunk = os.read(self.process.stdout.fileno(), count-len(data))
            except BlockingIOError: continue
            if not chunk: raise EOFError('native output ended')
            data += chunk
        return bytes(data)
    def send(self, payload, expect_reply=True):
        if type(payload) is not bytes: raise TypeError('immutable bytes payload required')
        require(len(payload) <= 65536, 'native experiment check failed')
        original = payload
        frame = len(original).to_bytes(4,'big')+original
        data = memoryview(frame)
        while data:
            self.wait_fd(self.process.stdin, selectors.EVENT_WRITE)
            try: count = os.write(self.process.stdin.fileno(), data)
            except BlockingIOError: continue
            data = data[count:]
        self.ordinal += 1
        (self.capture/f'{self.ordinal:03}-input.bin').write_bytes(original)
        (self.capture/f'{self.ordinal:03}-input-frame.bin').write_bytes(frame)
        if not expect_reply:
            self.sequence.append(dict(endpoint=self.endpoint,index=self.ordinal,reply=False))
            return None
        header = self.read(4)
        size = int.from_bytes(header, 'big')
        if size > 65536: raise ValueError('native reply bound')
        reply = self.read(size)
        (self.capture/f'{self.ordinal:03}-output.bin').write_bytes(reply)
        (self.capture/f'{self.ordinal:03}-output-frame.bin').write_bytes(header+reply)
        self.sequence.append(dict(endpoint=self.endpoint,index=self.ordinal,reply=True))
        return reply

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
        deadline = time.monotonic()+15
        receipt[name] = dict(binary=str(binary), binary_sha256=build['binary_sha256'], pid=process.pid,
            limits=effective_limits(process.pid), capture=str(capture), state='running',
            numeric_arguments=list(args) if all(str(arg).isdigit() for arg in args) else None)
        yield Peer(process, deadline, capture, receipt.setdefault('stream_sequence',[]), name)
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


def save_receipt(path, receipt):
    # This runs from experiment finally blocks. Evidence failure must not erase
    # a primary error carrying an unreaped process handle.
    primary = sys.exception()
    try:
        path.write_text(json.dumps(receipt,indent=2)+'\n')
        # The committed receipt is the final result. Diagnostic stdout is not
        # part of finalization; callers may separately report this file.
    except BaseException as error:
        if primary is None: raise
        previous = getattr(primary, 'evidence_failure', None)
        primary.evidence_failure = error if previous is None else BaseExceptionGroup(
            'native evidence writes failed', [previous, error])
        primary.add_note('Writing the final evidence receipt also failed.')


def nested_right(value, count):
    for _ in range(count): value=right(value)
    return value

def publisher_reply(data):
    accepted, output=product_fields(decode(data),2);output=un_option(output)
    if output is None:return un_bool(accepted),None
    source,published,announced,installed,fence,ack,dispatched,held=product_fields(output,8)
    _,source=source_reply(encode(pair(right(unit),right(source))))
    return un_bool(accepted),dict(source=source,published=published[1],announced=announced[1],
        installed=installed,fence=fence,ack=ack,dispatch=un_option(dispatched),held=un_option(held))

def capacity_reply(data):
    status,output=product_fields(decode(data),2)
    require(status in [nat(0),nat(1),nat(2)],'unknown private capacity status')
    _,snapshot=publisher_reply(encode(pair(right(unit) if status==nat(0) else left(unit),output)))
    return status[1],snapshot

def main(continuation_root=None, result_name="PUBLICATION_PROCESS_CHECK.json", owner_build_name="ENDPOINT_NATIVE_BUILD.json", source_build_name="PUBLICATION_NATIVE_BUILD.json", source_capacity=None, cohort_factory=None):
    continuation_root=Path(continuation_root) if continuation_root is not None else work/"source-continuation"
    source_build=json.loads((work/source_build_name).read_text())
    owner_build=json.loads((work/owner_build_name).read_text())
    root=Path(tempfile.mkdtemp(prefix='publication-process-',dir=work))
    receipt=dict(started=datetime.datetime.now(datetime.timezone.utc).isoformat(),root=str(root),state='running',
        scope='actual publisher and three owner endpoints over trusted private pipes; no QUIC, malicious supervisor resistance, exclusive physical scope or durable recovery claim',events=[])
    def read_publisher(data):
        if source_capacity is None:return publisher_reply(data)
        status,snapshot=capacity_reply(data)
        require(status!=2,'useful normal trace unexpectedly profile-refused')
        return status==0,snapshot
    source_args=['91','3','1','0','0','7','811']
    if source_capacity is not None:source_args.append(str(source_capacity))
    receipt['publisher_reply_codec']='legacy_boolean' if source_capacity is None else 'typed_capacity_status'
    try:
        with ExitStack() as stack:
            owner_args=[['91','3','1',str(i),'811','8'] for i in range(3)]
            if cohort_factory is None:
                source=stack.enter_context(native(source_build,source_args,'source',root,receipt))
                owners=[stack.enter_context(native(owner_build,args,f'owner{i}',root,receipt)) for i,args in enumerate(owner_args)]
            else:
                source,owners=stack.enter_context(cohort_factory(source_build,source_args,owner_build,owner_args,root,receipt))
            source.send((work/'source-bootstrap-actual.bin').read_bytes(),False)
            accepted,snapshot=read_publisher(source.send((work/'source-launch-actual.bin').read_bytes()))
            require(accepted and snapshot['source']['writes']==0, 'native experiment check failed')
            for owner in owners:
                require(decode(owner.send(encode(left(left(snapshot['source']['image'])))))==left(nat(10)), 'native experiment check failed')
            def event(command, accepted=True):
                nonlocal snapshot
                before=snapshot
                yes,snapshot=read_publisher(source.send(encode(right(command))))
                require(yes==accepted, (command,yes,snapshot['published']))
                if not accepted: require(snapshot==before, 'native experiment check failed')
                return snapshot
            def owner_command(index,command,code):
                response=decode(owners[index].send(encode(left(command))))
                require(response==left(nat(code)), (index,response,code))
            def publish_staged():
                revision=snapshot['announced'];require(revision==snapshot['published']+1, 'native experiment check failed')
                for i in range(3):
                    actual=decode(owners[i].send(encode(right(nat(revision)))))
                    require(actual==left(nat(12)), 'native experiment check failed')
                    event(nested_right(left(pair(nat(i),nat(revision))),2))
                    if source_capacity is None:event(nested_right(left(pair(nat(i),nat(revision))),3))
                    receipt['events'].append(dict(kind='actual_freeze_ack',endpoint=i,revision=revision))
                if source_capacity is not None:
                    for i in range(3):event(nested_right(left(pair(nat(i),nat(revision))),3))
                event(nested_right(left(unit),4))
                require(snapshot['published']==revision, 'native experiment check failed')
                for i in range(3):
                    install=nested_right(left(pair(nat(revision),snapshot['source']['image'])),3)
                    owner_command(i,install,7)
                    event(nested_right(left(pair(nat(i),nat(revision))),5))
                    # Retained old publication revision is rejected by the publisher.
                    event(nested_right(left(pair(nat(i),nat(revision-1))),5),False)
            def stage(command):
                event(left(command));publish_staged()
            replies=[]
            owner_ordinals=[0,0,0]
            def drive():
                nonlocal snapshot
                while snapshot['source']['remaining']:
                    stage(left(unit))
                    state=snapshot['source']
                    if state['phase']=='waiting':
                        event(left(left(unit)),False)
                        ticket=state['pending'];require(ticket is not None, 'native experiment check failed')
                        ticket_place=product_fields(ticket,15)[3]
                        require(ticket_place[0]=='nat' and 0<=ticket_place[1]<len(owners), 'native experiment check failed')
                        target=ticket_place[1]
                        if cohort_factory is None:
                            event(nested_right(left(nat(target)),6))
                            endpoint,scope,revision,retained_ticket=product_fields(snapshot['dispatch'],4)
                            require(endpoint==nat(target) and scope==nat(811) and revision==nat(snapshot['published']) and retained_ticket==ticket, 'native experiment check failed')
                            require(snapshot['held']==snapshot['source']['image'], 'native experiment check failed')
                            owner_command(target,right(left(ticket)),6)
                            require(decode(owners[target].send(encode(right(nat(snapshot['published']+1)))))==left(nat(2)), 'native experiment check failed')
                            response=decode(owners[target].send(encode(left(right(right(left(unit)))))))
                        else:
                            terminal,actual_reply=source.invoke_pending()
                            accepted,snapshot=read_publisher(terminal);require(accepted)
                            endpoint,scope,revision,retained_ticket=product_fields(snapshot['dispatch'],4)
                            require(endpoint==nat(target) and retained_ticket==ticket)
                            response=decode(actual_reply)
                        kind,envelope=un_sum(response);require(kind==1, 'native experiment check failed')
                        got_scope,ordinal,got_revision,got_ticket,result=product_fields(envelope,5)
                        require(got_scope==scope and ordinal==nat(owner_ordinals[target]) and got_revision==revision and got_ticket==ticket, 'native experiment check failed')
                        kind,value=un_sum(result);require(kind==1 and value[0]=='int', 'native experiment check failed')
                        if cohort_factory is None:event(nested_right(right(nat(target)),6))
                        # An actual matching payload cannot bypass retained envelope admission.
                        event(left(right(left(pair(ticket,value)))),False)
                        bad=pair(nat(812),pair(ordinal,pair(got_revision,pair(ticket,result))))
                        if cohort_factory is None:event(right(left(bad)),False)
                        event(right(left(envelope)))
                        publish_staged()
                        event(right(left(envelope)),False)
                        replies.append(value[1]);owner_ordinals[target]+=1
                        receipt['events'].append(dict(kind='actual_owner_result',endpoint=target,ordinal=ordinal[1],revision=revision[1],value=value[1]))
            drive()
            require(snapshot['source']['completed']==17 and snapshot['source']['writes']==17, 'native experiment check failed')
            continuation=decode((continuation_root/'continue.bin').read_bytes())
            tag,command=un_sum(continuation);require(tag==1, 'native experiment check failed')
            stage(command);drive()
            require(replies==[10,10,11,10,10], 'native experiment check failed')
            require(snapshot['source']['writes']==20 and snapshot['source']['completed']==3 and snapshot['published']==26, 'native experiment check failed')
            require(snapshot['dispatch'] is None and snapshot['held'] is None, 'native experiment check failed')
            # Fresh source launch cannot reset retained live process state.
            yes,unchanged=read_publisher(source.send((work/'source-launch-actual.bin').read_bytes()))
            require(not yes and unchanged==snapshot, 'native experiment check failed')
            receipt['final']={k:snapshot['source'][k] for k in ['phase','completed','remaining','writes','replies']}
            receipt['publications']=snapshot['published'];receipt['owner_values']=replies;receipt['owner_ordinals']=owner_ordinals
            receipt['inputs']={name:digest((work/name).read_bytes()) for name in
                ['source-bootstrap-actual.bin','source-launch-actual.bin','qualified-source/main.mir','publication_process_check.py']}
            receipt['continuation']={name:dict(path=str(continuation_root/name),sha256=digest((continuation_root/name).read_bytes())) for name in ['main.mir','continue.bin']}
        receipt['state']='passed'
    except BaseException as error:
        receipt['state']='failed';receipt['error']=type(error).__name__+': '+str(error)
        raise
    finally:
        receipt['finished']=datetime.datetime.now(datetime.timezone.utc).isoformat()
        save_receipt(work/result_name, receipt)

if __name__=='__main__':main()
