# Specifications

`Specification::decide_acceptance` decides the `CandidateDecision` of a candidate, an `Acceptance` or a
`Rejection`. Two decisions combine through the operators of Rust: `&` accepts where both
accept, `|` where either accepts, and `!` turns a decision around.

Specifications combine through the same operators. `Conjunction::from((left, right))` accepts
what both of its specifications accept, `Disjunction::from((left, right))` what either accepts,
and `Negation::from(inner)` turns the decision of its inner specification around. A
conjunction, a disjunction and a negation implement `BitAnd`, `BitOr` and `Not`, so
`conjunction & specification`, `conjunction | specification` and `!conjunction` yield the next
`Conjunction`, `Disjunction` and `Negation`. A specification from another crate implements the operators itself where it
needs to combine, because the orphan rule of Rust does not allow implementing them for every
specification at once.
