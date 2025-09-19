use std::marker::PhantomData;

use crate::{bag::Bag, pattern::Pattern, rule::Rule};

pub struct JoinSingleSingle<B: Bag>(pub PhantomData<B>);

impl<B: Bag> Rule<B> for JoinSingleSingle<B> {
    fn name(&self) -> &'static str {
        "join_single_single"
    }

    fn apply(&self, pattern: &Pattern<B>) -> Pattern<B> {
        match pattern {
            Pattern::Either(box Pattern::Single(a), box Pattern::Single(b)) => {
                return Pattern::Any(vec![Pattern::Single(*a), Pattern::Single(*b)]);
            }
            _ => pattern.clone(),
        }
    }
}
