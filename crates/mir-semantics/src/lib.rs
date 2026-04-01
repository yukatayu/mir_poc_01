#![doc = r#"
# mir-semantics

Type checking, effect rows, contracts, graph extraction, and semantic normalization.

This crate is currently a **placeholder skeleton** whose purpose is to make subsystem boundaries explicit.

It intentionally contains no production logic yet.
"#]

/// current L2 の parser-free 最小 interpreter が使う failure kind の最小集合。
///
/// これは最終 wire format ではなく、step semantics と oracle API の接続点を示す
/// companion skeleton である。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailureKind {
    ExplicitFailure,
    Reject,
}

/// current L2 の parser-free 最小 interpreter における terminal outcome。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalOutcome {
    Success,
    ExplicitFailure,
    Reject,
}

/// step semantics が node 1 個ぶんの前進で返す最小 control signal。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepControl {
    Continue,
    BubbleFailure(FailureKind),
    Halt(TerminalOutcome),
}

/// predicate evaluation の site。
///
/// 具体的な field 名や host API は未決定のままにし、site の区別だけを残す。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PredicateSite {
    RequestRequire,
    RequestEnsure,
    OptionAdmit,
}

/// predicate oracle が返す current L2 の最小 verdict。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PredicateVerdict {
    Satisfied,
    Unsatisfied,
}

/// effect oracle が返す current L2 の最小 verdict。
///
/// `Commit` は success-side carrier の placeholder であり、具体 layout は未決定。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EffectVerdict<Commit = ()> {
    Success { commit: Commit },
    ExplicitFailure,
}

/// current L2 の predicate oracle skeleton。
///
/// `Input` の concrete shape は spec 側で未固定のため generic のまま残す。
pub trait PredicateOracle<Input> {
    fn eval_predicate(&mut self, input: Input) -> PredicateVerdict;
}

/// current L2 の effect oracle skeleton。
///
/// `Input` と `Commit` の concrete shape は spec 側で未固定のため generic のまま残す。
pub trait EffectOracle<Input, Commit = ()> {
    fn apply_effect(&mut self, input: Input) -> EffectVerdict<Commit>;
}

pub fn crate_name() -> &'static str {
    "mir_semantics"
}
