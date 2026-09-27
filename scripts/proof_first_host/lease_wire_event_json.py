"""Finite conversion of generated private capture constructors to runtime data.
No Mir parsing or expected-state generation. Unknown syntax fails closed.
Each parsed constructor is rendered back to the same token sequence before use.
"""
import json,re
TOKEN=re.compile(r'\s*("(?:[^"\\]|\\.)*"|\.[A-Za-z]+|[0-9]+|true|false|none|some|[⟨⟩\[\](),])')
WRITER=('credits','revision','image','keys','lease','entered')
OBS=('bootstrapped','snapshot','initialized','freezes','installs','produced','payment','gate','retired','sourceOrdinal')

def convert(text):
    tokens=[];offset=0
    while offset<len(text):
        match=TOKEN.match(text,offset)
        if not match:raise ValueError(('unsupported artifact syntax',text[offset:offset+40]))
        tokens.append(match[1]);offset=match.end()
    index=0
    def value():
        nonlocal index
        token=tokens[index];index+=1
        if token in ('⟨','[','('):
            close={'⟨':'⟩','[':']','(':')'}[token]
            if token=='(' and tokens[index]=='some':
                index+=1;result=value();assert tokens[index]==')';index+=1;return ('some',result)
            children=[]
            if tokens[index]!=close:
                while True:
                    children.append(value())
                    if tokens[index]!=',':break
                    index+=1
            assert tokens[index]==close;index+=1;return (token,children)
        if token.startswith('"'):json.loads(token)
        elif not(token.isdecimal() or token.startswith('.') or token in ('none','true','false')):raise ValueError(token)
        return ('atom',token)
    def render(node):
        kind,data=node
        if kind=='atom':return [data]
        if kind=='some':return ['(','some']+render(data)+[')']
        result=[kind]
        for i,child in enumerate(data):
            if i:result.append(',')
            result+=render(child)
        return result+[{'⟨':'⟩','[':']','(':')'}[kind]]
    nodes=[]
    while index<len(tokens):nodes.append(value())
    assert [t for n in nodes for t in render(n)]==tokens,'constructor conversion token drift'
    def raw(node):
        kind,data=node
        if kind=='some':return raw(data)
        if kind!='atom':return [raw(x) for x in data]
        if data=='none':return None
        if data.startswith('.'):return data[1:]
        return json.loads(data)
    tag=raw(nodes[0]);args=[raw(n) for n in nodes[1:]]
    names={
      'begin':(),'finish':(),'failedEnd':(),'retired':('writers','snapshot'),'gateEnter':('callId',),'gateRelease':(),
      'bootstrapReturned':(),'cohortCommit':(),'shutdown':(),'rejectedReentry':(),
      'cohortObserve':('row','observation'),'source':('ordinal',),
      'sourceClaim':('ordinal','endpoint','before','claimed','snapshot'),
      'sourceWireClaim':('ordinal','endpoint','before','claimed','snapshot'),
      'wireStop':('source','endpoint','ordinal','cut','writers','snapshot'),
      'sourceStore':('kind','endpoint','memory','snapshot'),
      'owner':('endpoint','ordinal','memory'),'writerObserve':('endpoint','memory'),
      'writerReturn':('endpoint','memory'),'writerStatement':('endpoint','ordinal','occurrence','token','kind','memory')}
    assert tag in names and len(args)==len(names[tag]),'unknown constructor/arity'
    result={'tag':tag,**dict(zip(names[tag],args))}
    for key in ('memory','before','claimed','observation'):
        if key in result:
            columns=OBS if key=='observation' else WRITER
            assert len(result[key])==len(columns),'record arity'
            result[key]=dict(zip(columns,result[key]))
    if 'writers' in result:
        assert all(len(row)==len(WRITER) for row in result['writers'])
        result['writers']=[dict(zip(WRITER,row)) for row in result['writers']]
    return result
