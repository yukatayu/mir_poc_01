# Aborting assignment sequences — LAB candidate

This extends the reviewed fallible assignment evaluator to a finite list of
assignments. Execution stops on the first evaluator failure. The declarative
Safe judgment and Boolean checker are separate, with general equivalence.
Each statement must satisfy expression typing, expression-to-target flow and
program-counter-to-target flow. The tail program counter joins the previous
counter with the expression dependency rank, since successful completion is a
dependency of every later statement.

The general results establish conditional store typing, confinement under a
high program counter, low final-state equivalence and equality of projected
actual outcomes for two runs, and equality after the actual Passive collector.
The completion bit is handled separately: it is released only at an observer
level covering all expression dependencies. No initial-store typing premise is
needed for the two-run evaluator theorem; it remains necessary for the separate
type-preservation theorem. Arithmetic functions are common pure total Option
functions. Presence, authorization, resource failures and elapsed time are not
represented by those functions' mathematical failure contract.

Controls include a nonempty public write before private arithmetic, a rejected
private-overflow-then-public sequence with different real final states, and an
accepted sequence whose unconditionally public completion bit would leak.
A constant assignment to a private target can precede public work, so the
checker does not reject every private-target statement.

There are no loops, statement branches, dynamic address changes, source parser
or full runtime refinement in this module. Unbounded execution, final labels,
current release authority, metadata, initial observer-row sanitation and
physical resource isolation remain obligations. The profile does not combine
the separate GeneralLabels theory with fallible execution. Oracle review of
this frozen cut is pending; this draft records no acceptance.
