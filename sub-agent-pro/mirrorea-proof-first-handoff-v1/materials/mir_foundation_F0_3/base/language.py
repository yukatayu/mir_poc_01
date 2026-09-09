"""F0.2: checked public-scalar source, CFG code generator, reference semantics.
Python AST is a parser only; input source is NEVER eval/exec'ed. Mathematical
integers, Bool, first-order recursion, while, local structured spawn/join.
The executable source profile is explicitly public-data only. IFC proofs are
in a separate admitted profile, not implicitly claimed for this implementation.
"""
from __future__ import annotations
import ast
from dataclasses import dataclass, field, replace
import hashlib
import json
from typing import Any

class StaticError(ValueError): pass
class DomainFailure(Exception):
    def __init__(self, reason): self.reason=reason; super().__init__(reason)
INT,BOOL='Int','Bool'
def tyval(v):
    if type(v) is int:return INT
    if type(v) is bool:return BOOL
    raise StaticError('NotScalarValue')
def ann(a):
    if isinstance(a,ast.Name) and a.id in ('int','bool'):return {'int':INT,'bool':BOOL}[a.id]
    raise StaticError('ExplicitScalarTypeRequired')
def digest(v):return hashlib.sha256(json.dumps(v,sort_keys=True,separators=(',',':')).encode()).hexdigest()

@dataclass(frozen=True)
class Cell:
    name:str
    owner:str
    ty:str
    initial:Any
@dataclass(frozen=True)
class Signature:
    name:str
    owner:str
    params:tuple[tuple[str,str],...]
    result:str
    kind:str='owner'
    effect_bound:tuple[str,...]=()
    failures:tuple[str,...]=('AuthorityDenied','StaleCode','PreconditionFailed','ArgumentTypeMismatch','ResultTypeMismatch')
    @property
    def identity(self):return digest([self.name,self.owner,self.params,self.result,self.kind,self.effect_bound,self.failures])
@dataclass
class Operation:
    sig:Signature
    body:list[ast.stmt]|None=None
    target:str|None=None
    expression:ast.expr|None=None
    kind:str='named'
    reads:frozenset[str]=frozenset()
    writes:frozenset[str]=frozenset()
    span:int=0
    @property
    def identity(self):
        syntax=ast.dump(ast.Module(body=self.body,type_ignores=[])) if self.body is not None else ast.dump(self.expression) if self.expression else None
        return digest([self.sig.identity,self.target,syntax,self.kind,sorted(self.reads),sorted(self.writes)])
    def evaluate(self,state,args):
        if len(args)!=len(self.sig.params) or any(tyval(v)!=t for v,(_,t) in zip(args,self.sig.params)):raise DomainFailure('ArgumentTypeMismatch')
        env=dict(zip((n for n,_ in self.sig.params),args));local=dict(state)
        if self.kind=='read':return local,local[self.target]
        if self.kind=='assign':local[self.target]=pure_eval(self.expression,env,local);return local,0
        result=run_owner_block(self.body or [],env,local)
        if result is NO_RETURN:raise DomainFailure('MissingReturn')
        if tyval(result)!=self.sig.result:raise DomainFailure('ResultTypeMismatch')
        return local,result
NO_RETURN=object()
def binop(op,a,b):
    if op=='Add':return a+b
    if op=='Sub':return a-b
    if op=='Mult':return a*b
    if op=='Eq':return a==b
    if op=='Lt':return a<b
    if op=='LtE':return a<=b
    raise DomainFailure('UnknownOperator')
def pure_eval(e,env,state):
    if isinstance(e,ast.Constant):return e.value
    if isinstance(e,ast.Name):return env[e.id] if e.id in env else state[e.id]
    if isinstance(e,ast.UnaryOp):
        v=pure_eval(e.operand,env,state);return -v if isinstance(e.op,ast.USub) else not v
    if isinstance(e,ast.BinOp):return binop(type(e.op).__name__,pure_eval(e.left,env,state),pure_eval(e.right,env,state))
    if isinstance(e,ast.Compare):return binop(type(e.ops[0]).__name__,pure_eval(e.left,env,state),pure_eval(e.comparators[0],env,state))
    if isinstance(e,ast.IfExp):return pure_eval(e.body if pure_eval(e.test,env,state) else e.orelse,env,state)
    raise DomainFailure('UnsupportedPureExpression')
def run_owner_block(body,env,state):
    for s in body:
        if isinstance(s,ast.Return):return pure_eval(s.value,env,state)
        if isinstance(s,(ast.Assign,ast.AnnAssign)):
            n=(s.targets[0] if isinstance(s,ast.Assign) else s.target).id
            (state if n in state else env)[n]=pure_eval(s.value,env,state)
        elif isinstance(s,ast.If):
            out=run_owner_block(s.body if pure_eval(s.test,env,state) else s.orelse,env,state)
            if out is not NO_RETURN:return out
        elif isinstance(s,ast.Assert):
            if not pure_eval(s.test,env,state):raise DomainFailure('PreconditionFailed')
        elif not isinstance(s,ast.Pass):raise DomainFailure('UnsupportedOwnerStatement')
    return NO_RETURN
@dataclass
class Function:
    sig:Signature
    node:ast.FunctionDef
    variables:dict[str,str]=field(default_factory=dict)
    code:list[tuple]=field(default_factory=list)
    source_map:dict[int,int]=field(default_factory=dict)
@dataclass
class Program:
    name:str
    source:str
    cells:dict[str,Cell]
    functions:dict[str,Function]
    operations:dict[str,Operation]
    imports:dict[str,Signature]
    generated_edges:list[dict]=field(default_factory=list)
    @property
    def identity(self):return digest([self.name,self.source,sorted((n,s.identity) for n,s in self.imports.items())])
def returns(body):
    for s in body:
        if isinstance(s,ast.Return):return True
        if isinstance(s,ast.If) and returns(s.body) and returns(s.orelse):return True
    return False

def parse_check_compile(source,name='Example',interfaces=None):
    interfaces={} if interfaces is None else dict(interfaces)
    root=ast.parse(source);cells={};functions={};imports={}
    for s in root.body:
        if isinstance(s,ast.ImportFrom):
            if s.level or s.module is None:raise StaticError('UnsupportedImport')
            for a in s.names:
                q=s.module+'.'+a.name;n=a.asname or a.name
                if q not in interfaces:raise StaticError('MissingInterface:'+q)
                if n in cells or n in functions or n in imports:raise StaticError('DuplicateName')
                if interfaces[q].kind!='owner':raise StaticError('OnlyOperationInterfacesImportable')
                imports[n]=interfaces[q]
        elif isinstance(s,ast.AnnAssign) and isinstance(s.target,ast.Name):
            a=s.annotation
            if not isinstance(a,ast.Subscript) or not isinstance(a.value,ast.Name) or a.value.id!='Cell' or not isinstance(a.slice,ast.Tuple) or len(a.slice.elts)!=2:raise StaticError('CellAnnotationRequired')
            own,t=a.slice.elts
            if not isinstance(own,ast.Constant) or type(own.value) is not str:raise StaticError('DeclaredOwnerRequired')
            if not isinstance(s.value,ast.Constant):raise StaticError('LiteralInitializerRequired')
            c=Cell(s.target.id,own.value,ann(t),s.value.value)
            if c.ty!=tyval(c.initial):raise StaticError('InitializerTypeMismatch')
            if c.name in cells or c.name in functions or c.name in imports or c.name.startswith('__'):raise StaticError('DuplicateOrReservedName')
            cells[c.name]=c
        elif isinstance(s,ast.FunctionDef):
            if len(s.decorator_list)!=1:raise StaticError('OnePlacementAnnotationRequired')
            d=s.decorator_list[0]
            if not isinstance(d,ast.Call) or not isinstance(d.func,ast.Name) or d.func.id not in ('owner','task') or len(d.args)!=1 or d.keywords or not isinstance(d.args[0],ast.Constant) or type(d.args[0].value)is not str:raise StaticError('PlacementAnnotation')
            if s.args.posonlyargs or s.args.kwonlyargs or s.args.vararg or s.args.kwarg or s.args.defaults:raise StaticError('FirstOrderPositionalSignatureRequired')
            if s.name in cells or s.name in functions or s.name in imports or s.name in ('spawn','join') or s.name.startswith('__'):raise StaticError('DuplicateOrReservedName')
            ps=tuple((a.arg,ann(a.annotation)) for a in s.args.args)
            if len({n for n,_ in ps})!=len(ps):raise StaticError('DuplicateParameter')
            if any(n.startswith('__') for n,_ in ps):raise StaticError('ReservedParameter')
            functions[s.name]=Function(Signature(name+'.'+s.name,d.args[0].value,ps,ann(s.returns),d.func.id),s)
        else:raise StaticError('UnsupportedTopLevel')
    names=set(cells)|set(functions)|set(imports)|{'spawn','join'}
    if any(n in names for f in functions.values() for n,_ in f.sig.params):raise StaticError('ParameterShadowsGlobal')
    p=Program(name,source,cells,functions,{},imports)
    for f in functions.values():
        ck=Checker(p,f);ck.block(f.node.body,dict(f.sig.params),{n for n,_ in f.sig.params})
        if not returns(f.node.body):raise StaticError('AllPathsMustReturn')
        f.variables=ck.variables
        if f.sig.kind=='owner':
            f.sig=replace(f.sig,effect_bound=tuple(sorted(['read:'+n for n in ck.reads]+['write:'+n for n in ck.writes])))
            p.operations[f.sig.name]=Operation(f.sig,f.node.body,reads=frozenset(ck.reads),writes=frozenset(ck.writes),span=f.node.lineno)
    for f in functions.values():
        if f.sig.kind=='task':Compiler(p,f).compile()
    return p

class Checker:
    def __init__(self,p,f):self.p=p;self.f=f;self.variables=dict(f.sig.params);self.reads=set();self.writes=set()
    def expression(self,e,env,defined,pure=False):
        if isinstance(e,ast.Constant):return tyval(e.value)
        if isinstance(e,ast.Name):
            if e.id in env:
                if e.id not in defined:raise StaticError('PossiblyUnbound:'+e.id)
                return env[e.id]
            if e.id in self.p.cells:
                self.reads.add(e.id)
                if pure and self.p.cells[e.id].owner!=self.f.sig.owner:raise StaticError('CrossOwnerReadNeedsSnapshot')
                return self.p.cells[e.id].ty
            raise StaticError('UnknownValue:'+e.id)
        if isinstance(e,ast.UnaryOp) and isinstance(e.op,(ast.USub,ast.Not)):
            t=self.expression(e.operand,env,defined,pure);want=INT if isinstance(e.op,ast.USub) else BOOL
            if t!=want:raise StaticError('UnaryType')
            return want
        if isinstance(e,ast.BinOp) and isinstance(e.op,(ast.Add,ast.Sub,ast.Mult)):
            if self.expression(e.left,env,defined,pure)!=INT or self.expression(e.right,env,defined,pure)!=INT:raise StaticError('ArithmeticType')
            return INT
        if isinstance(e,ast.Compare) and len(e.ops)==1 and isinstance(e.ops[0],(ast.Eq,ast.Lt,ast.LtE)):
            a=self.expression(e.left,env,defined,pure);b=self.expression(e.comparators[0],env,defined,pure)
            if a!=b or a not in (INT,BOOL) or (not isinstance(e.ops[0],ast.Eq) and a!=INT):raise StaticError('ComparisonType')
            return BOOL
        if isinstance(e,ast.IfExp):
            if self.expression(e.test,env,defined,pure)!=BOOL:raise StaticError('ConditionType')
            a=self.expression(e.body,env,defined,pure);b=self.expression(e.orelse,env,defined,pure)
            if a!=b:raise StaticError('BranchType')
            return a
        if isinstance(e,ast.Call) and isinstance(e.func,ast.Name) and not e.keywords and not pure:
            n=e.func.id
            if n=='join':
                if len(e.args)!=1:raise StaticError('JoinArity')
                t=self.expression(e.args[0],env,defined)
                if not t.startswith('Future['):raise StaticError('JoinType')
                return t[7:-1]
            spawning=n=='spawn'
            if spawning:
                if not e.args or not isinstance(e.args[0],ast.Name):raise StaticError('NamedSpawnRequired')
                n=e.args[0].id;args=e.args[1:]
            else:args=e.args
            sig=self.p.functions[n].sig if n in self.p.functions else self.p.imports.get(n)
            if sig is None:raise StaticError('UnknownFunction:'+n)
            if spawning and (sig.kind!='task' or sig.owner!=self.f.sig.owner):raise StaticError('SpawnIsLocalActivityOnly')
            if sig.kind=='task' and sig.owner!=self.f.sig.owner:raise StaticError('RemoteTaskNeedsExplicitParticipation')
            if len(args)!=len(sig.params):raise StaticError('CallArity')
            if [self.expression(a,env,defined) for a in args]!=[t for _,t in sig.params]:raise StaticError('CallTypes')
            return 'Future['+sig.result+']' if spawning else sig.result
        raise StaticError('UnsupportedExpression:'+type(e).__name__)
    def block(self,body,env,defined):
        env=dict(env);defined=set(defined);owner=self.f.sig.kind=='owner'
        for s in body:
            if isinstance(s,(ast.Assign,ast.AnnAssign)):
                if isinstance(s,ast.Assign) and len(s.targets)!=1:raise StaticError('SingleAssignment')
                dst=s.targets[0] if isinstance(s,ast.Assign) else s.target
                if not isinstance(dst,ast.Name) or dst.id.startswith('__'):raise StaticError('LocalOrCellTarget')
                n=dst.id
                if n in self.p.functions or n in self.p.imports or n in ('spawn','join'):raise StaticError('FunctionIsNotVariable')
                if s.value is None:raise StaticError('InitializerRequired')
                if n in self.p.cells:
                    if isinstance(s,ast.AnnAssign):raise StaticError('CannotRedeclareCell')
                    if owner:
                        t=self.expression(s.value,env,defined,True)
                        if self.p.cells[n].owner!=self.f.sig.owner:raise StaticError('CrossOwnerWrite')
                    else:
                        old=self.f;self.f=Function(Signature('',self.p.cells[n].owner,(),INT),old.node)
                        try:t=self.expression(s.value,env,defined,True)
                        finally:self.f=old
                    if t!=self.p.cells[n].ty:raise StaticError('CellAssignmentType')
                    self.writes.add(n)
                else:
                    t=self.expression(s.value,env,defined,owner)
                    if isinstance(s,ast.AnnAssign) and ann(s.annotation)!=t:raise StaticError('AnnotatedTypeMismatch')
                    if n in env and env[n]!=t:raise StaticError('LocalTypeChange')
                    if n in self.variables and self.variables[n]!=t:raise StaticError('InconsistentLocalType')
                    env[n]=t;self.variables[n]=t;defined.add(n)
            elif isinstance(s,ast.Return):
                if s.value is None or self.expression(s.value,env,defined,owner)!=self.f.sig.result:raise StaticError('ReturnType')
                return env,defined
            elif isinstance(s,ast.If):
                if self.expression(s.test,env,defined,owner)!=BOOL:raise StaticError('ConditionType')
                a,da=self.block(s.body,env,defined);b,db=self.block(s.orelse,env,defined)
                if returns(s.body):env,defined=b,db
                elif returns(s.orelse):env,defined=a,da
                else:
                    common=da&db
                    if any(a[n]!=b[n] for n in common):raise StaticError('BranchEnvironmentMismatch')
                    env={n:a[n] for n in common};defined=common
            elif isinstance(s,ast.While) and not owner and not s.orelse:
                if self.expression(s.test,env,defined)!=BOOL:raise StaticError('ConditionType')
                self.block(s.body,env,defined)
            elif isinstance(s,ast.Assert) and owner and s.msg is None:
                if self.expression(s.test,env,defined,True)!=BOOL:raise StaticError('AssertionType')
            elif isinstance(s,ast.Expr) and isinstance(s.value,ast.Call) and not owner:self.expression(s.value,env,defined)
            elif not isinstance(s,ast.Pass):raise StaticError('UnsupportedStatement:'+type(s).__name__)
        return env,defined

class Compiler:
    def __init__(self,p,f):self.p=p;self.f=f;self.code=[];self.spans={};self.temp=0
    def emit(self,*i,span=0):k=len(self.code);self.code.append(tuple(i));self.spans[k]=span;return k
    def reg(self):self.temp+=1;return '__r'+str(self.temp)
    def request(self,dst,op,args,line):
        sig=self.p.operations[op].sig if op in self.p.operations else next(s for s in self.p.imports.values() if s.name==op)
        i=self.emit('request',dst,op,tuple(args),span=line)
        if self.f.sig.owner!=sig.owner:self.p.generated_edges.append({'site':[self.f.sig.name,i],'source_line':line,'operation':op,'from':self.f.sig.owner,'to':sig.owner,'contract':sig.identity,'directions':['request','outcome']})
    def expr(self,e):
        d=self.reg();line=getattr(e,'lineno',0)
        if isinstance(e,ast.Constant):self.emit('const',d,e.value,span=line)
        elif isinstance(e,ast.Name):
            if e.id in self.p.cells:
                n=self.p.name+'.__read_'+e.id;c=self.p.cells[e.id]
                self.p.operations.setdefault(n,Operation(Signature(n,c.owner,(),c.ty,effect_bound=('read:'+e.id,)),target=e.id,kind='read',reads=frozenset({e.id}),span=line))
                self.request(d,n,[],line)
            else:self.emit('move',d,e.id,span=line)
        elif isinstance(e,ast.UnaryOp):self.emit('unary',d,type(e.op).__name__,self.expr(e.operand),span=line)
        elif isinstance(e,ast.BinOp):
            a=self.expr(e.left);b=self.expr(e.right);self.emit('binary',d,type(e.op).__name__,a,b,span=line)
        elif isinstance(e,ast.Compare):
            a=self.expr(e.left);b=self.expr(e.comparators[0]);self.emit('binary',d,type(e.ops[0]).__name__,a,b,span=line)
        elif isinstance(e,ast.IfExp):
            c=self.expr(e.test);j=self.emit('branch',c,-1,-1,span=line);yes=len(self.code)
            a=self.expr(e.body);self.emit('move',d,a,span=line);jump=self.emit('jump',-1);no=len(self.code)
            b=self.expr(e.orelse);self.emit('move',d,b,span=line)
            self.code[j]=('branch',c,yes,no);self.code[jump]=('jump',len(self.code))
        elif isinstance(e,ast.Call):
            n=e.func.id
            if n=='join':self.emit('join',d,self.expr(e.args[0]),span=line)
            else:
                spawn=n=='spawn';target=e.args[0].id if spawn else n;args=[self.expr(a) for a in (e.args[1:] if spawn else e.args)]
                sig=self.p.functions[target].sig if target in self.p.functions else self.p.imports[target]
                if sig.kind=='owner':self.request(d,sig.name,args,line)
                else:self.emit('spawn' if spawn else 'call',d,sig.name,tuple(args),span=line)
        else:raise StaticError('CompilerUnsupported')
        return d
    def block(self,body):
        for s in body:
            line=s.lineno
            if isinstance(s,(ast.Assign,ast.AnnAssign)):
                n=(s.targets[0] if isinstance(s,ast.Assign) else s.target).id
                if n in self.p.cells:
                    op=self.f.sig.name+'.__write_'+str(line)+'_'+str(s.col_offset);c=self.p.cells[n]
                    names=sorted({a.id for a in ast.walk(s.value) if isinstance(a,ast.Name)}-set(self.p.cells))
                    ps=tuple((n,self.f.variables[n]) for n in names)
                    rd=frozenset(a.id for a in ast.walk(s.value) if isinstance(a,ast.Name) and a.id in self.p.cells)
                    self.p.operations[op]=Operation(Signature(op,c.owner,ps,INT,effect_bound=tuple(sorted(['read:'+x for x in rd]+['write:'+n]))),target=n,expression=s.value,kind='assign',reads=rd,writes=frozenset({n}),span=line)
                    self.request(self.reg(),op,names,line)
                else:self.emit('move',n,self.expr(s.value),span=line)
            elif isinstance(s,ast.Return):
                self.emit('return',self.expr(s.value),span=line);break
            elif isinstance(s,ast.If):
                c=self.expr(s.test);j=self.emit('branch',c,-1,-1,span=line);yes=len(self.code);self.block(s.body)
                jump=self.emit('jump',-1);no=len(self.code);self.block(s.orelse)
                self.code[j]=('branch',c,yes,no);self.code[jump]=('jump',len(self.code))
            elif isinstance(s,ast.While):
                head=len(self.code);c=self.expr(s.test);j=self.emit('branch',c,-1,-1,span=line);yes=len(self.code)
                self.block(s.body);self.emit('jump',head);self.code[j]=('branch',c,yes,len(self.code))
            elif isinstance(s,ast.Expr):self.expr(s.value)
            elif isinstance(s,ast.Pass):self.emit('nop',span=line)
            else:raise StaticError('CompilerUnsupportedStatement')
    def compile(self):self.block(self.f.node.body);self.f.code=self.code;self.f.source_map=self.spans

def interpret_activity(p,name,args,operations,state,budget=100000):
    """Independent AST control interpreter, no CFG, sequential differential oracle.
    The owner operation meaning is shared; this does not independently verify it.
    """
    fuel=[budget]
    def tick():
        fuel[0]-=1
        if fuel[0]<0:raise DomainFailure('ExecutionBudget')
    def expr(e,env):
        tick()
        if isinstance(e,ast.Call):
            n=e.func.id
            if n in ('spawn','join'):raise DomainFailure('ReferenceIsSequential')
            sig=p.functions[n].sig if n in p.functions else p.imports[n];vs=tuple(expr(a,env) for a in e.args)
            if sig.kind=='task':return call(n,vs)
            new,v=operations[sig.name].evaluate(state,vs);state.clear();state.update(new);return v
        if isinstance(e,ast.BinOp):return binop(type(e.op).__name__,expr(e.left,env),expr(e.right,env))
        if isinstance(e,ast.Compare):return binop(type(e.ops[0]).__name__,expr(e.left,env),expr(e.comparators[0],env))
        if isinstance(e,ast.UnaryOp):
            v=expr(e.operand,env);return -v if isinstance(e.op,ast.USub) else not v
        if isinstance(e,ast.IfExp):return expr(e.body if expr(e.test,env) else e.orelse,env)
        return pure_eval(e,env,state)
    def block(body,env):
        for s in body:
            tick()
            if isinstance(s,ast.Return):return expr(s.value,env)
            if isinstance(s,(ast.Assign,ast.AnnAssign)):
                n=(s.targets[0] if isinstance(s,ast.Assign) else s.target).id
                if n in p.cells:state[n]=pure_eval(s.value,env,state)
                else:env[n]=expr(s.value,env)
            elif isinstance(s,ast.If):
                o=block(s.body if expr(s.test,env) else s.orelse,env)
                if o is not NO_RETURN:return o
            elif isinstance(s,ast.While):
                while expr(s.test,env):
                    o=block(s.body,env)
                    if o is not NO_RETURN:return o
            elif isinstance(s,ast.Expr):expr(s.value,env)
        return NO_RETURN
    def call(n,vs):return block(p.functions[n].node.body,dict(zip((n for n,_ in p.functions[n].sig.params),vs)))
    return call(name,args)
