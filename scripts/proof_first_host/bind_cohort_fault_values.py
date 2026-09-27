"""Bind complete observed cohort values to captured native bytes.
Content keys are only filenames; the Lean consumer decodes and compares values.
This known-entry fault cut binds every observed snapshot to both its captured
host content and actual native bytes; a returned but unobserved future snapshot
need not already have been written to host memory. Payment still needs an
actual source command; unknown pending-payment tails remain outside this cut.
"""
from pathlib import Path
import hashlib,json
from check_joint_capture_recipes import h,source_form

def bind(receipt,trees):
    root=Path(receipt['root']);snapshots={};commands={};envelopes={};files={}
    def digest(data):return hashlib.sha256(data).hexdigest()
    def canonical(value):return json.dumps(value,sort_keys=True,separators=(',',':')).encode()
    def retain(table,key,data):
        if key in table:assert table[key]==data,'ambiguous captured content identity'
        table[key]=data
    for event in receipt['stream_sequence']:
        endpoint=event['endpoint'];ordinal=event['index']
        if not event['reply']:continue
        raw=(root/endpoint/f'{ordinal:03}-output.bin').read_bytes()
        if endpoint=='source':
            kind,value=source_form(h.decode((root/endpoint/f'{ordinal:03}-input.bin').read_bytes()))
            if kind!='execute':continue
            _,command=value;encoded=h.encode(command);retain(commands,digest(encoded),encoded)
            _,snapshot=h.capacity_reply(raw);key=digest(canonical(snapshot))
            if key in receipt['host_source_snapshots']:
                assert canonical(snapshot)==canonical(receipt['host_source_snapshots'][key]),'captured snapshot content differs from native bytes'
            retain(snapshots,key,h.encode(h.product_fields(h.decode(raw),2)[1]))
        else:
            kind,value=h.un_sum(h.decode(raw))
            if kind==1:
                encoded=h.encode(value);retain(envelopes,digest(encoded),encoded)
    def file(prefix,key,table):
        assert type(key) is str and key in table,'observed cohort value lacks captured bytes'
        name=prefix+key;path=trees/(name+'.bin');data=table[key]
        if path.exists():assert path.read_bytes()==data,'different value at retained filename'
        else:
            with path.open('xb') as output:output.write(data)
        files[str(path)]=dict(path=str(path),sha256=digest(data))
        return json.dumps(name)
    def nat(value):assert type(value) is int and value>=0;return str(value)
    def boolean(value):assert type(value) is bool;return str(value).lower()
    def pair(value):assert type(value) is list and len(value)==2;return '('+nat(value[0])+','+nat(value[1])+')'
    def array(value,show):assert type(value) is list;return '['+','.join(map(show,value))+']'
    def observation(state):
        if state['source_snapshot'] is not None:
            assert state['source_snapshot'] in receipt['host_source_snapshots'],'observed snapshot missing actual host content'
        snapshot='none' if state['source_snapshot'] is None else '(some '+file('source-snapshot-',state['source_snapshot'],snapshots)+')'
        pending=state['pending']
        if pending is None:payment='none'
        else:
            assert type(pending) is dict and set(pending)=={'command','ordinal'}
            payment='(some ('+file('cohort-command-',pending['command'],commands)+','+nat(pending['ordinal'])+'))'
        return '⟨'+','.join([boolean(state['bootstrapped']),snapshot,array(state['initialized'],nat),
            array(state['freezes'],pair),array(state['installs'],pair),
            array(state['produced'],lambda key:file('cohort-envelope-',key,envelopes)),payment,
            boolean(state['gate']),boolean(state['retired']),nat(state['ordinal'])])+'⟩'
    shown={row['sequence']:observation(row['state']) for row in receipt['host_commit_trace']}
    return shown,list(files.values())
