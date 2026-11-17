# participle

The check `participle`. It reads the spelling of every term of every context under
`crates/gates/vocabulary/gate/src/domain/contexts`, the array that each term binds as
`let spelling: &[&str] = &[...];`, on one line or over several and with words of letters and
digits, and refuses a term whose head, its last word, is a participle: a word ending in `ed` or
`ing` that is none of the nouns the check lists, or one of the irregular participles it lists,
such as `Written` and `Found`. It also refuses a context in which the terms it builds with
`TermText::new` and the heads the check reads are not equally many, because a term whose head
the check cannot read would otherwise pass unjudged.
