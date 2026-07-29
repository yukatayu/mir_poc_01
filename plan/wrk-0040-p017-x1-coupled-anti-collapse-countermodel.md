# WRK-0040 - P017 X1 coupled anti-collapse countermodel evidence

This is **LAB** evidence for `working/WRK-0040` after its registration cut.
It is a finite negative oracle, not a relation-state model. The four witness
labels are supplied fixture occurrences only: they do not define a Mir request,
identity, value, persistence key, saved object, protocol, or runtime carrier.
The two restore correspondences are supplied relation witnesses, not a restore
function and not cross-load equality.

The six rows are deliberately cumulative. `control` has no violation; `m1`
adds `SEP`; `m2` adds `PHASE`; `m3` adds `ONE`; `m4` adds `AUTH`; and `m5`
adds `OBS`. The five detector names stand only for pre-registered fault labels:
actual-state sharing from equal incidental facts, service/receipt-use collapse,
a second accepted-use witness after the supplied load correspondence, owner
success/provenance without live grounds, and exposure without a complete
separately authorized projection witness. No detector derives any fact about a
Mir execution.

The sole fenced source is extracted to a disposable file before compiling. Its
fixture-local constructors and predicates are not a stable module, schema,
validator input, public API, or runtime interface.

## Outcome Lean source

```lean
namespace P017X1CoupledAntiCollapseLab

-- All constructors below are finite fixture labels, not Mir-language objects.
inductive Fixture where
  | control
  | m1
  | m2
  | m3
  | m4
  | m5

inductive OccurrenceWitness where
  | q0
  | q1
  | r0
  | r1

inductive IncidentalAnnotation where
  | same

def incidentalAnnotation : OccurrenceWitness -> IncidentalAnnotation := fun _ => .same

-- This is a supplied correspondence relation, intentionally not a function.
inductive RestoreWitness : OccurrenceWitness -> OccurrenceWitness -> Prop where
  | q0r0 : RestoreWitness .q0 .r0
  | q1r1 : RestoreWitness .q1 .r1

-- These are seeded proof witnesses, not an execution datum or budget.
inductive AcceptedUseWitness : Fixture -> OccurrenceWitness -> Prop where
  | m3q0 : AcceptedUseWitness .m3 .q0
  | m3r0 : AcceptedUseWitness .m3 .r0
  | m4q0 : AcceptedUseWitness .m4 .q0
  | m4r0 : AcceptedUseWitness .m4 .r0
  | m5q0 : AcceptedUseWitness .m5 .q0
  | m5r0 : AcceptedUseWitness .m5 .r0

-- The five detector predicates classify only the finite fixtures below.
def SEP : Fixture -> Prop
  | .control => False
  | .m1 => True
  | .m2 => True
  | .m3 => True
  | .m4 => True
  | .m5 => True

def PHASE : Fixture -> Prop
  | .control => False
  | .m1 => False
  | .m2 => True
  | .m3 => True
  | .m4 => True
  | .m5 => True

def ONE : Fixture -> Prop
  | .control => False
  | .m1 => False
  | .m2 => False
  | .m3 => True
  | .m4 => True
  | .m5 => True

def AUTH : Fixture -> Prop
  | .control => False
  | .m1 => False
  | .m2 => False
  | .m3 => False
  | .m4 => True
  | .m5 => True

def OBS : Fixture -> Prop
  | .control => False
  | .m1 => False
  | .m2 => False
  | .m3 => False
  | .m4 => False
  | .m5 => True

-- Opaque fixture annotations make AUTH and OBS inspectable without an algorithm.
def OwnerSuccessProvenance : Fixture -> Prop
  | .control => False
  | .m1 => False
  | .m2 => False
  | .m3 => False
  | .m4 => True
  | .m5 => True

def LiveAuthoritativeGrounds : Fixture -> Prop
  | .control => True
  | .m1 => True
  | .m2 => True
  | .m3 => True
  | .m4 => False
  | .m5 => False

def ExposureAnnotation : Fixture -> Prop
  | .control => False
  | .m1 => False
  | .m2 => False
  | .m3 => False
  | .m4 => False
  | .m5 => True

def CompleteProjectionWitness : Fixture -> Prop
  | .control => True
  | .m1 => True
  | .m2 => True
  | .m3 => True
  | .m4 => True
  | .m5 => False

theorem q0_q1_distinct : OccurrenceWitness.q0 ≠ .q1 := by
  intro equal
  cases equal

theorem q0_r0_distinct : OccurrenceWitness.q0 ≠ .r0 := by
  intro equal
  cases equal

theorem equal_incidental_q0_q1 :
    incidentalAnnotation .q0 = incidentalAnnotation .q1 := by
  rfl

theorem supplied_restore_q0_r0 : RestoreWitness .q0 .r0 := by
  exact .q0r0

theorem supplied_restore_q1_r1 : RestoreWitness .q1 .r1 := by
  exact .q1r1

theorem control_has_no_detector :
    ¬ SEP .control /\ ¬ PHASE .control /\ ¬ ONE .control /\
      ¬ AUTH .control /\ ¬ OBS .control := by
  exact ⟨fun impossible => impossible, fun impossible => impossible,
    fun impossible => impossible, fun impossible => impossible,
    fun impossible => impossible⟩

theorem m1_detects_sep :
    SEP .m1 /\ ¬ PHASE .m1 /\ ¬ ONE .m1 /\ ¬ AUTH .m1 /\ ¬ OBS .m1 := by
  exact ⟨True.intro, fun impossible => impossible, fun impossible => impossible,
    fun impossible => impossible, fun impossible => impossible⟩

theorem m2_detects_sep_phase :
    SEP .m2 /\ PHASE .m2 /\ ¬ ONE .m2 /\ ¬ AUTH .m2 /\ ¬ OBS .m2 := by
  exact ⟨True.intro, True.intro, fun impossible => impossible,
    fun impossible => impossible, fun impossible => impossible⟩

theorem m3_detects_sep_phase_one :
    SEP .m3 /\ PHASE .m3 /\ ONE .m3 /\ ¬ AUTH .m3 /\ ¬ OBS .m3 := by
  exact ⟨True.intro, True.intro, True.intro, fun impossible => impossible,
    fun impossible => impossible⟩

theorem m3_has_two_distinct_accepted_use_witnesses :
    AcceptedUseWitness .m3 .q0 /\ AcceptedUseWitness .m3 .r0 /\
      OccurrenceWitness.q0 ≠ .r0 := by
  exact ⟨.m3q0, .m3r0, q0_r0_distinct⟩

theorem m4_detects_sep_phase_one_auth :
    SEP .m4 /\ PHASE .m4 /\ ONE .m4 /\ AUTH .m4 /\ ¬ OBS .m4 := by
  exact ⟨True.intro, True.intro, True.intro, True.intro,
    fun impossible => impossible⟩

theorem m4_has_provenance_without_live_grounds :
    OwnerSuccessProvenance .m4 /\ ¬ LiveAuthoritativeGrounds .m4 := by
  exact ⟨True.intro, fun impossible => impossible⟩

theorem m5_detects_all :
    SEP .m5 /\ PHASE .m5 /\ ONE .m5 /\ AUTH .m5 /\ OBS .m5 := by
  exact ⟨True.intro, True.intro, True.intro, True.intro, True.intro⟩

theorem m5_has_exposure_without_complete_projection :
    ExposureAnnotation .m5 /\ ¬ CompleteProjectionWitness .m5 := by
  exact ⟨True.intro, fun impossible => impossible⟩

end P017X1CoupledAntiCollapseLab
```

## Bound of the result

If this source compiles, it establishes only that this finite detector table is
not the all-clear control and that its seeded rows remain distinguishable. It
does not establish a positive exchange relation, state reachability, delivery,
fairness, retry, exact-once behavior, a receipt rule, a use representation,
saved-state behavior, authority enforcement, observation policy, a theorem/OBL,
or implementation readiness.
