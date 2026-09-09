"""Actual local framed journal and state-only replay reference.
Single writer, durable-fsync host contract, no adversarial files. Not a proof of
filesystem/power-failure behavior. The no-replay floor reclaims RAM results;
physical log compaction is a separate, unimplemented storage refinement.
"""
from __future__ import annotations
import hashlib,json,os,struct
from pathlib import Path
MAX_FRAME=1024*1024
class CorruptJournal(ValueError):pass
def encode(record):
    body=json.dumps(record,sort_keys=True,separators=(',',':')).encode()
    if len(body)>MAX_FRAME:raise ValueError('FrameTooLarge')
    return struct.pack('>I',len(body))+body+hashlib.sha256(body).digest()
def decode_prefix(data):
    result=[];offset=0
    while offset<len(data):
        if len(data)-offset<4:break
        n=struct.unpack('>I',data[offset:offset+4])[0]
        if n>MAX_FRAME:raise CorruptJournal('InvalidLength')
        end=offset+4+n+32
        if end>len(data):break
        body=data[offset+4:offset+4+n]
        if hashlib.sha256(body).digest()!=data[offset+4+n:end]:raise CorruptJournal('ChecksumMismatch')
        try:r=json.loads(body)
        except (ValueError,UnicodeError) as exc:raise CorruptJournal('InvalidRecord') from exc
        if not isinstance(r,dict):raise CorruptJournal('InvalidRecord')
        result.append(r);offset=end
    return result,offset
class Journal:
    def __init__(self,path):
        self.path=Path(path)
        if not self.path.exists():
            with self.path.open('xb') as f:f.flush();os.fsync(f.fileno())
            fd=os.open(str(self.path.parent),os.O_RDONLY)
            try:os.fsync(fd)
            finally:os.close(fd)
        data=self.path.read_bytes();self.records,self.valid_bytes=decode_prefix(data)
        if len(data)!=self.valid_bytes:
            with self.path.open('r+b') as f:f.truncate(self.valid_bytes);f.flush();os.fsync(f.fileno())
    def append(self,record):
        data=encode(record)
        with self.path.open('ab') as f:f.write(data);f.flush();os.fsync(f.fileno())
        self.records.append(record);self.valid_bytes+=len(data)
class DurableCell:
    def __init__(self,path,initial=0):
        self.journal=Journal(path);self.value=initial;self.decisions={};self.floor={};self.crashed=False
        for r in self.journal.records:
            if r['kind']=='commit':
                k=(r['incarnation'],r['sequence'])
                if k in self.decisions or k[1]<=self.floor.get(k[0],-1):raise CorruptJournal('DuplicateOrFencedCommittedKey')
                if r['after']!=self.value+r['request']['delta'] or r['result']!=r['after']:raise CorruptJournal('BrokenStateTransition')
                self.value=r['after'];self.decisions[k]=(r['request'],r['result'])
            elif r['kind']=='floor':
                inc,seq=r['incarnation'],r['sequence']
                if seq<self.floor.get(inc,-1):raise CorruptJournal('FloorRollback')
                self.floor[inc]=seq;self.decisions={k:v for k,v in self.decisions.items() if k[0]!=inc or k[1]>seq}
            else:raise CorruptJournal('UnknownRecord')
    def _live(self):
        if self.crashed:raise RuntimeError('ReopenAfterCrash')
    def add(self,incarnation,sequence,delta,crash=None):
        self._live()
        if type(incarnation)is not str or type(sequence)is not int or sequence<0 or type(delta)is not int:raise ValueError('InvalidRequest')
        k=(incarnation,sequence);payload={'delta':delta}
        if sequence<=self.floor.get(incarnation,-1):return {'failure':'ForgottenButFenced'}
        if k in self.decisions:
            old,result=self.decisions[k];return result if old==payload else {'failure':'IdentityCollision'}
        r={'kind':'commit','incarnation':incarnation,'sequence':sequence,'request':payload,'after':self.value+delta,'result':self.value+delta}
        if crash=='before_append':self.crashed=True;return None
        try:self.journal.append(r)
        except Exception:
            self.crashed=True
            raise
        if crash=='after_append':self.crashed=True;return None
        self.value=r['after'];self.decisions[k]=(payload,r['result']);return r['result']
    def forget_prefix(self,incarnation,sequence,settled_through):
        self._live()
        if sequence>settled_through:raise ValueError('UnsettledPrefix')
        if sequence<self.floor.get(incarnation,-1):raise ValueError('FloorRollback')
        try:self.journal.append({'kind':'floor','incarnation':incarnation,'sequence':sequence})
        except Exception:
            self.crashed=True
            raise
        self.floor[incarnation]=sequence;self.decisions={k:v for k,v in self.decisions.items() if k[0]!=incarnation or k[1]>sequence}
