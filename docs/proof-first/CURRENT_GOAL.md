# W4 physical refinement — W4-C active / B integrated candidate
Updated 2026-09-28T01:49:49.479878+00:00. Same active full-W4 goal, sole main/no subagents. Quota stopping waived for this run; only owner resets. W5+/Plan250-I3-4/Canon acceptance remain outside scope.

PL1/PL2/PL0 S4/S6, theory/checker boundary before implementation. REQ DS01/02/03/04/08 AU01/04/05/08 VF04/05; PT03/11/14 SC04/07 Q18. R02/R03/R04/R05/R09 are prerequisites before their first dependent D use.

One semantic goal: characterize admission of a request against the current retained source/owner context, preserving its full binding and explicit resource/entry conditions. Inputs: checked source-derived operation, observed peer/run identity, current owner/contract/generation and local custody/resource state. Output: scoped acceptance or typed refusal, without manufacturing authorization from a proof or transport identity. Direct consumer: existing owner admission/queue and private QUIC source/Core embedding in D.

Positive: a currently authorized source-derived write with matching context and available resources is admissible. Counterexamples: stale generation after revoke, cross-run/peer substitution, mismatched arguments/code/contract, duplicate consumed permit, alternative entry bypass. Candidate: current-context validation tied to the actual consumption transition. Smallest alternative: reusable preflight-only evidence; reject if a revoke-between-check-and-use witness succeeds. Q18 prepare reservations and commit reauthorization remain separate profiles; no silent merge.

Current consumer counterexample repaired locally: cloned sealed admissions shared
one M9 floor but independent caches, allowing a stale sibling write after revocation.
A retained floor guard now binds full authority facts through actual backend use.
The narrow general Lean adapter (14 lemmas/128 owned declarations) and review are
complete; 11 focused tests and all409 mir-runtime library tests pass. Actual ST/OW1
preflight-only mutants fail. Worker panic after write remains an ambiguous outcome,
not evidence of no write. Git/docs integration is in progress. Next C dependency:
staged I3 request versus current reservation/immutable W3 ticket correspondence;
all-entry/source/peer/custody/resource obligations remain open.

Exit: independent declarative rules versus executable checker, relative completeness for a named finite profile, general Lean preservation/nonvacuity where relied on, typed positives/negatives and entry inventory, mechanism/TCB mapping, neutral Oracle review and local dispositions. No source/runtime implementation before corresponding gate. C source/Core/queue/memory premises precede D; D verifies implemented correspondence. C is not complete.

B: 206 exact source modules (130 existing/76 new), fresh Lean4.29.1 trust0, standard logical axioms only. Model278/preparation235/physical68 commands had whole exit0; bindings9169/9973/34658 verified. Physical15 profiles/53 qualified controls remain private-pipe finite evidence, not QUIC/auth/privacy/recovery. Code/proof cut9d86052d. Final B boundary review retry2/tool13614 exit0, answer b10364efde6de6d2606b83c19ac74f5d849dc7910b7c1b1ab8755f239ff7f86c; local dispositions in W4_CHECK. Oracle did not execute validation or supply independent signature.

Preserve R01–R12 and Report2614. R09 W1 Abort/Address review and M8 effective-label gap before use. R06/R12 legacy local-split/renderer reporting discrepancies must be repaired or explicitly excluded before D consumer. E reconciles119 dispositions, I3 regression and all W4-critical residuals; it cannot defer C/D prerequisites. W4 incomplete, no final-public/alpha claim.
