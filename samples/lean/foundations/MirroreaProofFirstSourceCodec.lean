import MirroreaProofFirstOwnerTreeCodecs
import MirroreaProofFirstOwnerPacketCodec
import MirroreaProofFirstOwnerPayload
import MirroreaProofFirstQualifiedSession

namespace MirroreaProofFirst.SourceCodec
open OwnerCodecTree

-- Syntax transport for the fixed private source executor. This encodes input
-- programs, never an arbitrary populated Session, private binding or Stamp.
-- General structural laws do not select payload resource bounds or grant
-- permission to start/adopt a program.
def encodeExpr : SourceAuthoring.Expr → Tree
  | .integer value => .node 0 [.integer value]
  | .read name => .node 1 [.text name]
  | .add left right => .node 2 [encodeExpr left,encodeExpr right]
  | .mul left right => .node 3 [encodeExpr left,encodeExpr right]

def decodeExpr (tree : Tree) : Option SourceAuthoring.Expr := match tree with
  | .node 0 [.integer value] => some (.integer value)
  | .node 1 [.text name] => some (.read name)
  | .node 2 [left,right] => do return .add (← decodeExpr left) (← decodeExpr right)
  | .node 3 [left,right] => do return .mul (← decodeExpr left) (← decodeExpr right)
  | _ => none
termination_by sizeOf tree

theorem expr_roundtrip (expression : SourceAuthoring.Expr) :
    decodeExpr (encodeExpr expression) = some expression := by
  induction expression <;> simp_all [encodeExpr,decodeExpr]

theorem expr_canonical (tree : Tree) (expression : SourceAuthoring.Expr)
    (decoded : decodeExpr tree = some expression) : encodeExpr expression = tree := by
  induction tree using OwnerCodeCodec.tree_induction generalizing expression with
  | hn n => simp [decodeExpr] at decoded
  | hi i => simp [decodeExpr] at decoded
  | hs s => simp [decodeExpr] at decoded
  | node tag fields ih =>
    unfold decodeExpr at decoded
    split at decoded
    · rename_i value shape
      simp only [Option.some.injEq] at decoded
      subst expression
      exact shape.symm
    · rename_i name shape
      simp only [Option.some.injEq] at decoded
      subst expression
      exact shape.symm
    · rename_i left right shape
      rcases Tree.node.inj shape with ⟨rfl,rfl⟩
      cases hl : decodeExpr left with
      | none => simp [hl] at decoded
      | some a =>
        cases hr : decodeExpr right with
        | none => simp [hl,hr] at decoded
        | some b =>
          simp [hl,hr] at decoded
          subst expression
          simp only [encodeExpr]
          rw [ih left (by simp) a hl,ih right (by simp) b hr]
    · rename_i left right shape
      rcases Tree.node.inj shape with ⟨rfl,rfl⟩
      cases hl : decodeExpr left with
      | none => simp [hl] at decoded
      | some a =>
        cases hr : decodeExpr right with
        | none => simp [hl,hr] at decoded
        | some b =>
          simp [hl,hr] at decoded
          subst expression
          simp only [encodeExpr]
          rw [ih left (by simp) a hl,ih right (by simp) b hr]
    · cases decoded

def expression : Codec SourceAuthoring.Expr := ⟨encodeExpr,decodeExpr,expr_roundtrip,expr_canonical⟩

def capability : Codec FallbackStatic.Capability := iso boolean
  (fun value => match value with | .read => false | .readWrite => true)
  (fun value => if value then .readWrite else .read)
  (by intro value; cases value <;> rfl) (by intro value; cases value <;> rfl)

def edge : Codec FallbackStatic.EdgeDecl := iso (product text (product text boolean))
  (fun value => (value.predecessor,value.successor,value.sameLineage))
  (fun (predecessor,successor,sameLineage) => ⟨predecessor,successor,sameLineage⟩)
  (by intro value; cases value; rfl) (by intro value; rfl)

def optionSyntax : Codec ReferenceSourceData.OptionSyntax := iso
  (product text (product text (product (optional text) (product capability natural))))
  (fun value => (value.name,value.target,value.declaredAccess,value.capability,value.leaseUntil))
  (fun (name,target,access,capability,leaseUntil) => ⟨name,target,access,capability,leaseUntil⟩)
  (by intro value; cases value; rfl) (by intro value; rfl)

def chain : Codec ReferenceSourceData.ChainSyntax := iso
  (product text (product (list optionSyntax) (list (optional edge))))
  (fun value => (value.reader,value.options,value.edges))
  (fun (reader,options,edges) => ⟨reader,options,edges⟩)
  (by intro value; cases value; rfl) (by intro value; rfl)

def plainStatement : Codec SourceAuthoring.Statement := iso
  (sum (product text (product OwnerCodeCodec.definition (optional text))) (sum (product text (product text (product (list natural) (product (optional text) (OwnerTreeCodecs.formula text))))) (sum (product text text) (sum (product text (product text (optional text))) (sum (product text (product text text)) (sum (product text natural) (sum (product text natural) (sum (product text (product boolean expression)) (sum (product text expression) (product text (product text expression)))))))))))
  (fun value => match value with
    | .register name definition previous => (.inl (name,definition,previous))
    | .instantiate name definition places parent dependencies => (.inr (.inl (name,definition,places,parent,dependencies)))
    | .retire out target => (.inr (.inr (.inl (out,target))))
    | .reparent out target parent => (.inr (.inr (.inr (.inl (out,target,parent)))))
    | .replace out target definition => (.inr (.inr (.inr (.inr (.inl (out,target,definition))))))
    | .leave out place => (.inr (.inr (.inr (.inr (.inr (.inl (out,place)))))))
    | .join out place => (.inr (.inr (.inr (.inr (.inr (.inr (.inl (out,place))))))))
    | .localValue name mutable value => (.inr (.inr (.inr (.inr (.inr (.inr (.inr (.inl (name,mutable,value)))))))))
    | .assign name value => (.inr (.inr (.inr (.inr (.inr (.inr (.inr (.inr (.inl (name,value))))))))))
    | .invoke name target argument => (.inr (.inr (.inr (.inr (.inr (.inr (.inr (.inr (.inr (name,target,argument)))))))))))
  (fun value => match value with
    | (.inl (name,definition,previous)) => .register name definition previous
    | (.inr (.inl (name,definition,places,parent,dependencies))) => .instantiate name definition places parent dependencies
    | (.inr (.inr (.inl (out,target)))) => .retire out target
    | (.inr (.inr (.inr (.inl (out,target,parent))))) => .reparent out target parent
    | (.inr (.inr (.inr (.inr (.inl (out,target,definition)))))) => .replace out target definition
    | (.inr (.inr (.inr (.inr (.inr (.inl (out,place))))))) => .leave out place
    | (.inr (.inr (.inr (.inr (.inr (.inr (.inl (out,place)))))))) => .join out place
    | (.inr (.inr (.inr (.inr (.inr (.inr (.inr (.inl (name,mutable,value))))))))) => .localValue name mutable value
    | (.inr (.inr (.inr (.inr (.inr (.inr (.inr (.inr (.inl (name,value)))))))))) => .assign name value
    | (.inr (.inr (.inr (.inr (.inr (.inr (.inr (.inr (.inr (name,target,argument)))))))))) => .invoke name target argument)
  (by intro value; cases value <;> rfl)
  (by intro value; rcases value with ⟨name,definition,previous⟩ | ⟨name,definition,places,parent,dependencies⟩ | ⟨out,target⟩ | ⟨out,target,parent⟩ | ⟨out,target,definition⟩ | ⟨out,place⟩ | ⟨out,place⟩ | ⟨name,mutable,value⟩ | ⟨name,value⟩ | ⟨name,target,argument⟩ <;> rfl)

def statement : Codec ReferenceSourceData.Statement := iso
  (sum plainStatement (sum (product text chain) (sum (product text text) (sum (product text text) (product text text)))))
  (fun value => match value with
    | .plain statement => (.inl statement)
    | .acquire name chain => (.inr (.inl (name,chain)))
    | .alias name target => (.inr (.inr (.inl (name,target))))
    | .reacquire out target => (.inr (.inr (.inr (.inl (out,target)))))
    | .release out target => (.inr (.inr (.inr (.inr (out,target))))))
  (fun value => match value with
    | (.inl statement) => .plain statement
    | (.inr (.inl (name,chain))) => .acquire name chain
    | (.inr (.inr (.inl (name,target)))) => .alias name target
    | (.inr (.inr (.inr (.inl (out,target))))) => .reacquire out target
    | (.inr (.inr (.inr (.inr (out,target))))) => .release out target)
  (by intro value; cases value <;> rfl)
  (by intro value; rcases value with statement | ⟨name,chain⟩ | ⟨name,target⟩ | ⟨out,target⟩ | ⟨out,target⟩ <;> rfl)

def site : Codec ReferenceSourceData.Site := iso (product text natural)
  (fun value => (value.document,value.byteOffset)) (fun (document,offset) => ⟨document,offset⟩)
  (by intro value; cases value; rfl) (by intro value; rfl)
def located : Codec ReferenceSourceData.Located := iso (product site statement)
  (fun value => (value.site,value.statement)) (fun (site,statement) => ⟨site,statement⟩)
  (by intro value; cases value; rfl) (by intro value; rfl)
def raw : Codec QualifiedSource.Raw := iso (product located (optional natural))
  (fun value => (value.source,value.allocation)) (fun (source,allocation) => ⟨source,allocation⟩)
  (by intro value; cases value; rfl) (by intro value; rfl)
def program (p : Nat) : Codec (QualifiedSession.Program p) := iso (product (finite p) (list raw))
  (fun value => (value.caller,value.items)) (fun (caller,items) => ⟨caller,items⟩)
  (by intro value; cases value; rfl) (by intro value; rfl)

theorem program_bytes (value : QualifiedSession.Program p) :
    OwnerPacketCodec.decode (program p) (OwnerPacketCodec.encode (program p) value) = some value :=
  OwnerPacketCodec.roundtrip _ _

-- Structural decoding is not source typing or execution permission. This
-- separate entry retains the exact decoded program beside its checked output.
def checkedProgram (p : Nat) (env : ReferenceSourceData.Environment) (bytes : List UInt8) :
    Option (QualifiedSession.Program p × ReferenceSourceData.Environment) := do
  let source ← OwnerPacketCodec.decode (program p) bytes
  let output ← QualifiedSession.checkProgram env source
  return (source,output)

theorem checked_sound (accepted : checkedProgram p env bytes = some (source,output)) :
    OwnerPacketCodec.decode (program p) bytes = some source ∧
    QualifiedSession.ProgramTyped env source output := by
  unfold checkedProgram at accepted
  cases decoded : OwnerPacketCodec.decode (program p) bytes with
  | none => simp [decoded] at accepted
  | some value =>
      cases checked : QualifiedSession.checkProgram env value with
      | none => simp [decoded,checked] at accepted
      | some result =>
          simp only [decoded,checked,Option.bind_eq_bind,Option.bind_some,Option.pure_def,
            Option.some.injEq,Prod.mk.injEq] at accepted
          obtain ⟨rfl,rfl⟩ := accepted
          exact ⟨rfl,(QualifiedSession.program_exact _ _ _).mp checked⟩

theorem checked_complete {source : QualifiedSession.Program p}
    (typed : QualifiedSession.ProgramTyped env source output) :
    checkedProgram p env (OwnerPacketCodec.encode (program p) source) = some (source,output) := by
  simp [checkedProgram,program_bytes,(QualifiedSession.program_exact _ _ _).mpr typed]

#print axioms expr_roundtrip
#print axioms expr_canonical
#print axioms plainStatement
#print axioms statement
#print axioms program
#print axioms program_bytes
#print axioms checked_sound
#print axioms checked_complete

-- Intern whole documents once, preserving multiple documents and every exact
-- source offset. Packet canonicality is checked separately from source typing;
-- duplicate/unused tables cannot supply alternative bytes for one Program.
structure IndexedRaw where
  document : Nat
  byteOffset : Nat
  statement : ReferenceSourceData.Statement
  allocation : Option Nat
  deriving DecidableEq, Repr
structure Packet (p : Nat) where
  caller : Fin p
  documents : List String
  items : List IndexedRaw
  deriving DecidableEq, Repr

def indexed : Codec IndexedRaw := iso
  (product natural (product natural (product statement (optional natural))))
  (fun value => (value.document,value.byteOffset,value.statement,value.allocation))
  (fun (document,offset,statement,allocation) => ⟨document,offset,statement,allocation⟩)
  (by intro value; cases value; rfl) (by intro value; rfl)
def packet (p : Nat) : Codec (Packet p) := iso
  (product (finite p) (product (list text) (list indexed)))
  (fun value => (value.caller,value.documents,value.items))
  (fun (caller,documents,items) => ⟨caller,documents,items⟩)
  (by intro value; cases value; rfl) (by intro value; rfl)

def indexRaw (documents : List String) (raw : QualifiedSource.Raw) : IndexedRaw :=
  ⟨documents.idxOf raw.source.site.document,raw.source.site.byteOffset,raw.source.statement,raw.allocation⟩
def expandRaw (documents : List String) (raw : IndexedRaw) : Option QualifiedSource.Raw := do
  let document ← documents[raw.document]?
  return ⟨⟨⟨document,raw.byteOffset⟩,raw.statement⟩,raw.allocation⟩
def pack (source : QualifiedSession.Program p) : Packet p :=
  let documents := (source.items.map fun raw => raw.source.site.document).eraseDups
  ⟨source.caller,documents,source.items.map (indexRaw documents)⟩
def expand (value : Packet p) : Option (QualifiedSession.Program p) := do
  return ⟨value.caller,← value.items.mapM (expandRaw value.documents)⟩

theorem lookup_index (documents : List String) (document : String) (member : document ∈ documents) :
    documents[documents.idxOf document]? = some document := by
  induction documents with
  | nil => simp at member
  | cons first rest ih =>
    by_cases same : first = document
    · subst first; simp
    · have inside : document ∈ rest := by simpa [List.mem_cons,Ne.symm same] using member
      simpa [List.idxOf_cons,cond_eq_ite,beq_iff_eq,same] using ih inside

theorem expand_index (raw : QualifiedSource.Raw)
    (member : raw.source.site.document ∈ documents) :
    expandRaw documents (indexRaw documents raw) = some raw := by
  simp only [expandRaw,indexRaw,lookup_index _ _ member,Option.bind_eq_bind,Option.bind_some]
  rfl

theorem expand_items (items : List QualifiedSource.Raw)
    (member : ∀ raw ∈ items, raw.source.site.document ∈ documents) :
    (items.map (indexRaw documents)).mapM (expandRaw documents) = some items := by
  induction items with
  | nil => rfl
  | cons item rest ih =>
    simp only [List.map_cons,List.mapM_cons]
    rw [expand_index item (member item (by simp)),ih (by intro raw h; exact member raw (by simp [h]))]
    rfl

theorem expand_pack (source : QualifiedSession.Program p) : expand (pack source) = some source := by
  have member : ∀ raw ∈ source.items,
      raw.source.site.document ∈ (source.items.map fun raw => raw.source.site.document).eraseDups := by
    intro raw h
    exact List.mem_eraseDups.mpr (List.mem_map.mpr ⟨raw,h,rfl⟩)
  simp only [expand,pack,expand_items _ member,Option.bind_eq_bind,Option.bind_some,Option.pure_def]

def unpack (value : Packet p) : Option (QualifiedSession.Program p) := do
  let source ← expand value
  if pack source = value then some source else none

theorem unpack_pack (source : QualifiedSession.Program p) : unpack (pack source) = some source := by
  simp [unpack,expand_pack]
theorem pack_unpack (accepted : unpack value = some source) : pack source = value := by
  unfold unpack at accepted
  cases expanded : expand value with
  | none => simp [expanded] at accepted
  | some output =>
    simp only [expanded,Option.bind_eq_bind,Option.bind_some] at accepted
    split at accepted
    · rename_i same
      cases accepted
      exact same
    · cases accepted

def compact (p : Nat) : Codec (QualifiedSession.Program p) where
  encode := fun source => (packet p).encode (pack source)
  decode := fun tree => (packet p).decode tree >>= unpack
  roundtrip := by intro source; simp [(packet p).roundtrip,unpack_pack]
  canonical := by
    intro tree source decoded
    cases value : (packet p).decode tree with
    | none => simp [value] at decoded
    | some input =>
      have accepted : unpack input = some source := by simpa [value] using decoded
      rw [pack_unpack accepted]
      exact (packet p).canonical tree input value

def checkedCompact (p : Nat) (env : ReferenceSourceData.Environment) (bytes : List UInt8) :
    Option (QualifiedSession.Program p × ReferenceSourceData.Environment) := do
  let source ← OwnerPacketCodec.decode (compact p) bytes
  let output ← QualifiedSession.checkProgram env source
  return (source,output)

theorem compact_checked_sound (accepted : checkedCompact p env bytes = some (source,output)) :
    bytes = OwnerPacketCodec.encode (compact p) source ∧
    QualifiedSession.ProgramTyped env source output := by
  unfold checkedCompact at accepted
  cases decoded : OwnerPacketCodec.decode (compact p) bytes with
  | none => simp [decoded] at accepted
  | some value =>
    cases checked : QualifiedSession.checkProgram env value with
    | none => simp [decoded,checked] at accepted
    | some result =>
      simp only [decoded,checked,Option.bind_eq_bind,Option.bind_some,Option.pure_def,
        Option.some.injEq,Prod.mk.injEq] at accepted
      obtain ⟨rfl,rfl⟩ := accepted
      exact ⟨OwnerPacketCodec.canonical _ _ _ decoded,(QualifiedSession.program_exact _ _ _).mp checked⟩

theorem compact_checked_complete {source : QualifiedSession.Program p}
    (typed : QualifiedSession.ProgramTyped env source output) :
    checkedCompact p env (OwnerPacketCodec.encode (compact p) source) = some (source,output) := by
  simp [checkedCompact,OwnerPacketCodec.roundtrip,(QualifiedSession.program_exact _ _ _).mpr typed]

#print axioms expand_pack
#print axioms unpack_pack
#print axioms pack_unpack
#print axioms compact
#print axioms compact_checked_sound
#print axioms compact_checked_complete

-- Provisional private source-input budget. Source documents need a separate
-- text bound from an owner request's short names. This is not a CPU/liveness
-- theorem and has no bearing on permission to publish the document.
def compactFits (bytes : List UInt8) : Bool :=
  decide (bytes.length ≤ 65536) && OwnerPayload.digitCheck 128 0 bytes &&
    OwnerPayload.textCheck 4096 bytes

def boundedCompact (p : Nat) (env : ReferenceSourceData.Environment) (bytes : List UInt8) :
    Option (QualifiedSession.Program p × ReferenceSourceData.Environment) := do
  if !compactFits bytes then none else do
    let source ← OwnerPacketCodec.decodeAt (compact p) 256 bytes
    let output ← QualifiedSession.checkProgram env source
    return (source,output)

theorem bounded_sound (accepted : boundedCompact p env bytes = some (source,output)) :
    compactFits bytes = true ∧ bytes = OwnerPacketCodec.encode (compact p) source ∧
    QualifiedSession.ProgramTyped env source output := by
  unfold boundedCompact at accepted
  by_cases fits : compactFits bytes = true
  · simp only [fits,Bool.not_true,Bool.false_eq_true,ite_false] at accepted
    cases decoded : OwnerPacketCodec.decodeAt (compact p) 256 bytes with
    | none => simp [decoded] at accepted
    | some value =>
      cases checked : QualifiedSession.checkProgram env value with
      | none => simp [decoded,checked] at accepted
      | some result =>
        simp only [decoded,checked,Option.bind_eq_bind,Option.bind_some,Option.pure_def,
          Option.some.injEq,Prod.mk.injEq] at accepted
        obtain ⟨rfl,rfl⟩ := accepted
        exact ⟨fits,OwnerPacketCodec.bounded_canonical _ _ _ _ decoded,
          (QualifiedSession.program_exact _ _ _).mp checked⟩
  · simp [fits] at accepted

theorem bounded_complete {source : QualifiedSession.Program p}
    (typed : QualifiedSession.ProgramTyped env source output)
    (fits : compactFits (OwnerPacketCodec.encode (compact p) source) = true)
    (fuel : OwnerTreeBytes.treeCost ((compact p).encode source) ≤ 256) :
    boundedCompact p env (OwnerPacketCodec.encode (compact p) source) = some (source,output) := by
  simp [boundedCompact,fits,OwnerPacketCodec.bounded_roundtrip _ _ _ fuel,
    (QualifiedSession.program_exact _ _ _).mpr typed]

#print axioms bounded_sound
#print axioms bounded_complete
end MirroreaProofFirst.SourceCodec
