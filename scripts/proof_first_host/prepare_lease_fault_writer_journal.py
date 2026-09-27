"""Actual host-field/byte journal binder, never model-generated expected states."""
from pathlib import Path
import sys,os
if sys.flags.optimize or not __debug__:
    raise RuntimeError('evidence preparer requires Python assertions')
import argparse,datetime,hashlib,json,re
w=Path(__file__).resolve().parent;rec=w/'recovered';sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
parser=argparse.ArgumentParser();parser.add_argument('tag',nargs='?',default='');parser.add_argument('--capture',choices=['HOST_LEASE_KNOWN_FAULT_'+cut+suffix for cut in ('UNNOTIFIED','NOTIFIED','SNAPSHOT') for suffix in ('','_MIXED')],required=True);options=parser.parse_args()
assert not options.tag or re.fullmatch('[A-Z][A-Z0-9_]*',options.tag)
tag='_'+options.tag if options.tag else ''
result_path=w/(options.capture+'_RESULT.json');result=json.loads(result_path.read_text())
wire_profile=options.capture.startswith('HOST_WIRE_PREFIX_')
fault_profile=options.capture.startswith('HOST_LEASE_KNOWN_FAULT_');assert fault_profile and not wire_profile
path=Path(result['receipt']);assert result['status']==('passed attempted-wire capture; not replayed or normal completion' if wire_profile else 'passed preliminary known-fault capture controls; whole-cohort Lean replay NOT RUN' if fault_profile else 'passed') and sha(path)==result['receipt_sha256']
r=json.loads(path.read_text());assert r['state']==('failed' if fault_profile else 'passed');root=Path(r['root'])
if fault_profile:
 f=r['wire_fault' if wire_profile else 'source_entry_fault'];assert f['retired'] and not f['protocol_calls']
 if wire_profile:
  from check_host_wire_prefix_capture import check as wire_check
  wire_check(r)
from check_writer_parent_occurrences import check as check_occurrences
occurrences=check_occurrences(r)
build=w/'cold-native-2';native=json.loads((build/'RESULT.json').read_text());assert native['status']=='passed' and sha(build/'MANIFEST.json')==native['manifest_sha256']
args=r['source']['numeric_arguments'];assert len(args)==8 and all(type(x) is str and x.isascii() and x.isdecimal() for x in args)
realm,p,a,caller,member,principal,scope,sourcecap=map(int,args);assert 0<p<=64 and 0<a<=64 and caller<p and member<a and sourcecap<=512
caps=[];raw=[];names=['source']+['owner'+str(i) for i in range(p)]
for name in names:
 child=r[name];binary=native['binaries']['source' if name=='source' else 'owner']
 assert child['binary']==binary['path'] and child['binary_sha256']==binary['sha256']==sha(Path(child['binary']))
 assert child['state']=='reaped' and Path(child['capture'])==root/name
 if fault_profile:assert child['exit']==-9
 else:assert child['exit']==0 and child['stdout_eof'] is True
 if name!='source':
  ns=child['numeric_arguments'];assert len(ns)==6 and all(type(x) is str and x.isascii() and x.isdecimal() for x in ns)
  ns=list(map(int,ns));assert ns[:5]==[realm,p,a,int(name[5:]),scope] and ns[5]<=64;caps.append(ns[5])
counts={};sequence=r['stream_sequence']
for pos,event in enumerate(sequence):
 name,ordinal=event['endpoint'],event['index'];assert name in names and ordinal==counts.get(name,0)+1;counts[name]=ordinal
 assert event['reply'] is (pos!=0)
 if pos==0:assert event==dict(endpoint='source',index=1,reply=False)
 for direction in ['input']+(['output'] if event['reply'] else []):
  file=root/name/f'{ordinal:03}-{direction}.bin';frame=file.with_name(file.name.replace('.bin','-frame.bin'));data=file.read_bytes()
  assert len(data)<=65536 and frame.read_bytes()==len(data).to_bytes(4,'big')+data
  raw.extend(dict(path=str(f),sha256=sha(f)) for f in (file,frame))
for name,count in counts.items():
 extra=int(wire_profile and f['endpoint']==name and f['cut']!='BEFORE_WRITE')
 assert len(list((root/name).glob('*-input.bin')))==count+extra and len(list((root/name).glob('*-output.bin')))==count-int(name=='source')
 if extra:
  for ending in ('input.bin','input-frame.bin'):
   file=root/name/f'{count+1:03}-{ending}';raw.append(dict(path=str(file),sha256=sha(file)))
if wire_profile:
 for file in (root/'wire-fault').iterdir():
  assert file.name in ('input.bin','observed-reply.bin')
  raw.append(dict(path=str(file),sha256=sha(file)))
dest=rec/('source-entry-journal-trees'+tag);dest.mkdir()
for digest,encoded in r['host_tree_bytes'].items():
 assert re.fullmatch('[0-9a-f]{64}',digest) and type(encoded) is str
 data=bytes.fromhex(encoded);assert hashlib.sha256(data).hexdigest()==digest
 file=dest/(digest+'.bin')
 with file.open('xb') as f:f.write(data)
 raw.append(dict(path=str(file),sha256=sha(file)))
def observed(owner):
 assert owner['failed'] is False
 assert type(owner['credits']) is int and owner['credits']>=0 and type(owner['revision']) is int and owner['revision']>=0 and type(owner['entered']) is bool
 def key(value):
  if value is None:return 'none'
  assert value in r['host_tree_bytes'];return '(some '+json.dumps(value)+')'
 keys=owner['keys']
 if keys is None:shown='none'
 else:
  assert all(len(pair)==2 and all(type(n) is int and n>=0 for n in pair) for pair in keys)
  shown='(some ['+','.join('('+str(x)+','+str(y)+')' for x,y in keys)+'])'
 return '⟨'+','.join([str(owner['credits']),str(owner['revision']),key(owner['image']),shown,key(owner['lease']),str(owner['entered']).lower()])+'⟩'
events=[];active=None;before=None;seen=[];samples=0
for row in r['host_commit_trace']:
 state=row['state'];assert len(state['owners'])==p
 if row['point']=='native_return':
  assert active is None,'native IO before writer return'
  pos=row['native_position'];event=sequence[pos-1]
  if event['endpoint'].startswith('owner'):
   i=int(event['endpoint'][5:]);assert row['endpoint']==event['endpoint'] and row['ordinal']==event['index']
   reply=root/event['endpoint']/f"{event['index']:03}-output.bin";assert sha(reply)==row['reply_sha256']
   before=observed(state['owners'][i]);active=(i,pos);seen.append((event['endpoint'],event['index']))
   events.append(f'.beginWriter {i} {event["index"]} {before}')
 elif row['point']=='writer_return':
  assert active is not None and row['native_position']==active[1]
  event=sequence[active[1]-1];assert sha(root/event['endpoint']/f"{event['index']:03}-output.bin")==row['reply_sha256']
  events.append(f'.endWriter {active[0]} '+observed(state['owners'][active[0]]));active=None;before=None
 elif active is not None:
  assert row['native_position']==active[1] and state['gate'] is True and not state['retired']
  now=observed(state['owners'][active[0]])
  if now!=before:events.append(f'.observe {active[0]} '+now);before=now;samples+=1
assert active is None and seen==[(e['endpoint'],e['index']) for e in sequence if e['endpoint'].startswith('owner')]
runner=rec/('RunSourceEntryWriterJournalReplay'+tag+'.lean')
text='import WriterJournalCaptureReplay\n#eval WriterJournalCaptureReplay.check '+json.dumps(str(root))+' '+json.dumps(str(dest))+f' {realm} {p} {a} {scope}\n  ⟨#['+','.join(map(str,caps))+'],by decide⟩\n  ['+',\n   '.join(events)+']\n'
with runner.open('x') as f:f.write(text)
out=dict(python=dict(executable=sys.executable,optimize=sys.flags.optimize,version=sys.version,cache_prefix=sys.pycache_prefix,dont_write_bytecode=sys.dont_write_bytecode),python_imports=[dict(module=name,origin=os.path.abspath(module.__file__),path=str(Path(module.__file__).resolve()),sha256=sha(Path(module.__file__).resolve())) for name,module in sorted(sys.modules.items()) if getattr(module,"__file__",None) and (Path(os.path.abspath(module.__file__)).is_relative_to(w) or Path(module.__file__).resolve().is_relative_to(w))],at=datetime.datetime.now(datetime.timezone.utc).isoformat(),status='prepared, not executed',receipt_sha256=sha(path),capture_result_sha256=sha(result_path),generator_sha256=sha(Path(__file__)),consumer_sha256=sha(rec/'WriterJournalCaptureReplay.lean'),runner_sha256=sha(runner),occurrences=occurrences,actual_owner_replies=len(seen),observed_store_prefixes=samples,host_trees=len(r['host_tree_bytes']),raw=raw,scope=__doc__)
with (w/('SOURCE_ENTRY_WRITER_JOURNAL_INPUTS'+tag+'.json')).open('x') as f:json.dump(out,f,indent=2);f.write('\n')
print(json.dumps({key:value for key,value in out.items() if key!='raw'}));print('bound_files',len(raw))
