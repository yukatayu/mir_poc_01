"""Materialize symbolic verification conditions and deliberate counterexamples.
Unsat checks are validity checks over arbitrary symbolic values, not bounded
execution enumeration. Structural/trace induction itself is proved in prose.
"""
from pathlib import Path
import json
ROOT = Path(__file__).resolve().parent
records = []

def add(name, title, declarations, assumptions, conclusion, expect='unsat', category='symbolic-lemma'):
    number = len(records) + 1
    stem = f'{number:02}_{name}'
    pre = '(set-option :timeout 10000)\n(set-option :produce-proofs true)\n'
    script = pre + declarations.strip() + '\n'
    for a in assumptions:
        script += f'(assert {a})\n'
    script += f'(assert (not {conclusion}))\n'
    script += '(check-sat-using (then (using-params simplify :som true) smt))\n'
    script += '(get-proof)\n' if expect == 'unsat' else '(get-model)\n'
    (ROOT / 'vcs' / (stem + '.smt2')).write_text(script)
    records.append({'id': stem, 'title': title, 'file': stem + '.smt2',
                    'expected': expect, 'category': category})

INTS = lambda *xs: '\n'.join(f'(declare-const {x} Int)' for x in xs)
BOOLS = lambda *xs: '\n'.join(f'(declare-const {x} Bool)' for x in xs)
ARRAY = lambda name, val='Int': f'(declare-const {name} (Array Int {val}))'

add('store_frame', 'A write preserves all different cells',
    ARRAY('s')+'\n'+INTS('x','y','v'), ['(distinct x y)'],
    '(= (select (store s x v) y) (select s y))')
add('store_readback', 'A write is read back at its own location',
    ARRAY('s')+'\n'+INTS('x','v'), [], '(= (select (store s x v) x) v)')
add('independent_writes', 'Independent constant writes commute',
    ARRAY('s')+'\n'+INTS('x','y','a','b'), ['(distinct x y)'],
    '(= (store (store s x a) y b) (store (store s y b) x a))')
add('independent_rmw', 'Disjoint self-dependent affine RMWs commute',
    ARRAY('s')+'\n'+INTS('x','y','a','b'), ['(distinct x y)'],
    '(= (store (store s x (+ (select s x) a)) y (+ (select (store s x (+ (select s x) a)) y) b)) (store (store s y (+ (select s y) b)) x (+ (select (store s y (+ (select s y) b)) x) a)))')
add('rmw_serial', 'Sequential owner RMW is not two stale blind writes', INTS('hp','a','b'), [],
    '(= (- (- hp a) b) (- hp (+ a b)))')
add('stale_blind_write_mutant', 'Counterexample: requester-side stale value loses a write',
    INTS('hp','a','b'), ['(= hp 100)','(= a 10)','(= b 10)'],
    '(= (- hp b) (- (- hp a) b))', 'sat','counterexample')
add('projection_other_owner', 'One owner update does not alter another owner projection',
    '(declare-const s (Array Int (Array Int Int)))\n'+INTS('l','m','x','v'),
    ['(distinct l m)'], '(= (select (store s l (store (select s l) x v)) m) (select s m))')
add('projection_same_owner', 'Projection and one-owner state update commute',
    '(declare-const s (Array Int (Array Int Int)))\n'+INTS('l','x','v'), [],
    '(= (select (store s l (store (select s l) x v)) l) (store (select s l) x v))')
add('interval_add', 'Integer refinement: interval addition is sound',
    INTS('x','y','lx','ux','ly','uy'),
    ['(<= lx x)','(<= x ux)','(<= ly y)','(<= y uy)'],
    '(and (<= (+ lx ly) (+ x y)) (<= (+ x y) (+ ux uy)))')
add('interval_sub', 'Integer refinement: interval subtraction is sound',
    INTS('x','y','lx','ux','ly','uy'),
    ['(<= lx x)','(<= x ux)','(<= ly y)','(<= y uy)'],
    '(and (<= (- lx uy) (- x y)) (<= (- x y) (- ux ly)))')
add('absolute_value_contract', 'Concrete refined module: absolute value returns a nonnegative integer',
    INTS('x'), [], '(>= (ite (< x 0) (- x) x) 0)')
add('cross_contract_composition', 'Second module safely consumes the exported nonnegative fact',
    INTS('x'), [], '(> (+ (ite (< x 0) (- x) x) 1) 0)')
add('guarded_decrement', 'Actual arithmetic invariant for a guarded decrement',
    INTS('n','k'), ['(>= n 0)','(>= k 0)','(>= n k)'], '(>= (- n k) 0)')
add('guarded_decrement_mutant', 'Counterexample: removing the semantic precondition',
    INTS('n','k'), ['(>= n 0)','(>= k 0)'], '(>= (- n k) 0)', 'sat','counterexample')
add('rank_path_step', 'Induction step: strict rank along a nonempty dependency path',
    INTS('a','b','c'), ['(< a b)','(< b c)'], '(< a c)', category='induction-step')
add('rank_no_cycle', 'A strict rank cannot return to its start', INTS('a'), [], '(not (< a a))')
add('closed_removal', 'Removing a forward-closed dependent set preserves live-parent closure',
    BOOLS('lp','lc','dp','dc'), ['(=> lc lp)','(=> dp dc)'],
    '(=> (and lc (not dc)) (and lp (not dp)))')
add('max_history_mutant', 'Counterexample to the old position <= history maximum wording',
    INTS('old','new','histmax'), ['(= old 1)','(= new 0)','(= histmax 1)',
                               '(<= old histmax)','(<= new histmax)'],
    '(<= old new)', 'sat','counterexample')
add('fallback_search_step', 'Induction step for the first valid suffix search',
    INTS('i','n','tail')+'\n'+BOOLS('valid'),
    ['(<= 0 i)','(< i n)','(<= (+ i 1) tail)','(<= tail n)'],
    '(and (<= i (ite valid i tail)) (<= (ite valid i tail) n))',category='induction-step')
add('fallback_totality_step', 'Induction step: a valid suffix endpoint prevents exhaustion',
    INTS('i','n','tail')+'\n'+BOOLS('valid'),
    ['(<= 0 i)','(< i n)','(<= (+ i 1) tail)','(< tail n)'],
    '(< (ite valid i tail) n)',category='induction-step')
add('fallback_lineage_order', 'New lineage advances lexicographic (epoch,cursor) order',
    INTS('e','e2','p','p2'), ['(or (< e e2) (and (= e e2) (<= p p2)))'],
    '(not (or (> e e2) (and (= e e2) (> p p2))))')
add('chain_flatten_assoc', 'Pure ordered fallback flattening associativity',
    '(declare-const a (Seq Int))\n(declare-const b (Seq Int))\n(declare-const c (Seq Int))', [],
    '(= (seq.++ (seq.++ a b) c) (seq.++ a (seq.++ b c)))')
add('ancestor_not_authority', 'Counterexample: live ancestor alone does not imply valid access',
    BOOLS('ancestorLive','canRead'), ['ancestorLive','(not canRead)'],
    '(and ancestorLive canRead)', 'sat','counterexample')
add('revoked_epoch', 'A retired grant epoch is not valid after a strictly later epoch',
    INTS('g','current'), ['(< g current)'], '(not (= g current))')
add('aba_cell_handle', 'Re-created identifier cannot validate the retired incarnation',
    INTS('old','fresh'), ['(> fresh old)'], '(not (= old fresh))')
add('tombstone_union', 'Restore union cannot erase current revocation',
    ARRAY('old','Bool')+'\n'+ARRAY('current','Bool')+'\n'+INTS('x'),
    ['(select current x)'], '(or (select old x) (select current x))')
add('proof_not_grant', 'A proof result alone cannot pass a separately gated authority check',
    BOOLS('proof','grant','live'), ['proof','(not grant)'], '(not (and proof grant live))')
add('auth_conjunction', 'all_of requires every authority branch',
    BOOLS('a','b'), ['(and a b)'], '(and a b)')
add('auth_or_mutant', 'Counterexample: any_of silently substituted for all_of',
    BOOLS('a','b'), ['a','(not b)'], '(=> (or a b) (and a b))','sat','counterexample')
add('no_double_commit', 'Induction step: seen flag and count preserve at-most-once commit',
    INTS('count')+'\n'+BOOLS('seen','grant','eligible'),
    ['(= seen (= count 1))','(<= 0 count)','(<= count 1)'],
    '(let ((fire (and (not seen) grant eligible))) (and (<= (+ count (ite fire 1 0)) 1) (= (or seen fire) (= (+ count (ite fire 1 0)) 1))))',category='induction-step')
add('reply_loss_preserves_commit', 'Lost reply does not undo an already committed state transition',
    INTS('x','delta')+'\n'+BOOLS('delivered'), [],
    '(= (ite delivered (+ x delta) (+ x delta)) (+ x delta))')
add('post_loss_nonexecution_mutant', 'Counterexample: no reply is not a proof of no execution',
    INTS('before','after')+'\n'+BOOLS('received'),
    ['(= after (+ before 1))','(not received)'], '(= after before)','sat','counterexample')
add('freshness_locality', 'Changes outside a certificate read footprint do not stale its version',
    ARRAY('v')+'\n'+INTS('x','y','newver'), ['(distinct x y)'],
    '(= (select (store v y newver) x) (select v x))')
add('versioned_fact', 'Changing a state cell with a generation increment invalidates the old stamp',
    INTS('old','new'), ['(= new (+ old 1))'], '(not (= old new))')
add('historical_fact_mutant', 'Counterexample: a historical read fact is not a current-state assertion',
    INTS('sample','now'), ['(= sample 1)','(= now 2)'], '(= now sample)','sat','counterexample')
add('contract_replacement', 'Explicit contract weakening/post-strengthening preserves clients',
    BOOLS('pOld','pNew','qOld','qNew'),
    ['(=> pOld pNew)','(=> qNew qOld)','(=> pNew qNew)','pOld'], 'qOld')
add('precondition_strengthening_mutant', 'Counterexample: transparent replacement strengthens the precondition',
    INTS('x'), ['(= x 0)'], '(=> (>= x 0) (> x 0))','sat','counterexample')
add('effect_row_subset_transitive', 'Per-effect containment composes',
    BOOLS('eActual','eNew','eOld'), ['(=> eActual eNew)','(=> eNew eOld)'], '(=> eActual eOld)')
add('value_migration', 'Actual state migration preserves nonnegative invariant',
    INTS('x'), ['(>= x 0)'], '(>= (+ x 1) 0)')
add('patch_outside_frame', 'A migrated cell leaves outside observations unchanged',
    ARRAY('s')+'\n'+INTS('x','y'), ['(distinct x y)'],
    '(= (select (store s x (+ (select s x) 1)) y) (select s y))')
add('pc_join', 'No-write-down forces both branch and value dependencies below target',
    INTS('pc','e','target'), ['(<= 0 pc)','(<= pc 1)','(<= 0 e)','(<= e 1)',
                            '(= target 0)','(<= (ite (> pc e) pc e) target)'],
    '(and (= pc 0) (= e 0))')
add('implicit_flow_mutant', 'Counterexample: checking values but ignoring secret branch control',
    BOOLS('secret1','secret2'), ['secret1','(not secret2)'],
    '(= (ite secret1 1 0) (ite secret2 1 0))','sat','counterexample')
add('low_expression_add', 'Two-run expression step: equal public inputs have equal sums',
    INTS('x1','x2','y1','y2'), ['(= x1 x2)','(= y1 y2)'],
    '(= (+ x1 y1) (+ x2 y2))',category='induction-step')
add('low_expression_conditional', 'Two-run expression step: equal condition selects equal results',
    BOOLS('b1','b2')+'\n'+INTS('t1','t2','f1','f2'),
    ['(= b1 b2)','(= t1 t2)','(= f1 f2)'],
    '(= (ite b1 t1 f1) (ite b2 t2 f2))',category='induction-step')
add('snapshot_receive_send', 'Closed cut with send->receive cannot contain an orphan receive',
    BOOLS('sendIn','recvIn'), ['(=> recvIn sendIn)'], '(not (and recvIn (not sendIn)))')
add('channel_not_orphan_repair', 'Counterexample: channel membership cannot repair orphan receive',
    BOOLS('sendIn','recvIn','channel'), ['(not sendIn)','recvIn','channel'],
    '(=> recvIn sendIn)','sat','counterexample')
add('snapshot_inflight', 'In-flight channel entry is sent but not yet received in the cut',
    BOOLS('sendIn','recvIn'), ['sendIn','(not recvIn)'], '(and sendIn (not recvIn))')
add('replay_prefix_revocation', 'Current revocation remains valid even if historical snapshot lacks it',
    BOOLS('snapshotRevoked','currentRevoked'), ['(not snapshotRevoked)','currentRevoked'],
    '(or snapshotRevoked currentRevoked)')
add('restore_old_grant_mutant', 'Counterexample: restoring only old authorization resurrects a grant',
    BOOLS('snapshotRevoked','currentRevoked'), ['(not snapshotRevoked)','currentRevoked'],
    '(=> currentRevoked snapshotRevoked)','sat','counterexample')
add('rely_stable_read', 'Concrete modal instance: unrelated state update preserves a stable predicate',
    ARRAY('s')+'\n'+INTS('cfg','other','v'), ['(distinct cfg other)','(>= (select s cfg) 0)'],
    '(>= (select (store s other v) cfg) 0)')
add('disjoint_token_allocation', 'A freshly allocated region cannot alias an existing owned region',
    INTS('old','fresh'), ['(distinct old fresh)'], '(not (= old fresh))')
add('linear_token_split_mutant', 'Counterexample: erasing ownership permits duplicate use',
    INTS('uses'), ['(= uses 2)'], '(<= uses 1)','sat','counterexample')
add('while_sum_preservation', 'Annotated loop sum invariant: total+n(n+1)/2 stays constant',
    INTS('total','n','goal'), ['(>= n 1)','(= (+ (* 2 total) (* n (+ n 1))) goal)'],
    '(= (+ (* 2 (+ total n)) (* (- n 1) n)) goal)')
add('while_sum_exit', 'Annotated loop invariant entails its exit postcondition',
    INTS('total','n','goal'), ['(>= n 0)','(not (> n 0))','(= (+ (* 2 total) (* n (+ n 1))) goal)'],
    '(= (* 2 total) goal)')
add('while_variant', 'The selected loop has a decreasing nonnegative variant',
    INTS('n'), ['(> n 0)'], '(and (>= (- n 1) 0) (< (- n 1) n))')
add('i64_refinement_guard', 'Optional implementation bridge: range guard makes integer addition representable',
    INTS('x','y'), ['(<= (- 9223372036854775808) (+ x y))','(<= (+ x y) 9223372036854775807)'],
    '(and (>= (+ x y) (- 9223372036854775808)) (<= (+ x y) 9223372036854775807))')

add('checkpoint_raise', 'Lower-bound propagation stays below every satisfying completion',
    INTS('bs','br','ds','dr','s','r'),
    ['(<= bs ds)','(<= br dr)','(>= br r)','(=> (>= dr r) (>= ds s))'],
    '(<= (ite (> bs s) bs s) ds)',category='induction-step')
add('checkpoint_overflow_contradiction', 'A forced lower bound beyond the checkpoint ceiling excludes all completions',
    INTS('b','d','upper'), ['(<= b d)','(<= d upper)'], '(<= b upper)')
add('checkpoint_Z_cycle', 'The concrete two-message Z cycle excludes the middle A checkpoint',
    INTS('a','b'), ['(= a 1)','(<= 0 b)','(<= b 1)',
                    '(=> (>= a 1) (>= b 1))','(=> (>= b 1) (>= a 2))'], 'false')
add('region_split_disjoint', 'The two pieces of an interior region split are disjoint',
    INTS('base','n','k','x'), ['(> n 0)','(> k 0)','(< k n)'],
    '(not (and (<= base x) (< x (+ base k)) (<= (+ base k) x) (< x (+ base n))))')
add('region_split_positive_cover', 'Interior split preserves positive sizes and the total extent',
    INTS('base','n','k'), ['(> n 0)','(> k 0)','(< k n)'],
    '(and (> (- n k) 0) (= (+ k (- n k)) n) (= (+ (+ base k) (- n k)) (+ base n)))')

(ROOT/'manifest.json').write_text(json.dumps(records,ensure_ascii=False,indent=2)+'\n')
print(f'Wrote {len(records)} obligations, including {sum(r["expected"]=="sat" for r in records)} counterexamples')
