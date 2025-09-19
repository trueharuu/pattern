use std::marker::PhantomData;

use crate::{bag::Bag, pattern::Pattern, rule::Rule};

pub struct JoinSingleAny<B: Bag>(pub PhantomData<B>);

impl<B: Bag> Rule<B> for JoinSingleAny<B> {
    fn name(&self) -> &'static str {
        "join_single_any"
    }

    fn apply(&self, pattern: &Pattern<B>) -> Pattern<B> {
        match pattern {
            Pattern::Either(box Pattern::Single(c), box Pattern::Any(a))
            | Pattern::Either(box Pattern::Any(a), box Pattern::Single(c)) => {
                return Pattern::Any([vec![Pattern::Single(*c)], a.clone()].concat());
            }
            _ => pattern.clone(),
        }
    }
}
